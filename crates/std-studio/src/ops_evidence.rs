use std::path::{Path, PathBuf};

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
    pub status: OpsStatus,
    pub evidence: String,
    pub detail: String,
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
        let install_prefix = install_evidence_dir();
        Self {
            qa: OpsGate {
                title: "QA",
                command: "mise run quality".to_string(),
                status: quality_status(&root),
                evidence: "mise.toml, .github/workflows/quality.yml, crates/file_too_long"
                    .to_string(),
                detail: "rustfmt, clippy, dylint, cargo-deny, cargo-machete".to_string(),
            },
            doctor: OpsGate {
                title: "Doctor",
                command: "std doctor".to_string(),
                status: doctor_status(&root),
                evidence: "StdConfig, storage, registry, workspace quality".to_string(),
                detail: "doctor reports quality, release plan, install plan".to_string(),
            },
            release: OpsGate {
                title: "Release",
                command: format!("std release verify --dist {}", release_dir.display()),
                status: release_status(&release_dir),
                evidence: release_dir.display().to_string(),
                detail: "manifest, binaries, app bundles, docs, examples, quality".to_string(),
            },
            install: OpsGate {
                title: "Install",
                command: format!("std install verify --prefix {}", install_prefix.display()),
                status: install_status(&install_prefix),
                evidence: install_prefix.display().to_string(),
                detail: "installed binaries, app bundles, storage directories".to_string(),
            },
            runtime: OpsGate {
                title: "Runtime",
                command: "std-launcher --gui-hotkey-smoke Alt+Space 5000".to_string(),
                status: OpsStatus::Manual,
                evidence: "explicit opt-in desktop smoke".to_string(),
                detail: "default tests must not open GUI or external runners".to_string(),
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
                "{}={} command={} evidence={}",
                gate.title.to_ascii_lowercase(),
                gate.status.label(),
                gate.command,
                gate.evidence
            )
        })
        .collect()
    }
}

fn quality_status(root: &Path) -> OpsStatus {
    let required = [
        root.join("mise.toml"),
        root.join("clippy.toml"),
        root.join("rustfmt.toml"),
        root.join("deny.toml"),
        root.join(".github/workflows/quality.yml"),
        root.join("crates/file_too_long"),
    ];
    if required.iter().all(|path| path.exists()) {
        OpsStatus::Pass
    } else {
        OpsStatus::Missing
    }
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

fn release_evidence_dir(root: &Path) -> PathBuf {
    let current = root.join("dist").join("1.0.0-current");
    if current.exists() {
        current
    } else {
        root.join("dist").join("1.0.0")
    }
}

fn install_evidence_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".std-cli")
        .join("install-check")
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}
