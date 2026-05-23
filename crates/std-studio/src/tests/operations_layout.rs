#[test]
fn operations_narrow_layout_keeps_workspace_policy_visible() {
    let source = include_str!("../operations.rs");
    let narrow_branch = source
        .split("if available_width < 920.0 {")
        .nth(1)
        .and_then(|body| body.split("return;").next())
        .unwrap();

    assert!(narrow_branch.contains("render_evidence_gate(ui, &evidence.runtime)"));
    assert!(narrow_branch.contains("workspace_policy_evidence::render_with_state(ui, &self.app)"));
    assert!(narrow_branch.contains("render_completion_gate(ui, evidence)"));
}
