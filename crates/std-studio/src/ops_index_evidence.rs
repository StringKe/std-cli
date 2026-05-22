use std::path::Path;

pub(crate) fn index_result(root: &Path) -> String {
    if let Some(report) = runtime_report(root) {
        return if index_runtime_pass(&report) {
            "installed index four-layer coverage evidence PASS".to_string()
        } else {
            "installed index coverage evidence incomplete".to_string()
        };
    }
    let checks = index_checks(root);
    format!(
        "index coverage evidence {}/{} present; installed binary proof still manual",
        checks.iter().filter(|(_, present)| *present).count(),
        checks.len()
    )
}

pub(crate) fn index_output(root: &Path) -> String {
    if let Some(report) = runtime_report(root) {
        return [
            ("total", report.contains("index_total=PASS")),
            ("complete", report.contains("index_complete=PASS")),
            ("incomplete", report.contains("index_incomplete=PASS")),
            ("layers", report.contains("index_layers=PASS")),
            ("installed_binary", index_runtime_pass(&report)),
        ]
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ");
    }
    index_checks(root)
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn index_status(root: &Path) -> crate::OpsStatus {
    if runtime_report(root).is_some_and(|report| index_runtime_pass(&report)) {
        crate::OpsStatus::Pass
    } else {
        crate::OpsStatus::Manual
    }
}

pub(crate) fn index_runtime_pass(report: &str) -> bool {
    [
        "index_total=PASS",
        "index_complete=PASS",
        "index_incomplete=PASS",
        "index_layers=PASS",
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

fn index_checks(root: &Path) -> Vec<(&'static str, bool)> {
    [
        (
            "cli_coverage",
            source_contains(root, "crates/std-cli/src/index.rs", "Coverage"),
        ),
        (
            "overview",
            source_contains(root, "crates/std-index/src/coverage.rs", "entity_overview"),
        ),
        (
            "components",
            source_contains(root, "crates/std-index/src/coverage.rs", "component_digest"),
        ),
        (
            "relations",
            source_contains(
                root,
                "crates/std-index/src/coverage.rs",
                "symbol_relation_index",
            ),
        ),
        (
            "qa",
            source_contains(
                root,
                "crates/std-studio/src/smoke/analysis_smoke.rs",
                "qa=sources",
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
