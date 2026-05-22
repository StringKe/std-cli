use crate::{
    ui_metrics,
    ui_parts::{draw_focus_ring, keycap, quiet_label},
};
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, Elevation, FocusRing, Radius, Space, Text},
};
use std_launcher::{ActionPanelItem, LauncherFocusSection, LauncherFocusSource, LauncherState};
use std_types::ActionExecution;

pub(crate) fn render(
    ctx: &egui::Context,
    anchor_rect: egui::Rect,
    state: &mut LauncherState,
) -> ActionPanelRenderResult {
    if !state.action_panel.open {
        return ActionPanelRenderResult::default();
    }
    let rect = ui_metrics::action_panel_rect(anchor_rect, state.action_panel.items.len());
    let mut command = ActionPanelCommand::None;
    egui::Area::new("launcher_action_panel".into())
        .order(egui::Order::Foreground)
        .fixed_pos(rect.min)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(Color::bg_surface_1(ctx))
                .stroke(egui::Stroke::new(1.0, Color::stroke_border(ctx)))
                .corner_radius(egui::CornerRadius::same(Radius::lg()))
                .shadow(Elevation::level_2(ctx))
                .inner_margin(egui::Margin::same(Space::sm()))
                .show(ui, |ui| {
                    ui.set_width(rect.width());
                    header(ui, state);
                    ui.add_space(Space::xs() as f32);
                    search(ui, state);
                    ui.add_space(Space::xs() as f32);
                    command = actions(ui, state);
                });
        });
    ActionPanelRenderResult { command }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ActionPanelRenderResult {
    pub(crate) command: ActionPanelCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum ActionPanelCommand {
    #[default]
    None,
    Triggered(ActionPanelExecution),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ActionPanelExecution {
    pub(crate) execution: ActionExecution,
    pub(crate) copy_to_clipboard: bool,
}

fn header(ui: &mut egui::Ui, state: &LauncherState) {
    let ctx = ui.ctx().clone();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(i18n::t("launcher.action.actions"))
                .font(Text::body())
                .color(Color::fg_primary(&ctx))
                .strong(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            quiet_label(ui, &state.action_panel.action_name);
        });
    });
}

fn search(ui: &mut egui::Ui, state: &mut LauncherState) {
    let response = ui.add_sized(
        [
            ui.available_width(),
            ui_metrics::action_panel_search_height(),
        ],
        egui::TextEdit::singleline(&mut state.action_panel.query)
            .hint_text(i18n::t("launcher.action.filter.hint"))
            .font(Text::body()),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            action_panel_filter_a11y_label(&state.action_panel.query),
        )
    });
    if state.keyboard_focus_visible(LauncherFocusSection::ActionPanel) {
        response.request_focus();
    }
    if response.changed() {
        state.update_action_panel_query(state.action_panel.query.clone());
    }
    if state.keyboard_focus_visible(LauncherFocusSection::ActionPanel) {
        draw_focus_ring(ui, response.rect, FocusRing::launcher_action_panel());
    }
}

fn actions(ui: &mut egui::Ui, state: &mut LauncherState) -> ActionPanelCommand {
    let items = state
        .action_panel
        .visible_items()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    if items.is_empty() {
        quiet_label(ui, i18n::t("launcher.action.no_matches"));
        return ActionPanelCommand::None;
    }
    let mut command = ActionPanelCommand::None;
    for (index, item) in items.iter().enumerate() {
        if action_row(ui, item, index == state.action_panel.selected).clicked() {
            state.action_panel.selected = index;
            state.focus_section = LauncherFocusSection::ActionPanel;
            state.focus_source = LauncherFocusSource::Pointer;
            if let Some(execution) = state.trigger_action_panel_selection_by_user() {
                command = ActionPanelCommand::Triggered(ActionPanelExecution {
                    execution,
                    copy_to_clipboard: matches!(item, ActionPanelItem::CopyCommand(_)),
                });
            }
        }
        ui.add_space(Space::two_xs() as f32);
    }
    if command != ActionPanelCommand::None {
        return command;
    }
    if input::ime_composing(ui.ctx()) {
        return ActionPanelCommand::None;
    }
    if input::arrow_down().pressed(ui.ctx()) {
        state.handle_keyboard_input_by_user(std_launcher::LauncherKey::ArrowDown, false);
    }
    if input::arrow_up().pressed(ui.ctx()) {
        state.handle_keyboard_input_by_user(std_launcher::LauncherKey::ArrowUp, false);
    }
    ActionPanelCommand::None
}

fn action_panel_filter_a11y_label(query: &str) -> String {
    let value = if query.trim().is_empty() {
        i18n::t("launcher.action.filter.value.empty")
    } else {
        query.trim()
    };
    i18n::t("launcher.action.filter.input.a11y")
        .replace("{label}", i18n::t("launcher.action.filter.a11y"))
        .replace("{value}", value)
}

fn action_panel_row_a11y_label(item: &ActionPanelItem) -> String {
    i18n::t("launcher.action.row.a11y").replace("{label}", item.title())
}

fn action_row(ui: &mut egui::Ui, item: &ActionPanelItem, selected: bool) -> egui::Response {
    let ctx = ui.ctx().clone();
    let fill = if selected {
        Color::accent_weak(&ctx)
    } else {
        egui::Color32::TRANSPARENT
    };
    egui::Frame::new()
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()))
        .show(ui, |ui| {
            let response = ui.allocate_response(
                egui::vec2(ui.available_width(), ui_metrics::action_panel_row_height()),
                egui::Sense::click(),
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    action_panel_row_a11y_label(item),
                )
            });
            ui.scope_builder(egui::UiBuilder::new().max_rect(response.rect), |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(item.title())
                            .font(Text::body())
                            .color(Color::fg_primary(&ctx)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        keycap(ui, &item.shortcut_label());
                    });
                });
            });
            response
        })
        .inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_panel_focus_ring_uses_action_panel_section() {
        let mut state = LauncherState::new();
        state.update_query("terminal");
        state.open_action_panel();

        assert_eq!(state.focus_section, LauncherFocusSection::ActionPanel);
        assert!(state.keyboard_focus_visible(LauncherFocusSection::ActionPanel));
    }

    #[test]
    fn action_panel_focus_ring_suppresses_pointer_focus() {
        let mut state = LauncherState::new();
        state.update_query("terminal");
        state.open_action_panel();
        state.mark_pointer_focus(LauncherFocusSection::ActionPanel);

        assert_eq!(state.focus_section, LauncherFocusSection::ActionPanel);
        assert!(!state.keyboard_focus_visible(LauncherFocusSection::ActionPanel));
    }

    #[test]
    fn action_panel_row_click_triggers_selection_through_user_route() {
        let source = include_str!("ui_action_panel.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();
        let click_branch = production_source
            .split("action_row(ui, item, index == state.action_panel.selected).clicked()")
            .nth(1)
            .and_then(|body| body.split("ui.add_space").next())
            .unwrap();

        assert!(click_branch.contains("state.action_panel.selected = index"));
        assert!(click_branch.contains("LauncherFocusSource::Pointer"));
        assert!(click_branch.contains("state.trigger_action_panel_selection_by_user()"));
        assert!(production_source.contains("ActionPanelCommand::Triggered(ActionPanelExecution"));
        assert!(production_source
            .contains("copy_to_clipboard: matches!(item, ActionPanelItem::CopyCommand(_))"));
    }

    #[test]
    fn action_panel_arrow_shortcuts_respect_ime_guard() {
        let source = include_str!("ui_action_panel.rs");
        let guard_index = source.find("input::ime_composing(ui.ctx())").unwrap();
        let arrow_down_index = source
            .find("input::arrow_down().pressed(ui.ctx())")
            .unwrap();
        let arrow_up_index = source.find("input::arrow_up().pressed(ui.ctx())").unwrap();

        assert!(guard_index < arrow_down_index);
        assert!(guard_index < arrow_up_index);
    }

    #[test]
    fn action_panel_typed_filter_uses_text_events_after_ime_guard() {
        let source = include_str!("ui_action_panel.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();
        let guard_index = production_source
            .find("input::ime_composing(ui.ctx())")
            .unwrap();
        let changed_index = production_source.find("response.changed()").unwrap();
        let update_index = production_source
            .find("state.update_action_panel_query")
            .unwrap();

        assert!(changed_index < guard_index);
        assert!(changed_index < update_index);
        assert!(!production_source.contains("typed_action_panel_chars"));
    }

    #[test]
    fn action_panel_filter_requests_egui_focus_when_keyboard_focused() {
        let source = include_str!("ui_action_panel.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();
        let focus_branch = production_source
            .split("state.keyboard_focus_visible(LauncherFocusSection::ActionPanel)")
            .nth(1)
            .and_then(|body| body.split("if response.changed()").next())
            .unwrap();

        assert!(focus_branch.contains("response.request_focus();"));
    }

    #[test]
    fn action_panel_filter_textedit_owns_text_input() {
        let ctx = egui::Context::default();
        let mut state = LauncherState::new();
        state.update_query("terminal");
        state.open_action_panel();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                search(ui, &mut state);
            });
        });
        let _ = ctx.run(action_panel_text_input("co"), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                search(ui, &mut state);
            });
        });

        assert_eq!(state.action_panel.query, "co");
        assert_eq!(state.action_panel.selected, 0);
    }

    fn action_panel_text_input(text: &str) -> egui::RawInput {
        egui::RawInput {
            events: vec![egui::Event::Text(text.to_string())],
            ..Default::default()
        }
    }

    #[test]
    fn action_panel_filter_a11y_label_exposes_value() {
        assert_eq!(
            action_panel_filter_a11y_label("retry"),
            i18n::t("launcher.action.filter.input.a11y")
                .replace("{label}", i18n::t("launcher.action.filter.a11y"))
                .replace("{value}", "retry")
        );
        assert_eq!(
            action_panel_filter_a11y_label("  "),
            i18n::t("launcher.action.filter.input.a11y")
                .replace("{label}", i18n::t("launcher.action.filter.a11y"))
                .replace("{value}", i18n::t("launcher.action.filter.value.empty"))
        );
    }

    #[test]
    fn action_panel_row_a11y_label_uses_localized_template() {
        let item = ActionPanelItem::CopyCommand("std index rebuild".to_string());

        assert_eq!(
            action_panel_row_a11y_label(&item),
            i18n::t("launcher.action.row.a11y")
                .replace("{label}", i18n::t("launcher.action.copy_command"))
        );
    }
}
