use crate::{
    commands::{
        command_palette_items, filter_items, quick_open_items, selected_action,
        StudioCommandAction, StudioCommandItem,
    },
    ui, StudioEguiApp,
};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Elevation, Radius, Space},
};
use std_studio::StudioPane;

const HOST_OVERLAY_X: f32 = 0.0;
const HOST_OVERLAY_Y: f32 = 96.0;

impl StudioEguiApp {
    pub(crate) fn render_overlays(&mut self, ctx: &egui::Context) {
        if self.layout.command_palette_open {
            self.render_command_overlay(
                ctx,
                "studio_command_palette",
                i18n::t("studio.shell.command_palette.title"),
                "Mod+/ or Mod+Shift+P",
                command_palette_items(&self.app),
            );
        }
        if self.layout.quick_open_open {
            self.render_command_overlay(
                ctx,
                "studio_quick_open",
                i18n::t("studio.shell.quick_open.title"),
                "Mod+P",
                quick_open_items(&self.app),
            );
        }
    }

    pub(crate) fn handle_overlay_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::ime_composing(ctx) {
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
            StudioCommandAction::SwitchPane(pane) => self.app.switch_pane(pane),
            StudioCommandAction::FocusWorkspace(id) => {
                if self.app.focus_workspace_pane(id) {
                    self.pending_workspace_focus = Some(id);
                    self.status = format!("focused workspace pane {}", id.value());
                }
            }
            StudioCommandAction::OpenWorkspace(pane) => {
                let id = match pane {
                    StudioPane::Workflows => self
                        .app
                        .open_workflow_builder(self.app.core.config.workflows_dir()),
                    StudioPane::Analysis => self
                        .app
                        .open_analysis_workbench(std::path::PathBuf::from(&self.analysis_path)),
                    StudioPane::Plugins => self.app.open_plugin_manager_pane(),
                    StudioPane::Memory => self.app.open_memory_browser_pane(),
                    StudioPane::History => self.app.open_execution_history_pane(),
                    _ => self.app.open_workspace_pane(pane),
                };
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
