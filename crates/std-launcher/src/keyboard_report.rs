use crate::{LauncherEnterWindowReport, LauncherFocusSection, LauncherKeyboardReport};
use std_types::ActionExecutionStatus;

impl LauncherKeyboardReport {
    pub fn pass(&self) -> bool {
        self.selected_after_down > self.selected_before
            && self.selected_after_up == self.selected_before
            && self
                .navigation_boundary_path
                .starts_with("top:0->0;bottom:")
            && self.navigation_boundary_path.ends_with("->same")
            && self.direct_trigger_status.is_some()
            && self.trigger_status.is_some()
            && self.user_enter_status == Some(ActionExecutionStatus::NeedsExternalRunner)
            && self.user_enter_route == "Enter>handle_keyboard_input_by_user>LauncherUser"
            && self.user_enter_deferred
            && self.user_enter_defer_reason == "STD_TEST_MODE blocked desktop open"
            && self
                .user_enter_open_contract
                .contains("ui_enter=handle_keyboard_input_by_user")
            && self.user_enter_open_contract.contains("mode=LauncherUser")
            && self
                .user_enter_open_contract
                .contains("runner=open <app-path>")
            && self
                .user_enter_open_contract
                .contains("test_gate=STD_TEST_MODE blocks before runner")
            && self
                .user_enter_open_contract
                .contains("hide_policy=Completed->hide,NeedsExternalRunner->keep-open")
            && self.user_enter_feedback_visible
            && self.user_enter_feedback_title == std_egui::i18n::t("launcher.feedback.deferred")
            && self.user_enter_keeps_launcher_open
            && self.user_enter_window_commands == "none"
            && self.pinned_enter_status == Some(ActionExecutionStatus::Completed)
            && self.pinned_enter_keeps_launcher_open
            && self.pinned_enter_window_commands == "none"
            && self.closed_after_escape
            && self.ime_selection_unchanged
            && self.ime_action_panel_selection_unchanged
            && self.ime_trigger_blocked
            && self.ime_escape_blocked
            && self.ime_enter_owned_by_ime
            && self.ime_preedit_query_unchanged
            && self.ime_commit_query == "rebuild index"
            && self.ime_composition_path == "zh-preedit(index)>blocked>commit(rebuild index)>enter"
            && self.ime_commit_trigger_status.is_some()
            && self.empty_suggestion_keyboard_path == "0->1->2->2=> > studio"
            && self.focus_after_tab == LauncherFocusSection::Results
            && self.focus_after_shift_tab == LauncherFocusSection::Search
            && self.focus_path == "Search>Results>Search"
            && self.action_panel_focus_path == "ActionPanel>Search"
            && self.completed_query == "rebuild index"
            && self.completion_focus_contract
                == "search-tab-completes=rebuild index;results-tab-focuses=Search;query=reb"
            && self
                .focus_visible_contract
                .contains("focus-ring=Search|Results|ActionPanel|Feedback")
            && self
                .focus_visible_contract
                .contains("source=keyboard-visible,pointer-hidden")
            && self.focus_visible_contract.contains("enter-owned-by-ime")
            && self.shortcut_help_contract.contains("trigger=?")
            && self.shortcut_help_contract.contains("help_visible=true")
            && self
                .shortcut_help_contract
                .contains("nl_action_visible=false")
            && self
                .shortcut_help_contract
                .contains(std_egui::i18n::t("launcher.shortcut_help.title"))
            && self
                .shortcut_help_contract
                .contains(std_egui::i18n::t("launcher.shortcut_help.move_selection"))
            && !self.shortcut_help_contract.contains("Mod+")
            && self.normalized_query == "open terminal now"
            && self.token_delete_query == "open terminal"
            && self.token_delete_normalized_query == "open terminal"
            && self.enter_window.pass()
            && self.ui_handler_contract == "ui-handler=ime-before-cancel-enter"
            && self.ime_visible_state_contract
                == "ime-visible-state=search-preedit-visible,enter-owned-by-ime"
            && self.model_contract
                == "model=keyboard-navigation,ime-guard,user-enter-defer,no-desktop-events"
            && self.real_interaction_contract
                == "real-focus-enter-toggle=requires-STD_ALLOW_BACKGROUND_UI_AUTOMATION"
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_keyboard_smoke {}\nselected_before={}\nselected_after_down={}\nselected_after_up={}\nnavigation_boundary_path={}\ndirect_trigger_status={}\ntrigger_status={}\nuser_enter_status={}\nuser_enter_route={}\nuser_enter_deferred={}\nuser_enter_defer_reason={}\nuser_enter_open_contract={}\nuser_enter_feedback_visible={}\nuser_enter_feedback_title={}\nuser_enter_keeps_launcher_open={}\nuser_enter_window_commands={}\npinned_enter_status={}\npinned_enter_keeps_launcher_open={}\npinned_enter_window_commands={}\nclosed_after_escape={}\nime_selection_unchanged={}\nime_action_panel_selection_unchanged={}\nime_trigger_blocked={}\nime_escape_blocked={}\nime_enter_owned_by_ime={}\nime_composition_path={}\nime_preedit_query_unchanged={}\nime_commit_query={}\nime_commit_trigger_status={}\nempty_suggestion_keyboard_path={}\nfocus_after_tab={:?}\nfocus_after_shift_tab={:?}\nfocus_path={}\naction_panel_focus_path={}\ncompleted_query={}\ncompletion_focus_contract={}\nfocus_visible_contract={}\nshortcut_help_contract={}\nnormalized_query={}\ntoken_delete_query={}\ntoken_delete_normalized_query={}\nenter_window={}\nui_handler_contract={}\nime_visible_state_contract={}\nmodel_contract={}\nreal_interaction_contract={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.selected_before,
            self.selected_after_down,
            self.selected_after_up,
            self.navigation_boundary_path,
            status_label(self.direct_trigger_status.as_ref()),
            status_label(self.trigger_status.as_ref()),
            status_label(self.user_enter_status.as_ref()),
            self.user_enter_route,
            self.user_enter_deferred,
            self.user_enter_defer_reason,
            self.user_enter_open_contract,
            self.user_enter_feedback_visible,
            self.user_enter_feedback_title,
            self.user_enter_keeps_launcher_open,
            self.user_enter_window_commands,
            status_label(self.pinned_enter_status.as_ref()),
            self.pinned_enter_keeps_launcher_open,
            self.pinned_enter_window_commands,
            self.closed_after_escape,
            self.ime_selection_unchanged,
            self.ime_action_panel_selection_unchanged,
            self.ime_trigger_blocked,
            self.ime_escape_blocked,
            self.ime_enter_owned_by_ime,
            self.ime_composition_path,
            self.ime_preedit_query_unchanged,
            self.ime_commit_query,
            status_label(self.ime_commit_trigger_status.as_ref()),
            self.empty_suggestion_keyboard_path,
            self.focus_after_tab,
            self.focus_after_shift_tab,
            self.focus_path,
            self.action_panel_focus_path,
            self.completed_query,
            self.completion_focus_contract,
            self.focus_visible_contract,
            self.shortcut_help_contract,
            self.normalized_query,
            self.token_delete_query,
            self.token_delete_normalized_query,
            self.enter_window.summary(),
            self.ui_handler_contract,
            self.ime_visible_state_contract,
            self.model_contract,
            self.real_interaction_contract
        )
    }
}

impl LauncherEnterWindowReport {
    pub fn pass(&self) -> bool {
        self.completed_status == Some(ActionExecutionStatus::Completed)
            && self.completed_hide_requested
            && self.completed_window_commands == "ResizeToHiddenHost,Visible(false)"
            && self.deferred_status == Some(ActionExecutionStatus::NeedsExternalRunner)
            && !self.deferred_hide_requested
            && self.deferred_window_commands == "none"
    }

    pub fn summary(&self) -> String {
        format!(
            "completed_status={};completed_hide={};completed_commands={};deferred_status={};deferred_hide={};deferred_commands={}",
            status_label(self.completed_status.as_ref()),
            self.completed_hide_requested,
            self.completed_window_commands,
            status_label(self.deferred_status.as_ref()),
            self.deferred_hide_requested,
            self.deferred_window_commands
        )
    }
}

fn status_label(status: Option<&ActionExecutionStatus>) -> String {
    status
        .map(|status| format!("{status:?}"))
        .unwrap_or_else(|| "none".to_string())
}
