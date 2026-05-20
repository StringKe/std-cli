use crate::LauncherState;
use std_egui::{a11y::AccessibilityContext, motion::MotionContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherUiSemanticsReport {
    pub search_focused: bool,
    pub result_count: usize,
    pub selected_label: String,
    pub selected_position: String,
    pub feedback_label: String,
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
        state.trigger_selected();
        let feedback_label = state
            .view
            .feedback
            .as_ref()
            .map(|feedback| {
                format!(
                    "{}: {} {}",
                    feedback.title, feedback.action_name, feedback.detail
                )
            })
            .unwrap_or_else(|| "Ready".to_string());
        let motion = MotionContext::from_env();
        let a11y = AccessibilityContext::from_env();
        LauncherUiSemanticsReport {
            search_focused: state.controller.focused,
            result_count: state.view.results.len(),
            selected_label: selected,
            selected_position,
            feedback_label,
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
            && !self.feedback_label.is_empty()
            && (!self.reduce_motion || self.launcher_enter_ms == 0)
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_ui_semantics_smoke {}\nsearch_focused={}\nresult_count={}\nselected_label={}\nselected_position={}\nfeedback_label={}\nreduce_motion={}\nlauncher_enter_ms={}\nfocus_ring_width={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.search_focused,
            self.result_count,
            self.selected_label,
            self.selected_position,
            self.feedback_label,
            self.reduce_motion,
            self.launcher_enter_ms,
            self.focus_ring_width
        )
    }
}
