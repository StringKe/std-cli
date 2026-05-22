use std::path::Path;

pub(crate) fn index_result(root: &Path) -> String {
    let checks = index_checks(root);
    format!(
        "index coverage evidence {}/{} present; installed binary proof still manual",
        checks.iter().filter(|(_, present)| *present).count(),
        checks.len()
    )
}

pub(crate) fn index_output(root: &Path) -> String {
    index_checks(root)
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ")
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
