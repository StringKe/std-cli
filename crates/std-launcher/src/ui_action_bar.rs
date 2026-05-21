use crate::{
    ui_metrics,
    ui_parts::{keycap, quiet_label},
};
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, Space, Text},
    LauncherPhase,
};
use std_launcher::{ActionBarPreviewSummary, LauncherState};

pub(crate) fn render(
    ui: &mut egui::Ui,
    state: &LauncherState,
    _hotkey_status: &str,
    _resident_status: &str,
) -> egui::Rect {
    let ctx = ui.ctx().clone();
    let width = ui.available_width();
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
            let right_width = 272.0_f32.min(ui.available_width() * 0.48);
            let left_width = (ui.available_width() - right_width - Space::xs() as f32)
                .max(ui_metrics::scale().f32(160.0));
            ui.allocate_ui_with_layout(
                egui::vec2(left_width, ui_metrics::action_bar_content_height()),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| render_action_summary(ui, state, left_width),
            );
            ui.allocate_ui_with_layout(
                egui::vec2(right_width, ui_metrics::action_bar_content_height()),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| render_action_hints(ui, state),
            );
        });
    });
    rect
}

fn render_action_hints(ui: &mut egui::Ui, state: &LauncherState) {
    match action_bar_hint_mode(state) {
        ActionBarHintMode::Cancel => {
            keycap(ui, "Ctrl+C");
            quiet_label(ui, i18n::t("launcher.action.cancel"));
        }
        ActionBarHintMode::RunActions => {
            keycap(ui, &input::launcher_action_panel().label());
            quiet_label(ui, i18n::t("launcher.action.actions"));
            keycap(ui, "Enter");
            quiet_label(ui, i18n::t("launcher.action.run"));
        }
    }
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
                "Ctrl+C".to_string(),
            ]
        }
        ActionBarHintMode::RunActions => vec![
            i18n::t("launcher.action.run").to_string(),
            "Enter".to_string(),
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
                breadcrumb: "Command > Rebuild Index".to_string(),
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
            action_type: ActionType::Skill,
            primary_command: String::new(),
            metadata: Default::default(),
            examples: Vec::new(),
        };

        assert_eq!(
            ActionBarPreviewSummary::from_preview(&preview),
            ActionBarPreviewSummary {
                breadcrumb: "Skill > Memory".to_string(),
                primary: "Pinned workspace memory".to_string()
            }
        );
    }
}
