use crate::{
    ui,
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
        if available_width < 900.0 {
            self.render_memory_records(ui);
            ui.add_space(MEMORY_PANEL_GAP);
            self.render_memory_detail(ui);
            ui.add_space(MEMORY_PANEL_GAP);
            self.render_memory_writer(ui);
            return;
        }
        let column_width = (available_width - MEMORY_PANEL_GAP * 2.0) / 3.0;
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
                    [ui.available_width() - 110.0, 28.0],
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
                .max_height(590.0)
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
                .max_height(480.0)
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
            ui.label(i18n::t("studio.memory.scope"));
            ui.text_edit_singleline(&mut self.memory_scope);
            ui.label(i18n::t("studio.memory.item_title"));
            ui.text_edit_singleline(&mut self.memory_title);
            ui.label(i18n::t("studio.memory.body"));
            ui.add_sized(
                [ui.available_width(), 220.0],
                egui::TextEdit::multiline(&mut self.memory_body),
            );
            ui.label(i18n::t("studio.memory.tags"));
            ui.text_edit_singleline(&mut self.memory_tags);
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
