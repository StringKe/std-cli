use crate::{ui, workspace_panes::StudioWorkspaceCommand};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};
use std_studio::{WorkspacePane, WorkspacePaneId};

const WORKSPACE_TAB_HEIGHT: f32 = 28.0;
const WORKSPACE_TAB_MIN_WIDTH: f32 = 148.0;
const WORKSPACE_TAB_MAX_WIDTH: f32 = 220.0;
const WORKSPACE_TAB_CLOSE_WIDTH: f32 = 24.0;

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
    let width = workspace_tab_width(&spec.title);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, WORKSPACE_TAB_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            workspace_tab_a11y_label(spec),
        )
    });
    if response.clicked() {
        push_command(commands, StudioWorkspaceCommand::Focus(spec.id));
    }
    paint_workspace_tab(ui, rect, spec, response.hovered());
    let close_rect = egui::Rect::from_min_max(
        egui::pos2(rect.right() - WORKSPACE_TAB_CLOSE_WIDTH, rect.top()),
        rect.right_bottom(),
    );
    let close = ui.interact(
        close_rect,
        ui.id().with(("tab-close", spec.id.value())),
        egui::Sense::click(),
    );
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
    paint_workspace_tab_close(ui, close_rect, close.hovered());
}

fn workspace_tab_width(title: &str) -> f32 {
    (title.chars().count() as f32 * 7.0 + 56.0)
        .clamp(WORKSPACE_TAB_MIN_WIDTH, WORKSPACE_TAB_MAX_WIDTH)
}

fn paint_workspace_tab(ui: &egui::Ui, rect: egui::Rect, spec: &WorkspaceTabSpec, hovered: bool) {
    let ctx = ui.ctx();
    let fill = if spec.focused {
        ui::selected_bg(ctx)
    } else if hovered {
        Color::bg_surface_2(ctx)
    } else {
        Color::bg_surface_1(ctx)
    };
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(Radius::SM),
        fill,
        egui::Stroke::new(1.0, Color::stroke_divider(ctx)),
        egui::StrokeKind::Inside,
    );
    let text_rect = rect
        .shrink2(egui::vec2(Space::XS as f32, 0.0))
        .with_max_x(rect.right() - WORKSPACE_TAB_CLOSE_WIDTH);
    ui.painter().text(
        text_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        &spec.title,
        Text::body(),
        ui::strong_text(ctx),
    );
}

fn paint_workspace_tab_close(ui: &egui::Ui, rect: egui::Rect, hovered: bool) {
    let ctx = ui.ctx();
    if hovered {
        ui.painter().rect_filled(
            rect.shrink(4.0),
            egui::CornerRadius::same(Radius::SM),
            Color::bg_surface_2(ctx),
        );
    }
    let center = rect.center();
    let half = 4.0;
    let stroke = egui::Stroke::new(1.5, ui::muted_text(ctx));
    ui.painter().line_segment(
        [
            egui::pos2(center.x - half, center.y - half),
            egui::pos2(center.x + half, center.y + half),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(center.x + half, center.y - half),
            egui::pos2(center.x - half, center.y + half),
        ],
        stroke,
    );
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

    #[test]
    fn workspace_tab_width_is_stable_and_bounded() {
        assert_eq!(workspace_tab_width("Settings"), WORKSPACE_TAB_MIN_WIDTH);
        assert_eq!(
            workspace_tab_width("Very Long Workflow Builder Workspace"),
            WORKSPACE_TAB_MAX_WIDTH
        );
    }
}
