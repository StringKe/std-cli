use std_launcher::LauncherState;

#[test]
fn launcher_ui_semantics_covers_empty_query_suggestions() {
    let report = LauncherState::ui_semantics_smoke("index");
    let summary = report.summary();

    assert!(report.pass(), "{summary}");
    assert_eq!(report.empty_phase, "Empty");
    assert_eq!(report.empty_mode, "SuggestedWorkflows");
    assert!(report.empty_result_count > 0);
    assert_eq!(report.empty_title, "Suggested Workflows");
    assert!(report.empty_detail.contains("Press / for commands"));
    assert!(summary.contains("empty_phase=Empty"));
    assert!(summary.contains("empty_mode=SuggestedWorkflows"));
}
