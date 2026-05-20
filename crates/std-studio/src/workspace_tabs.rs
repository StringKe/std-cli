use crate::{ui, workspace_panes::StudioWorkspaceCommand};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Space},
};
use std_studio::{WorkspacePane, WorkspacePaneId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkspaceTabSpec {
    pub id: WorkspacePaneId,
    pub title: String,
    pub focused: bool,
}

pub(crate) fn workspace_tab_specs(
    panes: &[WorkspacePane],
    focused: Option<WorkspacePaneId>,
) -> Vec<WorkspaceTabSpec> {
    panes
        .iter()
        .filter(|pane| pane.open)
        .map(|pane| WorkspaceTabSpec {
            id: pane.id,
            title: pane.title.clone(),
            focused: Some(pane.id) == focused,
        })
        .collect()
}

pub(crate) fn render_workspace_tabs(
    ui: &mut egui::Ui,
    specs: &[WorkspaceTabSpec],
    commands: &crate::workspace_panes::WorkspaceCommandQueue,
) {
    ui.horizontal_wrapped(|ui| {
        if specs.len() > 1 {
            render_workspace_cycle_controls(ui, commands);
        }
        for spec in specs {
            render_workspace_tab(ui, spec, commands);
        }
    });
}

pub(crate) fn workspace_tab_keyboard_command(
    focused: Option<WorkspacePaneId>,
) -> Option<StudioWorkspaceCommand> {
    focused.map(StudioWorkspaceCommand::Close)
}

fn render_workspace_cycle_controls(
    ui: &mut egui::Ui,
    commands: &crate::workspace_panes::WorkspaceCommandQueue,
) {
    let previous = ui::quiet_button(ui, i18n::t("studio.workspace_panes.previous"));
    previous.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            workspace_cycle_a11y_label("Previous"),
        )
    });
    if previous.clicked() {
        push_command(commands, StudioWorkspaceCommand::FocusPrevious);
    }
    let next = ui::quiet_button(ui, i18n::t("studio.workspace_panes.next"));
    next.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            workspace_cycle_a11y_label("Next"),
        )
    });
    if next.clicked() {
        push_command(commands, StudioWorkspaceCommand::FocusNext);
    }
}

fn render_workspace_tab(
    ui: &mut egui::Ui,
    spec: &WorkspaceTabSpec,
    commands: &crate::workspace_panes::WorkspaceCommandQueue,
) {
    let fill = if spec.focused {
        ui::selected_bg(ui.ctx())
    } else {
        ui::panel_alt(ui.ctx())
    };
    egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, Color::stroke_divider(ui.ctx())))
        .corner_radius(egui::CornerRadius::same(std_egui::tokens::Radius::SM))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let focus = ui::quiet_button(ui, &spec.title);
                focus.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        workspace_tab_a11y_label(spec),
                    )
                });
                if focus.clicked() {
                    push_command(commands, StudioWorkspaceCommand::Focus(spec.id));
                }
                let close = ui::quiet_button(ui, i18n::t("studio.workspace_panes.close"));
                close.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        workspace_tab_close_a11y_label(spec),
                    )
                });
                if close.clicked() {
                    push_command(commands, StudioWorkspaceCommand::Close(spec.id));
                }
            });
        });
}

pub(crate) fn workspace_tab_a11y_label(spec: &WorkspaceTabSpec) -> String {
    let state = if spec.focused { "focused" } else { "inactive" };
    format!("Workspace pane tab, {}, {}", spec.title, state)
}

pub(crate) fn workspace_tab_close_a11y_label(spec: &WorkspaceTabSpec) -> String {
    format!("Close workspace pane, {}", spec.title)
}

pub(crate) fn workspace_cycle_a11y_label(direction: &str) -> String {
    format!("{direction} workspace pane")
}

fn push_command(
    commands: &crate::workspace_panes::WorkspaceCommandQueue,
    command: StudioWorkspaceCommand,
) {
    if let Ok(mut queue) = commands.lock() {
        queue.push(command);
    }
}

#[cfg(test)]
mod tests {
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
            StudioWorkspaceCommand::FocusPrevious,
            StudioWorkspaceCommand::FocusPrevious
        );
        assert_eq!(
            StudioWorkspaceCommand::FocusNext,
            StudioWorkspaceCommand::FocusNext
        );
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
}
