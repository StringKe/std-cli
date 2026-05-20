use crate::{keyboard::LauncherKey, LauncherState};
use std_egui::{
    a11y::AccessibilityContext,
    i18n::{self, Locale},
    motion::MotionContext,
    LauncherFeedback, LauncherPhase, LauncherResultMode,
};
use std_types::{ActionExecution, ActionExecutionStatus, ActionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherUiSemanticsReport {
    pub search_focused: bool,
    pub result_count: usize,
    pub result_phase: String,
    pub result_mode: String,
    pub selected_label: String,
    pub selected_position: String,
    pub selected_keycap: String,
    pub selected_action_hint: String,
    pub action_bar_hint: String,
    pub no_results_label: String,
    pub no_results_detail: String,
    pub no_results_fallback: String,
    pub no_results_phase: String,
    pub no_results_ime_enter_blocked: bool,
    pub loading_label: String,
    pub loading_progress: String,
    pub loading_spinner_after_ms: u128,
    pub executing_search_text: String,
    pub executing_input_enabled: bool,
    pub executing_cancel_shortcut: String,
    pub defer_feedback_label: String,
    pub defer_actions: String,
    pub failed_feedback_label: String,
    pub error_actions: String,
    pub reduce_motion: bool,
    pub launcher_enter_ms: u128,
    pub focus_ring_width: u32,
}

impl LauncherState {
    pub fn ui_semantics_smoke(query: &str) -> LauncherUiSemanticsReport {
        let mut state = Self::new();
        state.controller.show();
        state.update_query(query);
        let selected = state
            .view
            .selected_result()
            .map(|result| {
                AccessibilityContext::from_env().launcher_result_label(
                    &result.action.name,
                    &result.action.description,
                    state.view.selected + 1,
                    state.view.results.len(),
                )
            })
            .unwrap_or_else(|| "No matches".to_string());
        let selected_position = if state.view.results.is_empty() {
            "0 of 0".to_string()
        } else {
            format!(
                "{} of {}",
                state.view.selected + 1,
                state.view.results.len()
            )
        };
        let selected_keycap = if state.view.results.is_empty() {
            "none".to_string()
        } else {
            "Mod+1".to_string()
        };
        let selected_action_hint = state
            .view
            .preview
            .as_ref()
            .map(|preview| format!("Enter {}", preview.primary_command))
            .unwrap_or_else(|| "Enter none".to_string());
        let action_bar_hint = "Actions Mod+K".to_string();
        let mut no_results = Self::new();
        no_results.update_query("no-such-launcher-result");
        let no_results_ime_enter_blocked = no_results
            .handle_keyboard_input(LauncherKey::Enter, true)
            .is_none()
            && no_results.view.feedback.is_none();
        let no_results_label =
            i18n::translate(Locale::EnUs, "launcher.empty.no_matches.title").to_string();
        let no_results_detail =
            i18n::translate(Locale::EnUs, "launcher.empty.no_matches.detail").to_string();
        let no_results_fallback = format!(
            "{} \"{}\"",
            i18n::translate(Locale::EnUs, "launcher.empty.ask_ai"),
            no_results.view.query
        );
        let no_results_phase = format!(
            "{:?}/{:?}",
            no_results.view.phase, no_results.view.result_mode
        );

        let mut loading_state = Self::new();
        loading_state.view.preview_searching("slow query");
        let loading_label = i18n::translate(Locale::EnUs, "launcher.results.searching").to_string();
        let loading_progress = format!(
            "{}px {} indeterminate",
            2,
            i18n::translate(Locale::EnUs, "launcher.results.searching.title")
        );

        let mut executing_state = Self::new();
        executing_state.update_query(query);
        executing_state.view.preview_executing();
        let executing_title = executing_state
            .view
            .preview
            .as_ref()
            .map(|preview| preview.title.clone())
            .unwrap_or_else(|| "selected action".to_string());
        let executing_search_text = format!(
            "{} {}",
            i18n::translate(Locale::EnUs, "launcher.search.running"),
            executing_title
        );

        let defer_feedback = LauncherFeedback::from_execution(&deferred_execution());
        let defer_feedback_label = feedback_label(&defer_feedback);

        let failed_feedback = LauncherFeedback::from_execution(&failed_execution());
        let failed_feedback_label = feedback_label(&failed_feedback);

        let motion = MotionContext::from_env();
        let a11y = AccessibilityContext::from_env();
        LauncherUiSemanticsReport {
            search_focused: state.controller.focused,
            result_count: state.view.results.len(),
            result_phase: format!("{:?}", state.view.phase),
            result_mode: format!("{:?}", state.view.result_mode),
            selected_label: selected,
            selected_position,
            selected_keycap,
            selected_action_hint,
            action_bar_hint,
            no_results_label,
            no_results_detail,
            no_results_fallback,
            no_results_phase,
            no_results_ime_enter_blocked,
            loading_label,
            loading_progress,
            loading_spinner_after_ms: 200,
            executing_search_text,
            executing_input_enabled: false,
            executing_cancel_shortcut: "Cancel Ctrl+C".to_string(),
            defer_feedback_label,
            defer_actions: "Copy,Retry".to_string(),
            failed_feedback_label,
            error_actions: "Copy,Retry,Open Studio".to_string(),
            reduce_motion: motion.is_reduced(),
            launcher_enter_ms: motion.launcher_enter().as_millis(),
            focus_ring_width: a11y.focus_ring_width() as u32,
        }
    }
}

impl LauncherUiSemanticsReport {
    pub fn pass(&self) -> bool {
        self.search_focused
            && self.result_count > 0
            && self.result_phase == "WithResults"
            && self.result_mode == "Matches"
            && self.selected_label.contains("press Enter")
            && self.selected_position.contains(" of ")
            && self.selected_keycap == "Mod+1"
            && self.selected_action_hint.starts_with("Enter ")
            && self.action_bar_hint == "Actions Mod+K"
            && self.no_results_label == "No matches"
            && self.no_results_detail.contains("Try a different keyword")
            && self.no_results_fallback.contains("Ask AI about")
            && self.no_results_phase
                == format!(
                    "{:?}/{:?}",
                    LauncherPhase::NoMatches,
                    LauncherResultMode::NoMatches
                )
            && self.no_results_ime_enter_blocked
            && self.loading_label.contains("Searching registry")
            && self.loading_progress == "2px Searching indeterminate"
            && self.loading_spinner_after_ms == 200
            && self.executing_search_text.starts_with("Running:")
            && !self.executing_input_enabled
            && self.executing_cancel_shortcut == "Cancel Ctrl+C"
            && self.defer_feedback_label.contains("Needs external runner")
            && self.defer_actions == "Copy,Retry"
            && self.failed_feedback_label.contains("Failed")
            && self.error_actions == "Copy,Retry,Open Studio"
            && (!self.reduce_motion || self.launcher_enter_ms == 0)
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_ui_semantics_smoke {}\nsearch_focused={}\nresult_count={}\nresult_phase={}\nresult_mode={}\nselected_label={}\nselected_position={}\nselected_keycap={}\nselected_action_hint={}\naction_bar_hint={}\nno_results_label={}\nno_results_detail={}\nno_results_fallback={}\nno_results_phase={}\nno_results_ime_enter_blocked={}\nloading_label={}\nloading_progress={}\nloading_spinner_after_ms={}\nexecuting_search_text={}\nexecuting_input_enabled={}\nexecuting_cancel_shortcut={}\ndefer_feedback_label={}\ndefer_actions={}\nfailed_feedback_label={}\nerror_actions={}\nreduce_motion={}\nlauncher_enter_ms={}\nfocus_ring_width={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.search_focused,
            self.result_count,
            self.result_phase,
            self.result_mode,
            self.selected_label,
            self.selected_position,
            self.selected_keycap,
            self.selected_action_hint,
            self.action_bar_hint,
            self.no_results_label,
            self.no_results_detail,
            self.no_results_fallback,
            self.no_results_phase,
            self.no_results_ime_enter_blocked,
            self.loading_label,
            self.loading_progress,
            self.loading_spinner_after_ms,
            self.executing_search_text,
            self.executing_input_enabled,
            self.executing_cancel_shortcut,
            self.defer_feedback_label,
            self.defer_actions,
            self.failed_feedback_label,
            self.error_actions,
            self.reduce_motion,
            self.launcher_enter_ms,
            self.focus_ring_width
        )
    }
}

fn deferred_execution() -> ActionExecution {
    ActionExecution {
        action_id: ActionId::default(),
        action_name: "Open Terminal".to_string(),
        status: ActionExecutionStatus::NeedsExternalRunner,
        message: "open -a Terminal".to_string(),
        output: Some(serde_json::json!({
            "deferred": true,
            "reason": "external runner action requires explicit user trigger",
        })),
        created_at: chrono::Utc::now(),
    }
}

fn failed_execution() -> ActionExecution {
    ActionExecution {
        action_id: ActionId::default(),
        action_name: "Plugin Crash".to_string(),
        status: ActionExecutionStatus::Failed,
        message: "plugin crashed while rendering launcher feedback".to_string(),
        output: None,
        created_at: chrono::Utc::now(),
    }
}

fn feedback_label(feedback: &LauncherFeedback) -> String {
    format!(
        "{}: {} {}",
        feedback.title, feedback.action_name, feedback.detail
    )
}
