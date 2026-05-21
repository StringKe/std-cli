use std::time::Instant;
use std_core::StdCore;
use std_types::{ActionExecution, ActionExecutionStatus, ActionPreview, ActionType, SearchResult};

const EMPTY_QUERY_LIMIT: usize = 10;
const RESULT_DISPLAY_LIMIT: usize = 200;
const RESULT_FETCH_LIMIT: usize = RESULT_DISPLAY_LIMIT + 1;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LauncherTelemetry {
    pub last_search_ms: u128,
    pub last_preview_ms: u128,
    pub last_trigger_ms: u128,
    pub last_result_count: usize,
    pub last_total_matches: usize,
    pub last_overflowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherResultMode {
    SuggestedWorkflows,
    Matches,
    NoMatches,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherPhase {
    Empty,
    Searching,
    WithResults,
    NoMatches,
    Executing,
    Feedback,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LauncherViewModel {
    pub query: String,
    pub phase: LauncherPhase,
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
        let mut results = core.search("", EMPTY_QUERY_LIMIT).unwrap_or_default();
        sort_launcher_results(&mut results);
        let elapsed_ms = started_at.elapsed().as_millis();
        let result_count = results.len();
        let mut view = Self {
            query: String::new(),
            phase: LauncherPhase::Empty,
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
                last_total_matches: result_count,
                ..LauncherTelemetry::default()
            },
        };
        view.refresh_preview(core);
        view
    }

    pub fn update_query(&mut self, core: &StdCore, query: impl Into<String>) {
        let query = LauncherQueryRequest::parse(query);
        let action_only = query.action_only();
        self.query = query.display_query;
        let started_at = Instant::now();
        self.results = core
            .search(
                &query.search_query,
                search_limit_for_query(&query.search_query),
            )
            .unwrap_or_default();
        if action_only {
            self.results.retain(is_action_result);
        }
        sort_launcher_results(&mut self.results);
        let overflowed = self.results.len() > RESULT_DISPLAY_LIMIT;
        if overflowed {
            self.results.truncate(RESULT_DISPLAY_LIMIT);
        }
        self.result_mode = result_mode(&self.query, self.results.is_empty());
        self.phase = phase_for_results(&self.query, self.results.is_empty());
        self.telemetry.last_search_ms = started_at.elapsed().as_millis();
        self.telemetry.last_result_count = self.results.len();
        self.telemetry.last_total_matches = self.results.len() + usize::from(overflowed);
        self.telemetry.last_overflowed = overflowed;
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
        self.phase = LauncherPhase::Executing;
        let execution = core.execute_action(action.id).ok()?;
        self.telemetry.last_trigger_ms = started_at.elapsed().as_millis();
        let name = action.name;
        self.last_triggered = Some(name.clone());
        self.last_execution = Some(execution.clone());
        self.feedback = Some(LauncherFeedback::from_execution(&execution));
        self.phase = LauncherPhase::Feedback;
        Some(execution)
    }

    pub fn preview_searching(&mut self, query: impl Into<String>) {
        self.query = normalize_query(query.into());
        self.results.clear();
        self.preview = None;
        self.selected = 0;
        self.result_mode = LauncherResultMode::Matches;
        self.phase = LauncherPhase::Searching;
    }

    pub fn preview_executing(&mut self) {
        self.phase = LauncherPhase::Executing;
    }

    pub fn result_overflowed(&self) -> bool {
        self.telemetry.last_overflowed
    }
}

fn normalize_query(query: String) -> String {
    query.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LauncherQueryMode {
    All,
    Command,
    Actions,
    Ask,
}

struct LauncherQueryRequest {
    display_query: String,
    search_query: String,
    mode: LauncherQueryMode,
}

impl LauncherQueryRequest {
    fn parse(query: impl Into<String>) -> Self {
        let display_query = normalize_query(query.into());
        let mode = match display_query.trim_start().chars().next() {
            Some('/') => LauncherQueryMode::Command,
            Some('>') => LauncherQueryMode::Actions,
            Some('?') => LauncherQueryMode::Ask,
            _ => LauncherQueryMode::All,
        };
        let search_query = match mode {
            LauncherQueryMode::All => display_query.clone(),
            LauncherQueryMode::Command | LauncherQueryMode::Actions | LauncherQueryMode::Ask => {
                display_query
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .trim()
                    .to_string()
            }
        };
        Self {
            display_query,
            search_query,
            mode,
        }
    }

    fn action_only(&self) -> bool {
        self.mode == LauncherQueryMode::Actions
    }
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

fn phase_for_results(query: &str, empty_results: bool) -> LauncherPhase {
    if query.trim().is_empty() {
        LauncherPhase::Empty
    } else if empty_results {
        LauncherPhase::NoMatches
    } else {
        LauncherPhase::WithResults
    }
}

fn search_limit_for_query(query: &str) -> usize {
    if query.trim().is_empty() {
        EMPTY_QUERY_LIMIT
    } else {
        RESULT_FETCH_LIMIT
    }
}

fn is_action_result(result: &SearchResult) -> bool {
    matches!(
        result.action.action_type,
        ActionType::Command | ActionType::Workflow
    )
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
