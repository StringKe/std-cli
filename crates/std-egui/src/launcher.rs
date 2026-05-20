use std::time::Instant;
use std_core::StdCore;
use std_types::{ActionExecution, ActionExecutionStatus, ActionPreview, SearchResult};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LauncherTelemetry {
    pub last_search_ms: u128,
    pub last_preview_ms: u128,
    pub last_trigger_ms: u128,
    pub last_result_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LauncherViewModel {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub selected: usize,
    pub preview: Option<ActionPreview>,
    pub last_execution: Option<ActionExecution>,
    pub feedback: Option<LauncherFeedback>,
    pub last_triggered: Option<String>,
    pub telemetry: LauncherTelemetry,
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
}

impl LauncherViewModel {
    pub fn new(core: &StdCore) -> Self {
        let started_at = Instant::now();
        let results = core.search("", 10).unwrap_or_default();
        let elapsed_ms = started_at.elapsed().as_millis();
        let result_count = results.len();
        let mut view = Self {
            query: String::new(),
            results,
            selected: 0,
            preview: None,
            last_execution: None,
            feedback: None,
            last_triggered: None,
            telemetry: LauncherTelemetry {
                last_search_ms: elapsed_ms,
                last_result_count: result_count,
                ..LauncherTelemetry::default()
            },
        };
        view.refresh_preview(core);
        view
    }

    pub fn update_query(&mut self, core: &StdCore, query: impl Into<String>) {
        self.query = query.into();
        let started_at = Instant::now();
        self.results = core.search(&self.query, 10).unwrap_or_default();
        self.telemetry.last_search_ms = started_at.elapsed().as_millis();
        self.telemetry.last_result_count = self.results.len();
        self.selected = 0;
        self.refresh_preview(core);
    }

    pub fn move_selection(&mut self, delta: isize) {
        if self.results.is_empty() {
            self.selected = 0;
            self.preview = None;
            return;
        }

        let len = self.results.len() as isize;
        let next = (self.selected as isize + delta).rem_euclid(len);
        self.selected = next as usize;
    }

    pub fn move_selection_with_preview(&mut self, core: &StdCore, delta: isize) {
        self.move_selection(delta);
        self.refresh_preview(core);
    }

    pub fn selected_result(&self) -> Option<&SearchResult> {
        self.results.get(self.selected)
    }

    pub fn refresh_preview(&mut self, core: &StdCore) -> Option<ActionPreview> {
        let action_id = self.selected_result()?.action.id;
        let started_at = Instant::now();
        let preview = core.preview_action(action_id).ok()?;
        self.telemetry.last_preview_ms = started_at.elapsed().as_millis();
        self.preview = Some(preview.clone());
        Some(preview)
    }

    pub fn trigger_selected(&mut self, core: &StdCore) -> Option<ActionExecution> {
        let action = self.selected_result()?.action.clone();
        let started_at = Instant::now();
        let execution = core.execute_action(action.id).ok()?;
        self.telemetry.last_trigger_ms = started_at.elapsed().as_millis();
        let name = action.name;
        self.last_triggered = Some(name.clone());
        self.last_execution = Some(execution.clone());
        self.feedback = Some(LauncherFeedback::from_execution(&execution));
        Some(execution)
    }
}

fn feedback_title(status: &ActionExecutionStatus) -> String {
    match status {
        ActionExecutionStatus::Completed => "Completed".to_string(),
        ActionExecutionStatus::Failed => "Failed".to_string(),
        ActionExecutionStatus::NeedsExternalRunner => "Needs external runner".to_string(),
    }
}

fn feedback_detail(execution: &ActionExecution) -> String {
    execution
        .output
        .as_ref()
        .and_then(|output| output.get("reason"))
        .and_then(|reason| reason.as_str())
        .map(ToString::to_string)
        .unwrap_or_else(|| execution.message.clone())
}
