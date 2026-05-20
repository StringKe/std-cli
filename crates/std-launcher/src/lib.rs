//! std-launcher - Global hotkey floating panel.
//!
//! Extremely restrained implementation. Only search + trigger.

mod action_panel;
mod hotkey;
mod keyboard;
mod semantics;
mod voice;

pub use action_panel::{ActionPanel, ActionPanelItem};
pub use hotkey::{
    hotkey_smoke, GlobalHotkeyRuntime, HotkeyRegistrationPlan, HotkeySmokeReport, LauncherHotkey,
};
pub use keyboard::{LauncherKey, LauncherKeyboardReport};
pub use semantics::LauncherUiSemanticsReport;
use std::time::Instant;
use std_core::{StdConfig, StdCore};
use std_egui::LauncherViewModel;
use std_orchestration::{
    append_workflow_execution, load_workflow, resolve_workflow_input, WorkflowExecutor,
};
use std_types::{ActionExecution, ActionExecutionStatus, ActionPreview, ActionType};
pub use voice::clean_voice_transcript;

const SEARCH_BUDGET_MS: u128 = 16;
const PREVIEW_BUDGET_MS: u128 = 16;
const TRIGGER_BUDGET_MS: u128 = 80;
const HOTKEY_BUDGET_MS: u128 = 80;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherController {
    pub hotkey: LauncherHotkey,
    pub visible: bool,
    pub focused: bool,
    pub voice_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LauncherWindowCommand {
    SetVisible(bool),
    Focus,
}

impl LauncherController {
    pub fn new(config: &StdConfig) -> Self {
        Self {
            hotkey: LauncherHotkey::parse(&config.launcher_hotkey).unwrap_or_else(|| {
                LauncherHotkey {
                    modifiers: vec!["Alt".to_string()],
                    key: "Space".to_string(),
                }
            }),
            visible: true,
            focused: true,
            voice_active: false,
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        self.focused = self.visible;
    }

    pub fn show(&mut self) {
        self.visible = true;
        self.focused = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.focused = false;
        self.voice_active = false;
    }

    pub fn start_voice_input(&mut self) {
        self.visible = true;
        self.focused = true;
        self.voice_active = true;
    }

    pub fn finish_voice_input(&mut self) {
        self.voice_active = false;
    }

    pub fn registration_plan(&self) -> HotkeyRegistrationPlan {
        HotkeyRegistrationPlan {
            accelerator: self.hotkey.accelerator(),
            enabled: true,
        }
    }

    pub fn window_commands(
        previous_visible: bool,
        current_visible: bool,
    ) -> Vec<LauncherWindowCommand> {
        if previous_visible == current_visible {
            return Vec::new();
        }
        if current_visible {
            vec![
                LauncherWindowCommand::SetVisible(true),
                LauncherWindowCommand::Focus,
            ]
        } else {
            vec![LauncherWindowCommand::SetVisible(false)]
        }
    }
}

pub struct LauncherState {
    pub core: StdCore,
    pub view: LauncherViewModel,
    pub controller: LauncherController,
    pub action_panel: ActionPanel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherSmokeReport {
    pub query: String,
    pub preview_title: String,
    pub execution_status: ActionExecutionStatus,
    pub feedback_title: String,
    pub performance: LauncherPerformanceReport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherWindowSmokeReport {
    pub hidden_commands: Vec<LauncherWindowCommand>,
    pub shown_commands: Vec<LauncherWindowCommand>,
    pub final_visible: bool,
    pub focused: bool,
    pub elapsed_ms: u128,
    pub budget_ms: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherPerformanceReport {
    pub hotkey_budget_ms: u128,
    pub search_budget_ms: u128,
    pub preview_budget_ms: u128,
    pub trigger_budget_ms: u128,
    pub last_search_ms: u128,
    pub last_preview_ms: u128,
    pub last_trigger_ms: u128,
    pub result_count: usize,
}

impl LauncherPerformanceReport {
    pub fn pass(&self) -> bool {
        self.last_search_ms <= self.search_budget_ms
            && self.last_preview_ms <= self.preview_budget_ms
            && self.last_trigger_ms <= self.trigger_budget_ms
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_perf {}\nsearch_ms={}\npreview_ms={}\ntrigger_ms={}\nresults={}\nbudget_search_ms={}\nbudget_preview_ms={}\nbudget_trigger_ms={}\nbudget_hotkey_ms={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.last_search_ms,
            self.last_preview_ms,
            self.last_trigger_ms,
            self.result_count,
            self.search_budget_ms,
            self.preview_budget_ms,
            self.trigger_budget_ms,
            self.hotkey_budget_ms
        )
    }
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
        self.controller.hide();
    }

    pub fn update_query(&mut self, query: impl Into<String>) -> Option<ActionPreview> {
        self.action_panel.close();
        self.view.update_query(&self.core, query);
        self.view.preview.clone()
    }

    pub fn move_selection(&mut self, delta: isize) -> Option<ActionPreview> {
        self.action_panel.close();
        self.view.move_selection_with_preview(&self.core, delta);
        self.view.preview.clone()
    }

    pub fn open_action_panel(&mut self) -> bool {
        let Some(result) = self.view.selected_result() else {
            self.action_panel.close();
            return false;
        };
        self.action_panel.open_for(&result.action);
        true
    }

    pub fn close_action_panel(&mut self) {
        self.action_panel.close();
    }

    pub fn move_action_panel_selection(&mut self, delta: isize) {
        self.action_panel.move_selection(delta);
    }

    pub fn trigger_action_panel_selection(&mut self) -> Option<ActionExecution> {
        match self.action_panel.selected_item()?.clone() {
            ActionPanelItem::Run => self.trigger_selected_by_user(),
            ActionPanelItem::Defer => self.trigger_selected(),
            ActionPanelItem::CopyCommand(command) => Some(self.complete_action_panel_copy(command)),
        }
    }

    pub fn trigger_selected(&mut self) -> Option<ActionExecution> {
        self.trigger_selected_with_external_runner(false)
    }

    pub fn trigger_selected_by_user(&mut self) -> Option<ActionExecution> {
        self.trigger_selected_with_external_runner(true)
    }

    fn trigger_selected_with_external_runner(
        &mut self,
        allow_external_runner: bool,
    ) -> Option<ActionExecution> {
        let result = self.view.selected_result()?.clone();
        let started_at = Instant::now();
        let execution = if result.action.action_type == ActionType::Workflow {
            let preview = self.core.preview_action(result.action.id).ok()?;
            self.trigger_workflow_action(&result.action.name, preview.metadata.get("path"))
                .ok()?
        } else {
            self.core
                .execute_action_with_external_runner(result.action.id, allow_external_runner)
                .ok()?
        };
        self.view.telemetry.last_trigger_ms = started_at.elapsed().as_millis();
        self.view.last_triggered = Some(result.action.name);
        self.view.last_execution = Some(execution.clone());
        self.view.feedback = Some(std_egui::LauncherFeedback::from_execution(&execution));
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

impl LauncherSmokeReport {
    pub fn summary(&self) -> String {
        format!(
            "launcher_smoke {}\nquery={}\npreview={}\nstatus={:?}\nfeedback={}\n{}",
            if self.performance.pass() {
                "PASS"
            } else {
                "FAIL"
            },
            self.query,
            self.preview_title,
            self.execution_status,
            self.feedback_title,
            self.performance.summary()
        )
    }
}

impl LauncherWindowSmokeReport {
    pub fn pass(&self) -> bool {
        self.hidden_commands == vec![LauncherWindowCommand::SetVisible(false)]
            && self.shown_commands
                == vec![
                    LauncherWindowCommand::SetVisible(true),
                    LauncherWindowCommand::Focus,
                ]
            && self.final_visible
            && self.focused
            && self.elapsed_ms <= self.budget_ms
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_window_smoke {}\nhidden_commands={}\nshown_commands={}\nfinal_visible={}\nfocused={}\nelapsed_ms={}\nbudget_window_ms={}",
            if self.pass() { "PASS" } else { "FAIL" },
            format_window_commands(&self.hidden_commands),
            format_window_commands(&self.shown_commands),
            self.final_visible,
            self.focused,
            self.elapsed_ms,
            self.budget_ms
        )
    }
}

fn format_window_commands(commands: &[LauncherWindowCommand]) -> String {
    commands
        .iter()
        .map(|command| match command {
            LauncherWindowCommand::SetVisible(true) => "Visible(true)",
            LauncherWindowCommand::SetVisible(false) => "Visible(false)",
            LauncherWindowCommand::Focus => "Focus",
        })
        .collect::<Vec<_>>()
        .join(",")
}

pub fn launcher_version() -> &'static str {
    "0.1.0"
}

#[cfg(test)]
mod action_panel_tests;
#[cfg(test)]
mod tests;
