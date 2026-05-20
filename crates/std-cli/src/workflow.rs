use crate::CliError;
use clap::Subcommand;
use std::{fs, path::PathBuf};
use std_core::{AiPlanner, StdCore};
use std_orchestration::{
    add_workflow_step, append_workflow_execution, list_workflows, load_workflow,
    move_workflow_step, read_workflow_executions, remove_workflow_step, resolve_workflow_input,
    update_workflow_step, workflow_from_plan, write_workflow_markdown, StepType, Workflow,
    WorkflowExecutionOptions, WorkflowExecutor, WorkflowStep,
};
use std_types::{Action, ActionExecution, ActionExecutionStatus, ActionType, RegistryEntry};
use uuid::Uuid;

#[derive(Debug, Subcommand)]
pub enum WorkflowCommand {
    New {
        name: String,
        #[arg(short, long, default_value = "")]
        description: String,
    },
    List,
    Check {
        workflow: String,
    },
    History {
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    Trace {
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    Step {
        #[command(subcommand)]
        command: WorkflowStepCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum WorkflowStepCommand {
    Add {
        workflow: String,
        name: String,
        #[arg(long = "type", default_value = "action")]
        step_type: String,
        #[arg(long, default_value = "{}")]
        json: String,
    },
    Update {
        workflow: String,
        index: usize,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        json: Option<String>,
    },
    Remove {
        workflow: String,
        index: usize,
    },
    Move {
        workflow: String,
        from: usize,
        to: usize,
    },
}

pub(crate) fn handle_workflow(
    core: &StdCore,
    command: WorkflowCommand,
) -> Result<String, CliError> {
    match command {
        WorkflowCommand::New { name, description } => create_workflow(core, &name, &description),
        WorkflowCommand::List => list_saved_workflows(core),
        WorkflowCommand::Check { workflow } => check_workflow(core, &workflow),
        WorkflowCommand::History { limit } => workflow_history(core, limit),
        WorkflowCommand::Trace { limit } => workflow_trace(core, limit),
        WorkflowCommand::Step { command } => handle_workflow_step(core, command),
    }
}

pub(crate) fn handle_plan(
    core: &StdCore,
    goal: &str,
    workflow: bool,
    save: bool,
) -> Result<String, CliError> {
    let plan = AiPlanner::plan(core, goal)?;
    if save {
        save_planned_workflow(core, &workflow_from_plan(&plan))
    } else if workflow {
        Ok(serde_json::to_string_pretty(&workflow_from_plan(&plan))?)
    } else {
        Ok(serde_json::to_string_pretty(&plan)?)
    }
}

fn handle_workflow_step(core: &StdCore, command: WorkflowStepCommand) -> Result<String, CliError> {
    match command {
        WorkflowStepCommand::Add {
            workflow,
            name,
            step_type,
            json,
        } => add_step_to_workflow(core, &workflow, &name, &step_type, &json),
        WorkflowStepCommand::Update {
            workflow,
            index,
            name,
            json,
        } => update_step_in_workflow(core, &workflow, index, name.as_deref(), json.as_deref()),
        WorkflowStepCommand::Remove { workflow, index } => {
            remove_step_from_workflow(core, &workflow, index)
        }
        WorkflowStepCommand::Move { workflow, from, to } => {
            move_step_in_workflow(core, &workflow, from, to)
        }
    }
}

pub(crate) fn run_workflow(
    core: &StdCore,
    workflow_name: &str,
    allow_external: bool,
) -> Result<String, CliError> {
    core.ensure_storage()?;
    let workflow = match resolve_workflow_input(&core.config, workflow_name) {
        Some(path) => load_workflow(&path)?,
        None => built_in_smoke_workflow(core, workflow_name)?,
    };

    let executor = WorkflowExecutor::with_options(
        core.clone(),
        WorkflowExecutionOptions {
            allow_external_runner: allow_external,
        },
    );
    let execution = executor.execute_capture(&workflow)?;
    append_workflow_execution(&core.config.history_dir(), &execution)?;
    Ok(serde_json::to_string_pretty(&execution)?)
}

pub(crate) fn trigger_workflow_action(
    core: &StdCore,
    action_name: &str,
    workflow_path: Option<&String>,
    allow_external: bool,
) -> Result<ActionExecution, CliError> {
    core.ensure_storage()?;
    let workflow_name = action_name
        .strip_prefix("Run Workflow: ")
        .unwrap_or(action_name)
        .to_string();
    let path = workflow_path
        .map(PathBuf::from)
        .filter(|path| path.exists())
        .or_else(|| resolve_workflow_input(&core.config, &workflow_name))
        .ok_or(std_orchestration::OrchestrationError::WorkflowNotFound)?;
    let workflow = load_workflow(&path)?;
    let executor = WorkflowExecutor::with_options(
        core.clone(),
        WorkflowExecutionOptions {
            allow_external_runner: allow_external,
        },
    );
    let execution = executor.execute_capture(&workflow)?;
    append_workflow_execution(&core.config.history_dir(), &execution)?;
    Ok(ActionExecution {
        action_id: workflow.id,
        action_name: format!("Run Workflow: {}", workflow.name),
        status: match execution.status {
            std_orchestration::ExecutionStatus::Completed => ActionExecutionStatus::Completed,
            std_orchestration::ExecutionStatus::Failed => ActionExecutionStatus::Failed,
            _ => ActionExecutionStatus::NeedsExternalRunner,
        },
        message: format!(
            "workflow executed: {} steps, status {:?}",
            execution.results.len(),
            execution.status
        ),
        output: Some(serde_json::to_value(execution)?),
        created_at: chrono::Utc::now(),
    })
}

fn create_workflow(core: &StdCore, name: &str, description: &str) -> Result<String, CliError> {
    core.ensure_storage()?;
    let description = if description.trim().is_empty() {
        format!("{name} workflow")
    } else {
        description.to_string()
    };
    let path = write_workflow_markdown(&core.config.workflows_dir(), name, &description)?;
    core.register_local_content_actions()?;
    Ok(format!("workflow created\npath={}", path.display()))
}

fn list_saved_workflows(core: &StdCore) -> Result<String, CliError> {
    core.ensure_storage()?;
    let workflows = list_workflows(&core.config.workflows_dir())?;
    Ok(workflows
        .into_iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join("\n"))
}

fn add_step_to_workflow(
    core: &StdCore,
    workflow_name: &str,
    step_name: &str,
    step_type: &str,
    json: &str,
) -> Result<String, CliError> {
    core.ensure_storage()?;
    let path = resolve_workflow_input(&core.config, workflow_name)
        .ok_or(std_orchestration::OrchestrationError::WorkflowNotFound)?;
    let parameters = serde_json::from_str(json)?;
    let step = add_workflow_step(&path, step_name, parse_step_type(step_type)?, parameters)?;
    Ok(serde_json::to_string_pretty(&step)?)
}

fn update_step_in_workflow(
    core: &StdCore,
    workflow_name: &str,
    step_index: usize,
    name: Option<&str>,
    json: Option<&str>,
) -> Result<String, CliError> {
    core.ensure_storage()?;
    let path = resolve_workflow_input(&core.config, workflow_name)
        .ok_or(std_orchestration::OrchestrationError::WorkflowNotFound)?;
    let parameters = json.map(serde_json::from_str).transpose()?;
    let step = update_workflow_step(&path, step_index, name, parameters)?;
    Ok(serde_json::to_string_pretty(&step)?)
}

fn remove_step_from_workflow(
    core: &StdCore,
    workflow_name: &str,
    step_index: usize,
) -> Result<String, CliError> {
    core.ensure_storage()?;
    let path = resolve_workflow_input(&core.config, workflow_name)
        .ok_or(std_orchestration::OrchestrationError::WorkflowNotFound)?;
    let step = remove_workflow_step(&path, step_index)?;
    Ok(serde_json::to_string_pretty(&step)?)
}

fn move_step_in_workflow(
    core: &StdCore,
    workflow_name: &str,
    from_index: usize,
    to_index: usize,
) -> Result<String, CliError> {
    core.ensure_storage()?;
    let path = resolve_workflow_input(&core.config, workflow_name)
        .ok_or(std_orchestration::OrchestrationError::WorkflowNotFound)?;
    let step = move_workflow_step(&path, from_index, to_index)?;
    Ok(serde_json::to_string_pretty(&step)?)
}

fn check_workflow(core: &StdCore, workflow_name: &str) -> Result<String, CliError> {
    core.ensure_storage()?;
    let workflow = match resolve_workflow_input(&core.config, workflow_name) {
        Some(path) => load_workflow(&path)?,
        None => built_in_smoke_workflow(core, workflow_name)?,
    };

    let executor = WorkflowExecutor::new(core.clone());
    let dry_run = executor.dry_run(&workflow)?;
    Ok(serde_json::to_string_pretty(&dry_run)?)
}

fn workflow_history(core: &StdCore, limit: usize) -> Result<String, CliError> {
    core.ensure_storage()?;
    let executions = read_workflow_executions(&core.config.history_dir(), limit)?;
    Ok(serde_json::to_string_pretty(&executions)?)
}

fn workflow_trace(core: &StdCore, limit: usize) -> Result<String, CliError> {
    core.ensure_storage()?;
    let traces = std_orchestration::recent_workflow_traces(core, limit)?;
    Ok(serde_json::to_string_pretty(&traces)?)
}

fn save_planned_workflow(core: &StdCore, workflow: &Workflow) -> Result<String, CliError> {
    core.ensure_storage()?;
    let workflow_dir = core.config.workflows_dir().join(slugify(&workflow.name));
    fs::create_dir_all(&workflow_dir)?;
    let workflow_path = workflow_dir.join("workflow.json");
    std_orchestration::write_workflow(&workflow_path, workflow)?;
    core.register_local_content_actions()?;
    Ok(format!(
        "planned workflow saved\npath={}\nname={}\nsteps={}",
        workflow_path.display(),
        workflow.name,
        workflow.steps.len()
    ))
}

fn built_in_smoke_workflow(core: &StdCore, workflow_name: &str) -> Result<Workflow, CliError> {
    let action = Action::new(
        "CLI Echo",
        "Echo workflow execution parameters",
        "When validating the terminal Workflow path",
        ActionType::Command,
    );
    let action_id = action.id;
    core.register_action(RegistryEntry::from_action(
        action,
        vec!["workflow".to_string(), "cli".to_string()],
    ))?;

    Ok(Workflow {
        id: Uuid::new_v4(),
        name: workflow_name.to_string(),
        description: "Built-in CLI smoke workflow".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "echo".to_string(),
            action_id: Some(action_id),
            step_type: StepType::Action,
            parameters: serde_json::json!({ "workflow": workflow_name }),
        }],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            slug.push('-');
            previous_dash = true;
        }
    }
    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "workflow".to_string()
    } else {
        slug
    }
}

fn parse_step_type(value: &str) -> Result<StepType, CliError> {
    match value {
        "action" => Ok(StepType::Action),
        "condition" => Ok(StepType::Condition),
        "loop" => Ok(StepType::Loop),
        "ai_subtask" => Ok(StepType::AiSubtask),
        "user_interaction" => Ok(StepType::UserInteraction),
        _ => Err(
            std_orchestration::OrchestrationError::InvalidWorkflow(format!(
                "unsupported workflow step type: {value}"
            ))
            .into(),
        ),
    }
}
