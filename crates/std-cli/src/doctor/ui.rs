use crate::{
    doctor::{
        ui_checks::{check_all_ui_evidence, UI_DOCS},
        workspace::find_workspace_root,
    },
    CliError,
};

const LAUNCHER_GATES: [&str; 6] = [
    "theme-smoke",
    "surface-smoke",
    "ui-semantics-smoke",
    "keyboard-smoke",
    "app-localization-smoke",
    "preview-smoke",
];

const STUDIO_GATES: [&str; 5] = [
    "smoke",
    "workspace-policy-smoke",
    "theme-smoke",
    "surface-smoke",
    "preview-smoke",
];

pub(crate) struct UiDoctor {
    pub(crate) docs: &'static str,
    pub(crate) docs_count: usize,
    pub(crate) launcher_gates: Vec<&'static str>,
    pub(crate) studio_gates: Vec<&'static str>,
    pub(crate) manual_desktop_acceptance: &'static str,
    pub(crate) background_ui_acceptance: &'static str,
    pub(crate) desktop_automation_default: &'static str,
    pub(crate) completion: &'static str,
}

pub(crate) fn check_ui_completion_evidence() -> Result<UiDoctor, CliError> {
    let root = find_workspace_root()?;
    check_all_ui_evidence(&root)?;
    Ok(UiDoctor {
        docs: "PASS",
        docs_count: UI_DOCS.len(),
        launcher_gates: LAUNCHER_GATES.to_vec(),
        studio_gates: STUDIO_GATES.to_vec(),
        manual_desktop_acceptance: "explicit_opt_in_only",
        background_ui_acceptance: "explicit_opt_in_only",
        desktop_automation_default: "blocked",
        completion: "INCOMPLETE_REAL_GUI_REQUIRED",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_completion_evidence_tracks_docs_gates_and_opt_in_boundary() {
        let report = check_ui_completion_evidence().unwrap();

        assert_eq!(report.docs, "PASS");
        assert_eq!(report.docs_count, UI_DOCS.len());
        assert_eq!(report.manual_desktop_acceptance, "explicit_opt_in_only");
        assert_eq!(report.background_ui_acceptance, "explicit_opt_in_only");
        assert_eq!(report.desktop_automation_default, "blocked");
        assert_eq!(report.completion, "INCOMPLETE_REAL_GUI_REQUIRED");
        assert!(report.launcher_gates.contains(&"preview-smoke"));
        assert!(report.studio_gates.contains(&"surface-smoke"));
        assert!(report.studio_gates.contains(&"preview-smoke"));
    }
}
