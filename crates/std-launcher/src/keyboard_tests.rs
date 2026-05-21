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
    assert_focus_and_editing(&report, &summary);
    assert!(summary.contains("launcher_keyboard_smoke PASS"));
}

fn assert_navigation(report: &LauncherKeyboardReport) {
    assert_eq!(report.selected_before, 0);
    assert!(report.selected_after_down > report.selected_before);
    assert_eq!(report.selected_after_up, report.selected_before);
}

fn assert_trigger_paths(report: &LauncherKeyboardReport, summary: &str) {
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
    assert!(report.user_enter_deferred);
    assert!(report.user_enter_feedback_visible);
    assert!(report.user_enter_keeps_launcher_open);
    assert!(summary.contains("direct_trigger_status=Completed"));
    assert!(summary.contains("user_enter_status=NeedsExternalRunner"));
    assert!(summary.contains("user_enter_deferred=true"));
    assert!(summary.contains("user_enter_feedback_visible=true"));
    assert!(summary.contains("user_enter_keeps_launcher_open=true"));
}

fn assert_ime_guard(report: &LauncherKeyboardReport, summary: &str) {
    assert!(report.ime_selection_unchanged);
    assert!(report.ime_action_panel_selection_unchanged);
    assert!(report.ime_trigger_blocked);
    assert!(report.ime_escape_blocked);
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
    assert!(summary.contains("ime_preedit_query_unchanged=true"));
    assert!(summary.contains("ime_commit_query=rebuild index"));
    assert!(summary
        .contains("ime_composition_path=zh-preedit(index)>blocked>commit(rebuild index)>enter"));
    assert!(summary.contains("ime_commit_trigger_status=Completed"));
}

fn assert_focus_and_editing(report: &LauncherKeyboardReport, summary: &str) {
    assert_eq!(report.focus_path, "Search>Results>Search");
    assert_eq!(report.action_panel_focus_path, "ActionPanel>Search");
    assert_eq!(report.token_delete_query, "open terminal");
    assert!(summary.contains("focus_path=Search>Results>Search"));
    assert!(summary.contains("action_panel_focus_path=ActionPanel>Search"));
    assert!(summary.contains("token_delete_query=open terminal"));
}
