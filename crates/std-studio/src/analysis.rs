use crate::{ui, StudioEguiApp};
use eframe::egui;
use std_index::{
    IndexAnswer, IndexCoverage, IndexCoverageReport, IndexDocument, IndexInspection,
    IndexSearchResult,
};

impl StudioEguiApp {
    pub(crate) fn render_analysis(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            "Index Analysis",
            "four-layer local understanding and QA",
        );
        self.render_analysis_toolbar(ui);
        ui.add_space(10.0);
        ui.columns(3, |columns| {
            columns[0].vertical(|ui| self.render_active_analysis(ui));
            columns[1].vertical(|ui| self.render_analysis_query(ui));
            columns[2].vertical(|ui| self.render_analysis_coverage(ui));
        });
    }

    fn render_analysis_toolbar(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_sized(
                    [ui.available_width() - 120.0, 28.0],
                    egui::TextEdit::singleline(&mut self.analysis_path)
                        .hint_text("project, file, app, workflow path"),
                );
                if ui::quiet_button(ui, "Analyze").clicked() {
                    self.analyze_current_path();
                }
            });
        });
    }

    fn render_active_analysis(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Entity", "active index document");
            let Some(document) = &self.app.active_analysis else {
                ui::empty_state(ui, "No active analysis");
                return;
            };
            render_document_overview(ui, document);
            ui.add_space(8.0);
            render_components(ui, document);
        });
    }

    fn render_analysis_query(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Ask and Search", "saved index documents");
            ui.text_edit_singleline(&mut self.analysis_query);
            ui.horizontal(|ui| {
                if ui::quiet_button(ui, "Ask").clicked() {
                    self.ask_analysis();
                }
                if ui::quiet_button(ui, "Search").clicked() {
                    self.search_analysis();
                }
                if ui::quiet_button(ui, "Inspect").clicked() {
                    self.inspect_analysis();
                }
            });
            ui.add_space(8.0);
            render_output(ui, "Answer", &self.analysis_answer, 180.0);
            render_output(ui, "Search", &self.analysis_search_output, 180.0);
        });
    }

    fn render_analysis_coverage(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Coverage", "overview, components, relations, history");
            if ui::quiet_button(ui, "Refresh Coverage").clicked() {
                self.refresh_analysis_coverage();
            }
            if self.analysis_coverage_output.is_empty() {
                match self.app.analysis_coverage_report() {
                    Ok(report) => render_coverage_report(ui, &report),
                    Err(error) => {
                        ui.label(error.to_string());
                    }
                }
            } else {
                render_output(ui, "Coverage Report", &self.analysis_coverage_output, 460.0);
            }
        });
    }

    fn analyze_current_path(&mut self) {
        let path = std::path::PathBuf::from(self.analysis_path.clone());
        match self.app.analyze_entity(&path) {
            Ok(document) => {
                self.status = format!(
                    "analyzed {} components {} relations",
                    document.components.len(),
                    document.relations.len()
                );
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn ask_analysis(&mut self) {
        match self.app.ask_analyses(&self.analysis_query, 5) {
            Ok(answer) => self.analysis_answer = format_analysis_answer(&answer),
            Err(error) => self.analysis_answer = error.to_string(),
        }
    }

    fn search_analysis(&mut self) {
        match self.app.search_analyses(&self.analysis_query, 8) {
            Ok(results) => self.analysis_search_output = format_search_results(&results),
            Err(error) => self.analysis_search_output = error.to_string(),
        }
    }

    fn inspect_analysis(&mut self) {
        match self.app.inspect_analysis(&self.analysis_query, 8) {
            Ok(Some(inspection)) => self.analysis_answer = format_inspection(&inspection),
            Ok(None) => self.analysis_answer = "analysis not found".to_string(),
            Err(error) => self.analysis_answer = error.to_string(),
        }
    }

    fn refresh_analysis_coverage(&mut self) {
        match self.app.analysis_coverage_report() {
            Ok(report) => self.analysis_coverage_output = format_coverage_report(&report),
            Err(error) => self.analysis_coverage_output = error.to_string(),
        }
    }
}

fn render_document_overview(ui: &mut egui::Ui, document: &IndexDocument) {
    ui.label(
        egui::RichText::new(&document.overview.name)
            .strong()
            .size(18.0),
    );
    ui.horizontal_wrapped(|ui| {
        ui::chip(
            ui,
            &format!("{:?}", document.overview.kind),
            ui::selected_bg(ui.ctx()),
        );
        ui::chip(
            ui,
            &format!("components={}", document.components.len()),
            ui::panel_alt(ui.ctx()),
        );
        ui::chip(
            ui,
            &format!("relations={}", document.relations.len()),
            ui::panel_alt(ui.ctx()),
        );
        ui::chip(
            ui,
            &format!("history={}", document.history.len()),
            ui::panel_alt(ui.ctx()),
        );
    });
    ui.small(document.overview.path.display().to_string());
    ui.label(&document.overview.summary);
}

fn render_components(ui: &mut egui::Ui, document: &IndexDocument) {
    egui::ScrollArea::vertical()
        .max_height(450.0)
        .show(ui, |ui| {
            for component in document.components.iter().take(12) {
                ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                    ui.label(component.path.display().to_string());
                    ui.small(format!("{} {}", component.language, component.purpose));
                    if !component.symbols.is_empty() {
                        ui.small(format!("symbols: {}", component.symbols.join(", ")));
                    }
                });
            }
        });
}

fn render_coverage_report(ui: &mut egui::Ui, report: &IndexCoverageReport) {
    ui.horizontal_wrapped(|ui| {
        ui::chip(
            ui,
            &format!("total={}", report.total),
            ui::panel_alt(ui.ctx()),
        );
        ui::chip(
            ui,
            &format!("complete={}", report.complete),
            ui::ok_bg(ui.ctx()),
        );
        ui::chip(
            ui,
            &format!("incomplete={}", report.incomplete),
            ui::warn_bg(ui.ctx()),
        );
    });
    egui::ScrollArea::vertical()
        .max_height(460.0)
        .show(ui, |ui| {
            for item in &report.items {
                ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                    ui.label(&item.name);
                    render_coverage_chips(ui, &item.coverage);
                    ui.small(format!(
                        "components={} relations={} history={}",
                        item.component_count, item.relation_count, item.history_count
                    ));
                });
            }
        });
}

fn render_coverage_chips(ui: &mut egui::Ui, coverage: &IndexCoverage) {
    ui.horizontal_wrapped(|ui| {
        coverage_chip(ui, "overview", coverage.entity_overview);
        coverage_chip(ui, "components", coverage.component_digest);
        coverage_chip(ui, "relations", coverage.symbol_relation_index);
        coverage_chip(ui, "history", coverage.historical_context);
    });
}

fn coverage_chip(ui: &mut egui::Ui, label: &str, pass: bool) {
    ui::chip(
        ui,
        label,
        if pass {
            ui::ok_bg(ui.ctx())
        } else {
            ui::warn_bg(ui.ctx())
        },
    );
}

fn render_output(ui: &mut egui::Ui, title: &str, value: &str, height: f32) {
    ui.label(egui::RichText::new(title).strong());
    if value.is_empty() {
        ui::empty_state(ui, "No output");
    } else {
        egui::ScrollArea::vertical()
            .max_height(height)
            .show(ui, |ui| {
                ui.label(value);
            });
    }
}

fn format_search_results(results: &[IndexSearchResult]) -> String {
    if results.is_empty() {
        return "no index search results".to_string();
    }
    results
        .iter()
        .map(|result| {
            format!(
                "{} score={:.2} fields={}",
                result.document.overview.name,
                result.score,
                result.matched_fields.join(",")
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_analysis_answer(answer: &IndexAnswer) -> String {
    let mut lines = vec![answer.answer.clone()];
    for source in &answer.sources {
        lines.push(format!("source: {} {}", source.entity, source.reason));
        lines.extend(
            source
                .evidence
                .iter()
                .map(|evidence| format!("evidence: {evidence}")),
        );
    }
    lines.join("\n")
}

fn format_inspection(inspection: &IndexInspection) -> String {
    let mut lines = vec![
        format!("entity: {}", inspection.overview.name),
        format!("kind: {:?}", inspection.overview.kind),
        format!("path: {}", inspection.overview.path.display()),
        format!("summary: {}", inspection.overview.summary),
        format!("components: {}", inspection.component_count),
        format!("relations: {}", inspection.relation_count),
        format!("history: {}", inspection.history_count),
        format!(
            "coverage: overview={} components={} relations={} history={} complete={}",
            inspection.coverage.entity_overview,
            inspection.coverage.component_digest,
            inspection.coverage.symbol_relation_index,
            inspection.coverage.historical_context,
            inspection.coverage.complete()
        ),
    ];
    for component in &inspection.key_components {
        lines.push(format!(
            "component: {} [{}] {}",
            component.path.display(),
            component.language,
            component.purpose
        ));
    }
    for relation in &inspection.key_relations {
        lines.push(format!(
            "relation: {} {} {}",
            relation.symbol, relation.relation, relation.target
        ));
    }
    for history in &inspection.key_history {
        lines.push(format!("history: {} {}", history.source, history.summary));
    }
    lines.join("\n")
}

fn format_coverage_report(report: &IndexCoverageReport) -> String {
    let mut lines = vec![format!(
        "coverage: total={} complete={} incomplete={}",
        report.total, report.complete, report.incomplete
    )];
    for item in &report.items {
        lines.push(format!(
            "entity: {} complete={} components={} relations={} history={}",
            item.name,
            item.coverage.complete(),
            item.component_count,
            item.relation_count,
            item.history_count
        ));
    }
    lines.join("\n")
}
