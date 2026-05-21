//! std-launcher - Global hotkey floating panel.
//!
//! Extremely restrained implementation. Only search + trigger.

mod action_bar_preview;
mod action_panel;
mod action_panel_smoke;
mod close_smoke;
mod controller;
mod hotkey;
mod keyboard;
mod keyboard_smoke;
mod layout_contract;
mod query_mode;
mod reports;
mod semantics;
mod semantics_executing;
mod shortcut_help;
mod studio_intent;
mod suggested;
mod surface_contract;
mod surface_smoke;
mod viewport_contract;
mod voice;

pub use action_bar_preview::ActionBarPreviewSummary;
pub use action_panel::{ActionPanel, ActionPanelItem};
pub use action_panel_smoke::LauncherActionPanelSmokeReport;
pub use close_smoke::LauncherCloseSmokeReport;
pub use controller::{LauncherController, LauncherWindowCommand};
pub use hotkey::{
    hotkey_smoke, GlobalHotkeyRuntime, HotkeyRegistrationPlan, HotkeySmokeReport, LauncherHotkey,
};
pub use keyboard::{
    LauncherFocusSection, LauncherFocusSource, LauncherKey, LauncherKeyboardReport,
};
pub use layout_contract::{
    panel_width_for_available, PANEL_MIN_WIDTH, PANEL_VIEWPORT_WIDTH_RATIO, PANEL_WIDTH,
};
pub use query_mode::{LauncherQueryMode, LauncherQueryRequest};
pub use reports::{
    format_window_commands, LauncherPerformanceReport, LauncherSmokeReport,
    LauncherWindowSmokeReport, HOTKEY_BUDGET_MS, PREVIEW_BUDGET_MS, SEARCH_BUDGET_MS,
    TRIGGER_BUDGET_MS,
};
pub use semantics::LauncherUiSemanticsReport;
pub use shortcut_help::{
    launcher_shortcut_help_rows, launcher_shortcut_help_summary, launcher_shortcut_help_visible,
    LauncherShortcutHelpRow,
};
use std::time::Instant;
use std_core::{StdConfig, StdCore};
use std_egui::LauncherViewModel;
use std_orchestration::{
    append_workflow_execution, load_workflow, resolve_workflow_input, WorkflowExecutor,
};
use std_types::{ActionExecution, ActionExecutionStatus, ActionPreview, ActionType};
pub use studio_intent::{StudioLaunchIntent, StudioLaunchTarget};
pub use suggested::{suggested_workflow_rows, SuggestedWorkflowRow};
pub use surface_contract::LauncherSurfaceContract;
pub use surface_smoke::LauncherSurfaceSmokeReport;
pub use viewport_contract::{
    transparent_hidden_panel_contract, transparent_visible_panel_contract,
};
pub use voice::clean_voice_transcript;

pub struct LauncherState {
    pub core: StdCore,
    pub view: LauncherViewModel,
    pub controller: LauncherController,
    pub action_panel: ActionPanel,
    pub focus_section: LauncherFocusSection,
    pub focus_source: LauncherFocusSource,
    pub empty_suggestion_selected: usize,
    pub studio_intent: Option<StudioLaunchIntent>,
}

impl Default for LauncherState {
    fn default() -> Self {
        let core = StdCore::with_config(StdConfig::load());
        core.seed_builtin_actions().ok();
        let view = LauncherViewModel::new(&core);
        let controller = LauncherController::new(&core.config);
        Self {
            core,
            view,
            controller,
            action_panel: ActionPanel::closed(),
            focus_section: LauncherFocusSection::Search,
            focus_source: LauncherFocusSource::Keyboard,
            empty_suggestion_selected: 0,
            studio_intent: None,
        }
    }
}

impl LauncherState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_core(core: StdCore) -> Self {
        core.seed_builtin_actions().ok();
        let view = LauncherViewModel::new(&core);
        let controller = LauncherController::new(&core.config);
        Self {
            core,
            view,
            controller,
            action_panel: ActionPanel::closed(),
            focus_section: LauncherFocusSection::Search,
            focus_source: LauncherFocusSource::Keyboard,
            empty_suggestion_selected: 0,
            studio_intent: None,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.controller.toggle();
    }

    pub fn handle_hotkey_toggle(&mut self) -> Vec<LauncherWindowCommand> {
        let previous_visible = self.controller.visible;
        self.toggle_visibility();
        LauncherController::window_commands(previous_visible, self.controller.visible)
    }

    pub fn handle_show(&mut self) -> Vec<LauncherWindowCommand> {
        let previous_visible = self.controller.visible;
        self.controller.show();
        LauncherController::window_commands(previous_visible, self.controller.visible)
    }

    pub fn handle_escape_hide(&mut self) -> Vec<LauncherWindowCommand> {
        let previous_visible = self.controller.visible;
        self.hide();
        LauncherController::window_commands(previous_visible, self.controller.visible)
    }

    pub fn hide(&mut self) {
        self.action_panel.close();
        self.focus_section = LauncherFocusSection::Search;
        self.focus_source = LauncherFocusSource::Keyboard;
        self.controller.hide();
    }

    pub fn update_query(&mut self, query: impl Into<String>) -> Option<ActionPreview> {
        self.action_panel.close();
        self.focus_section = LauncherFocusSection::Search;
        self.focus_source = LauncherFocusSource::Keyboard;
        self.view.update_query(&self.core, query);
        self.empty_suggestion_selected = 0;
        self.view.preview.clone()
    }

    pub fn keyboard_focus_visible(&self, section: LauncherFocusSection) -> bool {
        self.focus_section == section && self.focus_source == LauncherFocusSource::Keyboard
    }

    pub fn mark_pointer_focus(&mut self, section: LauncherFocusSection) {
        self.focus_section = section;
        self.focus_source = LauncherFocusSource::Pointer;
    }

    pub fn no_match_fallback_query(&self) -> Option<String> {
        ask_ai_fallback_query(&self.view.query)
    }

    pub fn trigger_no_match_fallback(&mut self) -> bool {
        let Some(query) = self.no_match_fallback_query() else {
            return false;
        };
        self.update_query(query);
        true
    }

    pub fn empty_suggestion_count(&self) -> usize {
        3
    }

    pub fn move_empty_suggestion(&mut self, delta: isize) {
        let max = self.empty_suggestion_count().saturating_sub(1);
        self.empty_suggestion_selected = bounded_index(self.empty_suggestion_selected, delta, max);
        self.focus_section = LauncherFocusSection::Results;
        self.focus_source = LauncherFocusSource::Keyboard;
    }

    pub fn jump_empty_suggestion(&mut self, first: bool) {
        self.empty_suggestion_selected = if first {
            0
        } else {
            self.empty_suggestion_count().saturating_sub(1)
        };
        self.focus_section = LauncherFocusSection::Results;
        self.focus_source = LauncherFocusSource::Keyboard;
    }

    pub fn apply_empty_suggestion(&mut self) -> bool {
        let Some(row) = suggested_workflow_rows()
            .get(self.empty_suggestion_selected)
            .copied()
        else {
            return false;
        };
        self.update_query(row.query);
        true
    }

    pub fn empty_query_suggestions_visible(&self) -> bool {
        self.view.result_mode == std_egui::LauncherResultMode::SuggestedWorkflows
    }

    pub fn move_selection(&mut self, delta: isize) -> Option<ActionPreview> {
        self.action_panel.close();
        self.focus_section = LauncherFocusSection::Results;
        self.focus_source = LauncherFocusSource::Keyboard;
        self.view.move_selection_with_preview(&self.core, delta);
        self.view.preview.clone()
    }

    pub fn jump_selection(&mut self, first: bool) -> Option<ActionPreview> {
        self.action_panel.close();
        self.focus_section = LauncherFocusSection::Results;
        self.focus_source = LauncherFocusSource::Keyboard;
        self.view.jump_selection(&self.core, first);
        self.view.preview.clone()
    }

    pub fn open_action_panel(&mut self) -> bool {
        let Some(result) = self.view.selected_result() else {
            self.action_panel.close();
            return false;
        };
        self.action_panel.open_for(&result.action);
        self.focus_section = LauncherFocusSection::ActionPanel;
        self.focus_source = LauncherFocusSource::Keyboard;
        true
    }

    pub fn close_action_panel(&mut self) {
        self.action_panel.close();
        self.focus_section = LauncherFocusSection::Results;
        self.focus_source = LauncherFocusSource::Keyboard;
    }

    pub fn move_action_panel_selection(&mut self, delta: isize) {
        self.action_panel.move_selection(delta);
    }

    pub fn jump_action_panel_selection(&mut self, first: bool) {
        self.action_panel.jump_selection(first);
    }

    pub fn update_action_panel_query(&mut self, query: impl Into<String>) {
        self.action_panel.update_query(query);
    }

    pub fn trigger_action_panel_selection(&mut self) -> Option<ActionExecution> {
        match self.action_panel.selected_item()?.clone() {
            ActionPanelItem::Run => self.trigger_selected_by_user(),
            ActionPanelItem::ReviewFirst => self.trigger_selected_by_user(),
            ActionPanelItem::Defer => self.trigger_selected(),
            ActionPanelItem::OpenInStudio => {
                self.open_selected_action_in_studio()?;
                None
            }
            ActionPanelItem::CopyCommand(command) => Some(self.complete_action_panel_copy(command)),
        }
    }

    pub fn trigger_selected(&mut self) -> Option<ActionExecution> {
        self.trigger_selected_with_external_runner(false)
    }

    pub fn trigger_selected_by_user(&mut self) -> Option<ActionExecution> {
        self.trigger_selected_with_external_runner(true)
    }

    pub fn trigger_result_by_user(&mut self, index: usize) -> Option<ActionExecution> {
        self.trigger_result_with_external_runner(index, true)
    }

    fn trigger_result(&mut self, index: usize) -> Option<ActionExecution> {
        self.trigger_result_with_external_runner(index, false)
    }

    fn trigger_result_with_external_runner(
        &mut self,
        index: usize,
        allow_external_runner: bool,
    ) -> Option<ActionExecution> {
        if index >= self.view.results.len() || index >= 9 {
            return None;
        }
        self.action_panel.close();
        self.view.selected = index;
        self.view.refresh_preview(&self.core);
        self.trigger_selected_with_external_runner(allow_external_runner)
    }

    fn trigger_selected_with_external_runner(
        &mut self,
        allow_external_runner: bool,
    ) -> Option<ActionExecution> {
        if self.view.result_mode == std_egui::LauncherResultMode::NaturalLanguage {
            return self.trigger_nl_action();
        }
        let result = self.view.selected_result()?.clone();
        let started_at = Instant::now();
        self.view.preview_executing();
        let execution = if result.action.action_type == ActionType::Workflow {
            let preview = self.core.preview_action(result.action.id).ok()?;
            self.trigger_workflow_action(&result.action.name, preview.metadata.get("path"))
                .ok()?
        } else if allow_external_runner {
            self.core
                .execute_action_from_launcher_user(result.action.id)
                .ok()?
        } else {
            self.core.execute_action(result.action.id).ok()?
        };
        self.view.telemetry.last_trigger_ms = started_at.elapsed().as_millis();
        self.view.last_triggered = Some(result.action.name);
        self.view.last_execution = Some(execution.clone());
        self.view.feedback = Some(std_egui::LauncherFeedback::from_execution(&execution));
        self.view.selected_feedback_action = 0;
        self.view.phase = std_egui::LauncherPhase::Feedback;
        Some(execution)
    }

    fn trigger_nl_action(&mut self) -> Option<ActionExecution> {
        let suggestion = self.view.nl_suggestion.as_ref()?;
        let selected = self.view.selected_nl_action()?.to_string();
        if selected == "Search Actions" {
            self.update_query(format!("> {}", suggestion.query));
            return None;
        }
        let execution = ActionExecution {
            action_id: Default::default(),
            action_name: format!("Natural Language: {selected}"),
            status: ActionExecutionStatus::NeedsExternalRunner,
            message: format!("ask {}", suggestion.query),
            output: Some(serde_json::json!({
                "deferred": true,
                "reason": "natural language planner requires explicit AI provider opt-in",
                "query": suggestion.query,
            })),
            created_at: chrono::Utc::now(),
        };
        self.view.last_triggered = Some(execution.action_name.clone());
        self.view.last_execution = Some(execution.clone());
        self.view.feedback = Some(std_egui::LauncherFeedback::from_execution(&execution));
        self.view.selected_feedback_action = 0;
        self.view.phase = std_egui::LauncherPhase::Feedback;
        Some(execution)
    }

    fn complete_action_panel_copy(&mut self, command: String) -> ActionExecution {
        let action_name = self
            .view
            .selected_result()
            .map(|result| result.action.name.clone())
            .unwrap_or_else(|| "Selected Action".to_string());
        let execution = ActionExecution {
            action_id: self
                .view
                .selected_result()
                .map(|result| result.action.id)
                .unwrap_or_default(),
            action_name: format!("Copy Command: {action_name}"),
            status: ActionExecutionStatus::Completed,
            message: command.clone(),
            output: Some(serde_json::json!({ "copied": command })),
            created_at: chrono::Utc::now(),
        };
        self.view.last_execution = Some(execution.clone());
        self.view.feedback = Some(std_egui::LauncherFeedback::from_execution(&execution));
        self.view.selected_feedback_action = 0;
        execution
    }

    pub fn performance_report(&self) -> LauncherPerformanceReport {
        LauncherPerformanceReport {
            hotkey_budget_ms: HOTKEY_BUDGET_MS,
            search_budget_ms: SEARCH_BUDGET_MS,
            preview_budget_ms: PREVIEW_BUDGET_MS,
            trigger_budget_ms: TRIGGER_BUDGET_MS,
            last_search_ms: self.view.telemetry.last_search_ms,
            last_preview_ms: self.view.telemetry.last_preview_ms,
            last_trigger_ms: self.view.telemetry.last_trigger_ms,
            result_count: self.view.results.len(),
        }
    }

    pub fn smoke(query: &str) -> Option<LauncherSmokeReport> {
        let mut state = Self::new();
        let preview = state.update_query(query)?.title;
        let execution = state.trigger_selected()?;
        let feedback = state.view.feedback.as_ref()?.title.clone();
        Some(LauncherSmokeReport {
            query: state.view.query.clone(),
            preview_title: preview,
            execution_status: execution.status,
            feedback_title: feedback,
            performance: state.performance_report(),
        })
    }

    pub fn window_smoke() -> LauncherWindowSmokeReport {
        let mut state = Self::new();
        let started_at = Instant::now();
        state.handle_hotkey_toggle();
        let hidden_commands = state.handle_escape_hide();
        let shown_commands = state.handle_hotkey_toggle();
        LauncherWindowSmokeReport {
            hidden_commands,
            shown_commands,
            final_visible: state.controller.visible,
            focused: state.controller.focused,
            elapsed_ms: started_at.elapsed().as_millis(),
            budget_ms: HOTKEY_BUDGET_MS,
        }
    }

    fn trigger_workflow_action(
        &self,
        action_name: &str,
        workflow_path: Option<&String>,
    ) -> Result<ActionExecution, std_orchestration::OrchestrationError> {
        self.core.ensure_storage()?;
        let workflow_name = action_name
            .strip_prefix("Run Workflow: ")
            .unwrap_or(action_name);
        let path = workflow_path
            .map(std::path::PathBuf::from)
            .filter(|path| path.exists())
            .or_else(|| resolve_workflow_input(&self.core.config, workflow_name))
            .ok_or(std_orchestration::OrchestrationError::WorkflowNotFound)?;
        let workflow = load_workflow(&path)?;
        let execution = WorkflowExecutor::new(self.core.clone()).execute_capture(&workflow)?;
        append_workflow_execution(&self.core.config.history_dir(), &execution)?;
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
}

fn bounded_index(current: usize, delta: isize, max: usize) -> usize {
    current.saturating_add_signed(delta).min(max)
}

pub fn ask_ai_fallback_query(query: &str) -> Option<String> {
    let trimmed = query.trim();
    (!trimmed.is_empty()).then(|| format!("? {trimmed}"))
}

pub fn launcher_version() -> &'static str {
    "0.1.0"
}

#[cfg(test)]
mod action_panel_tests;
#[cfg(test)]
mod app_tests;
#[cfg(test)]
mod desktop_guard_tests;
#[cfg(test)]
mod keyboard_tests;
#[cfg(test)]
mod navigation_tests;
#[cfg(test)]
mod nl_tests;
#[cfg(test)]
mod shortcut_tests;
#[cfg(test)]
mod tests;
