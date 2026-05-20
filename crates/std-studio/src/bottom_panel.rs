use crate::{ui, StudioEguiApp};
use eframe::egui;
use std_egui::{i18n, tokens::Space};

const BOTTOM_ROW_HEIGHT: f32 = Space::XL as f32 + Space::TWO_XS as f32;
const STATUS_CHIP_WIDTH: f32 = 120.0;
const STATUS_CHIP_HEIGHT: f32 = Space::MD as f32 + Space::TWO_XS as f32;
const STATUS_CHIP_Y_OFFSET: f32 = STATUS_CHIP_HEIGHT / 2.0;
const ROW_TITLE_Y_OFFSET: f32 = -7.0;
const ROW_DETAIL_Y_OFFSET: f32 = 9.0;

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
            ui::section_header(
                ui,
                i18n::t("studio.shell.batch_debug.title"),
                i18n::t("studio.shell.batch_debug.detail"),
            );
            let snapshot = self.bottom_panel_snapshot();
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
