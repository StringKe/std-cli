use crate::{analysis_rows, ui, StudioEguiApp};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_index::{
    IndexAnswer, IndexCoverageReport, IndexDocument, IndexInspection, IndexSearchResult,
};
use std_studio::AnalysisWorkbenchViewModel;

const ANALYSIS_PANEL_GAP: f32 = Space::SM as f32;

impl StudioEguiApp {
    pub(crate) fn render_analysis(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            i18n::t("studio.analysis.title"),
            i18n::t("studio.analysis.detail"),
        );
        self.render_analysis_toolbar(ui);
        ui.add_space(Space::SM as f32);
        self.render_analysis_workspace(ui);
    }

    fn render_analysis_workspace(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        if available_width < 940.0 {
            self.render_active_analysis(ui);
            ui.add_space(ANALYSIS_PANEL_GAP);
            self.render_analysis_query(ui);
            ui.add_space(ANALYSIS_PANEL_GAP);
            self.render_analysis_coverage(ui);
            return;
        }
        let column_width = (available_width - ANALYSIS_PANEL_GAP * 2.0) / 3.0;
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_active_analysis(ui),
            );
            ui.add_space(ANALYSIS_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_analysis_query(ui),
            );
            ui.add_space(ANALYSIS_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_analysis_coverage(ui),
            );
        });
    }

    fn render_analysis_toolbar(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_sized(
                    [ui.available_width() - 120.0, 28.0],
                    egui::TextEdit::singleline(&mut self.analysis.path)
                        .hint_text(i18n::t("studio.analysis.path.hint")),
                );
                if ui::quiet_button(ui, i18n::t("studio.analysis.analyze")).clicked() {
                    self.analyze_current_path();
                }
            });
        });
    }

    fn render_active_analysis(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.analysis.entity.title"),
                i18n::t("studio.analysis.entity.detail"),
            );
            let model = self.analysis_workbench_model();
            render_analysis_tabs(ui, &model);
            ui.add_space(Space::XS as f32);
            let Some(document) = &self.app.active_analysis else {
                ui::empty_state(ui, i18n::t("studio.analysis.entity.empty"));
                return;
            };
            render_overview_cards(ui, &model);
            ui.add_space(Space::XS as f32);
            render_document_overview(ui, document);
            ui.add_space(Space::XS as f32);
            render_components(ui, document);
        });
    }

    fn render_analysis_query(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.analysis.query.title"),
                i18n::t("studio.analysis.query.detail"),
            );
            ui.text_edit_singleline(&mut self.analysis.query);
            ui.horizontal(|ui| {
                if ui::quiet_button(ui, i18n::t("studio.analysis.ask")).clicked() {
                    self.ask_analysis();
                }
                if ui::quiet_button(ui, i18n::t("studio.analysis.search")).clicked() {
                    self.search_analysis();
                }
                if ui::quiet_button(ui, i18n::t("studio.analysis.inspect")).clicked() {
                    self.inspect_analysis();
                }
            });
            ui.add_space(Space::XS as f32);
            render_output(
                ui,
                i18n::t("studio.analysis.answer"),
                &self.analysis.answer,
                180.0,
            );
            let model = self.analysis_workbench_model();
            render_answer_sources(ui, &model);
            render_output(
                ui,
                i18n::t("studio.analysis.search"),
                &self.analysis.search_output,
                180.0,
            );
            render_search_hits(ui, &model);
        });
    }

    fn render_analysis_coverage(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.analysis.coverage.title"),
                i18n::t("studio.analysis.coverage.detail"),
            );
            if ui::quiet_button(ui, i18n::t("studio.analysis.coverage.refresh")).clicked() {
                self.refresh_analysis_coverage();
            }
            let model = self.analysis_workbench_model();
            render_coverage_layers(ui, &model);
            ui.add_space(Space::XS as f32);
            if self.analysis.coverage_output.is_empty() {
                match self.cached_analysis_coverage_report() {
                    Ok(report) => render_coverage_report(ui, report),
                    Err(error) => {
                        ui.label(error.to_string());
                    }
                }
            } else {
                render_output(
                    ui,
                    i18n::t("studio.analysis.coverage.report"),
                    &self.analysis.coverage_output,
                    460.0,
                );
            }
        });
    }

    fn analysis_workbench_model(&self) -> AnalysisWorkbenchViewModel {
        AnalysisWorkbenchViewModel::build(
            self.app.active_analysis.as_ref(),
            self.analysis.coverage_report.as_ref(),
            self.analysis.last_answer.as_ref(),
            &self.analysis.search_results,
            self.analysis.last_inspection.as_ref(),
        )
    }

    fn cached_analysis_coverage_report(
        &self,
    ) -> Result<&IndexCoverageReport, std_index::IndexError> {
        self.analysis
            .coverage_report
            .as_ref()
            .ok_or_else(|| std_index::IndexError::Io(std::io::Error::other("coverage not loaded")))
    }

    fn analyze_current_path(&mut self) {
        let path = std::path::PathBuf::from(self.analysis.path.clone());
        match self.app.analyze_entity(&path) {
            Ok(document) => {
                self.status = format!(
                    "analyzed {} components {} relations",
                    document.components.len(),
                    document.relations.len()
                );
                self.refresh_analysis_coverage();
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn ask_analysis(&mut self) {
        match self.app.ask_analyses(&self.analysis.query, 5) {
            Ok(answer) => {
                self.analysis.answer = format_analysis_answer(&answer);
                self.analysis.last_answer = Some(answer);
            }
            Err(error) => self.analysis.answer = error.to_string(),
        }
    }

    fn search_analysis(&mut self) {
        match self.app.search_analyses(&self.analysis.query, 8) {
            Ok(results) => {
                self.analysis.search_output = format_search_results(&results);
                self.analysis.search_results = results;
            }
            Err(error) => self.analysis.search_output = error.to_string(),
        }
    }

    fn inspect_analysis(&mut self) {
        match self.app.inspect_analysis(&self.analysis.query, 8) {
            Ok(Some(inspection)) => {
                self.analysis.answer = format_inspection(&inspection);
                self.analysis.last_inspection = Some(inspection);
            }
            Ok(None) => self.analysis.answer = "analysis not found".to_string(),
            Err(error) => self.analysis.answer = error.to_string(),
        }
    }

    fn refresh_analysis_coverage(&mut self) {
        match self.app.analysis_coverage_report() {
            Ok(report) => {
                self.analysis.coverage_output = format_coverage_report(&report);
                self.analysis.coverage_report = Some(report);
            }
            Err(error) => self.analysis.coverage_output = error.to_string(),
        }
    }
}

fn render_analysis_tabs(ui: &mut egui::Ui, model: &AnalysisWorkbenchViewModel) {
    ui.horizontal_wrapped(|ui| {
        for tab in &model.tabs {
            ui::chip(
                ui,
                &format!("{} {}", tab.label, tab.count),
                ui::panel_alt(ui.ctx()),
            );
        }
    });
}

fn render_overview_cards(ui: &mut egui::Ui, model: &AnalysisWorkbenchViewModel) {
    ui.horizontal_wrapped(|ui| {
        for card in &model.overview_cards {
            ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                ui::metric(ui, card.title, &card.value, &card.detail);
            });
        }
    });
}

fn render_coverage_layers(ui: &mut egui::Ui, model: &AnalysisWorkbenchViewModel) {
    ui.horizontal_wrapped(|ui| {
        for layer in &model.coverage_layers {
            let fill = if layer.complete {
                ui::ok_bg(ui.ctx())
            } else {
                ui::warn_bg(ui.ctx())
            };
            ui::chip(ui, &format!("{} {}", layer.label, layer.count), fill);
        }
    });
}

fn render_answer_sources(ui: &mut egui::Ui, model: &AnalysisWorkbenchViewModel) {
    if model.answer_sources.is_empty() {
        return;
    }
    ui.label(egui::RichText::new("Sources").strong());
    for source in &model.answer_sources {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::metric(
                ui,
                &source.entity,
                format!("{} evidence", source.evidence_count),
                &source.detail,
            );
        });
    }
}

fn render_search_hits(ui: &mut egui::Ui, model: &AnalysisWorkbenchViewModel) {
    if model.search_hits.is_empty() {
        return;
    }
    ui.label(egui::RichText::new("Search hits").strong());
    for hit in &model.search_hits {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::metric(ui, &hit.title, &hit.score, &hit.detail);
        });
    }
}

fn render_document_overview(ui: &mut egui::Ui, document: &IndexDocument) {
    analysis_rows::document_overview_row(ui, document);
    ui.label(&document.overview.summary);
}

fn render_components(ui: &mut egui::Ui, document: &IndexDocument) {
    egui::ScrollArea::vertical()
        .max_height(450.0)
        .show(ui, |ui| {
            for component in document.components.iter().take(12) {
                analysis_rows::component_row(ui, component);
            }
        });
}

fn render_coverage_report(ui: &mut egui::Ui, report: &IndexCoverageReport) {
    analysis_rows::coverage_summary(ui, report.total, report.complete, report.incomplete);
    egui::ScrollArea::vertical()
        .max_height(460.0)
        .show(ui, |ui| {
            for item in &report.items {
                analysis_rows::coverage_item_row(ui, item);
            }
        });
}

fn render_output(ui: &mut egui::Ui, title: &str, value: &str, height: f32) {
    ui.label(egui::RichText::new(title).strong());
    if value.is_empty() {
        ui::empty_state(ui, i18n::t("studio.analysis.output.empty"));
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
