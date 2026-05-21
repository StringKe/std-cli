use super::*;
use std_studio::{StudioPane, WorkspacePane, WorkspacePaneKind};

#[test]
fn workspace_tab_specs_mark_only_focused_pane() {
    let first = WorkspacePane::new(
        WorkspacePaneId::new(1),
        WorkspacePaneKind::Pane(StudioPane::Dashboard),
        1,
    );
    let second = WorkspacePane::new(
        WorkspacePaneId::new(2),
        WorkspacePaneKind::Pane(StudioPane::Settings),
        2,
    );

    let specs = workspace_tab_specs(&[first, second], Some(WorkspacePaneId::new(2)));

    assert_eq!(specs.len(), 2);
    assert!(!specs[0].focused);
    assert!(specs[1].focused);
    assert_eq!(specs[1].title, "Settings");
}

#[test]
fn close_tab_keyboard_command_targets_focused_pane() {
    assert_eq!(
        workspace_tab_keyboard_command(Some(WorkspacePaneId::new(7))),
        Some(StudioWorkspaceCommand::Close(WorkspacePaneId::new(7)))
    );
    assert_eq!(workspace_tab_keyboard_command(None), None);
}

#[test]
fn cycle_controls_use_workspace_focus_commands() {
    assert_eq!(i18n::t("studio.workspace_panes.previous"), "Previous");
    assert_eq!(i18n::t("studio.workspace_panes.next"), "Next");
    assert_eq!(
        workspace_tab_cycle_commands(),
        [
            StudioWorkspaceCommand::FocusPrevious,
            StudioWorkspaceCommand::FocusNext
        ]
    );
    assert!(std_egui::input::studio_previous_workspace_pane()
        .label()
        .contains("Shift+Up"));
    assert!(std_egui::input::studio_next_workspace_pane()
        .label()
        .contains("Shift+Down"));
}

#[test]
fn workspace_tab_a11y_labels_include_role_title_and_state() {
    let spec = WorkspaceTabSpec {
        id: WorkspacePaneId::new(9),
        title: "Workflow Builder".to_string(),
        focused: true,
    };

    assert_eq!(
        workspace_tab_a11y_label(&spec),
        "Workspace pane tab, Workflow Builder, focused"
    );
    assert_eq!(
        workspace_tab_close_a11y_label(&spec),
        "Close workspace pane, Workflow Builder"
    );
    assert_eq!(workspace_cycle_a11y_label("Next"), "Next workspace pane");
}

#[test]
fn workspace_tab_width_is_stable_and_bounded() {
    assert_eq!(workspace_tab_width("Settings"), TAB_MIN_WIDTH);
    assert_eq!(
        workspace_tab_width("Very Long Workflow Builder Workspace"),
        TAB_MAX_WIDTH
    );
}

#[test]
fn workspace_tab_close_rect_uses_token_sized_hit_target() {
    let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(180.0, TAB_HEIGHT));
    let close = workspace_tab_close_rect(rect);

    assert_eq!(close.width(), TAB_CLOSE_HIT_SIZE);
    assert_eq!(close.height(), TAB_HEIGHT);
    assert_eq!(close.right(), rect.right());
}
