use crate::LauncherUiSemanticsReport;
use std_egui::{
    i18n::{self, Locale},
    input, LauncherPhase, LauncherResultMode,
};

impl LauncherUiSemanticsReport {
    pub fn pass(&self) -> bool {
        self.search_focused
            && self.result_count > 0
            && self.result_phase == "WithResults"
            && self.result_mode == "Matches"
            && self.empty_phase == "Empty"
            && self.empty_mode == "SuggestedWorkflows"
            && self.empty_result_count > 0
            && self.empty_title == "Suggested Workflows"
            && self.empty_detail.contains("Press / for commands")
            && self.search_reader_label.contains("index")
            && self
                .result_group_label
                .contains(i18n::t("launcher.results.group.action_workflow"))
            && self.selected_label.contains("按 Enter")
            && self.selected_reader_label == self.selected_label
            && self.selected_position.contains(" of ")
            && self.selected_keycap == input::launcher_result_keycap(0).unwrap()
            && self
                .selected_action_hint
                .starts_with(&format!("{} ", input::enter().label()))
            && self.action_bar_hint
                == format!(
                    "{} {}",
                    i18n::translate(Locale::EnUs, "launcher.action.actions"),
                    input::launcher_action_panel().label()
                )
            && self
                .action_panel_actions
                .contains(i18n::t("launcher.action.open_in_studio"))
            && self.action_panel_reader_label.contains("Rebuild Index")
            && self.action_panel_reader_label.contains("3")
            && self
                .action_panel_open_studio_command
                .starts_with("studio-pane://")
            && self.no_results_label == "No matches"
            && self.no_results_detail.contains("Try a different keyword")
            && self.no_results_fallback.contains("Ask AI about")
            && self.no_results_phase
                == format!(
                    "{:?}/{:?}",
                    LauncherPhase::NoMatches,
                    LauncherResultMode::NoMatches
                )
            && self.no_results_enter_query == "? no-such-launcher-result"
            && self.no_results_ime_enter_blocked
            && self.loading_label.contains("Searching registry")
            && self.loading_progress == "2px Searching indeterminate"
            && self.loading_spinner_after_ms == 200
            && self.executing_search_text.starts_with("Running:")
            && self
                .running_reader_label
                .starts_with(i18n::t("launcher.a11y.running").trim_end_matches("{action}"))
            && !self.executing_input_enabled
            && self.executing_cancel_shortcut
                == format!(
                    "{} {}",
                    i18n::translate(Locale::EnUs, "launcher.action.cancel"),
                    input::launcher_cancel().label()
                )
            && self.executing_background_shortcut
                == format!(
                    "{} {}",
                    i18n::translate(Locale::EnUs, "launcher.action.background"),
                    input::enter().label()
                )
            && self
                .defer_feedback_label
                .contains(std_egui::i18n::t("launcher.feedback.deferred"))
            && self.defer_actions == "Copy,Retry"
            && self
                .failed_feedback_label
                .contains(std_egui::i18n::t("launcher.feedback.failed"))
            && self
                .completion_reader_label
                .contains("external runner action requires explicit user trigger")
            && self.error_actions == "Copy,Retry,Open Studio"
            && self.feedback_keyboard_path.contains(&format!(
                "{}:Retry>{}:OpenStudio>{}:studio-pane://history",
                input::arrow_down().label(),
                input::arrow_down().label(),
                input::enter().label()
            ))
            && self.error_open_studio_target == "ExecutionHistory"
            && self.error_open_studio_command == "studio-pane://history"
            && self.shortcut_help_summary.contains("trigger=?")
            && self
                .shortcut_help_summary
                .contains(&input::launcher_action_panel().label())
            && !self.shortcut_help_summary.contains("Mod+")
            && self.docs23_contract == "docs/23#launcher-screen-reader"
            && self.locale_contract == "zh-CN,en-US"
            && (!self.reduce_motion || self.launcher_enter_ms == 0)
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_ui_semantics_smoke {}\nsearch_focused={}\nresult_count={}\nresult_phase={}\nresult_mode={}\nempty_phase={}\nempty_mode={}\nempty_result_count={}\nempty_title={}\nempty_detail={}\nsearch_reader_label={}\nresult_group_label={}\nselected_label={}\nselected_reader_label={}\nselected_position={}\nselected_keycap={}\nselected_action_hint={}\naction_bar_hint={}\naction_panel_actions={}\naction_panel_reader_label={}\naction_panel_open_studio_command={}\nno_results_label={}\nno_results_detail={}\nno_results_fallback={}\nno_results_phase={}\nno_results_enter_query={}\nno_results_ime_enter_blocked={}\nloading_label={}\nloading_progress={}\nloading_spinner_after_ms={}\nexecuting_search_text={}\nrunning_reader_label={}\nexecuting_input_enabled={}\nexecuting_cancel_shortcut={}\nexecuting_background_shortcut={}\ndefer_feedback_label={}\ndefer_actions={}\nfailed_feedback_label={}\ncompletion_reader_label={}\nerror_actions={}\nfeedback_keyboard_path={}\nerror_open_studio_target={}\nerror_open_studio_command={}\nshortcut_help={}\ndocs23_contract={}\nlocale_contract={}\nreduce_motion={}\nlauncher_enter_ms={}\nfocus_ring_width={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.search_focused,
            self.result_count,
            self.result_phase,
            self.result_mode,
            self.empty_phase,
            self.empty_mode,
            self.empty_result_count,
            self.empty_title,
            self.empty_detail,
            self.search_reader_label,
            self.result_group_label,
            self.selected_label,
            self.selected_reader_label,
            self.selected_position,
            self.selected_keycap,
            self.selected_action_hint,
            self.action_bar_hint,
            self.action_panel_actions,
            self.action_panel_reader_label,
            self.action_panel_open_studio_command,
            self.no_results_label,
            self.no_results_detail,
            self.no_results_fallback,
            self.no_results_phase,
            self.no_results_enter_query,
            self.no_results_ime_enter_blocked,
            self.loading_label,
            self.loading_progress,
            self.loading_spinner_after_ms,
            self.executing_search_text,
            self.running_reader_label,
            self.executing_input_enabled,
            self.executing_cancel_shortcut,
            self.executing_background_shortcut,
            self.defer_feedback_label,
            self.defer_actions,
            self.failed_feedback_label,
            self.completion_reader_label,
            self.error_actions,
            self.feedback_keyboard_path,
            self.error_open_studio_target,
            self.error_open_studio_command,
            self.shortcut_help_summary,
            self.docs23_contract,
            self.locale_contract,
            self.reduce_motion,
            self.launcher_enter_ms,
            self.focus_ring_width
        )
    }
}
