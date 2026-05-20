use crate::{ui, windows::StudioWorkspaceCommand};
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
    commands: &crate::windows::WorkspaceCommandQueue,
) {
    ui.horizontal_wrapped(|ui| {
        for spec in specs {
            render_workspace_tab(ui, spec, commands);
        }
    });
}

fn render_workspace_tab(
    ui: &mut egui::Ui,
    spec: &WorkspaceTabSpec,
    commands: &crate::windows::WorkspaceCommandQueue,
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
                if ui::quiet_button(ui, &spec.title).clicked() {
                    push_command(commands, StudioWorkspaceCommand::Focus(spec.id));
                }
                if ui::quiet_button(ui, i18n::t("studio.windows.close")).clicked() {
                    push_command(commands, StudioWorkspaceCommand::Close(spec.id));
                }
            });
        });
}

fn push_command(commands: &crate::windows::WorkspaceCommandQueue, command: StudioWorkspaceCommand) {
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
}
