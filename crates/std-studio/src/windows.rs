use crate::{ui, StudioEguiApp};
use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std_egui::{i18n, tokens::Space};
use std_studio::{StudioPane, WorkspacePane, WorkspacePaneId, WorkspacePaneKind};

pub(crate) type WorkspaceCommandQueue = Arc<Mutex<Vec<StudioWorkspaceCommand>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StudioWorkspaceCommand {
    Focus(WorkspacePaneId),
    Close(WorkspacePaneId),
    ShowInMain(StudioPane),
    Refresh,
    PreviewWorkflow(PathBuf),
    RunWorkflow(PathBuf),
    Analyze(PathBuf),
    ReloadPlugins,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioWorkspaceSpec {
    pub id: WorkspacePaneId,
    pub title: String,
    pub content_key: &'static str,
    pub heading: String,
    pub lines: Vec<String>,
    pub pane: StudioPane,
    pub workflow_path: Option<PathBuf>,
    pub analysis_path: Option<PathBuf>,
}

impl StudioWorkspaceSpec {
    fn from_pane(app: &std_studio::StudioApp, pane: &WorkspacePane) -> Self {
        let content = app.workspace_pane_content(&pane.kind);
        let (workflow_path, analysis_path) = match &pane.kind {
            WorkspacePaneKind::WorkflowBuilder { workflow_path } => {
                (Some(workflow_path.clone()), None)
            }
            WorkspacePaneKind::AnalysisWorkbench { entity_path } => {
                (None, Some(entity_path.clone()))
            }
            _ => (None, None),
        };
        let main_pane = match &pane.kind {
            WorkspacePaneKind::Pane(pane) => *pane,
            WorkspacePaneKind::WorkflowBuilder { .. } => StudioPane::Workflows,
            WorkspacePaneKind::AnalysisWorkbench { .. } => StudioPane::Analysis,
            WorkspacePaneKind::AppManager => StudioPane::Apps,
            WorkspacePaneKind::MemoryBrowser => StudioPane::Memory,
            WorkspacePaneKind::ExecutionHistory => StudioPane::History,
            WorkspacePaneKind::PluginManager => StudioPane::Plugins,
        };
        Self {
            id: pane.id,
            title: pane.title.clone(),
            content_key: pane.kind.content_key(),
            heading: content.heading,
            lines: content.lines,
            pane: main_pane,
            workflow_path,
            analysis_path,
        }
    }
}

pub(crate) fn focused_workspace_spec(app: &std_studio::StudioApp) -> Option<StudioWorkspaceSpec> {
    let focused = app.focused_pane?;
    app.open_workspace_panes()
        .find(|pane| pane.id == focused)
        .map(|pane| StudioWorkspaceSpec::from_pane(app, pane))
        .or_else(|| {
            app.open_workspace_panes()
                .last()
                .map(|pane| StudioWorkspaceSpec::from_pane(app, pane))
        })
}

impl StudioEguiApp {
    pub(crate) fn render_workspace_panes(&mut self, ui: &mut egui::Ui) {
        let Some(spec) = focused_workspace_spec(&self.app) else {
            return;
        };

        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.windows.title"),
                self.app.workspace_policy.summary(),
            );
            let tabs = crate::workspace_tabs::workspace_tab_specs(
                &self.app.workspace_panes,
                self.app.focused_pane,
            );
            crate::workspace_tabs::render_workspace_tabs(ui, &tabs, &self.workspace_commands);
            ui.add_space(Space::XS as f32);
            render_spec(
                ui,
                &spec,
                i18n::t("studio.windows.active"),
                &self.workspace_commands,
            );
        });
    }

    pub(crate) fn consume_workspace_commands(&mut self) {
        let commands = self
            .workspace_commands
            .lock()
            .map(|mut queue| queue.drain(..).collect::<Vec<_>>())
            .unwrap_or_default();

        for command in commands {
            self.apply_workspace_command(command);
        }
    }

    fn apply_workspace_command(&mut self, command: StudioWorkspaceCommand) {
        match command {
            StudioWorkspaceCommand::Focus(id) => {
                if self.app.focus_workspace_pane(id) {
                    self.status = format!("focused workspace pane {}", id.value());
                }
            }
            StudioWorkspaceCommand::Close(id) => {
                if self.app.close_workspace_pane(id) {
                    self.status = format!("closed workspace pane {}", id.value());
                }
            }
            StudioWorkspaceCommand::ShowInMain(pane) => {
                self.app.switch_pane(pane);
                self.status = format!("showing {} in main workspace", pane.label());
            }
            StudioWorkspaceCommand::Refresh => {
                self.app.refresh();
                self.status = "refreshed workspace state".to_string();
            }
            StudioWorkspaceCommand::PreviewWorkflow(path) => {
                match self.app.preview_workflow_path(&path) {
                    Ok(report) => self.status = format!("workspace preview {:?}", report.status),
                    Err(error) => self.status = error.to_string(),
                }
            }
            StudioWorkspaceCommand::RunWorkflow(path) => match self.app.run_workflow_path(&path) {
                Ok(execution) => self.status = format!("workspace run {:?}", execution.status),
                Err(error) => self.status = error.to_string(),
            },
            StudioWorkspaceCommand::Analyze(path) => match self.app.analyze_entity(&path) {
                Ok(document) => {
                    self.analysis_path = path.display().to_string();
                    self.status = format!(
                        "workspace analyzed {} components",
                        document.components.len()
                    );
                }
                Err(error) => self.status = error.to_string(),
            },
            StudioWorkspaceCommand::ReloadPlugins => match self.app.reload_plugins() {
                Ok(manager) => {
                    self.status = format!(
                        "workspace reloaded {} manifests",
                        manager.manifest_paths.len()
                    )
                }
                Err(error) => self.status = error.to_string(),
            },
        }
    }
}

fn render_spec(
    ui: &mut egui::Ui,
    spec: &StudioWorkspaceSpec,
    class_label: &str,
    commands: &WorkspaceCommandQueue,
) {
    ui::surface_frame(ui.ctx()).show(ui, |ui| {
        ui::section_header(ui, &spec.heading, class_label);
        render_workspace_summary(ui, spec);
        ui.add_space(Space::XS as f32);
        for line in &spec.lines {
            render_workspace_line(ui, line);
        }
        ui.add_space(Space::XS as f32);
        render_workspace_actions(ui, spec, commands);
    });
}

fn render_workspace_summary(ui: &mut egui::Ui, spec: &StudioWorkspaceSpec) {
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new(spec.content_key).color(ui::muted_text(ui.ctx())));
        if spec.workflow_path.is_some() {
            ui.label(
                egui::RichText::new(i18n::t("studio.windows.workflow_editor"))
                    .color(ui::muted_text(ui.ctx())),
            );
        }
        if spec.analysis_path.is_some() {
            ui.label(
                egui::RichText::new(i18n::t("studio.windows.analysis_target"))
                    .color(ui::muted_text(ui.ctx())),
            );
        }
    });
}

fn render_workspace_line(ui: &mut egui::Ui, line: &str) {
    let (label, value) = line.split_once('=').unwrap_or(("detail", line));
    ui.horizontal(|ui| {
        ui.set_min_height(24.0);
        ui.label(egui::RichText::new(display_label(label)).color(ui::muted_text(ui.ctx())));
        ui.label(egui::RichText::new(display_value(value)).color(ui::strong_text(ui.ctx())));
    });
}

fn display_label(label: &str) -> &str {
    match label {
        "plugin_actions" => "Plugin actions",
        "memories" => "Memory records",
        "trace" => "Trace",
        "path" => "Path",
        "command" => "Command",
        "config_path" => "Config",
        "action" => "Actions",
        "actions" => "Actions",
        _ => label,
    }
}

fn display_value(value: &str) -> String {
    value.replace(',', ", ")
}

fn render_workspace_actions(
    ui: &mut egui::Ui,
    spec: &StudioWorkspaceSpec,
    commands: &WorkspaceCommandQueue,
) {
    ui.horizontal_wrapped(|ui| {
        if ui::quiet_button(ui, i18n::t("studio.windows.show_in_main")).clicked() {
            push_command(commands, StudioWorkspaceCommand::ShowInMain(spec.pane));
        }
        if let Some(path) = &spec.workflow_path {
            if ui::quiet_button(ui, i18n::t("studio.windows.preview_workflow")).clicked() {
                push_command(
                    commands,
                    StudioWorkspaceCommand::PreviewWorkflow(path.clone()),
                );
            }
            if ui::quiet_button(ui, i18n::t("studio.windows.run_workflow")).clicked() {
                push_command(commands, StudioWorkspaceCommand::RunWorkflow(path.clone()));
            }
        }
        if let Some(path) = &spec.analysis_path {
            if ui::quiet_button(ui, i18n::t("studio.windows.analyze")).clicked() {
                push_command(commands, StudioWorkspaceCommand::Analyze(path.clone()));
            }
        }
        if spec.content_key == "plugins"
            && ui::quiet_button(ui, i18n::t("studio.windows.reload_plugins")).clicked()
        {
            push_command(commands, StudioWorkspaceCommand::ReloadPlugins);
        }
        if ui::quiet_button(ui, i18n::t("studio.windows.refresh")).clicked() {
            push_command(commands, StudioWorkspaceCommand::Refresh);
        }
        if ui::quiet_button(ui, i18n::t("studio.windows.close")).clicked() {
            push_command(commands, StudioWorkspaceCommand::Close(spec.id));
        }
    });
}

fn push_command(commands: &WorkspaceCommandQueue, command: StudioWorkspaceCommand) {
    if let Ok(mut queue) = commands.lock() {
        queue.push(command);
    }
}
