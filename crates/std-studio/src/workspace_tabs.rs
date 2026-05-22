use crate::{ui, workspace_panes::StudioWorkspaceCommand};
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, Radius, Space, Text},
};
use std_studio::{WorkspacePane, WorkspacePaneId};

const TAB_HEIGHT: f32 = 28.0;
const TAB_MIN_WIDTH: f32 = 148.0;
const TAB_MAX_WIDTH: f32 = 220.0;
const TAB_CLOSE_HIT_SIZE: f32 = TAB_HEIGHT;
const TAB_CLOSE_HOVER_INSET: f32 = Space::TWO_XS as f32;
const TAB_CLOSE_GLYPH_HALF: f32 = Space::TWO_XS as f32;
const TAB_CHAR_WIDTH: f32 = 7.0;
const TAB_TEXT_RESERVED_WIDTH: f32 = Space::XL as f32 + TAB_CLOSE_HIT_SIZE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkspaceTabSpec {
    pub id: WorkspacePaneId,
    pub title: String,
    pub focused: bool,
    pub position: usize,
    pub total: usize,
}

pub(crate) fn workspace_tab_specs(
    panes: &[WorkspacePane],
    focused: Option<WorkspacePaneId>,
) -> Vec<WorkspaceTabSpec> {
    let open_panes = panes.iter().filter(|pane| pane.open).collect::<Vec<_>>();
    let total = open_panes.len();
    open_panes
        .into_iter()
        .enumerate()
        .map(|(index, pane)| WorkspaceTabSpec {
            id: pane.id,
            title: pane.title.clone(),
            focused: Some(pane.id) == focused,
            position: index + 1,
            total,
        })
        .collect()
}

pub(crate) fn workspace_tabs_contract(specs: &[WorkspaceTabSpec]) -> String {
    let focused = specs
        .iter()
        .find(|spec| spec.focused)
        .map(|spec| spec.title.as_str())
        .unwrap_or("none");
    let first_label = specs
        .first()
        .map(workspace_tab_a11y_label)
        .unwrap_or_else(|| "none".to_string());
    let focused_close = specs
        .iter()
        .find(|spec| spec.focused)
        .map(workspace_tab_close_a11y_label)
        .unwrap_or_else(|| "none".to_string());
    format!(
        "tabs={},focused={},cycle=previous|next,close_hit={}x{},a11y={},close_a11y={},keyboard_close={}",
        specs.len(),
        focused,
        TAB_CLOSE_HIT_SIZE as u32,
        TAB_HEIGHT as u32,
        first_label,
        focused_close,
        workspace_tab_keyboard_command(specs.iter().find(|spec| spec.focused).map(|spec| spec.id))
            .is_some()
    )
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

#[cfg(test)]
pub(crate) fn workspace_tab_cycle_commands() -> [StudioWorkspaceCommand; 2] {
    [
        StudioWorkspaceCommand::FocusPrevious,
        StudioWorkspaceCommand::FocusNext,
    ]
}

fn render_workspace_cycle_controls(
    ui: &mut egui::Ui,
    commands: &crate::workspace_panes::WorkspaceCommandQueue,
) {
    let previous_label = i18n::t("studio.workspace_panes.previous");
    let previous = ui::quiet_button(ui, previous_label);
    previous.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            workspace_cycle_a11y_label(
                previous_label,
                input::studio_previous_workspace_pane().label().as_str(),
            ),
        )
    });
    if previous.clicked() {
        push_command(commands, StudioWorkspaceCommand::FocusPrevious);
    }
    let next_label = i18n::t("studio.workspace_panes.next");
    let next = ui::quiet_button(ui, next_label);
    next.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            workspace_cycle_a11y_label(
                next_label,
                input::studio_next_workspace_pane().label().as_str(),
            ),
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
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, TAB_HEIGHT), egui::Sense::click());
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
    let close_rect = workspace_tab_close_rect(rect);
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
    (title.chars().count() as f32 * TAB_CHAR_WIDTH + TAB_TEXT_RESERVED_WIDTH)
        .clamp(TAB_MIN_WIDTH, TAB_MAX_WIDTH)
}

fn workspace_tab_close_rect(rect: egui::Rect) -> egui::Rect {
    egui::Rect::from_min_max(
        egui::pos2(rect.right() - TAB_CLOSE_HIT_SIZE, rect.top()),
        rect.right_bottom(),
    )
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
        .with_max_x(rect.right() - TAB_CLOSE_HIT_SIZE);
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
            rect.shrink(TAB_CLOSE_HOVER_INSET),
            egui::CornerRadius::same(Radius::SM),
            Color::bg_surface_2(ctx),
        );
    }
    let center = rect.center();
    let half = TAB_CLOSE_GLYPH_HALF;
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
    let state_key = if spec.focused {
        "studio.workspace_panes.state.focused"
    } else {
        "studio.workspace_panes.state.inactive"
    };
    i18n::t("studio.workspace_panes.tab.a11y")
        .replace("{title}", &spec.title)
        .replace("{state}", i18n::t(state_key))
        .replace("{position}", &spec.position.to_string())
        .replace("{total}", &spec.total.to_string())
}

pub(crate) fn workspace_tab_close_a11y_label(spec: &WorkspaceTabSpec) -> String {
    i18n::t("studio.workspace_panes.tab.close.a11y").replace("{title}", &spec.title)
}

pub(crate) fn workspace_cycle_a11y_label(direction: &str, shortcut: &str) -> String {
    i18n::t("studio.workspace_panes.cycle.a11y")
        .replace("{direction}", direction)
        .replace("{shortcut}", shortcut)
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
#[path = "workspace_tabs_tests.rs"]
mod tests;
