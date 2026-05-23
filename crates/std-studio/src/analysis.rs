use crate::{
    analysis_format::{format_analysis_answer, format_coverage_report, format_inspection},
    analysis_query_panel::{self, AnalysisQueryAction},
    analysis_state::AnalysisFocusArea,
    analysis_tab_content::{self, AnalysisTabRenderState},
    studio_metrics, ui, StudioEguiApp,
};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_index::IndexSearchResult;
use std_studio::{AnalysisWorkbenchTab, AnalysisWorkbenchViewModel};

impl StudioEguiApp {
    pub(crate) fn handle_analysis_workbench_keyboard(&mut self, ctx: &egui::Context) {
        if self.app.focused_studio_pane() != Some(std_studio::StudioPane::Analysis)
            || std_egui::input::ime_action_guard(ctx).blocks_actions()
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
        self.render_analysis_tabs_bar(ui);
        ui.add_space(Space::SM as f32);
        self.render_analysis_content(ui);
    }

    fn render_analysis_toolbar(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui.set_min_height(studio_metrics::TOOLBAR_HEIGHT);
            ui.horizontal(|ui| {
                let available = ui.available_width();
                let target_width = ((available * 0.38)
                    .max(studio_metrics::ANALYSIS_FIELD_MIN_WIDTH))
                .min(studio_metrics::toolbar_field_width(
                    available,
                    studio_metrics::ANALYSIS_TOOLBAR_ACTIONS_WIDTH,
                ));
                let response = ui.add_sized(
                    [target_width, studio_metrics::INPUT_HEIGHT],
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
                if ui::quiet_button(ui, i18n::t("studio.analysis.reindex")).clicked() {
                    self.analyze_current_path();
                }
                ui.separator();
                let query_width = studio_metrics::toolbar_field_width(
                    ui.available_width(),
                    studio_metrics::ANALYSIS_QUERY_ACTIONS_WIDTH,
                );
                match analysis_query_panel::render_toolbar_query(
                    ui,
                    &mut self.analysis.query,
                    self.analysis.focus_area,
                    query_width,
                ) {
                    AnalysisQueryAction::Ask => self.ask_analysis(),
                    AnalysisQueryAction::Search => self.search_analysis(),
                    AnalysisQueryAction::Inspect => self.inspect_analysis(),
                    AnalysisQueryAction::None => {}
                }
            });
        });
    }

    fn render_analysis_tabs_bar(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui.set_min_height(studio_metrics::TAB_BAR_HEIGHT);
            let model = self.analysis_workbench_model();
            render_analysis_tabs(
                ui,
                &model,
                &mut self.analysis.active_tab,
                self.analysis.focus_area,
            );
        });
    }

    fn render_analysis_content(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.analysis.workbench.title"),
                i18n::t("studio.analysis.workbench.detail"),
            );
            ui::chip(
                ui,
                &format!("Focus {}", self.analysis.focus_area.label()),
                ui::selected_bg(ui.ctx()),
            );
            ui.add_space(Space::XS as f32);
            let model = self.analysis_workbench_model();
            analysis_tab_content::render_tab_content(ui, self.analysis_render_state(&model));
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
    fn analysis_uses_docs_22_workbench_shell_not_three_panel_layout() {
        let source = include_str!("analysis.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("studio_metrics::TOOLBAR_HEIGHT"));
        assert!(implementation.contains("studio_metrics::TAB_BAR_HEIGHT"));
        assert!(implementation.contains("fn render_analysis_toolbar"));
        assert!(implementation.contains("fn render_analysis_tabs_bar"));
        assert!(implementation.contains("fn render_analysis_content"));
        assert!(implementation.contains("analysis_query_panel::render_toolbar_query"));
        assert!(implementation.contains("studio.analysis.reindex"));
        assert!(implementation.contains("studio.analysis.workbench.title"));
        assert!(!implementation.contains("fn render_analysis_workspace"));
        assert!(!implementation.contains("fn render_analysis_query"));
        assert!(!implementation.contains("fn render_analysis_coverage"));
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
