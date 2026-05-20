use crate::{
    commands::{
        command_palette_items, filter_items, quick_open_items, selected_action,
        StudioCommandAction, StudioCommandItem,
    },
    shell_parts::{panel_frame, path_label},
    ui, StudioEguiApp,
};
use eframe::egui;
use std_studio::StudioPane;

impl StudioEguiApp {
    pub(crate) fn render_shell(&mut self, ctx: &egui::Context) {
        self.handle_overlay_keyboard(ctx);
        egui::TopBottomPanel::top("studio_app_chrome")
            .exact_height(52.0)
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_app_chrome(ui));
        egui::SidePanel::left("studio_nav")
            .resizable(self.layout.sidebar_open)
            .default_width(self.layout.sidebar_width())
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_navigation(ui));
        if self.layout.inspector_open {
            egui::SidePanel::right("studio_context")
                .resizable(true)
                .default_width(self.layout.inspector_width())
                .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
                .show(ctx, |ui| self.render_context(ui));
        }
        if self.layout.bottom_panel_open {
            egui::TopBottomPanel::bottom("studio_bottom_panel")
                .resizable(true)
                .default_height(self.layout.bottom_panel_height())
                .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
                .show(ctx, |ui| self.render_bottom_panel(ui));
        }
        egui::TopBottomPanel::bottom("studio_status")
            .exact_height(24.0)
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_status_bar(ui));
        egui::CentralPanel::default()
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_0(ctx)))
            .show(ctx, |ui| self.render_active_workspace(ui));
        self.render_overlays(ctx);
    }

    fn render_navigation(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        if !self.layout.sidebar_open {
            for pane in StudioPane::all() {
                let label = pane.label().chars().next().unwrap_or('?').to_string();
                let selected = self.app.active_pane == pane;
                if ui
                    .selectable_label(selected, label)
                    .on_hover_text(pane.label())
                    .clicked()
                {
                    self.app.switch_pane(pane);
                }
            }
            return;
        }
        ui.vertical(|ui| {
            ui::section_header(ui, "Workspace", "main views");
            for pane in StudioPane::all() {
                let selected = self.app.active_pane == pane;
                if ui
                    .add_sized(
                        [ui.available_width(), 32.0],
                        egui::Button::new(
                            egui::RichText::new(pane.label()).color(ui::strong_text(ui.ctx())),
                        )
                        .fill(if selected {
                            ui::selected_bg(ui.ctx())
                        } else {
                            egui::Color32::TRANSPARENT
                        }),
                    )
                    .clicked()
                {
                    self.app.switch_pane(pane);
                }
            }
        });
        ui.add_space(18.0);
        ui.vertical(|ui| {
            ui::section_header(ui, "Open", "workspace panes");
            self.open_row(
                ui,
                "Workflow Builder",
                "edit and run",
                StudioPane::Workflows,
            );
            self.open_row(
                ui,
                "Analysis Workbench",
                "index and ask",
                StudioPane::Analysis,
            );
            self.open_row(ui, "Plugin Manager", "manifest checks", StudioPane::Plugins);
            self.open_row(ui, "Memory Browser", "local recall", StudioPane::Memory);
            self.open_row(ui, "Execution History", "trace review", StudioPane::History);
        });
        ui.add_space(18.0);
        self.render_workspace_pane_manager(ui);
    }

    fn render_active_workspace(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.app.active_pane {
                StudioPane::Dashboard => self.render_dashboard(ui),
                StudioPane::Workflows => self.render_workflows(ui),
                StudioPane::Apps => self.render_apps(ui),
                StudioPane::Memory => self.render_memory(ui),
                StudioPane::Plugins => self.render_plugins(ui),
                StudioPane::Analysis => self.render_analysis(ui),
                StudioPane::History => self.render_history(ui),
                StudioPane::Operations => self.render_operations(ui),
                StudioPane::Settings => self.render_settings(ui),
            });
        });
        self.render_workspace_panes(ui);
    }

    fn render_context(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Context", "workspace signals");
            ui::metric(ui, "Actions", self.app.dashboard.action_count, "registered");
            ui.add_space(8.0);
            ui::metric(ui, "Memory", self.app.dashboard.memory_count, "records");
            ui.add_space(8.0);
            ui::metric(
                ui,
                "Audit Events",
                self.app.dashboard.audit_event_count,
                "recent local trail",
            );
        });
        ui.add_space(10.0);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Runtime", "local paths");
            path_label(ui, "Config", self.app.config_path().display().to_string());
            path_label(
                ui,
                "Data",
                self.app.core.config.data_dir.display().to_string(),
            );
            path_label(
                ui,
                "Workflows",
                self.app.core.config.workflows_dir().display().to_string(),
            );
        });
        ui.add_space(10.0);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Status", "latest result");
            if self.status.is_empty() {
                ui.label(egui::RichText::new("Idle").color(ui::muted_text(ui.ctx())));
            } else {
                ui.label(egui::RichText::new(&self.status).color(ui::strong_text(ui.ctx())));
            }
        });
    }

    fn render_status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new(self.app.active_pane.content_key())
                    .color(ui::muted_text(ui.ctx())),
            );
            ui.separator();
            ui.label(format!("{} panes", self.app.open_workspace_panes().count()));
            ui.separator();
            ui.label(if self.layout.inspector_open {
                "inspector"
            } else {
                "inspector hidden"
            });
            ui.separator();
            ui.label(if self.layout.bottom_panel_open {
                "bottom panel"
            } else {
                "bottom hidden"
            });
            ui.separator();
            ui.label(format!(
                "{} plugins",
                self.app.plugin_manager.manifest_paths.len()
            ));
            ui.separator();
            ui.label(format!(
                "{} memories",
                self.app.memory_browser.memories.len()
            ));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("external actions deferred")
                        .color(ui::muted_text(ui.ctx())),
                );
            });
        });
    }

    fn render_bottom_panel(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Batch Debug", "Logs / Problems / Performance");
            if let Some(report) = self.app.last_batch_report.as_ref() {
                ui.label(egui::RichText::new(format!("batch {:?}", report.status)));
            } else if let Some(execution) = self.app.last_workflow_execution.as_ref() {
                ui.label(egui::RichText::new(format!(
                    "workflow {:?}",
                    execution.status
                )));
            } else if self.status.is_empty() {
                ui.label(egui::RichText::new("Idle").color(ui::muted_text(ui.ctx())));
            } else {
                ui.label(egui::RichText::new(&self.status).color(ui::strong_text(ui.ctx())));
            }
        });
    }

    fn render_overlays(&mut self, ctx: &egui::Context) {
        if self.layout.settings_open {
            self.render_settings_overlay(ctx);
        }
        if self.layout.command_palette_open {
            self.render_command_overlay(
                ctx,
                "studio_command_palette",
                "Command Palette",
                "Mod+/ or Mod+Shift+P",
                command_palette_items(&self.app),
            );
        }
        if self.layout.quick_open_open {
            self.render_command_overlay(
                ctx,
                "studio_quick_open",
                "Quick Open",
                "Mod+P",
                quick_open_items(&self.app),
            );
        }
    }

    fn handle_overlay_keyboard(&mut self, ctx: &egui::Context) {
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
        if items.is_empty() && !self.layout.settings_open {
            return;
        }

        if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
            self.layout.close_overlays();
            return;
        }
        if !items.is_empty() && ctx.input(|input| input.key_pressed(egui::Key::ArrowDown)) {
            self.layout.move_overlay_selection(1, items.len());
        }
        if !items.is_empty() && ctx.input(|input| input.key_pressed(egui::Key::ArrowUp)) {
            self.layout.move_overlay_selection(-1, items.len());
        }
        if ctx.input(|input| input.key_pressed(egui::Key::Enter)) {
            if let Some(action) = selected_action(&items, self.layout.overlay_selected) {
                self.apply_command_action(action);
            } else if self.layout.settings_open {
                self.app.switch_pane(StudioPane::Settings);
                self.layout.close_overlays();
            }
        }
    }

    fn render_settings_overlay(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .id(egui::Id::new("studio_settings_overlay"))
            .collapsible(false)
            .resizable(false)
            .default_width(560.0)
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 96.0))
            .show(ctx, |ui| {
                ui::section_header(ui, "Settings", "Mod+,");
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Config").color(ui::muted_text(ctx)));
                    ui.label(self.app.config_path().display().to_string());
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Hotkey").color(ui::muted_text(ctx)));
                    ui.label(&self.settings_hotkey);
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Theme").color(ui::muted_text(ctx)));
                    ui.label(&self.settings_theme);
                });
                if ui::quiet_button(ui, "Open Settings Pane").clicked() {
                    self.app.switch_pane(StudioPane::Settings);
                    self.layout.close_overlays();
                }
            });
    }

    fn render_command_overlay(
        &mut self,
        ctx: &egui::Context,
        id: &'static str,
        title: &str,
        shortcut: &str,
        items: Vec<StudioCommandItem>,
    ) {
        egui::Window::new(title)
            .id(egui::Id::new(id))
            .collapsible(false)
            .resizable(false)
            .default_width(520.0)
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 96.0))
            .show(ctx, |ui| {
                ui::section_header(ui, title, shortcut);
                let query = if id == "studio_command_palette" {
                    &mut self.layout.command_query
                } else {
                    &mut self.layout.quick_open_query
                };
                let response = ui.add(
                    egui::TextEdit::singleline(query)
                        .hint_text("Filter commands")
                        .desired_width(f32::INFINITY),
                );
                if response.changed() {
                    self.layout.overlay_selected = 0;
                }
                response.request_focus();
                ui.add_space(8.0);

                let filtered_items = filter_items(&items, query);
                self.layout.clamp_overlay_selection(filtered_items.len());
                if filtered_items.is_empty() {
                    ui.label(egui::RichText::new("No matches").color(ui::muted_text(ctx)));
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
            .inner_margin(egui::Margin::symmetric(8, 4))
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
            StudioCommandAction::OpenSettings => self.app.switch_pane(StudioPane::Settings),
            StudioCommandAction::Refresh => self.app.refresh(),
        }
        self.layout.close_overlays();
    }
}

impl StudioEguiApp {
    fn open_row(&mut self, ui: &mut egui::Ui, title: &str, detail: &str, pane: StudioPane) {
        let response = ui
            .horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(title).color(ui::strong_text(ui.ctx())));
                    ui.label(egui::RichText::new(detail).color(ui::muted_text(ui.ctx())));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui::quiet_button(ui, "Open")
                })
                .inner
            })
            .inner;
        if response.clicked() {
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
    }
}
