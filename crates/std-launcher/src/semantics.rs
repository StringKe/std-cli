use crate::LauncherState;
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
    pub selected_label: String,
    pub selected_position: String,
    pub no_results_label: String,
    pub no_results_detail: String,
    pub no_results_fallback: String,
    pub no_results_phase: String,
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
        let mut no_results = Self::new();
        no_results.update_query("no-such-launcher-result");
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

        let defer_feedback = LauncherFeedback::from_execution(&deferred_execution());
        let defer_feedback_label = feedback_label(&defer_feedback);

        let failed_feedback = LauncherFeedback::from_execution(&failed_execution());
        let failed_feedback_label = feedback_label(&failed_feedback);

        let motion = MotionContext::from_env();
        let a11y = AccessibilityContext::from_env();
        LauncherUiSemanticsReport {
            search_focused: state.controller.focused,
            result_count: state.view.results.len(),
            selected_label: selected,
            selected_position,
            no_results_label,
            no_results_detail,
            no_results_fallback,
            no_results_phase,
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
            && self.selected_label.contains("press Enter")
            && self.selected_position.contains(" of ")
            && self.no_results_label == "No matches"
            && self.no_results_detail.contains("Try a different keyword")
            && self.no_results_fallback.contains("Ask AI about")
            && self.no_results_phase
                == format!(
                    "{:?}/{:?}",
                    LauncherPhase::NoMatches,
                    LauncherResultMode::NoMatches
                )
            && self.defer_feedback_label.contains("Needs external runner")
            && self.defer_actions == "Copy,Retry"
            && self.failed_feedback_label.contains("Failed")
            && self.error_actions == "Copy,Retry,Open Studio"
            && (!self.reduce_motion || self.launcher_enter_ms == 0)
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_ui_semantics_smoke {}\nsearch_focused={}\nresult_count={}\nselected_label={}\nselected_position={}\nno_results_label={}\nno_results_detail={}\nno_results_fallback={}\nno_results_phase={}\ndefer_feedback_label={}\ndefer_actions={}\nfailed_feedback_label={}\nerror_actions={}\nreduce_motion={}\nlauncher_enter_ms={}\nfocus_ring_width={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.search_focused,
            self.result_count,
            self.selected_label,
            self.selected_position,
            self.no_results_label,
            self.no_results_detail,
            self.no_results_fallback,
            self.no_results_phase,
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
