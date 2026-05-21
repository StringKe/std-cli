use crate::{
    analysis_format::{format_analysis_answer, format_coverage_report, format_inspection},
    analysis_query_panel::{self, AnalysisQueryAction, AnalysisQueryPanelState},
    analysis_state::AnalysisFocusArea,
    analysis_tab_content::{self, AnalysisTabRenderState},
    ui, StudioEguiApp,
};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_index::{IndexCoverageReport, IndexSearchResult};
use std_studio::{AnalysisWorkbenchTab, AnalysisWorkbenchViewModel};

const ANALYSIS_PANEL_GAP: f32 = Space::SM as f32;

impl StudioEguiApp {
    pub(crate) fn handle_analysis_workbench_keyboard(&mut self, ctx: &egui::Context) {
        if self.app.active_pane != std_studio::StudioPane::Analysis
            || std_egui::input::ime_composing(ctx)
        {
            return;
        }
        if std_egui::input::studio_analysis_relation_toggle().pressed(ctx) {
            self.analysis.toggle_relations_view();
        }
        if std_egui::input::studio_analysis_qa_focus().pressed(ctx) {
            self.analysis.focus_qa();
        }
        if std_egui::input::tab().pressed(ctx) {
            self.analysis.focus_next();
        }
        if std_egui::input::shift_tab().pressed(ctx) {
            self.analysis.focus_previous();
        }
    }

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
                let response = ui.add_sized(
                    [ui.available_width() - 120.0, 28.0],
                    egui::TextEdit::singleline(&mut self.analysis.path)
                        .id(AnalysisFocusArea::Target.focus_id())
                        .hint_text(i18n::t("studio.analysis.path.hint")),
                );
                if self.analysis.focus_area == AnalysisFocusArea::Target {
                    response.request_focus();
                }
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::TextEdit,
                        ui.is_enabled(),
                        analysis_target_a11y_label(&self.analysis.path),
                    )
                });
                if ui::quiet_button(ui, i18n::t("studio.analysis.analyze")).clicked() {
                    self.analyze_current_path();
                }
            });
        });
    }

    fn render_active_analysis(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.analysis.entity.title"),
                i18n::t("studio.analysis.entity.detail"),
            );
            ui::chip(
                ui,
                &format!("Focus {}", self.analysis.focus_area.label()),
                ui::selected_bg(ui.ctx()),
            );
            ui.add_space(Space::XS as f32);
            let model = self.analysis_workbench_model();
            render_analysis_tabs(
                ui,
                &model,
                &mut self.analysis.active_tab,
                self.analysis.focus_area,
            );
            ui.add_space(Space::XS as f32);
            analysis_tab_content::render_tab_content(ui, self.analysis_render_state(&model));
        });
    }

    fn render_analysis_query(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            let model = self.analysis_workbench_model();
            match analysis_query_panel::render(
                ui,
                AnalysisQueryPanelState {
                    query: &mut self.analysis.query,
                    answer: &self.analysis.answer,
                    search_output: &self.analysis.search_output,
                    model: &model,
                    focus_area: self.analysis.focus_area,
                },
            ) {
                AnalysisQueryAction::Ask => self.ask_analysis(),
                AnalysisQueryAction::Search => self.search_analysis(),
                AnalysisQueryAction::Inspect => self.inspect_analysis(),
                AnalysisQueryAction::None => {}
            }
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
            if self.analysis.coverage_output.is_empty() {
                match self.cached_analysis_coverage_report() {
                    Ok(report) => {
                        let model = self.analysis_workbench_model();
                        let mut state = self.analysis_render_state(&model);
                        state.active_tab = AnalysisWorkbenchTab::Overview;
                        state.coverage = Some(report);
                        analysis_tab_content::render_tab_content(ui, state);
                    }
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

    fn analysis_render_state<'a>(
        &'a self,
        model: &'a AnalysisWorkbenchViewModel,
    ) -> AnalysisTabRenderState<'a> {
        AnalysisTabRenderState {
            active_tab: self.analysis.active_tab,
            model,
            document: self.app.active_analysis.as_ref(),
            coverage: self.analysis.coverage_report.as_ref(),
            answer: &self.analysis.answer,
            search_output: &self.analysis.search_output,
            relations_graph_mode: self.analysis.relations_graph_mode,
            focus_area: self.analysis.focus_area,
        }
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

fn render_analysis_tabs(
    ui: &mut egui::Ui,
    model: &AnalysisWorkbenchViewModel,
    active_tab: &mut AnalysisWorkbenchTab,
    focus_area: AnalysisFocusArea,
) {
    ui.horizontal_wrapped(|ui| {
        for tab in &model.tabs {
            let selected = tab.tab == *active_tab;
            let label = format!("{} {}", tab.tab.label(), tab.count);
            let fill = if selected {
                ui::selected_bg(ui.ctx())
            } else {
                ui::panel_alt(ui.ctx())
            };
            let response = ui.add(egui::Button::new(label.clone()).fill(fill));
            if selected {
                if focus_area == AnalysisFocusArea::Tabs {
                    response.request_focus();
                }
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        format!("selected {label}"),
                    )
                });
            }
            if response.clicked() {
                *active_tab = tab.tab;
            }
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

fn analysis_target_a11y_label(path: &str) -> String {
    let value = if path.trim().is_empty() {
        "empty"
    } else {
        path.trim()
    };
    format!("Analysis target path, text box, value {value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analysis_target_input_has_textbox_semantics() {
        let source = include_str!("analysis.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("AnalysisFocusArea::Target.focus_id()"));
        assert!(implementation.contains("WidgetType::TextEdit"));
        assert!(implementation.contains("analysis_target_a11y_label"));
        assert!(implementation.contains("response.request_focus()"));
    }

    #[test]
    fn analysis_target_a11y_label_exposes_value() {
        assert_eq!(
            analysis_target_a11y_label("/tmp/project"),
            "Analysis target path, text box, value /tmp/project"
        );
        assert_eq!(
            analysis_target_a11y_label(" "),
            "Analysis target path, text box, value empty"
        );
    }
}
