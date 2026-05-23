use crate::i18n;
use std_types::{ActionExecution, ActionExecutionStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherFeedbackAction {
    Copy,
    Retry,
    OpenStudio,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherFeedback {
    pub action_name: String,
    pub status: ActionExecutionStatus,
    pub title: String,
    pub detail: String,
    pub deferred: bool,
}

impl LauncherFeedback {
    pub fn from_execution(execution: &ActionExecution) -> Self {
        let deferred = execution.status == ActionExecutionStatus::NeedsExternalRunner;
        Self {
            action_name: execution.action_name.clone(),
            status: execution.status.clone(),
            title: feedback_title(&execution.status),
            detail: feedback_detail(execution),
            deferred,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "{} {:?} {}",
            self.action_name,
            self.status,
            self.detail.trim()
        )
    }

    pub fn actions(&self) -> Vec<LauncherFeedbackAction> {
        match self.status {
            ActionExecutionStatus::Completed => vec![LauncherFeedbackAction::Copy],
            ActionExecutionStatus::NeedsExternalRunner => {
                vec![LauncherFeedbackAction::Copy, LauncherFeedbackAction::Retry]
            }
            ActionExecutionStatus::Failed => vec![
                LauncherFeedbackAction::Copy,
                LauncherFeedbackAction::Retry,
                LauncherFeedbackAction::OpenStudio,
            ],
        }
    }

    pub fn status_label(&self) -> &str {
        self.title.as_str()
    }
}

fn feedback_title(status: &ActionExecutionStatus) -> String {
    match status {
        ActionExecutionStatus::Completed => i18n::t("launcher.feedback.completed").to_string(),
        ActionExecutionStatus::Failed => i18n::t("launcher.feedback.failed").to_string(),
        ActionExecutionStatus::NeedsExternalRunner => {
            i18n::t("launcher.feedback.deferred").to_string()
        }
    }
}

fn feedback_detail(execution: &ActionExecution) -> String {
    if execution.status == ActionExecutionStatus::NeedsExternalRunner {
        return i18n::t("launcher.feedback.deferred.detail").to_string();
    }
    execution
        .output
        .as_ref()
        .and_then(|output| output.get("reason"))
        .and_then(|reason| reason.as_str())
        .map(ToString::to_string)
        .unwrap_or_else(|| execution.message.clone())
}
