use super::*;

#[test]
fn closeguard_restore_replaces_same_id_identity_conflict() {
    let mut studio = test_studio();
    let workflow_path = studio
        .create_workflow("Identity Conflict", "Restore exact pane")
        .unwrap();
    let workflow = studio.open_workflow_builder(workflow_path.clone());
    assert!(studio.focus_workspace_pane(workflow));

    let closeguard = studio.close_workspace_instance();
    let conflicted = studio
        .workspace_panes
        .iter_mut()
        .find(|pane| pane.id == workflow)
        .unwrap();
    conflicted.kind = WorkspacePaneKind::Settings;
    conflicted.title = "Wrong Settings".to_string();
    conflicted.open = false;

    studio.restore_workspace_closeguard(&closeguard);

    let restored = studio
        .workspace_panes
        .iter()
        .find(|pane| pane.id == workflow)
        .unwrap();
    assert_eq!(
        restored.kind,
        WorkspacePaneKind::WorkflowBuilder {
            workflow_path: workflow_path.clone()
        }
    );
    assert_eq!(
        restored.kind.identity_key(),
        format!("workflow:{}", workflow_path.display())
    );
    assert_eq!(restored.title, "Workflow Builder: workflow.md");
    assert!(restored.open);
    assert_eq!(studio.focused_pane, Some(workflow));
    assert!(studio
        .workspace_pane_content(&restored.kind)
        .lines
        .contains(&format!("path={}", workflow_path.display())));
}
