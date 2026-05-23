use crate::ui;
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};
use std_studio::{StudioWorkspacePolicy, WorkspacePane, WorkspacePaneId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkspaceLifecycleSpec {
    pub open_count: usize,
    pub focused_title: String,
    pub focused_key: String,
    pub restored_count: usize,
    pub policy_summary: &'static str,
    pub native_windows_allowed: bool,
    pub detached_allowed: bool,
}

impl WorkspaceLifecycleSpec {
    pub(crate) fn from_panes(
        panes: &[WorkspacePane],
        focused: Option<WorkspacePaneId>,
        policy: StudioWorkspacePolicy,
    ) -> Self {
        let open = panes.iter().filter(|pane| pane.open).collect::<Vec<_>>();
        let focused_pane = focused.and_then(|id| open.iter().find(|pane| pane.id == id).copied());
        Self {
            open_count: open.len(),
            focused_title: focused_pane
                .map(|pane| pane.title.clone())
                .unwrap_or_else(|| i18n::t("studio.workspace_lifecycle.none").to_string()),
            focused_key: focused_pane
                .map(|pane| pane.kind.content_key().to_string())
                .unwrap_or_else(|| "none".to_string()),
            restored_count: panes.iter().filter(|pane| !pane.open).count(),
            policy_summary: policy.summary(),
            native_windows_allowed: policy.allows_native_child_windows(),
            detached_allowed: policy.allows_detached_panels(),
        }
    }
}

pub(crate) fn render_workspace_lifecycle(ui: &mut egui::Ui, spec: &WorkspaceLifecycleSpec) {
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_1(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_divider(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::SM))
        .inner_margin(egui::Margin::symmetric(Space::SM, Space::XS))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(i18n::t("studio.workspace_lifecycle.title"))
                        .font(Text::caption())
                        .strong()
                        .color(ui::strong_text(&ctx)),
                );
                lifecycle_chip(
                    ui,
                    &i18n::t("studio.workspace_lifecycle.open")
                        .replace("{count}", &spec.open_count.to_string()),
                    ui::panel_alt(&ctx),
                );
                lifecycle_chip(
                    ui,
                    &i18n::t("studio.workspace_lifecycle.focused")
                        .replace("{title}", &spec.focused_title),
                    Color::accent_weak(&ctx),
                );
                lifecycle_chip(
                    ui,
                    &i18n::t("studio.workspace_lifecycle.restore")
                        .replace("{count}", &spec.restored_count.to_string()),
                    ui::panel_alt(&ctx),
                );
                lifecycle_chip(ui, spec.policy_summary, policy_fill(&ctx, spec));
            });
        })
        .response
        .widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Label,
                ui.is_enabled(),
                workspace_lifecycle_a11y_label(spec),
            )
        });
}

pub(crate) fn workspace_lifecycle_contract(spec: &WorkspaceLifecycleSpec) -> String {
    format!(
        "workspace_lifecycle=open:{};focused:{};key:{};closed_restore:{};policy:{};native_child_windows:{};detached_panels:{}",
        spec.open_count,
        spec.focused_title,
        spec.focused_key,
        spec.restored_count,
        spec.policy_summary,
        spec.native_windows_allowed,
        spec.detached_allowed
    )
}

pub(crate) fn workspace_lifecycle_a11y_label(spec: &WorkspaceLifecycleSpec) -> String {
    i18n::t("studio.workspace_lifecycle.a11y")
        .replace("{open}", &spec.open_count.to_string())
        .replace("{focused}", &spec.focused_title)
        .replace("{key}", &spec.focused_key)
        .replace("{closed}", &spec.restored_count.to_string())
        .replace("{policy}", spec.policy_summary)
}

fn lifecycle_chip(ui: &mut egui::Ui, text: &str, fill: egui::Color32) {
    ui::chip(ui, text, fill);
}

fn policy_fill(ctx: &egui::Context, spec: &WorkspaceLifecycleSpec) -> egui::Color32 {
    if spec.native_windows_allowed || spec.detached_allowed {
        ui::danger_bg(ctx)
    } else {
        ui::ok_bg(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_studio::{StudioPane, WorkspacePaneKind};

    #[test]
    fn lifecycle_spec_exposes_open_focus_restore_and_policy() {
        let mut dashboard = WorkspacePane::new(
            WorkspacePaneId::new(1),
            WorkspacePaneKind::Pane(StudioPane::Dashboard),
            1,
        );
        let settings = WorkspacePane::new(WorkspacePaneId::new(2), WorkspacePaneKind::Settings, 2);
        dashboard.open = false;

        let spec = WorkspaceLifecycleSpec::from_panes(
            &[dashboard, settings],
            Some(WorkspacePaneId::new(2)),
            StudioWorkspacePolicy::studio_v1(),
        );

        assert_eq!(spec.open_count, 1);
        assert_eq!(spec.focused_key, "settings");
        assert_eq!(spec.restored_count, 1);
        assert!(!spec.native_windows_allowed);
        assert!(!spec.detached_allowed);
        let contract = workspace_lifecycle_contract(&spec);
        assert!(contract.contains("open:1"));
        assert!(contract.contains("closed_restore:1"));
        assert!(workspace_lifecycle_a11y_label(&spec).contains("1"));
    }

    #[test]
    fn lifecycle_renderer_uses_tokenized_surface_and_no_native_window_api() {
        let source = include_str!("workspace_lifecycle.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("Color::bg_surface_1"));
        assert!(implementation.contains("Color::stroke_divider"));
        assert!(implementation.contains("Color::accent_weak"));
        assert!(implementation.contains("WidgetType::Label"));
        assert!(!implementation.contains("egui::Window"));
        assert!(!implementation.contains("show_viewport"));
    }

    #[test]
    fn lifecycle_policy_violation_uses_danger_token() {
        let ctx = egui::Context::default();
        std_egui::tokens::apply_theme(&ctx, std_egui::tokens::ThemeMode::Light);
        let mut spec =
            WorkspaceLifecycleSpec::from_panes(&[], None, StudioWorkspacePolicy::studio_v1());

        assert_eq!(policy_fill(&ctx, &spec), ui::ok_bg(&ctx));

        spec.detached_allowed = true;
        assert_eq!(policy_fill(&ctx, &spec), ui::danger_bg(&ctx));
    }
}
