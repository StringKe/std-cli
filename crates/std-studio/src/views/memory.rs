use crate::{ui, StudioEguiApp};
use eframe::egui;
use std_types::MemoryRecord;

impl StudioEguiApp {
    pub(crate) fn render_memory(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            "Memory Browser",
            "search, inspect, write through std-core storage",
        );
        self.render_memory_toolbar(ui);
        ui.add_space(10.0);
        ui.columns(3, |columns| {
            columns[0].vertical(|ui| self.render_memory_records(ui));
            columns[1].vertical(|ui| self.render_memory_detail(ui));
            columns[2].vertical(|ui| self.render_memory_writer(ui));
        });
    }

    fn render_memory_toolbar(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_sized(
                    [ui.available_width() - 110.0, 28.0],
                    egui::TextEdit::singleline(&mut self.memory_query)
                        .hint_text("title, body, tag, scope"),
                );
                if ui::quiet_button(ui, "Search").clicked() {
                    let query = self.memory_query.clone();
                    let results = self.app.search_memory(&query);
                    self.status = format!("{} memories", results.len());
                }
            });
        });
    }

    fn render_memory_records(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Records", "local recall results");
            if self.app.memory_browser.memories.is_empty() {
                ui::empty_state(ui, "No memory records");
                return;
            }
            let mut clicked_memory = None;
            egui::ScrollArea::vertical()
                .max_height(590.0)
                .show(ui, |ui| {
                    for (index, memory) in self.app.memory_browser.memories.iter().enumerate() {
                        let selected = index == self.app.memory_browser.selected;
                        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                            if ui.selectable_label(selected, &memory.title).clicked() {
                                clicked_memory = Some(index);
                            }
                            ui.horizontal_wrapped(|ui| {
                                ui::chip(ui, &memory.scope, ui::selected_bg(ui.ctx()));
                                for tag in &memory.tags {
                                    ui::chip(ui, tag, ui::panel_alt(ui.ctx()));
                                }
                            });
                            ui.small(memory_preview(&memory.body));
                        });
                    }
                });
            if let Some(index) = clicked_memory {
                self.app.select_memory(index);
            }
        });
    }

    fn render_memory_detail(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Detail", "selected memory");
            let Some(memory) = self.app.memory_browser.selected_memory() else {
                ui::empty_state(ui, "Select a memory");
                return;
            };
            render_memory_metadata(ui, memory);
            ui.add_space(8.0);
            egui::ScrollArea::vertical()
                .max_height(480.0)
                .show(ui, |ui| {
                    ui.label(&memory.body);
                });
        });
    }

    fn render_memory_writer(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Write", "persist new context");
            ui.label("Scope");
            ui.text_edit_singleline(&mut self.memory_scope);
            ui.label("Title");
            ui.text_edit_singleline(&mut self.memory_title);
            ui.label("Body");
            ui.add_sized(
                [ui.available_width(), 220.0],
                egui::TextEdit::multiline(&mut self.memory_body),
            );
            ui.label("Tags");
            ui.text_edit_singleline(&mut self.memory_tags);
            ui.horizontal(|ui| {
                if ui::quiet_button(ui, "Remember").clicked() {
                    self.write_memory_from_form();
                }
                if ui::quiet_button(ui, "Clear").clicked() {
                    self.memory_title.clear();
                    self.memory_body.clear();
                    self.memory_tags.clear();
                }
            });
            if let Some(memory) = &self.app.memory_browser.last_written {
                ui.add_space(8.0);
                ui::chip(ui, &format!("last: {}", memory.title), ui::ok_bg(ui.ctx()));
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

fn render_memory_metadata(ui: &mut egui::Ui, memory: &MemoryRecord) {
    ui.label(egui::RichText::new(&memory.title).strong().size(18.0));
    ui.horizontal_wrapped(|ui| {
        ui::chip(ui, &memory.scope, ui::selected_bg(ui.ctx()));
        for tag in &memory.tags {
            ui::chip(ui, tag, ui::panel_alt(ui.ctx()));
        }
    });
    ui.small(format!("id={}", memory.id));
    ui.small(format!("updated={}", memory.updated_at));
}

fn memory_preview(body: &str) -> String {
    let preview = body
        .lines()
        .next()
        .unwrap_or("")
        .chars()
        .take(96)
        .collect::<String>();
    if preview.len() < body.len() {
        format!("{preview}...")
    } else {
        preview
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
