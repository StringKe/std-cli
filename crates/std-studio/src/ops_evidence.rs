use std::{
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use crate::ops_runbook;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpsEvidence {
    pub qa: OpsGate,
    pub doctor: OpsGate,
    pub release: OpsGate,
    pub install: OpsGate,
    pub runtime: OpsGate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpsGate {
    pub title: &'static str,
    pub command: String,
    pub steps: Vec<crate::ops_steps::OpsStep>,
    pub runbook: String,
    pub status: OpsStatus,
    pub evidence: String,
    pub result: String,
    pub detail: String,
    pub artifact: String,
    pub output: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpsStatus {
    Pass,
    Missing,
    Manual,
}

impl OpsStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Missing => "MISSING",
            Self::Manual => "MANUAL",
        }
    }
}

impl OpsEvidence {
    pub fn load() -> Self {
        let root = workspace_root();
        let release_dir = release_evidence_dir(&root);
        let install_prefix = install_evidence_dir(&root);
        Self {
            qa: OpsGate {
                title: "QA",
                command: "mise run quality".to_string(),
                steps: crate::ops_steps::quality_steps(),
                runbook: ops_runbook::quality_runbook(),
                status: quality_status(&release_dir),
                evidence: "mise.toml, .github/workflows/quality.yml, crates/file_too_long"
                    .to_string(),
                result: quality_result(&root),
                detail: "rustfmt, clippy, dylint, cargo-deny, cargo-machete".to_string(),
                artifact: root.join("mise.toml").display().to_string(),
                output: quality_output(&root),
            },
            doctor: OpsGate {
                title: "Doctor",
                command: "std doctor".to_string(),
                steps: Vec::new(),
                runbook: ops_runbook::doctor_runbook(),
                status: doctor_status(&root),
                evidence: "StdConfig, storage, registry, workspace quality".to_string(),
                result: doctor_result(&root),
                detail: "doctor reports quality, release plan, install plan".to_string(),
                artifact: root
                    .join("crates/std-cli/src/doctor.rs")
                    .display()
                    .to_string(),
                output: doctor_output(&root),
            },
            release: OpsGate {
                title: "Release",
                command: format!("std release verify --dist {}", release_dir.display()),
                steps: crate::ops_steps::release_steps(&release_dir),
                runbook: ops_runbook::release_runbook(&release_dir),
                status: release_status(&release_dir),
                evidence: release_dir.display().to_string(),
                result: release_result(&release_dir),
                detail: "manifest, binaries, app bundles, docs, examples, quality".to_string(),
                artifact: release_dir
                    .join("release-manifest.json")
                    .display()
                    .to_string(),
                output: release_output(&release_dir),
            },
            install: OpsGate {
                title: "Install",
                command: format!("std install verify --prefix {}", install_prefix.display()),
                steps: crate::ops_steps::install_steps(&install_prefix, &release_dir),
                runbook: ops_runbook::install_runbook(&install_prefix, &release_dir),
                status: install_status(&install_prefix),
                evidence: install_prefix.display().to_string(),
                result: install_result(&install_prefix),
                detail: "installed binaries, app bundles, storage directories".to_string(),
                artifact: install_prefix.display().to_string(),
                output: install_output(&install_prefix),
            },
            runtime: OpsGate {
                title: "Runtime",
                command: "mise run ui-background-acceptance".to_string(),
                steps: Vec::new(),
                runbook: ops_runbook::runtime_runbook(),
                status: OpsStatus::Manual,
                evidence: "isolated background UI harness plus install hotkey smoke".to_string(),
                result: "manual background UI opt-in required".to_string(),
                detail: "default tests must not open GUI, apps, Terminal, or external runners"
                    .to_string(),
                artifact: "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1".to_string(),
                output: "SKIP until explicit background UI opt-in".to_string(),
            },
        }
    }

    pub fn lines(&self) -> Vec<String> {
        [
            &self.qa,
            &self.doctor,
            &self.release,
            &self.install,
            &self.runtime,
        ]
        .into_iter()
        .map(|gate| {
            format!(
                "{}={} command={} runbook={} evidence={} result={} artifact={} output={}",
                gate.title.to_ascii_lowercase(),
                gate.status.label(),
                gate.command,
                gate.runbook,
                gate.evidence,
                gate.result,
                gate.artifact,
                gate.output,
            )
        })
        .collect()
    }
}

fn quality_status(release_dir: &Path) -> OpsStatus {
    if release_dir
        .join("quality")
        .join("quality-report.txt")
        .is_file()
    {
        OpsStatus::Pass
    } else {
        OpsStatus::Missing
    }
}

fn quality_result(root: &Path) -> String {
    let report = release_evidence_dir(root)
        .join("quality")
        .join("quality-report.txt");
    if report.is_file() {
        return format!("packaged report {}", file_summary(&report));
    }
    let tools = quality_tools_present(root);
    format!(
        "workspace quality configured: {}/{} tools",
        tools.iter().filter(|(_, present)| *present).count(),
        tools.len()
    )
}

fn quality_output(root: &Path) -> String {
    quality_tools_present(root)
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn quality_tools_present(root: &Path) -> Vec<(&'static str, bool)> {
    [
        ("rustfmt", root.join("rustfmt.toml").is_file()),
        ("clippy", root.join("clippy.toml").is_file()),
        ("dylint", root.join("crates/file_too_long").is_dir()),
        ("cargo-deny", root.join("deny.toml").is_file()),
        ("cargo-machete", root.join("mise.toml").is_file()),
    ]
    .to_vec()
}

fn doctor_status(root: &Path) -> OpsStatus {
    if root.join("crates/std-cli/src/doctor.rs").is_file()
        && root
            .join("crates/std-cli/src/doctor/workspace.rs")
            .is_file()
    {
        OpsStatus::Pass
    } else {
        OpsStatus::Missing
    }
}

fn doctor_result(root: &Path) -> String {
    let checks = doctor_checks(root);
    format!(
        "doctor source gates {}/{} mapped to std doctor",
        checks.iter().filter(|(_, present)| *present).count(),
        checks.len()
    )
}

fn doctor_output(root: &Path) -> String {
    doctor_checks(root)
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn doctor_checks(root: &Path) -> Vec<(&'static str, bool)> {
    [
        (
            "storage",
            source_contains(root, "crates/std-cli/src/doctor.rs", "check_storage"),
        ),
        (
            "quality",
            source_contains(
                root,
                "crates/std-cli/src/doctor/workspace.rs",
                "check_workspace_quality",
            ),
        ),
        (
            "ui",
            source_contains(
                root,
                "crates/std-cli/src/doctor/ui.rs",
                "check_ui_completion_evidence",
            ),
        ),
        (
            "release",
            source_contains(root, "crates/std-cli/src/doctor.rs", "release_plan"),
        ),
        (
            "install",
            source_contains(root, "crates/std-cli/src/doctor.rs", "install_plan"),
        ),
    ]
    .to_vec()
}

fn release_status(dist_dir: &Path) -> OpsStatus {
    let required = [
        dist_dir.join("release-manifest.json"),
        dist_dir.join("bin").join("std"),
        dist_dir.join("bin").join("std-launcher"),
        dist_dir.join("bin").join("std-studio"),
        dist_dir.join("quality").join("quality-report.txt"),
        dist_dir.join("Applications").join("std Launcher.app"),
        dist_dir.join("Applications").join("std Studio.app"),
    ];
    if required.iter().all(|path| path.exists()) {
        OpsStatus::Pass
    } else {
        OpsStatus::Missing
    }
}

fn release_result(dist_dir: &Path) -> String {
    let manifest = dist_dir.join("release-manifest.json");
    if !manifest.is_file() {
        return format!("release verify blocked: missing {}", manifest.display());
    }
    let checks = release_checks(dist_dir);
    format!(
        "release verify evidence {}/{} present",
        checks.iter().filter(|(_, present)| *present).count(),
        checks.len()
    )
}

fn release_output(dist_dir: &Path) -> String {
    release_checks(dist_dir)
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn release_checks(dist_dir: &Path) -> Vec<(&'static str, bool)> {
    [
        ("manifest", dist_dir.join("release-manifest.json").is_file()),
        ("std", dist_dir.join("bin").join("std").is_file()),
        (
            "launcher",
            dist_dir.join("bin").join("std-launcher").is_file(),
        ),
        ("studio", dist_dir.join("bin").join("std-studio").is_file()),
        (
            "quality",
            dist_dir
                .join("quality")
                .join("quality-report.txt")
                .is_file(),
        ),
        (
            "launcher_app",
            dist_dir
                .join("Applications")
                .join("std Launcher.app")
                .is_dir(),
        ),
        (
            "studio_app",
            dist_dir
                .join("Applications")
                .join("std Studio.app")
                .is_dir(),
        ),
    ]
    .to_vec()
}

fn install_status(prefix: &Path) -> OpsStatus {
    let required = [
        prefix.join("bin").join("std"),
        prefix.join("bin").join("std-launcher"),
        prefix.join("bin").join("std-studio"),
        prefix.join("Applications").join("std Launcher.app"),
        prefix.join("Applications").join("std Studio.app"),
    ];
    if required.iter().all(|path| path.exists()) {
        OpsStatus::Pass
    } else {
        OpsStatus::Missing
    }
}

fn install_result(prefix: &Path) -> String {
    let checks = install_checks(prefix);
    format!(
        "install verify evidence {}/{} present in {}",
        checks.iter().filter(|(_, present)| *present).count(),
        checks.len(),
        prefix.display()
    )
}

fn install_output(prefix: &Path) -> String {
    install_checks(prefix)
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn install_checks(prefix: &Path) -> Vec<(&'static str, bool)> {
    [
        ("std", prefix.join("bin").join("std").is_file()),
        (
            "launcher",
            prefix.join("bin").join("std-launcher").is_file(),
        ),
        ("studio", prefix.join("bin").join("std-studio").is_file()),
        (
            "launcher_app",
            prefix
                .join("Applications")
                .join("std Launcher.app")
                .is_dir(),
        ),
        (
            "studio_app",
            prefix.join("Applications").join("std Studio.app").is_dir(),
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

fn file_summary(path: &Path) -> String {
    std::fs::metadata(path)
        .and_then(|metadata| {
            metadata
                .modified()
                .map(|modified| (metadata.len(), modified))
        })
        .map(|(len, modified)| {
            let modified_secs = modified
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_secs())
                .unwrap_or_default();
            format!("{} bytes modified_at_unix={}", len, modified_secs)
        })
        .unwrap_or_else(|_| path.display().to_string())
}

fn release_evidence_dir(root: &Path) -> PathBuf {
    let current = root.join("dist").join("1.0.0-current");
    if current.exists() {
        current
    } else {
        root.join("dist").join("1.0.0")
    }
}

fn install_evidence_dir(root: &Path) -> PathBuf {
    root.join(".std-cli").join("install-check")
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}
