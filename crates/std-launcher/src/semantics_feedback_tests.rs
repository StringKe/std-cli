use super::feedback_semantics;

#[test]
fn feedback_semantics_contract_covers_action_order_keyboard_and_a11y() {
    let feedback = feedback_semantics();
    assert_eq!(feedback.defer_actions, "Copy,Retry");
    assert_eq!(feedback.error_actions, "Copy,Retry,Open Studio");
    assert_eq!(
        feedback.contract,
        "defer=Copy>Retry,error=Copy>Retry>OpenStudio,keyboard=copy>retry>open-studio"
    );
    assert_eq!(
        feedback.a11y_contract,
        "panel=status>target>actions,actions=action>target>status>enter"
    );
    assert!(feedback.keyboard_path.contains("Retry"));
    assert!(feedback.keyboard_path.contains("OpenStudio"));
    assert!(feedback.keyboard_path.contains("studio-pane://history"));
}

#[test]
fn feedback_error_open_studio_routes_to_execution_history() {
    let feedback = feedback_semantics();
    assert_eq!(feedback.open_studio_target, "ExecutionHistory");
    assert_eq!(feedback.open_studio_command, "studio-pane://history");
}
