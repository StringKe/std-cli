use crate::{
    shell_parts::path_label,
    ui,
    workspace_panes::StudioWorkspaceSpec,
    workspace_tabs::{workspace_tab_specs, WorkspaceTabSpec},
};
use eframe::egui;
use std_egui::{i18n, tokens::Space};

const DOCS_CONTRACT: &str = "docs/22#host-chrome-current-workspace-pane-count";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkspaceContextSummary {
    pub title: String,
    pub content_key: &'static str,
    pub signal_count: usize,
    pub focused_position: usize,
    pub total_panes: usize,
    pub actions: String,
    pub docs_contract: &'static str,
    pub a11y_label: String,
}

impl WorkspaceContextSummary {
    pub(crate) fn from_spec_and_tabs(
        spec: &StudioWorkspaceSpec,
        tabs: &[WorkspaceTabSpec],
    ) -> Self {
        let focused_tab = tabs.iter().find(|tab| tab.id == spec.id);
        let focused_position = focused_tab.map(|tab| tab.position).unwrap_or(1);
        let total_panes = focused_tab
            .map(|tab| tab.total)
            .unwrap_or_else(|| tabs.len().max(1));
        Self {
            title: spec.title.clone(),
            content_key: spec.content_key,
            signal_count: spec.lines.len(),
            focused_position,
            total_panes,
            actions: workspace_context_actions(spec),
            docs_contract: DOCS_CONTRACT,
            a11y_label: workspace_context_a11y_label(spec, focused_position, total_panes),
        }
    }

    #[cfg(test)]
    pub(crate) fn contract(&self) -> String {
        format!(
            "inspector_context=pane:{};kind:{};position:{}/{};lines={};actions={};docs={};a11y={}",
            self.title,
            self.content_key,
            self.focused_position,
            self.total_panes,
            self.signal_count,
            self.actions,
            self.docs_contract,
            self.a11y_label
        )
    }
}

pub(crate) fn workspace_context_summary(
    spec: &StudioWorkspaceSpec,
    tabs: &[WorkspaceTabSpec],
) -> WorkspaceContextSummary {
    WorkspaceContextSummary::from_spec_and_tabs(spec, tabs)
}

pub(crate) fn workspace_context_summary_for_app(
    app: &std_studio::StudioApp,
    spec: &StudioWorkspaceSpec,
) -> WorkspaceContextSummary {
    let tabs = workspace_tab_specs(&app.workspace_panes, app.focused_pane);
    workspace_context_summary(spec, &tabs)
}

pub(crate) fn render_workspace_context(
    ui: &mut egui::Ui,
    spec: &StudioWorkspaceSpec,
    summary: &WorkspaceContextSummary,
) {
    ui::surface_frame(ui.ctx()).show(ui, |ui| {
        ui::section_header(
            ui,
            i18n::t("studio.shell.context.title"),
            i18n::t("studio.shell.context.detail"),
        );
        path_label(ui, "Pane", spec.title.clone());
        path_label(ui, "Kind", spec.content_key.to_string());
        path_label(
            ui,
            "Position",
            format!("{}/{}", summary.focused_position, summary.total_panes),
        );
        if let Some(path) = &spec.workflow_path {
            path_label(ui, "Workflow", path.display().to_string());
        }
        if let Some(path) = &spec.analysis_path {
            path_label(ui, "Analysis", path.display().to_string());
        }
        for line in spec.lines.iter().take(3) {
            path_label(ui, "Signal", line.clone());
        }
        ui.add_space(Space::TWO_XS as f32);
        path_label(ui, "Actions", summary.actions.clone());
        path_label(ui, "Contract", summary.docs_contract.to_string());
        ui.label(
            egui::RichText::new(&summary.a11y_label)
                .font(std_egui::tokens::Text::caption())
                .color(ui::muted_text(ui.ctx())),
        );
    });
}

fn workspace_context_actions(spec: &StudioWorkspaceSpec) -> String {
    let mut actions = vec!["show-main", "refresh", "close"];
    if spec.workflow_path.is_some() {
        actions.extend(["preview", "run"]);
    }
    if spec.analysis_path.is_some() {
        actions.push("analyze");
    }
    if spec.content_key == "plugins" {
        actions.push("reload");
    }
    actions.join(",")
}

fn workspace_context_a11y_label(
    spec: &StudioWorkspaceSpec,
    focused_position: usize,
    total_panes: usize,
) -> String {
    format!(
        "workspace context, {}, {}, pane {} of {}, {} actions",
        spec.title,
        spec.content_key,
        focused_position,
        total_panes,
        workspace_context_actions(spec)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_panes::focused_workspace_spec;

    #[test]
    fn context_summary_exposes_position_count_actions_and_docs_contract() {
        let mut app = crate::StudioEguiApp::default();
        app.app.open_plugin_manager_pane();
        let spec = focused_workspace_spec(&app.app).unwrap();
        let summary = workspace_context_summary_for_app(&app.app, &spec);

        assert_eq!(summary.title, "插件管理");
        assert_eq!(summary.content_key, "plugins");
        assert_eq!(summary.focused_position, 2);
        assert_eq!(summary.total_panes, 2);
        assert_eq!(summary.actions, "show-main,refresh,close,reload");
        assert_eq!(summary.docs_contract, DOCS_CONTRACT);
        assert!(summary.a11y_label.contains("pane 2 of 2"));
    }

    #[test]
    fn context_contract_is_stable_for_smoke_reports() {
        let mut app = crate::StudioEguiApp::default();
        app.app.open_plugin_manager_pane();
        let spec = focused_workspace_spec(&app.app).unwrap();
        let summary = workspace_context_summary_for_app(&app.app, &spec).contract();

        assert!(summary.contains("inspector_context=pane:插件管理;kind:plugins"));
        assert!(summary.contains("position:2/2"));
        assert!(summary.contains("actions=show-main,refresh,close,reload"));
        assert!(summary.contains("docs/22#host-chrome-current-workspace-pane-count"));
    }
}
