use std::time::Instant;
use std_core::StdCore;
use std_types::{ActionExecution, ActionExecutionStatus, ActionPreview, ActionType, SearchResult};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LauncherTelemetry {
    pub last_search_ms: u128,
    pub last_preview_ms: u128,
    pub last_trigger_ms: u128,
    pub last_result_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherResultMode {
    SuggestedWorkflows,
    Matches,
    NoMatches,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LauncherViewModel {
    pub query: String,
    pub result_mode: LauncherResultMode,
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
        let mut results = core.search("", 10).unwrap_or_default();
        sort_launcher_results(&mut results);
        let elapsed_ms = started_at.elapsed().as_millis();
        let result_count = results.len();
        let mut view = Self {
            query: String::new(),
            result_mode: LauncherResultMode::SuggestedWorkflows,
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
        self.query = normalize_query(query.into());
        let started_at = Instant::now();
        self.results = core.search(&self.query, 10).unwrap_or_default();
        sort_launcher_results(&mut self.results);
        self.result_mode = result_mode(&self.query, self.results.is_empty());
        self.telemetry.last_search_ms = started_at.elapsed().as_millis();
        self.telemetry.last_result_count = self.results.len();
        self.selected = 0;
        self.refresh_preview(core);
    }

    pub fn delete_previous_query_token(&mut self, core: &StdCore) {
        let mut tokens = self.query.split_whitespace().collect::<Vec<_>>();
        tokens.pop();
        self.update_query(core, tokens.join(" "));
    }

    pub fn move_selection(&mut self, delta: isize) {
        if self.results.is_empty() {
            self.selected = 0;
            self.preview = None;
            return;
        }

        let last = self.results.len() - 1;
        self.selected = self.selected.saturating_add_signed(delta).min(last);
    }

    pub fn move_selection_with_preview(&mut self, core: &StdCore, delta: isize) {
        self.move_selection(delta);
        self.refresh_preview(core);
    }

    pub fn jump_selection(&mut self, core: &StdCore, first: bool) {
        if self.results.is_empty() {
            self.selected = 0;
            self.preview = None;
            return;
        }
        self.selected = if first { 0 } else { self.results.len() - 1 };
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

fn normalize_query(query: String) -> String {
    query.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn result_mode(query: &str, empty_results: bool) -> LauncherResultMode {
    if query.trim().is_empty() {
        LauncherResultMode::SuggestedWorkflows
    } else if empty_results {
        LauncherResultMode::NoMatches
    } else {
        LauncherResultMode::Matches
    }
}

fn sort_launcher_results(results: &mut [SearchResult]) {
    results.sort_by(|left, right| {
        group_rank(&left.action.action_type)
            .cmp(&group_rank(&right.action.action_type))
            .then_with(|| right.score.total_cmp(&left.score))
            .then_with(|| left.action.name.cmp(&right.action.name))
    });
}

fn group_rank(action_type: &ActionType) -> u8 {
    match action_type {
        ActionType::Workflow | ActionType::Command => 0,
        ActionType::AppLaunch => 1,
        ActionType::Custom(kind) if kind == "file" => 1,
        ActionType::Clipboard => 2,
        ActionType::Skill => 3,
        ActionType::Custom(_) => 4,
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
