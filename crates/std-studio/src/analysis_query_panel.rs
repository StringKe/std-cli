use crate::{analysis_state::AnalysisFocusArea, ui};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_studio::AnalysisWorkbenchViewModel;

pub(crate) struct AnalysisQueryPanelState<'a> {
    pub(crate) query: &'a mut String,
    pub(crate) answer: &'a str,
    pub(crate) search_output: &'a str,
    pub(crate) model: &'a AnalysisWorkbenchViewModel,
    pub(crate) focus_area: AnalysisFocusArea,
}

pub(crate) fn render(ui: &mut egui::Ui, state: AnalysisQueryPanelState<'_>) -> AnalysisQueryAction {
    let mut action = AnalysisQueryAction::None;
    ui::section_header(
        ui,
        i18n::t("studio.analysis.query.title"),
        i18n::t("studio.analysis.query.detail"),
    );
    let response = ui.add(
        egui::TextEdit::singleline(state.query)
            .hint_text(i18n::t("studio.analysis.query.title"))
            .id(AnalysisFocusArea::Query.focus_id()),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            analysis_query_a11y_label(state.query),
        )
    });
    if state.focus_area == AnalysisFocusArea::Query {
        response.request_focus();
    }
    ui.horizontal(|ui| {
        if ui::quiet_button(ui, i18n::t("studio.analysis.ask")).clicked() {
            action = AnalysisQueryAction::Ask;
        }
        if ui::quiet_button(ui, i18n::t("studio.analysis.search")).clicked() {
            action = AnalysisQueryAction::Search;
        }
        if ui::quiet_button(ui, i18n::t("studio.analysis.inspect")).clicked() {
            action = AnalysisQueryAction::Inspect;
        }
    });
    ui.add_space(Space::XS as f32);
    render_answer(ui, state.answer, state.model);
    ui.add_space(Space::XS as f32);
    render_search(ui, state.search_output, state.model);
    action
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AnalysisQueryAction {
    None,
    Ask,
    Search,
    Inspect,
}

fn render_answer(ui: &mut egui::Ui, answer: &str, model: &AnalysisWorkbenchViewModel) {
    ui.label(egui::RichText::new(i18n::t("studio.analysis.answer")).strong());
    if answer.is_empty() {
        ui::empty_state(ui, i18n::t("studio.analysis.output.empty"));
    } else {
        egui::ScrollArea::vertical()
            .max_height(120.0)
            .show(ui, |ui| {
                ui.label(answer);
            });
    }
    if model.answer_sources.is_empty() {
        return;
    }
    ui.label(egui::RichText::new("Evidence Sources").strong());
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

fn render_search(ui: &mut egui::Ui, output: &str, model: &AnalysisWorkbenchViewModel) {
    ui.label(egui::RichText::new(i18n::t("studio.analysis.search")).strong());
    if model.search_hits.is_empty() {
        render_raw_output(ui, output);
        return;
    }
    for hit in &model.search_hits {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::metric(ui, &hit.title, &hit.score, &hit.detail);
        });
    }
}

fn render_raw_output(ui: &mut egui::Ui, value: &str) {
    if value.is_empty() {
        ui::empty_state(ui, i18n::t("studio.analysis.output.empty"));
        return;
    }
    egui::ScrollArea::vertical()
        .max_height(120.0)
        .show(ui, |ui| {
            ui.label(value);
        });
}

pub(crate) fn analysis_query_a11y_label(query: &str) -> String {
    let value = if query.trim().is_empty() {
        "empty"
    } else {
        query.trim()
    };
    format!("Analysis query, text box, value {value}")
}
