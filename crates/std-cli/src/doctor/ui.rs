use crate::{
    doctor::workspace::{check_text, find_workspace_root, read_required},
    CliError,
};

const UI_DOCS: [&str; 7] = [
    "docs/18_UI_Philosophy_and_Visual_Language.md",
    "docs/19_Motion_and_Interaction_Rhythm.md",
    "docs/20_Keyboard_Focus_and_Input.md",
    "docs/21_Launcher_UX_Spec.md",
    "docs/22_Studio_UX_Spec.md",
    "docs/23_Accessibility_and_Localization.md",
    "docs/24_egui_Implementation_Constraints.md",
];

const LAUNCHER_GATES: [&str; 5] = [
    "theme-smoke",
    "surface-smoke",
    "ui-semantics-smoke",
    "keyboard-smoke",
    "preview-smoke",
];

const STUDIO_GATES: [&str; 4] = [
    "smoke",
    "workspace-policy-smoke",
    "theme-smoke",
    "preview-smoke",
];

pub(crate) struct UiDoctor {
    pub(crate) docs: &'static str,
    pub(crate) docs_count: usize,
    pub(crate) launcher_gates: Vec<&'static str>,
    pub(crate) studio_gates: Vec<&'static str>,
    pub(crate) desktop_automation_default: &'static str,
    pub(crate) completion: &'static str,
}

pub(crate) fn check_ui_completion_evidence() -> Result<UiDoctor, CliError> {
    let root = find_workspace_root()?;
    check_ui_docs(&root)?;
    check_quality_report_gates(&root)?;
    check_preview_matrices(&root)?;
    check_desktop_automation_boundary(&root)?;
    Ok(UiDoctor {
        docs: "PASS",
        docs_count: UI_DOCS.len(),
        launcher_gates: LAUNCHER_GATES.to_vec(),
        studio_gates: STUDIO_GATES.to_vec(),
        desktop_automation_default: "blocked",
        completion: "INCOMPLETE_REAL_GUI_REQUIRED",
    })
}

fn check_ui_docs(root: &std::path::Path) -> Result<(), CliError> {
    for doc in UI_DOCS {
        let body = read_required(&root.join(doc))?;
        check_text(&body, "# ")?;
    }
    Ok(())
}

fn check_quality_report_gates(root: &std::path::Path) -> Result<(), CliError> {
    let body = read_required(&root.join("crates/std-cli/src/release/quality.rs"))?;
    for required in [
        "std-launcher --theme-smoke",
        "std-launcher --surface-smoke",
        "std-launcher --ui-semantics-smoke index",
        "std-launcher --keyboard-smoke index",
        "std-launcher --preview-smoke",
        "std-studio --smoke",
        "std-studio --workspace-policy-smoke",
        "std-studio --theme-smoke",
        "std-studio --preview-smoke",
        "explicit-opt-in",
    ] {
        check_text(&body, required)?;
    }
    Ok(())
}

fn check_preview_matrices(root: &std::path::Path) -> Result<(), CliError> {
    let launcher = read_required(&root.join("crates/std-launcher/src/preview.rs"))?;
    for required in [
        "dark-results",
        "light-results",
        "dark-no-results",
        "light-defer",
        "dark-error",
    ] {
        check_text(&launcher, required)?;
    }
    let studio = read_required(&root.join("crates/std-studio/src/preview.rs"))?;
    for required in [
        "dark-dashboard",
        "light-dashboard",
        "dark-workflow",
        "light-analysis",
        "dark-plugins",
        "light-operations",
        "dark-settings",
        "light-panes",
    ] {
        check_text(&studio, required)?;
    }
    Ok(())
}

fn check_desktop_automation_boundary(root: &std::path::Path) -> Result<(), CliError> {
    let core = read_required(&root.join("crates/std-core/src/lib.rs"))?;
    check_text(&core, "pub fn desktop_automation_allowed()")?;
    check_text(&core, "cfg!(test) || std_test_mode_enabled()")?;
    check_text(&core, "STD_ALLOW_DESKTOP_AUTOMATION")?;
    let guard = read_required(&root.join("crates/std-cli/tests/external_runner_guard.rs"))?;
    check_text(&guard, "binary_test_mode_blocks_dangerous_command_text")?;
    check_text(&guard, "binary_test_mode_blocks_registered_app_launch")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_completion_evidence_tracks_docs_gates_and_opt_in_boundary() {
        let report = check_ui_completion_evidence().unwrap();

        assert_eq!(report.docs, "PASS");
        assert_eq!(report.docs_count, UI_DOCS.len());
        assert_eq!(report.desktop_automation_default, "blocked");
        assert_eq!(report.completion, "INCOMPLETE_REAL_GUI_REQUIRED");
        assert!(report.launcher_gates.contains(&"preview-smoke"));
        assert!(report.studio_gates.contains(&"preview-smoke"));
    }
}
