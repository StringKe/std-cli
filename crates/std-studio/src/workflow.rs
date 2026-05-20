use crate::StudioApp;
use std_core::StdCore;
use std_orchestration::{
    add_workflow_step, append_workflow_execution, list_workflows, load_workflow,
    move_workflow_step, read_workflow_executions, remove_workflow_step, update_workflow_step,
    workflow_from_plan, write_workflow_markdown, OrchestrationError, Workflow, WorkflowDryRun,
    WorkflowExecution, WorkflowExecutor, WorkflowStep,
};
use std_types::{Action, ActionType, RegistryEntry};
use uuid::Uuid;

impl StudioApp {
    pub fn create_workflow(
        &mut self,
        name: &str,
        description: &str,
    ) -> Result<std::path::PathBuf, OrchestrationError> {
        self.core.ensure_storage()?;
        let path = write_workflow_markdown(&self.core.config.workflows_dir(), name, description)?;
        self.core.register_local_content_actions()?;
        self.refresh();
        Ok(path)
    }

    pub fn saved_workflows(&self) -> Result<Vec<std::path::PathBuf>, OrchestrationError> {
        list_workflows(&self.core.config.workflows_dir())
    }

    pub fn recent_workflow_executions(
        &self,
        limit: usize,
    ) -> Result<Vec<WorkflowExecution>, OrchestrationError> {
        read_workflow_executions(&self.core.config.history_dir(), limit)
    }

    pub fn add_workflow_step(
        &mut self,
        workflow_path: &std::path::Path,
        name: &str,
        parameters: serde_json::Value,
    ) -> Result<WorkflowStep, OrchestrationError> {
        let step = add_workflow_step(
            workflow_path,
            name,
            std_orchestration::StepType::Action,
            parameters,
        )?;
        self.preview_workflow_path(workflow_path)?;
        Ok(step)
    }

    pub fn update_workflow_step(
        &mut self,
        workflow_path: &std::path::Path,
        step_index: usize,
        name: Option<&str>,
        parameters: Option<serde_json::Value>,
    ) -> Result<WorkflowStep, OrchestrationError> {
        let step = update_workflow_step(workflow_path, step_index, name, parameters)?;
        self.preview_workflow_path(workflow_path)?;
        Ok(step)
    }

    pub fn remove_workflow_step(
        &mut self,
        workflow_path: &std::path::Path,
        step_index: usize,
    ) -> Result<WorkflowStep, OrchestrationError> {
        let step = remove_workflow_step(workflow_path, step_index)?;
        self.preview_workflow_path(workflow_path)?;
        Ok(step)
    }

    pub fn move_workflow_step(
        &mut self,
        workflow_path: &std::path::Path,
        from_index: usize,
        to_index: usize,
    ) -> Result<WorkflowStep, OrchestrationError> {
        let step = move_workflow_step(workflow_path, from_index, to_index)?;
        self.preview_workflow_path(workflow_path)?;
        Ok(step)
    }

    pub fn preview_workflow_path(
        &mut self,
        path: &std::path::Path,
    ) -> Result<&WorkflowDryRun, OrchestrationError> {
        let workflow = load_workflow(path)?;
        self.preview_workflow(&workflow)
    }

    pub fn preview_workflow(
        &mut self,
        workflow: &Workflow,
    ) -> Result<&WorkflowDryRun, OrchestrationError> {
        let executor = WorkflowExecutor::new(self.core.clone());
        self.workflow_debug = Some(executor.dry_run(workflow)?);
        Ok(self.workflow_debug.as_ref().expect("workflow_debug is set"))
    }

    pub fn plan_workflow(&mut self, goal: &str) -> Result<Workflow, std_core::CoreError> {
        let plan = std_core::AiPlanner::plan(&self.core, goal)?;
        let workflow = workflow_from_plan(&plan);
        self.planned_workflow = Some(workflow.clone());
        Ok(workflow)
    }

    pub fn save_planned_workflow(&mut self) -> Result<std::path::PathBuf, OrchestrationError> {
        let workflow = self.planned_workflow.as_ref().ok_or_else(|| {
            OrchestrationError::InvalidWorkflow("missing planned workflow".to_string())
        })?;
        self.core.ensure_storage()?;
        let workflow_dir = self
            .core
            .config
            .workflows_dir()
            .join(slugify(&workflow.name));
        std::fs::create_dir_all(&workflow_dir)?;
        let workflow_path = workflow_dir.join("workflow.json");
        std_orchestration::write_workflow(&workflow_path, workflow)?;
        self.core.register_local_content_actions()?;
        self.refresh();
        Ok(workflow_path)
    }

    pub fn run_workflow_path(
        &mut self,
        path: &std::path::Path,
    ) -> Result<&WorkflowExecution, OrchestrationError> {
        self.core.ensure_storage()?;
        let workflow = load_workflow(path)?;
        let execution = WorkflowExecutor::new(self.core.clone()).execute_capture(&workflow)?;
        append_workflow_execution(&self.core.config.history_dir(), &execution)?;
        self.last_workflow_execution = Some(execution);
        self.refresh();
        Ok(self
            .last_workflow_execution
            .as_ref()
            .expect("last_workflow_execution is set"))
    }
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

pub fn built_in_studio_preview_workflow(core: &StdCore) -> Result<Workflow, std_core::CoreError> {
    let action = Action::new(
        "Studio Preview Echo",
        "Echo workflow preview parameters",
        "When validating Studio workflow preview",
        ActionType::Command,
    );
    let action_id = action.id;
    core.register_action(RegistryEntry::from_action(
        action,
        vec!["workflow".to_string(), "studio".to_string()],
    ))?;

    Ok(Workflow {
        id: Uuid::new_v4(),
        name: "Studio Preview".to_string(),
        description: "Built-in Studio workflow preview".to_string(),
        steps: vec![std_orchestration::WorkflowStep {
            id: Uuid::new_v4(),
            name: "preview".to_string(),
            action_id: Some(action_id),
            step_type: std_orchestration::StepType::Action,
            parameters: serde_json::json!({"surface": "studio"}),
        }],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}
