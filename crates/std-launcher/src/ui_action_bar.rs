use crate::{
    ui_metrics,
    ui_parts::{keycap, quiet_label},
};
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, LauncherSize, Space, Text},
    LauncherPhase,
};
use std_launcher::{ActionBarPreviewSummary, LauncherState};

pub(crate) fn render(
    ui: &mut egui::Ui,
    state: &LauncherState,
    _hotkey_status: &str,
    _resident_status: &str,
) -> ActionBarRenderResult {
    let ctx = ui.ctx().clone();
    let width = ui.available_width();
    let mut command = ActionBarCommand::None;
    let (rect, _response) = ui.allocate_exact_size(
        egui::vec2(width, ui_metrics::action_bar_height()),
        egui::Sense::hover(),
    );
    ui.painter().line_segment(
        [rect.left_top(), rect.right_top()],
        egui::Stroke::new(1.0, Color::stroke_divider(&ctx)),
    );
    let content_rect = rect.shrink2(egui::vec2(Space::xs() as f32, Space::two_xs() as f32));
    ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
        ui.horizontal(|ui| {
            let right_width = LauncherSize::action_bar_right_width(ui.available_width());
            let left_width = LauncherSize::action_bar_left_width(
                ui_metrics::scale(),
                ui.available_width(),
                right_width,
            );
            ui.allocate_ui_with_layout(
                egui::vec2(left_width, ui_metrics::action_bar_content_height()),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| render_action_summary(ui, state, left_width),
            );
            ui.allocate_ui_with_layout(
                egui::vec2(right_width, ui_metrics::action_bar_content_height()),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| command = render_action_hints(ui, state),
            );
        });
    });
    ActionBarRenderResult { rect, command }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ActionBarRenderResult {
    pub(crate) rect: egui::Rect,
    pub(crate) command: ActionBarCommand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ActionBarCommand {
    None,
    CancelExecuting,
    MoveExecutingToBackground,
}

fn render_action_hints(ui: &mut egui::Ui, state: &LauncherState) -> ActionBarCommand {
    match action_bar_hint_mode(state) {
        ActionBarHintMode::Cancel => render_executing_controls(ui),
        ActionBarHintMode::RunActions => {
            keycap(ui, &input::launcher_action_panel().label());
            quiet_label(ui, i18n::t("launcher.action.actions"));
            keycap(ui, &input::enter().label());
            quiet_label(ui, i18n::t("launcher.action.run"));
            ActionBarCommand::None
        }
    }
}

fn render_executing_controls(ui: &mut egui::Ui) -> ActionBarCommand {
    let mut command = ActionBarCommand::None;
    if action_bar_control(
        ui,
        i18n::t("launcher.action.background"),
        &input::enter().label(),
    )
    .clicked()
    {
        command = ActionBarCommand::MoveExecutingToBackground;
    }
    if action_bar_control(
        ui,
        i18n::t("launcher.action.cancel"),
        &input::launcher_cancel().label(),
    )
    .clicked()
    {
        command = ActionBarCommand::CancelExecuting;
    }
    command
}

fn action_bar_control(ui: &mut egui::Ui, label: &str, shortcut: &str) -> egui::Response {
    let ctx = ui.ctx().clone();
    let text = format!("{label} {shortcut}");
    let response = ui.add(
        egui::Button::new(
            egui::RichText::new(text)
                .font(Text::caption())
                .color(Color::fg_primary(&ctx)),
        )
        .fill(Color::bg_surface_1(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(std_egui::tokens::Radius::sm())),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            action_bar_control_a11y_label(label, shortcut),
        )
    });
    response
}

fn action_bar_control_a11y_label(label: &str, shortcut: &str) -> String {
    i18n::t("launcher.action.control.a11y")
        .replace("{label}", label)
        .replace("{shortcut}", shortcut)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionBarHintMode {
    RunActions,
    Cancel,
}

fn action_bar_hint_mode(state: &LauncherState) -> ActionBarHintMode {
    if state.view.phase == LauncherPhase::Executing {
        ActionBarHintMode::Cancel
    } else {
        ActionBarHintMode::RunActions
    }
}

#[cfg(test)]
fn action_bar_visible_hint_labels(state: &LauncherState) -> Vec<String> {
    match action_bar_hint_mode(state) {
        ActionBarHintMode::Cancel => {
            vec![
                i18n::t("launcher.action.cancel").to_string(),
                input::launcher_cancel().label(),
                i18n::t("launcher.action.background").to_string(),
                input::enter().label(),
            ]
        }
        ActionBarHintMode::RunActions => vec![
            i18n::t("launcher.action.run").to_string(),
            input::enter().label(),
            i18n::t("launcher.action.actions").to_string(),
            input::launcher_action_panel().label(),
        ],
    }
}

#[cfg(test)]
fn action_bar_visual_contract() -> &'static str {
    "height=36;top-divider=1px;rounded-frame=false;background=panel"
}

fn render_action_summary(ui: &mut egui::Ui, state: &LauncherState, max_width: f32) {
    let ctx = ui.ctx().clone();
    if state.view.phase == LauncherPhase::Executing {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label(
                egui::RichText::new(i18n::t("launcher.action.executing"))
                    .font(Text::footnote())
                    .color(Color::fg_primary(&ctx))
                    .strong(),
            );
        });
        return;
    }
    if let Some(preview) = state.view.preview.as_ref() {
        let summary = ActionBarPreviewSummary::from_preview(preview);
        ui.add_sized(
            [max_width * 0.34, ui_metrics::action_summary_label_height()],
            egui::Label::new(
                egui::RichText::new(summary.breadcrumb.as_str())
                    .font(Text::footnote())
                    .color(Color::fg_primary(&ctx))
                    .strong(),
            )
            .truncate(),
        );
        ui.add_sized(
            [max_width * 0.62, ui_metrics::action_summary_label_height()],
            egui::Label::new(
                egui::RichText::new(summary.primary.as_str())
                    .font(Text::code())
                    .color(Color::fg_secondary(&ctx)),
            )
            .truncate(),
        );
        return;
    }
    ui.add_sized(
        [max_width, ui_metrics::action_summary_label_height()],
        egui::Label::new(
            egui::RichText::new(i18n::t("launcher.action.command_hint"))
                .font(Text::footnote())
                .color(Color::fg_secondary(&ctx)),
        )
        .truncate(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_types::{ActionId, ActionPreview, ActionType};

    #[test]
    fn action_bar_hint_switches_to_cancel_while_executing() {
        let mut state = LauncherState::new();

        assert_eq!(action_bar_hint_mode(&state), ActionBarHintMode::RunActions);

        state.view.preview_executing();

        assert_eq!(action_bar_hint_mode(&state), ActionBarHintMode::Cancel);
        assert_eq!(
            action_bar_visible_hint_labels(&state),
            vec![
                i18n::t("launcher.action.cancel").to_string(),
                input::launcher_cancel().label(),
                i18n::t("launcher.action.background").to_string(),
                input::enter().label(),
            ]
        );
    }

    #[test]
    fn executing_action_bar_exposes_clickable_controls() {
        let source = include_str!("ui_action_bar.rs");

        assert!(source.contains("ActionBarCommand::CancelExecuting"));
        assert!(source.contains("ActionBarCommand::MoveExecutingToBackground"));
        assert!(source.contains("WidgetType::Button"));
        assert!(source.contains("launcher.action.control.a11y"));
    }

    #[test]
    fn action_bar_control_accessibility_label_uses_i18n_template() {
        let label = action_bar_control_a11y_label("Cancel", "Esc");

        assert!(label.contains("Cancel"));
        assert!(label.contains("Esc"));
        assert!(!label.contains("{label}"));
        assert!(!label.contains("{shortcut}"));
    }

    #[test]
    fn action_bar_hides_runtime_status_noise_from_user_hints() {
        let state = LauncherState::new();
        let labels = action_bar_visible_hint_labels(&state);

        assert!(labels.contains(&i18n::t("launcher.action.run").to_string()));
        assert!(labels.contains(&i18n::t("launcher.action.actions").to_string()));
        assert!(!labels.contains(&"registered".to_string()));
        assert!(!labels.contains(&"preview".to_string()));
        assert!(!labels.iter().any(|label| label.contains("menu bar")));
    }

    #[test]
    fn action_bar_uses_inline_status_row_not_nested_card() {
        assert_eq!(
            action_bar_visual_contract(),
            "height=36;top-divider=1px;rounded-frame=false;background=panel"
        );
    }

    #[test]
    fn action_bar_summary_uses_type_and_title_as_breadcrumb() {
        let preview = ActionPreview {
            action_id: ActionId::default(),
            title: "Rebuild Index".to_string(),
            subtitle: "Refresh local index".to_string(),
            action_type: ActionType::Command,
            primary_command: "std index rebuild .".to_string(),
            metadata: Default::default(),
            examples: Vec::new(),
        };

        assert_eq!(
            ActionBarPreviewSummary::from_preview(&preview),
            ActionBarPreviewSummary {
                breadcrumb: "命令 > Rebuild Index".to_string(),
                primary: "std index rebuild .".to_string()
            }
        );
    }

    #[test]
    fn action_bar_summary_falls_back_to_subtitle_when_command_is_empty() {
        let preview = ActionPreview {
            action_id: ActionId::default(),
            title: "Memory".to_string(),
            subtitle: "Pinned workspace memory".to_string(),
            action_type: ActionType::Memory,
            primary_command: String::new(),
            metadata: Default::default(),
            examples: Vec::new(),
        };

        assert_eq!(
            ActionBarPreviewSummary::from_preview(&preview),
            ActionBarPreviewSummary {
                breadcrumb: "记忆 > Memory".to_string(),
                primary: "Pinned workspace memory".to_string()
            }
        );
    }
}
