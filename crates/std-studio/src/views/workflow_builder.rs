use crate::{ui, StudioEguiApp};
use eframe::egui;
use std::path::Path;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};

const BUILDER_STEP_ROW_HEIGHT: f32 = 48.0;
const BUILDER_PANEL_GAP: f32 = Space::SM as f32;

impl StudioEguiApp {
    pub(crate) fn render_workflow_builder(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflow_builder.title"),
                i18n::t("studio.workflow_builder.detail"),
            );
            self.render_builder_toolbar(ui);
            ui.add_space(Space::XS as f32);
            self.render_builder_workspace(ui);
            ui.add_space(Space::XS as f32);
            self.render_ai_assist_panel(ui);
        });
    }

    fn render_builder_workspace(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        if available_width < 560.0 {
            self.render_builder_steps(ui);
            ui.add_space(BUILDER_PANEL_GAP);
            self.render_step_properties(ui);
            return;
        }
        let left_width = ((available_width - BUILDER_PANEL_GAP) * 0.48).max(260.0);
        let right_width = (available_width - left_width - BUILDER_PANEL_GAP).max(260.0);
        ui.horizontal_top(|ui| {
            ui.set_min_width(available_width);
            ui.allocate_ui_with_layout(
                egui::vec2(left_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_builder_steps(ui),
            );
            ui.add_space(BUILDER_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(right_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_step_properties(ui),
            );
        });
    }

    fn render_builder_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.add_sized(
                [ui.available_width().min(260.0), 28.0],
                egui::TextEdit::singleline(&mut self.workflow_goal)
                    .hint_text(i18n::t("studio.workflow_builder.goal.hint")),
            );
            if ui::quiet_button(ui, i18n::t("studio.workflow_builder.plan")).clicked() {
                self.plan_workflow_from_goal();
            }
            if ui::quiet_button(ui, i18n::t("studio.workflow_builder.simulate")).clicked() {
                self.preview_active_workflow();
            }
            if ui::quiet_button(ui, i18n::t("studio.workflow_builder.run")).clicked() {
                self.run_active_workflow();
            }
            if ui::quiet_button(ui, i18n::t("studio.workflow_builder.save")).clicked() {
                self.save_planned_workflow();
            }
        });
    }

    fn render_builder_steps(&mut self, ui: &mut egui::Ui) {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflow_builder.steps.title"),
                i18n::t("studio.workflow_builder.steps.detail"),
            );
            if self.app.planned_workflow.is_some() {
                self.render_planned_steps(ui);
                return;
            }
            if self.workflow_selected_path.is_some() {
                self.render_debug_steps(ui);
            } else {
                ui::empty_state(ui, i18n::t("studio.workflow_builder.steps.empty"));
            }
        });
    }

    fn render_planned_steps(&mut self, ui: &mut egui::Ui) {
        let Some(workflow) = &self.app.planned_workflow else {
            return;
        };
        let name = workflow.name.clone();
        let rows = workflow
            .steps
            .iter()
            .map(|step| (step.name.clone(), format!("{:?}", step.step_type)))
            .collect::<Vec<_>>();
        workflow_step_summary(ui, &name, rows.len());
        for (index, (name, detail)) in rows.iter().enumerate() {
            self.step_row(ui, index, name, detail);
        }
    }

    fn render_debug_steps(&mut self, ui: &mut egui::Ui) {
        let Some(debug) = &self.app.workflow_debug else {
            ui::empty_state(ui, i18n::t("studio.workflow_builder.preview.empty"));
            return;
        };
        let name = debug.workflow_name.clone();
        let rows = debug
            .steps
            .iter()
            .map(|step| (step.step_name.clone(), format!("{:?}", step.step_type)))
            .collect::<Vec<_>>();
        workflow_step_summary(ui, &name, rows.len());
        for (index, (name, detail)) in rows.iter().enumerate() {
            self.step_row(ui, index, name, detail);
        }
    }

    fn step_row(&mut self, ui: &mut egui::Ui, index: usize, name: &str, detail: &str) {
        let selected = self.workflow_edit_index.trim() == index.to_string();
        let response = workflow_step_row(ui, index, name, detail, selected);
        if response.clicked() {
            self.workflow_edit_index = index.to_string();
            self.workflow_step_name = name.to_string();
        }
    }

    fn render_step_properties(&mut self, ui: &mut egui::Ui) {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflow_builder.properties.title"),
                i18n::t("studio.workflow_builder.properties.detail"),
            );
            let selected = self.workflow_selected_path.clone();
            let Some(path) = selected else {
                ui::empty_state(ui, i18n::t("studio.workflow_builder.properties.empty"));
                return;
            };
            ui.small(path.display().to_string());
            ui.label(i18n::t("studio.workflow_builder.step_name"));
            ui.text_edit_singleline(&mut self.workflow_step_name);
            ui.label(i18n::t("studio.workflow_builder.parameters"));
            ui.add_sized(
                [ui.available_width(), 92.0],
                egui::TextEdit::multiline(&mut self.workflow_step_parameters),
            );
            ui.horizontal(|ui| {
                ui.label(i18n::t("studio.workflow_builder.index"));
                ui.add_sized(
                    [48.0, 24.0],
                    egui::TextEdit::singleline(&mut self.workflow_edit_index),
                );
                if ui::quiet_button(ui, i18n::t("studio.workflow_builder.add")).clicked() {
                    self.add_step_to_selected(&path);
                }
                if ui::quiet_button(ui, i18n::t("studio.workflow_builder.update")).clicked() {
                    self.update_selected_step(&path);
                }
            });
            ui.horizontal_wrapped(|ui| {
                if ui::quiet_button(ui, i18n::t("studio.workflow_builder.move_up")).clicked() {
                    self.move_selected_step(&path, -1);
                }
                if ui::quiet_button(ui, i18n::t("studio.workflow_builder.move_down")).clicked() {
                    self.move_selected_step(&path, 1);
                }
                if ui::quiet_button(ui, i18n::t("studio.workflow_builder.remove")).clicked() {
                    self.remove_selected_step(&path);
                }
            });
        });
    }

    fn render_ai_assist_panel(&mut self, ui: &mut egui::Ui) {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflow_builder.ai.title"),
                i18n::t("studio.workflow_builder.ai.detail"),
            );
            ui.label(i18n::t("studio.workflow_builder.ai.prompt"));
            ui.add_sized(
                [ui.available_width(), 32.0],
                egui::TextEdit::singleline(&mut self.workflow_goal),
            );
        });
    }

    fn preview_active_workflow(&mut self) {
        if let Some(path) = self.workflow_selected_path.clone() {
            self.preview_workflow_path(&path);
        } else {
            self.preview_planned_workflow();
        }
    }

    fn run_active_workflow(&mut self) {
        if let Some(path) = self.workflow_selected_path.clone() {
            self.run_workflow_path(&path);
        } else {
            self.status = "select saved workflow before running".to_string();
        }
    }

    pub(crate) fn plan_workflow_from_goal(&mut self) {
        let goal = self.workflow_goal.clone();
        match self.app.plan_workflow(&goal) {
            Ok(workflow) => {
                self.status = format!("planned {} steps={}", workflow.name, workflow.steps.len())
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn preview_planned_workflow(&mut self) {
        match self.app.planned_workflow.clone() {
            Some(workflow) => match self.app.preview_workflow(&workflow) {
                Ok(preview) => {
                    self.status = format!(
                        "dry-run {} steps={}",
                        preview.workflow_name,
                        preview.steps.len()
                    )
                }
                Err(error) => self.status = error.to_string(),
            },
            None => self.status = "missing planned workflow".to_string(),
        }
    }

    pub(crate) fn save_planned_workflow(&mut self) {
        match self.app.save_planned_workflow() {
            Ok(path) => {
                self.workflow_selected_path = Some(path.clone());
                self.status = format!("saved {}", path.display());
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn preview_workflow_path(&mut self, path: &Path) {
        match self.app.preview_workflow_path(path) {
            Ok(preview) => {
                self.status = format!(
                    "dry-run {} steps={}",
                    preview.workflow_name,
                    preview.steps.len()
                )
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn run_workflow_path(&mut self, path: &Path) {
        match self.app.run_workflow_path(path) {
            Ok(execution) => {
                self.status = format!(
                    "run {:?} steps={}",
                    execution.status,
                    execution.results.len()
                )
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn add_step_to_selected(&mut self, path: &Path) {
        let Ok(parameters) = serde_json::from_str(&self.workflow_step_parameters) else {
            self.status = "invalid step parameters JSON".to_string();
            return;
        };
        match self
            .app
            .add_workflow_step(path, &self.workflow_step_name, parameters)
        {
            Ok(step) => self.status = format!("added step {}", step.name),
            Err(error) => self.status = error.to_string(),
        }
    }

    fn update_selected_step(&mut self, path: &Path) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        let Ok(parameters) = serde_json::from_str(&self.workflow_step_parameters) else {
            self.status = "invalid step parameters JSON".to_string();
            return;
        };
        match self.app.update_workflow_step(
            path,
            index,
            Some(&self.workflow_step_name),
            Some(parameters),
        ) {
            Ok(step) => self.status = format!("updated step {}", step.name),
            Err(error) => self.status = error.to_string(),
        }
    }

    fn move_selected_step(&mut self, path: &Path, offset: isize) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        let target = index.saturating_add_signed(offset);
        match self.app.move_workflow_step(path, index, target) {
            Ok(step) => {
                self.workflow_edit_index = target.to_string();
                self.status = format!("moved step {}", step.name);
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn remove_selected_step(&mut self, path: &Path) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        match self.app.remove_workflow_step(path, index) {
            Ok(step) => self.status = format!("removed step {}", step.name),
            Err(error) => self.status = error.to_string(),
        }
    }

    fn selected_step_index(&mut self) -> Option<usize> {
        match self.workflow_edit_index.trim().parse::<usize>() {
            Ok(index) => Some(index),
            Err(_) => {
                self.status = "invalid step index".to_string();
                None
            }
        }
    }
}

fn workflow_step_summary(ui: &mut egui::Ui, name: &str, steps: usize) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(name)
                .font(Text::body())
                .color(ui::strong_text(ui.ctx())),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!("steps={steps}"))
                    .font(Text::caption())
                    .color(ui::muted_text(ui.ctx())),
            );
        });
    });
    ui.add_space(Space::TWO_XS as f32);
}

fn workflow_step_row(
    ui: &mut egui::Ui,
    index: usize,
    name: &str,
    detail: &str,
    selected: bool,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), BUILDER_STEP_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), name));
    if ui.is_rect_visible(rect) {
        paint_workflow_step_row(ui, rect, response.hovered(), selected);
        let index_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + Space::XS as f32, rect.center().y - 11.0),
            egui::vec2(32.0, 22.0),
        );
        paint_step_index(ui, index_rect, index + 1, selected);
        let text_x = index_rect.right() + Space::XS as f32;
        ui.painter().text(
            egui::pos2(text_x, rect.top() + 17.0),
            egui::Align2::LEFT_CENTER,
            name,
            Text::body(),
            ui::strong_text(ui.ctx()),
        );
        ui.painter().text(
            egui::pos2(text_x, rect.top() + 34.0),
            egui::Align2::LEFT_CENTER,
            detail,
            Text::caption(),
            ui::muted_text(ui.ctx()),
        );
    }
    ui.add_space(Space::TWO_XS as f32);
    response
}

fn paint_workflow_step_row(ui: &mut egui::Ui, rect: egui::Rect, hovered: bool, selected: bool) {
    let fill = if selected {
        Color::accent_weak(ui.ctx())
    } else if hovered {
        Color::bg_surface_3(ui.ctx())
    } else {
        Color::bg_surface_2(ui.ctx())
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(Radius::SM),
        egui::Stroke::new(1.0, Color::stroke_divider(ui.ctx())),
        egui::StrokeKind::Inside,
    );
    if selected {
        let strip = egui::Rect::from_min_max(
            rect.left_top(),
            egui::pos2(rect.left() + 3.0, rect.bottom()),
        );
        ui.painter().rect_filled(
            strip,
            egui::CornerRadius::same(Radius::SM),
            Color::accent_base(ui.ctx()),
        );
    }
}

fn paint_step_index(ui: &mut egui::Ui, rect: egui::Rect, number: usize, selected: bool) {
    let fill = if selected {
        Color::accent_base(ui.ctx())
    } else {
        Color::bg_surface_1(ui.ctx())
    };
    let text_color = if selected {
        Color::bg_surface_0(ui.ctx())
    } else {
        Color::fg_secondary(ui.ctx())
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        number.to_string(),
        Text::caption(),
        text_color,
    );
}
