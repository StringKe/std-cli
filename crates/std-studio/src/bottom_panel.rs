use crate::{ui, StudioEguiApp};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};
use std_orchestration::ExecutionStatus;

const BOTTOM_ROW_HEIGHT: f32 = Space::XL as f32 + Space::TWO_XS as f32;
const STATUS_CHIP_WIDTH: f32 = 120.0;
const STATUS_CHIP_HEIGHT: f32 = Space::MD as f32 + Space::TWO_XS as f32;
const STATUS_CHIP_Y_OFFSET: f32 = STATUS_CHIP_HEIGHT / 2.0;
const ROW_TITLE_Y_OFFSET: f32 = -7.0;
const ROW_DETAIL_Y_OFFSET: f32 = 9.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BottomPanelTab {
    BatchDebug,
    Logs,
    Problems,
    Performance,
}

impl BottomPanelTab {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::BatchDebug => i18n::t("studio.shell.bottom.batch_debug"),
            Self::Logs => i18n::t("studio.shell.bottom.logs"),
            Self::Problems => i18n::t("studio.shell.bottom.problems"),
            Self::Performance => i18n::t("studio.shell.bottom.performance"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BottomPanelTabModel {
    pub(crate) tabs: Vec<BottomPanelTab>,
    pub(crate) selected: BottomPanelTab,
}

impl BottomPanelTabModel {
    pub(crate) fn for_selected(selected: BottomPanelTab) -> Self {
        Self {
            tabs: vec![
                BottomPanelTab::BatchDebug,
                BottomPanelTab::Logs,
                BottomPanelTab::Problems,
                BottomPanelTab::Performance,
            ],
            selected,
        }
    }

    pub(crate) fn docs22_default() -> Self {
        Self::for_selected(BottomPanelTab::BatchDebug)
    }

    pub(crate) fn labels(&self) -> Vec<&'static str> {
        self.tabs.iter().map(|tab| tab.label()).collect()
    }

    pub(crate) fn contract(&self) -> String {
        format!(
            "tabs={};selected={};role=bottom-panel-tabs",
            self.labels().join("|"),
            self.selected.label()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BottomPanelSnapshot {
    pub title: String,
    pub status: String,
    pub rows: Vec<BottomPanelRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BottomPanelRow {
    pub name: String,
    pub status: String,
    pub detail: String,
}

impl StudioEguiApp {
    pub(crate) fn render_bottom_panel(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            let tabs = BottomPanelTabModel::for_selected(self.bottom_panel_tab);
            if let Some(tab) = render_bottom_panel_tabs(ui, &tabs) {
                self.bottom_panel_tab = tab;
            }
            ui.add_space(Space::XS as f32);
            let snapshot = self.bottom_panel_snapshot();
            ui::section_header(
                ui,
                &snapshot.title,
                i18n::t("studio.shell.batch_debug.detail"),
            );
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&snapshot.title).color(ui::strong_text(ui.ctx())));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(&snapshot.status).color(ui::muted_text(ui.ctx())));
                });
            });
            ui.add_space(Space::XS as f32);
            if snapshot.rows.is_empty() {
                ui::empty_state(ui, i18n::t("studio.shell.idle"));
                return;
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                for row in &snapshot.rows {
                    render_bottom_panel_row(ui, row);
                }
            });
        });
    }

    pub(crate) fn bottom_panel_snapshot(&self) -> BottomPanelSnapshot {
        match self.bottom_panel_tab {
            BottomPanelTab::BatchDebug => self.batch_debug_snapshot(),
            BottomPanelTab::Logs => self.logs_snapshot(),
            BottomPanelTab::Problems => self.problems_snapshot(),
            BottomPanelTab::Performance => self.performance_snapshot(),
        }
    }

    fn batch_debug_snapshot(&self) -> BottomPanelSnapshot {
        if let Some(report) = self.app.last_batch_report.as_ref() {
            return BottomPanelSnapshot {
                title: "Batch Debug".to_string(),
                status: format!("{:?}", report.status),
                rows: report
                    .steps
                    .iter()
                    .map(|step| BottomPanelRow {
                        name: step.name.clone(),
                        status: format!("{:?}", step.status),
                        detail: format!("{:?} {}", step.kind, step.target),
                    })
                    .collect(),
            };
        }
        if let Some(execution) = self.app.last_workflow_execution.as_ref() {
            return BottomPanelSnapshot {
                title: execution.workflow_name.clone(),
                status: format!("{:?}", execution.status),
                rows: execution
                    .results
                    .iter()
                    .map(|step| BottomPanelRow {
                        name: step.step_name.clone(),
                        status: format!("{:?}", step.status),
                        detail: format!(
                            "started={} finished={}",
                            step.started_at, step.finished_at
                        ),
                    })
                    .collect(),
            };
        }
        if let Some(debug) = self.app.workflow_debug.as_ref() {
            return BottomPanelSnapshot {
                title: debug.workflow_name.clone(),
                status: format!("{:?}", debug.status),
                rows: debug
                    .steps
                    .iter()
                    .map(|step| BottomPanelRow {
                        name: step.step_name.clone(),
                        status: format!("{:?}", step.status),
                        detail: step.message.clone(),
                    })
                    .collect(),
            };
        }
        BottomPanelSnapshot {
            title: "Batch Debug".to_string(),
            status: if self.status.is_empty() {
                i18n::t("studio.shell.idle").to_string()
            } else {
                self.status.clone()
            },
            rows: Vec::new(),
        }
    }

    fn logs_snapshot(&self) -> BottomPanelSnapshot {
        BottomPanelSnapshot {
            title: "Logs".to_string(),
            status: if self.status.is_empty() {
                i18n::t("studio.shell.idle").to_string()
            } else {
                self.status.clone()
            },
            rows: vec![BottomPanelRow {
                name: "Latest Studio status".to_string(),
                status: "Info".to_string(),
                detail: if self.status.is_empty() {
                    i18n::t("studio.shell.idle").to_string()
                } else {
                    self.status.clone()
                },
            }],
        }
    }

    fn problems_snapshot(&self) -> BottomPanelSnapshot {
        let mut rows = Vec::new();
        if let Some(execution) = self.app.last_workflow_execution.as_ref() {
            rows.extend(
                execution
                    .results
                    .iter()
                    .filter(|step| step.status != ExecutionStatus::Completed)
                    .map(|step| BottomPanelRow {
                        name: step.step_name.clone(),
                        status: format!("{:?}", step.status),
                        detail: problem_detail(&step.output),
                    }),
            );
        }
        BottomPanelSnapshot {
            title: "Problems".to_string(),
            status: format!("{} issues", rows.len()),
            rows,
        }
    }

    fn performance_snapshot(&self) -> BottomPanelSnapshot {
        BottomPanelSnapshot {
            title: "Performance".to_string(),
            status: "interactive".to_string(),
            rows: vec![
                BottomPanelRow {
                    name: "Workspace panes".to_string(),
                    status: self.app.open_workspace_panes().count().to_string(),
                    detail: "open internal egui panes".to_string(),
                },
                BottomPanelRow {
                    name: "Bottom panel height".to_string(),
                    status: format!("{}", self.layout.bottom_panel_height() as u32),
                    detail: "docs/22 default 240".to_string(),
                },
            ],
        }
    }
}

fn problem_detail(output: &serde_json::Value) -> String {
    output
        .get("error")
        .or_else(|| output.get("message"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("No error")
        .to_string()
}

fn render_bottom_panel_tabs(
    ui: &mut egui::Ui,
    model: &BottomPanelTabModel,
) -> Option<BottomPanelTab> {
    let mut selected = None;
    ui.horizontal_wrapped(|ui| {
        for tab in &model.tabs {
            if render_bottom_panel_tab(ui, *tab, *tab == model.selected).clicked() {
                selected = Some(*tab);
            }
        }
    });
    selected
}

fn render_bottom_panel_tab(
    ui: &mut egui::Ui,
    tab: BottomPanelTab,
    selected: bool,
) -> egui::Response {
    let ctx = ui.ctx().clone();
    let fill = if selected {
        Color::accent_weak(&ctx)
    } else {
        Color::bg_surface_2(&ctx)
    };
    let stroke = if selected {
        Color::accent_base(&ctx)
    } else {
        Color::stroke_divider(&ctx)
    };
    let response = egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
        .corner_radius(egui::CornerRadius::same(Radius::SM))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(tab.label())
                    .font(Text::caption())
                    .color(ui::strong_text(&ctx)),
            );
        })
        .response;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            tab.label(),
        )
    });
    response.interact(egui::Sense::click())
}

fn render_bottom_panel_row(ui: &mut egui::Ui, row: &BottomPanelRow) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), BOTTOM_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &row.name)
    });
    if ui.is_rect_visible(rect) {
        let fill = if response.hovered() {
            std_egui::tokens::Color::bg_surface_3(ui.ctx())
        } else {
            std_egui::tokens::Color::bg_surface_2(ui.ctx())
        };
        ui.painter().rect_filled(
            rect,
            egui::CornerRadius::same(std_egui::tokens::Radius::SM),
            fill,
        );
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(
                rect.left() + Space::XS as f32,
                rect.center().y - STATUS_CHIP_Y_OFFSET,
            ),
            egui::vec2(STATUS_CHIP_WIDTH, STATUS_CHIP_HEIGHT),
        );
        paint_status_chip(ui, chip_rect, &row.status);
        let text_x = chip_rect.right() + Space::XS as f32;
        ui.painter().text(
            egui::pos2(text_x, rect.center().y + ROW_TITLE_Y_OFFSET),
            egui::Align2::LEFT_CENTER,
            &row.name,
            std_egui::tokens::Text::body(),
            ui::strong_text(ui.ctx()),
        );
        ui.painter().text(
            egui::pos2(text_x, rect.center().y + ROW_DETAIL_Y_OFFSET),
            egui::Align2::LEFT_CENTER,
            &row.detail,
            std_egui::tokens::Text::caption(),
            ui::muted_text(ui.ctx()),
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

fn paint_status_chip(ui: &mut egui::Ui, rect: egui::Rect, status: &str) {
    let fill = match status {
        "Completed" => ui::ok_bg(ui.ctx()),
        "Failed" | "NeedsExternalRunner" => ui::warn_bg(ui.ctx()),
        "Running" => ui::selected_bg(ui.ctx()),
        _ => ui::panel_alt(ui.ctx()),
    };
    ui.painter().rect_filled(
        rect,
        egui::CornerRadius::same(std_egui::tokens::Radius::SM),
        fill,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        status,
        std_egui::tokens::Text::caption(),
        ui::strong_text(ui.ctx()),
    );
}

#[cfg(test)]
pub(crate) fn completed_status() -> String {
    format!("{:?}", std_types::ActionExecutionStatus::Completed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bottom_panel_tabs_match_docs22_default_order() {
        let model = BottomPanelTabModel::docs22_default();

        assert_eq!(
            model.labels(),
            vec!["Batch Debug", "Logs", "Problems", "Performance"]
        );
        assert_eq!(model.selected, BottomPanelTab::BatchDebug);
        assert_eq!(
            model.contract(),
            "tabs=Batch Debug|Logs|Problems|Performance;selected=Batch Debug;role=bottom-panel-tabs"
        );
    }

    #[test]
    fn bottom_panel_snapshots_switch_by_selected_tab() {
        let mut app = StudioEguiApp {
            status: "Indexed 4 files".to_string(),
            bottom_panel_tab: BottomPanelTab::Logs,
            ..Default::default()
        };

        assert_eq!(app.bottom_panel_snapshot().title, "Logs");

        app.bottom_panel_tab = BottomPanelTab::Problems;
        assert_eq!(app.bottom_panel_snapshot().title, "Problems");

        app.bottom_panel_tab = BottomPanelTab::Performance;
        let performance = app.bottom_panel_snapshot();
        assert_eq!(performance.title, "Performance");
        assert!(performance
            .rows
            .iter()
            .any(|row| row.name == "Workspace panes"));
    }
}
