use crate::{analysis_rows, analysis_state::AnalysisFocusArea, ui};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_index::{IndexCoverageReport, IndexDocument};
use std_studio::{AnalysisWorkbenchTab, AnalysisWorkbenchViewModel};

pub(crate) struct AnalysisTabRenderState<'a> {
    pub(crate) active_tab: AnalysisWorkbenchTab,
    pub(crate) model: &'a AnalysisWorkbenchViewModel,
    pub(crate) document: Option<&'a IndexDocument>,
    pub(crate) coverage: Option<&'a IndexCoverageReport>,
    pub(crate) answer: &'a str,
    pub(crate) search_output: &'a str,
    pub(crate) relations_graph_mode: bool,
    pub(crate) focus_area: AnalysisFocusArea,
}

pub(crate) fn render_tab_content(ui: &mut egui::Ui, state: AnalysisTabRenderState<'_>) {
    focus_sentinel(
        ui,
        AnalysisFocusArea::Content,
        "Analysis content region",
        state.focus_area,
    );
    match state.active_tab {
        AnalysisWorkbenchTab::Overview => render_overview(ui, state.model, state.document),
        AnalysisWorkbenchTab::Components => render_components(ui, state.document),
        AnalysisWorkbenchTab::Symbols => render_symbols(ui, state.model, state.search_output),
        AnalysisWorkbenchTab::Relations => {
            render_relations(ui, state.document, state.model, state.relations_graph_mode)
        }
        AnalysisWorkbenchTab::Qa => render_qa(ui, state.model, state.answer),
    }
    ui.add_space(Space::XS as f32);
    focus_sentinel(
        ui,
        AnalysisFocusArea::Coverage,
        "Analysis coverage region",
        state.focus_area,
    );
    render_coverage_layers(ui, state.model);
    if let Some(coverage) = state.coverage {
        render_coverage_report(ui, coverage);
    }
}

fn render_overview(
    ui: &mut egui::Ui,
    model: &AnalysisWorkbenchViewModel,
    document: Option<&IndexDocument>,
) {
    render_overview_cards(ui, model);
    ui.add_space(Space::XS as f32);
    if let Some(document) = document {
        analysis_rows::document_overview_row(ui, document);
        ui.label(&document.overview.summary);
    } else {
        ui::empty_state(ui, i18n::t("studio.analysis.entity.empty"));
    }
}

fn render_components(ui: &mut egui::Ui, document: Option<&IndexDocument>) {
    let Some(document) = document else {
        ui::empty_state(ui, i18n::t("studio.analysis.entity.empty"));
        return;
    };
    egui::ScrollArea::vertical()
        .max_height(450.0)
        .show(ui, |ui| {
            for component in document.components.iter().take(12) {
                analysis_rows::component_row(ui, component);
            }
        });
}

fn render_symbols(ui: &mut egui::Ui, model: &AnalysisWorkbenchViewModel, search_output: &str) {
    render_search_hits(ui, model);
    render_output(ui, i18n::t("studio.analysis.search"), search_output, 220.0);
}

fn render_relations(
    ui: &mut egui::Ui,
    document: Option<&IndexDocument>,
    model: &AnalysisWorkbenchViewModel,
    graph_mode: bool,
) {
    ui::chip(
        ui,
        if graph_mode {
            "Graph mode"
        } else {
            "List mode"
        },
        ui::selected_bg(ui.ctx()),
    );
    ui.add_space(Space::XS as f32);
    if let Some(summary) = &model.inspection_summary {
        ui::metric(ui, &summary.entity, summary.relations, "relations indexed");
    }
    let Some(document) = document else {
        ui::empty_state(ui, i18n::t("studio.analysis.entity.empty"));
        return;
    };
    for relation in document.relations.iter().take(24) {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::metric(ui, &relation.symbol, &relation.relation, &relation.target);
        });
    }
}

fn render_qa(ui: &mut egui::Ui, model: &AnalysisWorkbenchViewModel, answer: &str) {
    render_output(ui, i18n::t("studio.analysis.answer"), answer, 220.0);
    render_answer_sources(ui, model);
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

fn focus_sentinel(
    ui: &mut egui::Ui,
    area: AnalysisFocusArea,
    label: &'static str,
    focus_area: AnalysisFocusArea,
) {
    let response = ui.interact(
        egui::Rect::from_min_size(ui.cursor().min, egui::Vec2::ZERO),
        area.focus_id(),
        egui::Sense::focusable_noninteractive(),
    );
    if focus_area == area {
        response.request_focus();
    }
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
}

fn render_coverage_layers(ui: &mut egui::Ui, model: &AnalysisWorkbenchViewModel) {
    ui.horizontal_wrapped(|ui| {
        for layer in &model.coverage_layers {
            let fill = if layer.complete {
                ui::ok_bg(ui.ctx())
            } else {
                ui::warn_bg(ui.ctx())
            };
            ui::chip(
                ui,
                &format!("{} {} {}", layer.label, layer.count, layer.status_label()),
                fill,
            );
        }
    });
}

fn render_answer_sources(ui: &mut egui::Ui, model: &AnalysisWorkbenchViewModel) {
    if model.answer_sources.is_empty() {
        return;
    }
    ui.label(egui::RichText::new(i18n::t("studio.analysis.sources")).strong());
    for source in &model.answer_sources {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::metric(
                ui,
                &source.entity,
                format!("{} evidence", source.evidence_count),
                &format!("{} {}", source.jump_target, source.detail),
            );
        });
    }
}

fn render_search_hits(ui: &mut egui::Ui, model: &AnalysisWorkbenchViewModel) {
    if model.search_hits.is_empty() {
        return;
    }
    ui.label(egui::RichText::new(i18n::t("studio.analysis.search_hits")).strong());
    for hit in &model.search_hits {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::metric(ui, &hit.title, &hit.score, &hit.detail);
        });
    }
}

fn render_coverage_report(ui: &mut egui::Ui, report: &IndexCoverageReport) {
    analysis_rows::coverage_summary(ui, report.total, report.complete, report.incomplete);
    egui::ScrollArea::vertical()
        .max_height(220.0)
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
