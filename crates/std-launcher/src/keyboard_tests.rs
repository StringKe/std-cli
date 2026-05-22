use super::*;
use std_types::ActionExecutionStatus;

#[test]
fn launcher_keyboard_smoke_validates_navigation_trigger_escape_and_ime_guard() {
    let report = LauncherState::keyboard_smoke("index");
    let summary = report.summary();

    assert!(report.pass(), "{summary}");
    assert_navigation(&report);
    assert_trigger_paths(&report, &summary);
    assert_ime_guard(&report, &summary);
    assert_empty_suggestions(&report, &summary);
    assert_focus_and_editing(&report, &summary);
    assert_interaction_boundary(&report, &summary);
    assert!(summary.contains("launcher_keyboard_smoke PASS"));
}

#[test]
fn launcher_ui_keyboard_uses_input_ime_guard_before_actions() {
    let source = include_str!("ui_keyboard.rs");
    let cancel_index = source
        .find("input::launcher_cancel().pressed(ctx)")
        .unwrap();
    let guard_index = source.find("input::ime_composing(ctx)").unwrap();
    let enter_index = source.find("input::enter().pressed(ctx)").unwrap();
    let action_panel_index = source.find("launcher_action_panel().pressed(ctx)").unwrap();
    let direct_trigger_index = source.find("pressed_mod_number(ctx, 9)").unwrap();

    assert!(!source.contains("tokens::ime_composing"));
    assert!(guard_index < cancel_index);
    assert!(guard_index < enter_index);
    assert!(guard_index < action_panel_index);
    assert!(guard_index < direct_trigger_index);
}

#[test]
fn launcher_ui_cancel_respects_ime_composition() {
    let source = include_str!("ui_keyboard.rs");
    let cancel_index = source
        .find("input::launcher_cancel().pressed(ctx)")
        .unwrap();
    let cancel_route_index = source.find("LauncherKey::CancelExecuting, false").unwrap();
    let guard_index = source.find("input::ime_composing(ctx)").unwrap();

    assert!(guard_index < cancel_index);
    assert!(guard_index < cancel_route_index);
}

#[test]
fn launcher_ui_tab_completes_query_before_focus_cycling_when_search_is_focused() {
    let source = include_str!("ui_keyboard.rs");
    let tab_index = source.find("input::tab().pressed(ctx)").unwrap();
    let search_focus_index = source
        .find("state.focus_section == std_launcher::LauncherFocusSection::Search")
        .unwrap();
    let complete_index = source.find("LauncherKey::CompleteSelectedQuery").unwrap();
    let focus_index = source.find("LauncherKey::FocusNext").unwrap();

    assert!(tab_index < complete_index);
    assert!(search_focus_index < complete_index);
    assert!(complete_index < focus_index);
}

fn assert_empty_suggestions(report: &LauncherKeyboardReport, summary: &str) {
    assert_eq!(
        report.empty_suggestion_keyboard_path,
        "0->1->2->2=> > studio"
    );
    assert!(summary.contains("empty_suggestion_keyboard_path=0->1->2->2=> > studio"));
}

fn assert_navigation(report: &LauncherKeyboardReport) {
    assert_eq!(report.selected_before, 0);
    assert!(report.selected_after_down > report.selected_before);
    assert_eq!(report.selected_after_up, report.selected_before);
    assert!(report
        .navigation_boundary_path
        .starts_with("top:0->0;bottom:"));
    assert!(report.navigation_boundary_path.ends_with("->same"));
}

fn assert_trigger_paths(report: &LauncherKeyboardReport, summary: &str) {
    assert_trigger_statuses(report);
    assert_deferred_enter_feedback(report);
    assert_trigger_summary(summary);
}

fn assert_trigger_statuses(report: &LauncherKeyboardReport) {
    assert_eq!(
        report.trigger_status,
        Some(ActionExecutionStatus::Completed)
    );
    assert_eq!(
        report.direct_trigger_status,
        Some(ActionExecutionStatus::Completed)
    );
    assert_eq!(
        report.user_enter_status,
        Some(ActionExecutionStatus::NeedsExternalRunner)
    );
    assert_eq!(
        report.user_enter_route,
        "Enter>handle_keyboard_input_by_user>LauncherUser"
    );
}

fn assert_deferred_enter_feedback(report: &LauncherKeyboardReport) {
    assert!(report.user_enter_deferred);
    assert_eq!(
        report.user_enter_defer_reason,
        "STD_TEST_MODE blocked desktop open"
    );
    assert!(report.user_enter_feedback_visible);
    assert_eq!(
        report.user_enter_feedback_title,
        std_egui::i18n::t("launcher.feedback.deferred")
    );
    assert!(report.user_enter_keeps_launcher_open);
    assert_eq!(report.user_enter_window_commands, "none");
    assert_eq!(
        report.pinned_enter_status,
        Some(ActionExecutionStatus::Completed)
    );
    assert!(report.pinned_enter_keeps_launcher_open);
    assert_eq!(report.pinned_enter_window_commands, "none");
    assert!(report.enter_window.pass());
}

fn assert_trigger_summary(summary: &str) {
    assert!(summary.contains("direct_trigger_status=Completed"));
    assert!(summary.contains("user_enter_status=NeedsExternalRunner"));
    assert!(summary.contains("user_enter_route=Enter>handle_keyboard_input_by_user>LauncherUser"));
    assert!(summary.contains("user_enter_deferred=true"));
    assert!(summary.contains("user_enter_defer_reason=STD_TEST_MODE blocked desktop open"));
    assert!(summary.contains("user_enter_feedback_visible=true"));
    assert!(summary.contains(&format!(
        "user_enter_feedback_title={}",
        std_egui::i18n::t("launcher.feedback.deferred")
    )));
    assert!(summary.contains("user_enter_keeps_launcher_open=true"));
    assert!(summary.contains("user_enter_window_commands=none"));
    assert!(summary.contains("pinned_enter_status=Completed"));
    assert!(summary.contains("pinned_enter_keeps_launcher_open=true"));
    assert!(summary.contains("pinned_enter_window_commands=none"));
    assert!(summary.contains("enter_window=completed_status=Completed"));
    assert!(summary.contains("completed_hide=true"));
    assert!(summary.contains("completed_commands=ResizeToHiddenHost,Visible(false)"));
    assert!(summary.contains("deferred_status=NeedsExternalRunner"));
    assert!(summary.contains("deferred_hide=false"));
    assert!(summary.contains("deferred_commands=none"));
}

fn assert_ime_guard(report: &LauncherKeyboardReport, summary: &str) {
    assert!(report.ime_selection_unchanged);
    assert!(report.ime_action_panel_selection_unchanged);
    assert!(report.ime_trigger_blocked);
    assert!(report.ime_escape_blocked);
    assert!(report.ime_enter_owned_by_ime);
    assert!(report.ime_preedit_query_unchanged);
    assert_eq!(report.ime_commit_query, "rebuild index");
    assert_eq!(
        report.ime_composition_path,
        "zh-preedit(index)>blocked>commit(rebuild index)>enter"
    );
    assert_eq!(
        report.ime_commit_trigger_status,
        Some(ActionExecutionStatus::Completed)
    );
    assert!(summary.contains("ime_action_panel_selection_unchanged=true"));
    assert!(summary.contains("ime_enter_owned_by_ime=true"));
    assert!(summary.contains("ime_preedit_query_unchanged=true"));
    assert!(summary.contains("ime_commit_query=rebuild index"));
    assert!(summary
        .contains("ime_composition_path=zh-preedit(index)>blocked>commit(rebuild index)>enter"));
    assert!(summary.contains("ime_commit_trigger_status=Completed"));
}

fn assert_focus_and_editing(report: &LauncherKeyboardReport, summary: &str) {
    assert_eq!(report.focus_path, "Search>Results>Search");
    assert_eq!(report.action_panel_focus_path, "ActionPanel>Search");
    assert_eq!(report.completed_query, "rebuild index");
    assert_eq!(
        report.completion_focus_contract,
        "search-tab-completes=rebuild index;results-tab-focuses=Search;query=reb"
    );
    assert!(report
        .focus_visible_contract
        .contains("focus-ring=Search|Results|ActionPanel|Feedback"));
    assert!(report
        .focus_visible_contract
        .contains("source=keyboard-visible,pointer-hidden"));
    assert!(report.focus_visible_contract.contains("enter-owned-by-ime"));
    assert_eq!(report.normalized_query, "open terminal now");
    assert_eq!(report.token_delete_query, "open terminal");
    assert_eq!(report.token_delete_normalized_query, "open terminal");
    assert!(summary.contains("focus_path=Search>Results>Search"));
    assert!(summary.contains("action_panel_focus_path=ActionPanel>Search"));
    assert!(summary.contains("completed_query=rebuild index"));
    assert!(summary.contains(
        "completion_focus_contract=search-tab-completes=rebuild index;results-tab-focuses=Search;query=reb"
    ));
    assert!(
        summary.contains("focus_visible_contract=focus-ring=Search|Results|ActionPanel|Feedback")
    );
    assert!(summary.contains("source=keyboard-visible,pointer-hidden"));
    assert!(summary.contains("ime=preedit-keeps-focus,enter-owned-by-ime"));
    assert!(summary.contains("normalized_query=open terminal now"));
    assert!(summary.contains("token_delete_query=open terminal"));
    assert!(summary.contains("token_delete_normalized_query=open terminal"));
    assert!(summary.contains("navigation_boundary_path=top:0->0;bottom:"));
}

fn assert_interaction_boundary(report: &LauncherKeyboardReport, summary: &str) {
    assert_eq!(
        report.model_contract,
        "model=keyboard-navigation,ime-guard,user-enter-defer,no-desktop-events"
    );
    assert_eq!(
        report.ui_handler_contract,
        "ui-handler=ime-before-cancel-enter"
    );
    assert_eq!(
        report.ime_visible_state_contract,
        "ime-visible-state=search-preedit-visible,enter-owned-by-ime"
    );
    assert_eq!(
        report.real_interaction_contract,
        "real-focus-enter-toggle=requires-STD_ALLOW_BACKGROUND_UI_AUTOMATION"
    );
    assert!(summary.contains("ui_handler_contract=ui-handler=ime-before-cancel-enter"));
    assert!(summary.contains(
        "ime_visible_state_contract=ime-visible-state=search-preedit-visible,enter-owned-by-ime"
    ));
    assert!(summary.contains(
        "model_contract=model=keyboard-navigation,ime-guard,user-enter-defer,no-desktop-events"
    ));
    assert!(summary.contains(
        "real_interaction_contract=real-focus-enter-toggle=requires-STD_ALLOW_BACKGROUND_UI_AUTOMATION"
    ));
}
