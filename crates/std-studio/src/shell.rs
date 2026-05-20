use crate::{ui, StudioEguiApp};
use eframe::egui;
use std_studio::StudioPane;

impl StudioEguiApp {
    pub(crate) fn render_shell(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("studio_app_chrome")
            .exact_height(52.0)
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_app_chrome(ui));
        egui::SidePanel::left("studio_nav")
            .resizable(false)
            .exact_width(208.0)
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_navigation(ui));
        egui::SidePanel::right("studio_context")
            .resizable(true)
            .default_width(300.0)
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_context(ui));
        egui::TopBottomPanel::bottom("studio_status")
            .exact_height(24.0)
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_status_bar(ui));
        egui::CentralPanel::default()
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_0(ctx)))
            .show(ctx, |ui| self.render_active_workspace(ui));
    }

    fn render_app_chrome(&mut self, ui: &mut egui::Ui) {
        let frame = egui::Frame::new()
            .fill(std_egui::tokens::Color::bg_surface_1(ui.ctx()))
            .inner_margin(egui::Margin::symmetric(14, 8));
        frame.show(ui, |ui| {
            let drag_rect = ui.max_rect();
            let drag_response = ui.interact(
                drag_rect,
                ui.id().with("host_drag"),
                egui::Sense::click_and_drag(),
            );
            if drag_response.drag_started() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }
            ui.horizontal(|ui| {
                self.render_top_identity(ui);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    host_window_controls(ui, &mut self.host_maximized);
                    ui.add_space(12.0);
                    self.render_top_actions(ui);
                });
            });
        });
    }

    fn render_top_identity(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new(&self.app.name)
                    .font(std_egui::tokens::Text::headline())
                    .strong()
                    .color(ui::strong_text(ui.ctx())),
            );
            ui.label(
                egui::RichText::new(self.app.active_pane.label()).color(ui::muted_text(ui.ctx())),
            );
        });
    }

    fn render_top_actions(&mut self, ui: &mut egui::Ui) {
        if ui::quiet_button(ui, "Refresh").clicked() {
            self.app.refresh();
            self.status = "refreshed workspace state".to_string();
        }
        if ui::quiet_button(ui, "Open Current Pane").clicked() {
            let id = self.app.open_workspace_pane(self.app.active_pane);
            self.status = format!("opened workspace pane {}", id.value());
        }
        ui.label(
            egui::RichText::new(format!(
                "{} workspace panes",
                self.app.open_workspace_panes().count()
            ))
            .color(ui::muted_text(ui.ctx())),
        );
    }

    fn render_navigation(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
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
        self.render_window_manager(ui);
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

fn host_window_controls(ui: &mut egui::Ui, host_maximized: &mut bool) {
    ui.horizontal(|ui| {
        if host_control(ui, "Exit", "Close Studio").clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if host_control(ui, "Hide", "Minimize Studio").clicked() {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }
        let maximize_label = if *host_maximized { "Fit" } else { "Fill" };
        if host_control(ui, maximize_label, "Toggle Studio size").clicked() {
            *host_maximized = !*host_maximized;
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Maximized(*host_maximized));
        }
    });
}

fn host_control(ui: &mut egui::Ui, label: &str, tooltip: &str) -> egui::Response {
    ui.add(
        egui::Button::new(
            egui::RichText::new(label)
                .font(std_egui::tokens::Text::caption())
                .color(ui::muted_text(ui.ctx())),
        )
        .min_size(egui::vec2(40.0, 24.0))
        .fill(egui::Color32::TRANSPARENT)
        .stroke(egui::Stroke::new(
            1.0,
            std_egui::tokens::Color::stroke_divider(ui.ctx()),
        ))
        .corner_radius(egui::CornerRadius::same(4)),
    )
    .on_hover_text(tooltip)
}

fn path_label(ui: &mut egui::Ui, label: &str, value: String) {
    ui.label(egui::RichText::new(label).color(ui::muted_text(ui.ctx())));
    ui.label(
        egui::RichText::new(value)
            .monospace()
            .color(ui::strong_text(ui.ctx())),
    );
    ui.add_space(4.0);
}

fn panel_frame(ctx: &egui::Context, fill: egui::Color32) -> egui::Frame {
    egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(
            1.0,
            std_egui::tokens::Color::stroke_divider(ctx),
        ))
        .inner_margin(egui::Margin::symmetric(10, 6))
}
