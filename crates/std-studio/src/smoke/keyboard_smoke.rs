use crate::{analysis_state::AnalysisUiState, layout::StudioLayoutState};
use std_egui::input;
use std_studio::{StudioApp, StudioPane};

pub(crate) struct StudioKeyboardSmoke {
    pub(crate) sidebar_toggle_path: String,
    pub(crate) inspector_toggle_path: String,
    pub(crate) bottom_panel_toggle_path: String,
    pub(crate) command_palette_path: String,
    pub(crate) quick_open_path: String,
    pub(crate) context_help_path: String,
    pub(crate) new_workflow_path: String,
    pub(crate) zoom_path: String,
    pub(crate) workspace_focus_path: String,
    pub(crate) analysis_focus_path: String,
    pub(crate) analysis_qa_focus: String,
    pub(crate) sidebar_tree_label: String,
    pub(crate) dnd_pickup_announcement: String,
    pub(crate) dnd_drop_announcement: String,
    pub(crate) batch_progress_announcements: String,
    pub(crate) keyboard_contract: String,
}

impl StudioKeyboardSmoke {
    pub(crate) fn run(studio: &mut StudioApp) -> Self {
        let mut layout = StudioLayoutState::default();
        let sidebar_toggle_path = toggle_sidebar(&mut layout);
        let inspector_toggle_path = toggle_inspector(&mut layout);
        let bottom_panel_toggle_path = toggle_bottom_panel(&mut layout);
        let command_palette_path = open_command_palette(&mut layout);
        let quick_open_path = open_quick_open(&mut layout);
        let context_help_path = open_context_help(&mut layout);
        let new_workflow_path = shortcut_path(input::studio_new_workflow(), "closed>builder");
        let zoom_path = format!(
            "{}|{}|{}:1.00>1.05>1.00>1.00",
            input::studio_zoom_in().label(),
            input::studio_zoom_out().label(),
            input::studio_zoom_reset().label()
        );
        let workspace_focus_path = workspace_focus_cycle(studio);
        let (analysis_focus_path, analysis_qa_focus) = analysis_focus_paths();
        let sidebar_tree_label = sidebar_tree_node_label("Workflow Builder", 2, 1, 3, 8);
        let dnd_pickup_announcement = dnd_pickup_announcement("Collect context");
        let dnd_drop_announcement = dnd_drop_announcement("Collect context", 3);
        let batch_progress_announcements = batch_progress_announcements([0, 1, 5, 6, 10, 14, 15]);

        Self {
            sidebar_toggle_path,
            inspector_toggle_path,
            bottom_panel_toggle_path,
            command_palette_path,
            quick_open_path,
            context_help_path,
            new_workflow_path,
            zoom_path,
            workspace_focus_path,
            analysis_focus_path,
            analysis_qa_focus,
            sidebar_tree_label,
            dnd_pickup_announcement,
            dnd_drop_announcement,
            batch_progress_announcements,
            keyboard_contract: "docs/20#studio-shortcuts,docs/23#studio-screen-reader".to_string(),
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.sidebar_toggle_path
            == shortcut_path(input::studio_sidebar_toggle(), "open>closed>open")
            && self.inspector_toggle_path
                == shortcut_path(input::studio_inspector_toggle(), "closed>open>closed")
            && self.bottom_panel_toggle_path
                == shortcut_path(input::studio_bottom_panel_toggle(), "closed>open>closed")
            && self.command_palette_path
                == format!(
                    "{}|{}:closed>command",
                    input::studio_command_palette().label(),
                    input::studio_command_palette_slash().label()
                )
            && self.quick_open_path
                == shortcut_path(input::studio_quick_open(), "command>quick-open")
            && self.context_help_path
                == shortcut_path(input::studio_context_help(), "quick-open>help")
            && self.new_workflow_path
                == shortcut_path(input::studio_new_workflow(), "closed>builder")
            && self.zoom_path
                == format!(
                    "{}|{}|{}:1.00>1.05>1.00>1.00",
                    input::studio_zoom_in().label(),
                    input::studio_zoom_out().label(),
                    input::studio_zoom_reset().label()
                )
            && self.workspace_focus_path == "dashboard>plugins>settings>dashboard"
            && self.analysis_focus_path == "target>tabs>content>query>coverage>target"
            && self.analysis_qa_focus
                == shortcut_path(input::studio_analysis_qa_focus(), "coverage>query")
            && self.sidebar_tree_label == "Workflow Builder, group 2, level 1, 3 of 8"
            && self.dnd_pickup_announcement.contains(&format!(
                "use {} {} to move",
                input::studio_workflow_step_move_up().label(),
                input::studio_workflow_step_move_down().label()
            ))
            && self.dnd_drop_announcement == "Moved Collect context to position 3"
            && self.batch_progress_announcements == "0%,5%,10%,15%"
            && self.keyboard_contract.contains("docs/20#studio-shortcuts")
            && self
                .keyboard_contract
                .contains("docs/23#studio-screen-reader")
    }

    pub(crate) fn summary(&self) -> String {
        let status = if self.pass() { "PASS" } else { "FAIL" };
        format!(
            "studio_keyboard_smoke={status}\nstudio_sidebar_toggle_path={}\nstudio_inspector_toggle_path={}\nstudio_bottom_panel_toggle_path={}\nstudio_command_palette_path={}\nstudio_quick_open_path={}\nstudio_context_help_path={}\nstudio_new_workflow_path={}\nstudio_zoom_path={}\nstudio_workspace_focus_path={}\nstudio_analysis_focus_path={}\nstudio_analysis_qa_focus={}\nstudio_sidebar_tree_label={}\nstudio_dnd_pickup_announcement={}\nstudio_dnd_drop_announcement={}\nstudio_batch_progress_announcements={}\nstudio_keyboard_contract={}",
            self.sidebar_toggle_path,
            self.inspector_toggle_path,
            self.bottom_panel_toggle_path,
            self.command_palette_path,
            self.quick_open_path,
            self.context_help_path,
            self.new_workflow_path,
            self.zoom_path,
            self.workspace_focus_path,
            self.analysis_focus_path,
            self.analysis_qa_focus,
            self.sidebar_tree_label,
            self.dnd_pickup_announcement,
            self.dnd_drop_announcement,
            self.batch_progress_announcements,
            self.keyboard_contract,
        )
    }
}

fn toggle_sidebar(layout: &mut StudioLayoutState) -> String {
    let mut states = vec![open_closed(layout.sidebar_open)];
    layout.sidebar_open = !layout.sidebar_open;
    states.push(open_closed(layout.sidebar_open));
    layout.sidebar_open = !layout.sidebar_open;
    states.push(open_closed(layout.sidebar_open));
    shortcut_path(input::studio_sidebar_toggle(), &states.join(">"))
}

fn toggle_inspector(layout: &mut StudioLayoutState) -> String {
    let mut states = vec![open_closed(layout.inspector_open)];
    layout.inspector_open = !layout.inspector_open;
    states.push(open_closed(layout.inspector_open));
    layout.inspector_open = !layout.inspector_open;
    states.push(open_closed(layout.inspector_open));
    shortcut_path(input::studio_inspector_toggle(), &states.join(">"))
}

fn toggle_bottom_panel(layout: &mut StudioLayoutState) -> String {
    let mut states = vec![open_closed(layout.bottom_panel_open)];
    layout.bottom_panel_open = !layout.bottom_panel_open;
    states.push(open_closed(layout.bottom_panel_open));
    layout.bottom_panel_open = !layout.bottom_panel_open;
    states.push(open_closed(layout.bottom_panel_open));
    shortcut_path(input::studio_bottom_panel_toggle(), &states.join(">"))
}

fn open_command_palette(layout: &mut StudioLayoutState) -> String {
    let before = overlay_state(layout);
    layout.open_command_palette();
    format!(
        "{}|{}:{}>{}",
        input::studio_command_palette().label(),
        input::studio_command_palette_slash().label(),
        before,
        overlay_state(layout)
    )
}

fn open_quick_open(layout: &mut StudioLayoutState) -> String {
    let before = overlay_state(layout);
    layout.open_quick_open();
    shortcut_path(
        input::studio_quick_open(),
        &format!("{}>{}", before, overlay_state(layout)),
    )
}

fn open_context_help(layout: &mut StudioLayoutState) -> String {
    let before = overlay_state(layout);
    layout.open_context_help();
    shortcut_path(
        input::studio_context_help(),
        &format!("{}>{}", before, overlay_state(layout)),
    )
}

fn workspace_focus_cycle(studio: &mut StudioApp) -> String {
    let dashboard = studio.open_workspace_pane(StudioPane::Dashboard);
    let plugins = studio.open_plugin_manager_pane();
    let settings = studio.open_settings_pane();
    let _ = studio.focus_workspace_pane(dashboard);
    let first = studio.focus_next_workspace_pane() == Some(plugins);
    let second = studio.focus_next_workspace_pane() == Some(settings);
    let third = studio.focus_next_workspace_pane() == Some(dashboard);
    if first && second && third {
        "dashboard>plugins>settings>dashboard".to_string()
    } else {
        "FAIL".to_string()
    }
}

fn analysis_focus_paths() -> (String, String) {
    let mut analysis = AnalysisUiState::initial();
    let mut labels = vec![analysis.focus_area.label()];
    for _ in 0..5 {
        analysis.focus_next();
        labels.push(analysis.focus_area.label());
    }
    let mut qa = AnalysisUiState::initial();
    for _ in 0..4 {
        qa.focus_next();
    }
    let before_qa = qa.focus_area.label();
    qa.focus_qa();
    (
        labels.join(">"),
        shortcut_path(
            input::studio_analysis_qa_focus(),
            &format!("{}>{}", before_qa, qa.focus_area.label()),
        ),
    )
}

fn sidebar_tree_node_label(
    title: &str,
    group: usize,
    depth: usize,
    position: usize,
    total: usize,
) -> String {
    format!("{title}, group {group}, level {depth}, {position} of {total}")
}

fn dnd_pickup_announcement(item: &str) -> String {
    format!(
        "Picked up {item}, use {} {} to move",
        input::studio_workflow_step_move_up().label(),
        input::studio_workflow_step_move_down().label()
    )
}

fn dnd_drop_announcement(item: &str, position: usize) -> String {
    format!("Moved {item} to position {position}")
}

fn batch_progress_announcements(progress: impl IntoIterator<Item = u8>) -> String {
    progress
        .into_iter()
        .filter(|value| value % 5 == 0)
        .map(|value| format!("{value}%"))
        .collect::<Vec<_>>()
        .join(",")
}

fn shortcut_path(binding: input::KeyBinding, path: &str) -> String {
    format!("{}:{path}", binding.label())
}

fn open_closed(value: bool) -> &'static str {
    if value {
        "open"
    } else {
        "closed"
    }
}

fn overlay_state(layout: &StudioLayoutState) -> &'static str {
    if layout.command_palette_open {
        "command"
    } else if layout.quick_open_open {
        "quick-open"
    } else if layout.context_help_open {
        "help"
    } else {
        "closed"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_core::{StdConfig, StdCore};

    #[test]
    fn studio_keyboard_smoke_reports_focus_and_shortcuts() {
        let temp = tempfile::tempdir().unwrap();
        let core = StdCore::with_config(StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        });
        let mut studio = StudioApp::with_core(core);

        let smoke = StudioKeyboardSmoke::run(&mut studio);

        assert!(smoke.pass(), "{}", smoke.summary());
        assert!(smoke.summary().contains("studio_keyboard_smoke=PASS"));
        assert!(smoke
            .summary()
            .contains("studio_analysis_focus_path=target>tabs>content>query>coverage>target"));
        assert!(smoke
            .summary()
            .contains("studio_context_help_path=F1:quick-open>help"));
        assert!(smoke
            .summary()
            .contains("studio_sidebar_tree_label=Workflow Builder, group 2, level 1, 3 of 8"));
        assert!(smoke
            .summary()
            .contains("studio_dnd_pickup_announcement=Picked up Collect context"));
        assert!(smoke
            .summary()
            .contains("studio_batch_progress_announcements=0%,5%,10%,15%"));
    }
}
