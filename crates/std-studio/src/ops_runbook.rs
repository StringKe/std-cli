pub(crate) fn quality_runbook() -> String {
    [
        "mise run fmt",
        "mise run clippy",
        "mise run dylint",
        "mise run dylint-test",
        "mise run file-limits",
        "mise run a11y-audit",
        "mise run test",
        "mise run deny",
        "mise run machete",
        "mise run quality",
    ]
    .join(" && ")
}

pub(crate) fn doctor_runbook() -> String {
    [
        "std doctor",
        "std release plan --version 1.0.0",
        "std install plan",
    ]
    .join(" && ")
}

pub(crate) fn release_runbook(dist_dir: &std::path::Path) -> String {
    format!(
        "cargo build --release --workspace && std release package --version 1.0.0 --from target/release --dist {} && std release verify --dist {}",
        dist_dir.display(),
        dist_dir.display()
    )
}

pub(crate) fn install_runbook(prefix: &std::path::Path, dist_dir: &std::path::Path) -> String {
    format!(
        "std install run --prefix {} --from {} && std install verify --prefix {}",
        prefix.display(),
        dist_dir.join("bin").display(),
        prefix.display()
    )
}

pub(crate) fn runtime_runbook() -> String {
    [
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 mise run ui-background-acceptance",
        "STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space 5000",
        "STD_ALLOW_UI_PREVIEW=1 cargo run -p std-launcher -- --ui-preview light defer 8000",
        "STD_ALLOW_UI_PREVIEW=1 cargo run -p std-studio -- --ui-preview light panes 8000",
    ]
    .join(" && ")
}
