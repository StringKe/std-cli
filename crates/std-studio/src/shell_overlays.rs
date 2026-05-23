use crate::{
    commands::{
        command_palette_items, filter_items, quick_open_items, selected_action,
        StudioCommandAction, StudioCommandItem,
    },
    ui, StudioEguiApp,
};
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, Elevation, Radius, Space},
};

const HOST_OVERLAY_X: f32 = 0.0;
const HOST_OVERLAY_Y: f32 = 96.0;

impl StudioEguiApp {
    pub(crate) fn render_overlays(&mut self, ctx: &egui::Context) {
        if self.layout.command_palette_open {
            self.render_command_overlay(
                ctx,
                "studio_command_palette",
                i18n::t("studio.shell.command_palette.title"),
                &command_palette_shortcut_hint(),
                command_palette_items(&self.app),
            );
        }
        if self.layout.quick_open_open {
            self.render_command_overlay(
                ctx,
                "studio_quick_open",
                i18n::t("studio.shell.quick_open.title"),
                &input::studio_quick_open().label(),
                quick_open_items(&self.app),
            );
        }
    }

    pub(crate) fn handle_overlay_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::ime_action_guard(ctx).blocks_actions() {
            return;
        }
        let items = if self.layout.command_palette_open {
            filter_items(
                &command_palette_items(&self.app),
                &self.layout.command_query,
            )
        } else if self.layout.quick_open_open {
            filter_items(&quick_open_items(&self.app), &self.layout.quick_open_query)
        } else {
            Vec::new()
        };
        self.layout.clamp_overlay_selection(items.len());
        if items.is_empty() {
            return;
        }

        if std_egui::input::escape().pressed(ctx) {
            self.layout.close_overlays();
            return;
        }
        if !items.is_empty() && std_egui::input::arrow_down().pressed(ctx) {
            self.layout.move_overlay_selection(1, items.len());
        }
        if !items.is_empty() && std_egui::input::arrow_up().pressed(ctx) {
            self.layout.move_overlay_selection(-1, items.len());
        }
        if std_egui::input::enter().pressed(ctx) {
            if let Some(action) = selected_action(&items, self.layout.overlay_selected) {
                self.apply_command_action(action);
            }
        }
    }

    fn render_command_overlay(
        &mut self,
        ctx: &egui::Context,
        id: &'static str,
        title: &str,
        shortcut: &str,
        items: Vec<StudioCommandItem>,
    ) {
        render_host_overlay(ctx, id, 520.0, |ui| {
            ui::section_header(ui, title, shortcut);
            let query = if id == "studio_command_palette" {
                &mut self.layout.command_query
            } else {
                &mut self.layout.quick_open_query
            };
            let response = ui.add(
                egui::TextEdit::singleline(query)
                    .hint_text(i18n::t("studio.shell.filter.hint"))
                    .desired_width(f32::INFINITY),
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::TextEdit,
                    ui.is_enabled(),
                    overlay_query_a11y_label(title, query),
                )
            });
            if response.changed() {
                self.layout.overlay_selected = 0;
            }
            response.request_focus();
            ui.add_space(Space::XS as f32);

            let filtered_items = filter_items(&items, query);
            self.layout.clamp_overlay_selection(filtered_items.len());
            if filtered_items.is_empty() {
                ui.label(
                    egui::RichText::new(i18n::t("studio.shell.no_matches"))
                        .color(ui::muted_text(ctx)),
                );
                return;
            }
            for (index, item) in filtered_items.into_iter().enumerate() {
                self.render_command_item(ui, item, index == self.layout.overlay_selected);
            }
        });
    }

    fn render_command_item(&mut self, ui: &mut egui::Ui, item: StudioCommandItem, selected: bool) {
        egui::Frame::new()
            .fill(if selected {
                ui::selected_bg(ui.ctx())
            } else {
                egui::Color32::TRANSPARENT
            })
            .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(&item.title).color(ui::strong_text(ui.ctx())));
                        ui.label(egui::RichText::new(&item.detail).color(ui::muted_text(ui.ctx())));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui::quiet_button(ui, &item.shortcut).clicked() {
                            self.apply_command_action(item.action);
                        }
                    });
                });
            });
    }

    fn apply_command_action(&mut self, action: StudioCommandAction) {
        match action {
            StudioCommandAction::FocusWorkspace(id) => {
                if self.app.focus_workspace_pane(id) {
                    self.pending_workspace_focus = Some(id);
                    self.status = format!("focused workspace pane {}", id.value());
                }
            }
            StudioCommandAction::OpenWorkspace(pane) => {
                let id = self.open_workspace_pane_for_nav(pane);
                self.pending_workspace_focus = Some(id);
                self.status = format!("opened workspace pane {}", id.value());
            }
            StudioCommandAction::OpenSettings => {
                self.open_settings_workspace_pane();
            }
            StudioCommandAction::Refresh => self.app.refresh(),
        }
        self.layout.close_overlays();
    }

    pub(crate) fn open_settings_workspace_pane(&mut self) {
        let id = self.app.open_settings_pane();
        self.status = format!("opened workspace pane {}", id.value());
    }
}

fn command_palette_shortcut_hint() -> String {
    format!(
        "{} or {}",
        input::studio_command_palette_slash().label(),
        input::studio_command_palette().label()
    )
}

fn render_host_overlay(
    ctx: &egui::Context,
    id: &'static str,
    width: f32,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    egui::Area::new(egui::Id::new(id))
        .anchor(
            egui::Align2::CENTER_TOP,
            egui::vec2(HOST_OVERLAY_X, HOST_OVERLAY_Y),
        )
        .order(egui::Order::Foreground)
        .constrain(true)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(Color::bg_surface_1(ctx))
                .stroke(egui::Stroke::new(1.0, Color::stroke_border(ctx)))
                .corner_radius(egui::CornerRadius::same(Radius::MD))
                .shadow(Elevation::level_2(ctx))
                .inner_margin(egui::Margin::same(Space::MD))
                .show(ui, |ui| {
                    ui.set_width(width);
                    add_contents(ui);
                });
        });
}

fn overlay_query_a11y_label(title: &str, query: &str) -> String {
    let value = if query.trim().is_empty() {
        "empty"
    } else {
        query.trim()
    };
    format!("{title}, text box, value {value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_overlay_query_has_textbox_semantics() {
        let source = include_str!("shell_overlays.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("WidgetType::TextEdit"));
        assert!(implementation.contains("overlay_query_a11y_label"));
        assert!(implementation.contains("response.request_focus()"));
        assert!(implementation.contains("std_egui::input::ime_action_guard(ctx).blocks_actions()"));
    }

    #[test]
    fn overlay_query_a11y_label_exposes_value() {
        assert_eq!(
            overlay_query_a11y_label("Command Palette", "settings"),
            "Command Palette, text box, value settings"
        );
        assert_eq!(
            overlay_query_a11y_label("Quick Open", " "),
            "Quick Open, text box, value empty"
        );
    }

    #[test]
    fn command_overlay_shortcuts_respect_ime_preedit_frame() {
        let ctx = egui::Context::default();
        let mut app = StudioEguiApp::default();
        app.layout.open_command_palette();
        app.layout.overlay_selected = 0;
        let before_pane = app.app.focused_pane;

        let _ = ctx.run(ime_preedit_overlay_input(), |ctx| {
            app.handle_overlay_keyboard(ctx);
        });

        assert!(app.layout.command_palette_open);
        assert_eq!(app.layout.overlay_selected, 0);
        assert_eq!(app.app.focused_pane, before_pane);
        assert!(app.status.is_empty());
    }

    #[test]
    fn quick_open_shortcuts_respect_ime_preedit_frame() {
        let ctx = egui::Context::default();
        let mut app = StudioEguiApp::default();
        let focused = app.app.open_plugin_manager_pane();
        app.layout.open_quick_open();
        app.layout.overlay_selected = 0;

        let _ = ctx.run(ime_preedit_overlay_input(), |ctx| {
            app.handle_overlay_keyboard(ctx);
        });

        assert!(app.layout.quick_open_open);
        assert_eq!(app.layout.overlay_selected, 0);
        assert_eq!(app.app.focused_pane, Some(focused));
        assert!(app.status.is_empty());
    }

    fn ime_preedit_overlay_input() -> egui::RawInput {
        egui::RawInput {
            events: vec![
                egui::Event::Ime(egui::ImeEvent::Preedit("ming".to_string())),
                egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    physical_key: Some(egui::Key::ArrowDown),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::Key {
                    key: egui::Key::Enter,
                    physical_key: Some(egui::Key::Enter),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::Key {
                    key: egui::Key::Escape,
                    physical_key: Some(egui::Key::Escape),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            ..Default::default()
        }
    }
}
