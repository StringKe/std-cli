use std::path::Path;

pub(crate) fn plugin_result(root: &Path) -> String {
    let checks = plugin_checks(root);
    format!(
        "plugin runtime evidence {}/{} present; installed binary proof still manual",
        checks.iter().filter(|(_, present)| *present).count(),
        checks.len()
    )
}

pub(crate) fn plugin_output(root: &Path) -> String {
    plugin_checks(root)
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ")
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
