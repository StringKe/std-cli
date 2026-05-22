use std_egui::{
    a11y::AccessibilityContext,
    i18n::{self, Locale},
};

const REQUIRED_KEYS: [&str; 14] = [
    "launcher.empty.no_matches.title",
    "launcher.empty.no_matches.detail",
    "launcher.feedback.failed",
    "launcher.feedback.deferred",
    "launcher.results.group.action_workflow",
    "studio.settings.title",
    "studio.settings.motion.reduce",
    "studio.settings.contrast.high",
    "studio.dashboard.gate.quality",
    "studio.operations.completion.note",
    "studio.workflow_builder.properties.empty",
    "studio.analysis.coverage.report",
    "studio.workspace_panes.preview_workflow",
    "studio.shell.workspace.detail",
];

fn main() {
    let report = A11yAuditReport::run();
    println!("{}", report.summary());
    if !report.pass() {
        std::process::exit(1);
    }
}

struct A11yAuditReport {
    required_keys: usize,
    missing: Vec<String>,
    search_label: String,
    result_label: String,
    action_panel_label: String,
    focus_ring_standard: f32,
    focus_ring_high_contrast: f32,
    docs23_contract: String,
}

impl A11yAuditReport {
    fn run() -> Self {
        let standard = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: false,
            high_contrast: false,
            bold_text: false,
        };
        let high_contrast = AccessibilityContext {
            high_contrast: true,
            ..standard.clone()
        };
        Self {
            required_keys: REQUIRED_KEYS.len(),
            missing: missing_i18n_keys(),
            search_label: standard.launcher_search_label("index"),
            result_label: standard.launcher_result_label("Rebuild Index", "Workflow", 1, 4),
            action_panel_label: standard.launcher_action_panel_label("Rebuild Index", 3),
            focus_ring_standard: standard.focus_ring_width(),
            focus_ring_high_contrast: high_contrast.focus_ring_width(),
            docs23_contract: "docs/23#a11y-i18n-static-audit".to_string(),
        }
    }

    fn pass(&self) -> bool {
        self.required_keys == REQUIRED_KEYS.len()
            && self.missing.is_empty()
            && self.search_label.contains("Launcher, search field")
            && self.result_label.contains("press Enter to run")
            && self.action_panel_label == "Actions for Rebuild Index, list of 3"
            && self.focus_ring_standard == 2.0
            && self.focus_ring_high_contrast == 3.0
            && self.docs23_contract == "docs/23#a11y-i18n-static-audit"
    }

    fn summary(&self) -> String {
        format!(
            "a11y_audit={}\nrequired_keys={}\nmissing_keys={}\nsearch_label={}\nresult_label={}\naction_panel_label={}\nfocus_ring_standard={}\nfocus_ring_high_contrast={}\ndocs23_contract={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.required_keys,
            if self.missing.is_empty() {
                "none".to_string()
            } else {
                self.missing.join(",")
            },
            self.search_label,
            self.result_label,
            self.action_panel_label,
            self.focus_ring_standard,
            self.focus_ring_high_contrast,
            self.docs23_contract
        )
    }
}

fn missing_i18n_keys() -> Vec<String> {
    REQUIRED_KEYS
        .iter()
        .flat_map(|key| {
            [
                missing_locale(Locale::ZhCn, "zh-CN", key),
                missing_locale(Locale::EnUs, "en-US", key),
            ]
        })
        .flatten()
        .collect()
}

fn missing_locale(locale: Locale, label: &str, key: &str) -> Option<String> {
    let value = i18n::translate(locale, key);
    (value == "UNKNOWN_I18N_KEY").then(|| format!("{label}:{key}"))
}
