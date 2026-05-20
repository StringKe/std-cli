use crate::{
    ai_subtasks::{execute_ai_subtask_step, preview_ai_subtask_step},
    conditions::{execute_condition_step, preview_condition_step},
    executor_support::{
        action_schema_tuple, failed_step_result, step_failure_error, summarize_json,
        validate_step_parameters, ResolvedStepAction,
    },
    loops::{preview_loop_step, LoopDefinition},
    user_interactions::{execute_user_interaction_step, preview_user_interaction_step},
    ExecutionStatus, OrchestrationError, StepDryRun, StepResult, StepType, Workflow,
    WorkflowDryRun, WorkflowExecution, WorkflowStep,
};
use chrono::Utc;
use std_core::{EventBus, StdCore};
use std_types::{ActionExecutionStatus, StdEvent, StdEventType};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WorkflowExecutionOptions {
    pub allow_external_runner: bool,
}

pub struct WorkflowExecutor {
    core: StdCore,
    options: WorkflowExecutionOptions,
}

struct StepFailure {
    error: OrchestrationError,
    started_at: chrono::DateTime<chrono::Utc>,
}

impl WorkflowExecutor {
    pub fn new(core: StdCore) -> Self {
        Self {
            core,
            options: WorkflowExecutionOptions::default(),
        }
    }

    pub fn with_options(core: StdCore, options: WorkflowExecutionOptions) -> Self {
        Self { core, options }
    }

    pub fn execute(&self, workflow: &Workflow) -> Result<WorkflowExecution, OrchestrationError> {
        self.execute_internal(workflow, false)
    }

    pub fn execute_capture(
        &self,
        workflow: &Workflow,
    ) -> Result<WorkflowExecution, OrchestrationError> {
        self.execute_internal(workflow, true)
    }

    fn execute_internal(
        &self,
        workflow: &Workflow,
        capture_failure: bool,
    ) -> Result<WorkflowExecution, OrchestrationError> {
        let mut execution = WorkflowExecution {
            workflow_id: workflow.id,
            workflow_name: workflow.name.clone(),
            status: ExecutionStatus::Running,
            current_step: 0,
            started_at: Utc::now(),
            finished_at: None,
            results: vec![],
        };

        self.core.publish(StdEvent::new(
            StdEventType::WorkflowStarted,
            "std-orchestration",
            serde_json::json!({
                "workflow_id": workflow.id,
                "workflow_name": workflow.name,
            }),
        ))?;

        for (i, step) in workflow.steps.iter().enumerate() {
            execution.current_step = i;
            match self.execute_step(step, &execution.results) {
                Ok(result) => {
                    self.core.publish(StdEvent::new(
                        StdEventType::WorkflowStepCompleted,
                        "std-orchestration",
                        serde_json::json!({
                            "workflow_id": workflow.id,
                            "step_id": step.id,
                            "step_name": step.name,
                        }),
                    ))?;
                    execution.results.push(result);
                }
                Err(failure) => {
                    execution.status = ExecutionStatus::Failed;
                    execution.finished_at = Some(Utc::now());
                    self.core.publish(StdEvent::new(
                        StdEventType::WorkflowFailed,
                        "std-orchestration",
                        serde_json::json!({
                            "workflow_id": workflow.id,
                            "step_id": step.id,
                            "step_name": step.name,
                            "error": failure.error.to_string(),
                        }),
                    ))?;
                    execution.results.push(failed_step_result(
                        step,
                        failure.error,
                        failure.started_at,
                    ));
                    if capture_failure {
                        return Ok(execution);
                    }
                    return Err(step_failure_error(&execution));
                }
            }
        }

        execution.status = ExecutionStatus::Completed;
        execution.finished_at = Some(Utc::now());
        self.core.publish(StdEvent::new(
            StdEventType::WorkflowCompleted,
            "std-orchestration",
            serde_json::json!({
                "workflow_id": workflow.id,
                "workflow_name": workflow.name,
                "step_count": workflow.steps.len(),
            }),
        ))?;
        Ok(execution)
    }

    pub fn dry_run(&self, workflow: &Workflow) -> Result<WorkflowDryRun, OrchestrationError> {
        let steps = workflow
            .steps
            .iter()
            .map(|step| self.preview_step(step))
            .collect::<Result<Vec<_>, _>>()?;
        let status = if steps
            .iter()
            .all(|step| step.status == ExecutionStatus::Completed)
        {
            ExecutionStatus::Completed
        } else {
            ExecutionStatus::Failed
        };

        Ok(WorkflowDryRun {
            workflow_id: workflow.id,
            workflow_name: workflow.name.clone(),
            status,
            checked_at: Utc::now(),
            steps,
        })
    }

    fn execute_step(
        &self,
        step: &WorkflowStep,
        previous_results: &[StepResult],
    ) -> Result<StepResult, StepFailure> {
        let started_at = Utc::now();
        let output = match step.step_type {
            StepType::Action => self
                .execute_action_step(step)
                .map_err(|error| StepFailure { error, started_at })?,
            StepType::Condition => execute_condition_step(step, previous_results)
                .map_err(|error| StepFailure { error, started_at })?,
            StepType::Loop => self
                .execute_loop_step(step, previous_results)
                .map_err(|error| StepFailure { error, started_at })?,
            StepType::AiSubtask => execute_ai_subtask_step(&self.core, step)
                .map_err(|error| StepFailure { error, started_at })?,
            StepType::UserInteraction => execute_user_interaction_step(step)
                .map_err(|error| StepFailure { error, started_at })?,
        };

        Ok(StepResult {
            step_id: step.id,
            step_name: step.name.clone(),
            status: ExecutionStatus::Completed,
            output,
            started_at,
            finished_at: Utc::now(),
        })
    }

    fn execute_loop_step(
        &self,
        step: &WorkflowStep,
        previous_results: &[StepResult],
    ) -> Result<serde_json::Value, OrchestrationError> {
        let definition = LoopDefinition::parse(step)?;
        let mut context = previous_results.to_vec();
        let mut iterations = Vec::new();
        for iteration_index in 0..definition.count {
            let mut step_results = Vec::new();
            for body_step in &definition.steps {
                let result = self
                    .execute_step(body_step, &context)
                    .map_err(|failure| failure.error)?;
                context.push(result.clone());
                step_results.push(result);
            }
            iterations.push(serde_json::json!({
                "index": iteration_index,
                "results": step_results,
            }));
        }
        Ok(serde_json::json!({
            "step_type": "loop",
            "count": definition.count,
            "iterations": iterations,
        }))
    }

    fn execute_action_step(
        &self,
        step: &WorkflowStep,
    ) -> Result<serde_json::Value, OrchestrationError> {
        let Some(action) = self.resolve_step_action(step)? else {
            return Ok(serde_json::json!({
                "step_type": "action",
                "parameters": step.parameters,
            }));
        };
        validate_step_parameters(step, action.input_schema.as_ref())?;
        let preview = self.core.preview_action(action.id)?;
        let execution =
            if !self.options.allow_external_runner && preview.action_type.needs_external_runner() {
                std_types::ActionExecution {
                    action_id: action.id,
                    action_name: action.name.clone(),
                    status: ActionExecutionStatus::NeedsExternalRunner,
                    message: preview.primary_command,
                    output: Some(serde_json::json!({
                        "deferred": true,
                        "reason": "workflow external runner action requires explicit user trigger",
                    })),
                    created_at: Utc::now(),
                }
            } else {
                self.core.execute_action_with_external_runner(
                    action.id,
                    self.options.allow_external_runner,
                )?
            };
        Ok(serde_json::json!({
            "action_id": action.id,
            "action_name": action.name,
            "status": execution.status,
            "message": execution.message,
            "output": execution.output,
            "parameters": step.parameters,
        }))
    }

    fn preview_step(&self, step: &WorkflowStep) -> Result<StepDryRun, OrchestrationError> {
        let parameter_summary = summarize_json(&step.parameters);
        match step.step_type {
            StepType::Action => self.preview_action_step(step, parameter_summary),
            StepType::Condition => Ok(preview_condition_step(step, parameter_summary)),
            StepType::Loop => Ok(preview_loop_step(step, parameter_summary)),
            StepType::AiSubtask => Ok(preview_ai_subtask_step(step, parameter_summary)),
            StepType::UserInteraction => Ok(preview_user_interaction_step(step, parameter_summary)),
        }
    }

    fn preview_action_step(
        &self,
        step: &WorkflowStep,
        parameter_summary: String,
    ) -> Result<StepDryRun, OrchestrationError> {
        if let Some(action) = self.resolve_step_action(step)? {
            let schema_message = validate_step_parameters(step, action.input_schema.as_ref())
                .err()
                .map(|error| error.to_string());
            let status = if schema_message.is_some() {
                ExecutionStatus::Failed
            } else {
                ExecutionStatus::Completed
            };
            return Ok(StepDryRun {
                step_id: step.id,
                step_name: step.name.clone(),
                step_type: step.step_type.clone(),
                status,
                action_name: Some(action.name),
                input_schema: action.input_schema,
                output_schema: action.output_schema,
                parameter_summary,
                message: schema_message.unwrap_or_else(|| "action resolved".to_string()),
            });
        }
        let (status, message) = match step.action_id {
            Some(action_id) => (
                ExecutionStatus::Failed,
                format!("action not found: {action_id}"),
            ),
            None => (ExecutionStatus::Completed, "inline action step".to_string()),
        };
        Ok(StepDryRun {
            step_id: step.id,
            step_name: step.name.clone(),
            step_type: step.step_type.clone(),
            status,
            action_name: None,
            input_schema: None,
            output_schema: None,
            parameter_summary,
            message,
        })
    }

    fn resolve_step_action(
        &self,
        step: &WorkflowStep,
    ) -> Result<Option<ResolvedStepAction>, OrchestrationError> {
        let registry = self
            .core
            .registry
            .read()
            .map_err(|_| std_core::CoreError::RegistryLockPoisoned)?;
        if let Some(action_id) = step.action_id {
            if let Some(entry) = registry.get(action_id) {
                return Ok(Some(action_schema_tuple(entry)));
            }
        }
        Ok(registry.get_by_name(&step.name).map(action_schema_tuple))
    }
}

impl Default for WorkflowExecutor {
    fn default() -> Self {
        Self::new(StdCore::new())
    }
}
