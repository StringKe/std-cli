use crate::{
    studio_metrics, ui,
    views::memory_rows::{self, MemoryRowEvent},
    StudioEguiApp,
};
use eframe::egui;
use std_egui::{i18n, tokens::Space};

const MEMORY_PANEL_GAP: f32 = Space::SM as f32;

impl StudioEguiApp {
    pub(crate) fn render_memory(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            i18n::t("studio.memory.title"),
            i18n::t("studio.memory.detail"),
        );
        self.render_memory_toolbar(ui);
        ui.add_space(Space::SM as f32);
        self.render_memory_workspace(ui);
    }

    fn render_memory_workspace(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        if available_width < studio_metrics::WIDE_WORKSPACE_BREAKPOINT {
            self.render_memory_records(ui);
            ui.add_space(MEMORY_PANEL_GAP);
            self.render_memory_detail(ui);
            ui.add_space(MEMORY_PANEL_GAP);
            self.render_memory_writer(ui);
            return;
        }
        let column_width = studio_metrics::thirds_column_width(available_width, MEMORY_PANEL_GAP);
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_memory_records(ui),
            );
            ui.add_space(MEMORY_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_memory_detail(ui),
            );
            ui.add_space(MEMORY_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_memory_writer(ui),
            );
        });
    }

    fn render_memory_toolbar(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui.horizontal(|ui| {
                let query_response = ui.add_sized(
                    [
                        studio_metrics::toolbar_field_width(
                            ui.available_width(),
                            studio_metrics::FORM_BUTTON_RESERVE_WIDTH,
                        ),
                        studio_metrics::INPUT_HEIGHT,
                    ],
                    egui::TextEdit::singleline(&mut self.memory_query)
                        .hint_text(i18n::t("studio.memory.search.hint")),
                );
                query_response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::TextEdit,
                        ui.is_enabled(),
                        memory_query_a11y_label(&self.memory_query),
                    )
                });
                if ui::quiet_button(ui, i18n::t("studio.memory.search")).clicked() {
                    let query = self.memory_query.clone();
                    let results = self.app.search_memory(&query);
                    self.status = format!("{} memories", results.len());
                }
            });
        });
    }

    fn render_memory_records(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.memory.records.title"),
                i18n::t("studio.memory.records.detail"),
            );
            if self.app.memory_browser.memories.is_empty() {
                ui::empty_state(ui, i18n::t("studio.memory.records.empty"));
                return;
            }
            let mut clicked_memory = None;
            egui::ScrollArea::vertical()
                .max_height(studio_metrics::MEMORY_LIST_MAX_HEIGHT)
                .show(ui, |ui| {
                    for (index, memory) in self.app.memory_browser.memories.iter().enumerate() {
                        let selected = index == self.app.memory_browser.selected;
                        if let MemoryRowEvent::Select(index) =
                            memory_rows::memory_row(ui, index, memory, selected)
                        {
                            clicked_memory = Some(index);
                        }
                    }
                });
            if let Some(index) = clicked_memory {
                self.app.select_memory(index);
            }
        });
    }

    fn render_memory_detail(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.memory.detail.title"),
                i18n::t("studio.memory.detail.detail"),
            );
            let Some(memory) = self.app.memory_browser.selected_memory() else {
                ui::empty_state(ui, i18n::t("studio.memory.detail.empty"));
                return;
            };
            memory_rows::memory_metadata(ui, memory);
            ui.add_space(Space::XS as f32);
            egui::ScrollArea::vertical()
                .max_height(studio_metrics::DETAIL_BODY_MAX_HEIGHT)
                .show(ui, |ui| {
                    ui.label(&memory.body);
                });
        });
    }

    fn render_memory_writer(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.memory.write.title"),
                i18n::t("studio.memory.write.detail"),
            );
            memory_text_input(ui, i18n::t("studio.memory.scope"), &mut self.memory_scope);
            memory_text_input(
                ui,
                i18n::t("studio.memory.item_title"),
                &mut self.memory_title,
            );
            memory_body_input(ui, i18n::t("studio.memory.body"), &mut self.memory_body);
            memory_text_input(ui, i18n::t("studio.memory.tags"), &mut self.memory_tags);
            ui.horizontal(|ui| {
                if ui::quiet_button(ui, i18n::t("studio.memory.remember")).clicked() {
                    self.write_memory_from_form();
                }
                if ui::quiet_button(ui, i18n::t("studio.memory.clear")).clicked() {
                    self.memory_title.clear();
                    self.memory_body.clear();
                    self.memory_tags.clear();
                }
            });
            if let Some(memory) = &self.app.memory_browser.last_written {
                ui.add_space(Space::XS as f32);
                memory_rows::last_written(ui, memory);
            }
        });
    }

    fn write_memory_from_form(&mut self) {
        let tags = parse_tags(&self.memory_tags);
        match self.app.remember_from_studio(
            &self.memory_scope,
            &self.memory_title,
            &self.memory_body,
            tags,
        ) {
            Ok(memory) => {
                self.status = format!("remembered {}", memory.title);
                self.memory_title.clear();
                self.memory_body.clear();
                self.memory_tags.clear();
            }
            Err(error) => self.status = error.to_string(),
        }
    }
}

fn parse_tags(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn memory_query_a11y_label(query: &str) -> String {
    let value = if query.trim().is_empty() {
        "empty"
    } else {
        query.trim()
    };
    format!("Memory search, text box, value {value}")
}

fn memory_text_input(ui: &mut egui::Ui, label: &str, value: &mut String) {
    ui.label(label);
    let response = ui.add_sized(
        [ui.available_width(), studio_metrics::INPUT_HEIGHT],
        egui::TextEdit::singleline(value),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            memory_field_a11y_label(label, value),
        )
    });
}

fn memory_body_input(ui: &mut egui::Ui, label: &str, value: &mut String) {
    ui.label(label);
    let response = ui.add_sized(
        [ui.available_width(), studio_metrics::MULTILINE_INPUT_HEIGHT],
        egui::TextEdit::multiline(value),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            memory_field_a11y_label(label, value),
        )
    });
}

fn memory_field_a11y_label(label: &str, value: &str) -> String {
    let value = if value.trim().is_empty() {
        "empty"
    } else {
        value.trim()
    };
    format!("{label}, text box, value {value}")
}
