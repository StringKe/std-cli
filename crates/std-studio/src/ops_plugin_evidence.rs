use std::path::Path;

pub(crate) fn plugin_result(root: &Path) -> String {
    if let Some(report) = runtime_report(root) {
        return if plugin_runtime_pass(&report) {
            "installed plugin JS and TS runtime evidence PASS".to_string()
        } else {
            "installed plugin runtime evidence incomplete".to_string()
        };
    }
    let checks = plugin_checks(root);
    format!(
        "plugin runtime evidence {}/{} present; installed binary proof still manual",
        checks.iter().filter(|(_, present)| *present).count(),
        checks.len()
    )
}

pub(crate) fn plugin_output(root: &Path) -> String {
    if let Some(report) = runtime_report(root) {
        return [
            ("js_runtime", report.contains("plugin_js=PASS")),
            ("ts_runtime", report.contains("plugin_ts=PASS")),
            ("deno_core", report.contains("plugin_runtime=PASS")),
            ("exit_code", report.contains("plugin_exit=PASS")),
            ("installed_binary", plugin_runtime_pass(&report)),
        ]
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ");
    }
    plugin_checks(root)
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn plugin_status(root: &Path) -> crate::OpsStatus {
    if runtime_report(root).is_some_and(|report| plugin_runtime_pass(&report)) {
        crate::OpsStatus::Pass
    } else {
        crate::OpsStatus::Manual
    }
}

pub(crate) fn plugin_runtime_pass(report: &str) -> bool {
    [
        "plugin_js=PASS",
        "plugin_ts=PASS",
        "plugin_runtime=PASS",
        "plugin_exit=PASS",
    ]
    .iter()
    .all(|needle| report.contains(needle))
}

fn runtime_report(root: &Path) -> Option<String> {
    std::fs::read_to_string(
        root.join(".std-cli")
            .join("install-check")
            .join("runtime-evidence.txt"),
    )
    .ok()
}

fn plugin_checks(root: &Path) -> Vec<(&'static str, bool)> {
    [
        (
            "js_runtime",
            source_contains(
                root,
                "crates/std-studio/src/smoke/plugin_smoke.rs",
                "main.js",
            ),
        ),
        (
            "ts_runtime",
            source_contains(
                root,
                "crates/std-studio/src/smoke/plugin_smoke.rs",
                "main.ts",
            ),
        ),
        (
            "deno_core",
            source_contains(root, "crates/std-core/src/plugins/runtime.rs", "deno_core"),
        ),
        (
            "manifest",
            source_contains(
                root,
                "crates/std-core/src/plugins/loader.rs",
                "check_plugin_manifest",
            ),
        ),
        (
            "permission_boundary",
            source_contains(
                root,
                "crates/std-studio/src/plugin_security.rs",
                "boundary_summary",
            ),
        ),
    ]
    .to_vec()
}

fn source_contains(root: &Path, relative: &str, needle: &str) -> bool {
    std::fs::read_to_string(root.join(relative))
        .map(|body| body.contains(needle))
        .unwrap_or(false)
}

fn status_word(present: bool) -> &'static str {
    if present {
        "PASS"
    } else {
        "MISSING"
    }
}
