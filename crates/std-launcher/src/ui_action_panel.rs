use crate::{
    ui_metrics,
    ui_parts::{draw_focus_ring, keycap, quiet_label},
};
use eframe::egui;
use std_egui::{
    a11y::AccessibilityContext,
    i18n, input,
    tokens::{Color, Elevation, Radius, Space, Text},
};
use std_launcher::{ActionPanelItem, LauncherFocusSection, LauncherKey, LauncherState};

pub(crate) fn render(ctx: &egui::Context, anchor_rect: egui::Rect, state: &mut LauncherState) {
    if !state.action_panel.open {
        return;
    }
    let rect = ui_metrics::action_panel_rect(anchor_rect, state.action_panel.items.len());
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
                    actions(ui, state);
                });
        });
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
    if response.changed() {
        state.update_action_panel_query(state.action_panel.query.clone());
    }
    if state.focus_section == LauncherFocusSection::ActionPanel {
        let a11y = AccessibilityContext::from_env();
        draw_focus_ring(
            ui,
            response.rect,
            Radius::md(),
            ui_metrics::action_panel_focus_expand(),
            a11y.focus_ring_width(),
        );
    }
}

fn actions(ui: &mut egui::Ui, state: &mut LauncherState) {
    let items = state
        .action_panel
        .visible_items()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    if items.is_empty() {
        quiet_label(ui, i18n::t("launcher.action.no_matches"));
        return;
    }
    for (index, item) in items.iter().enumerate() {
        if action_row(ui, item, index == state.action_panel.selected).clicked() {
            state.action_panel.selected = index;
        }
        ui.add_space(Space::two_xs() as f32);
    }
    if input::ime_composing(ui.ctx()) {
        return;
    }
    for ch in typed_action_panel_chars(ui.ctx()) {
        state.handle_keyboard_input_by_user(LauncherKey::TypeActionPanelQuery(ch), false);
    }
    if input::arrow_down().pressed(ui.ctx()) {
        state.handle_keyboard_input_by_user(LauncherKey::ArrowDown, false);
    }
    if input::arrow_up().pressed(ui.ctx()) {
        state.handle_keyboard_input_by_user(LauncherKey::ArrowUp, false);
    }
}

fn typed_action_panel_chars(ctx: &egui::Context) -> Vec<char> {
    ctx.input(|input| {
        input
            .events
            .iter()
            .filter_map(|event| match event {
                egui::Event::Text(text) => text.chars().next(),
                _ => None,
            })
            .collect()
    })
}

fn action_panel_filter_a11y_label(query: &str) -> String {
    let value = if query.trim().is_empty() {
        "empty"
    } else {
        query.trim()
    };
    format!(
        "{}, text box, value {value}",
        i18n::t("launcher.action.filter.a11y")
    )
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
                    format!("{} action", item.title()),
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
        let guard_index = source.find("input::ime_composing(ui.ctx())").unwrap();
        let typed_index = source.find("typed_action_panel_chars(ui.ctx())").unwrap();
        let key_index = source.find("LauncherKey::TypeActionPanelQuery").unwrap();

        assert!(guard_index < typed_index);
        assert!(typed_index < key_index);
    }

    #[test]
    fn action_panel_filter_a11y_label_exposes_value() {
        assert_eq!(
            action_panel_filter_a11y_label("retry"),
            format!(
                "{}, text box, value retry",
                i18n::t("launcher.action.filter.a11y")
            )
        );
        assert_eq!(
            action_panel_filter_a11y_label("  "),
            format!(
                "{}, text box, value empty",
                i18n::t("launcher.action.filter.a11y")
            )
        );
    }
}
