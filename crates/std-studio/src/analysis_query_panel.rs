use crate::{analysis_state::AnalysisFocusArea, ui};
use eframe::egui;
use std_egui::i18n;

pub(crate) fn render_toolbar_query(
    ui: &mut egui::Ui,
    query: &mut String,
    focus_area: AnalysisFocusArea,
    width: f32,
) -> AnalysisQueryAction {
    let mut action = AnalysisQueryAction::None;
    let response = ui.add_sized(
        [width, 28.0],
        egui::TextEdit::singleline(query)
            .hint_text(i18n::t("studio.analysis.qa.hint"))
            .id(AnalysisFocusArea::Query.focus_id()),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            analysis_query_a11y_label(query),
        )
    });
    if focus_area == AnalysisFocusArea::Query {
        response.request_focus();
    }
    if ui::quiet_button(ui, i18n::t("studio.analysis.ask")).clicked() {
        action = AnalysisQueryAction::Ask;
    }
    if ui::quiet_button(ui, i18n::t("studio.analysis.search")).clicked() {
        action = AnalysisQueryAction::Search;
    }
    if ui::quiet_button(ui, i18n::t("studio.analysis.inspect")).clicked() {
        action = AnalysisQueryAction::Inspect;
    }
    action
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AnalysisQueryAction {
    None,
    Ask,
    Search,
    Inspect,
}

pub(crate) fn analysis_query_a11y_label(query: &str) -> String {
    let value = if query.trim().is_empty() {
        "empty"
    } else {
        query.trim()
    };
    format!("Analysis query, text box, value {value}")
}
