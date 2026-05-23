use std::path::Path;

use crate::ops_evidence::OpsStatus;

pub(crate) fn release_status(dist_dir: &Path) -> OpsStatus {
    if release_checks(dist_dir).iter().all(|(_, present)| *present) {
        OpsStatus::Pass
    } else {
        OpsStatus::Missing
    }
}

pub(crate) fn release_result(dist_dir: &Path) -> String {
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

pub(crate) fn release_output(dist_dir: &Path) -> String {
    release_checks(dist_dir)
        .into_iter()
        .map(|(name, present)| format!("{name}={}", status_word(present)))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn install_status(prefix: &Path) -> OpsStatus {
    if install_checks(prefix).iter().all(|(_, present)| *present) {
        OpsStatus::Pass
    } else {
        OpsStatus::Missing
    }
}

pub(crate) fn install_result(prefix: &Path) -> String {
    let checks = install_checks(prefix);
    format!(
        "install verify evidence {}/{} present in {}",
        checks.iter().filter(|(_, present)| *present).count(),
        checks.len(),
        prefix.display()
    )
}

pub(crate) fn install_output(prefix: &Path) -> String {
    install_checks(prefix)
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

fn status_word(present: bool) -> &'static str {
    if present {
        "PASS"
    } else {
        "MISSING"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_and_install_outputs_name_required_artifacts() {
        let temp = tempfile::tempdir().unwrap();
        let dist = temp.path().join("dist");
        let prefix = temp.path().join("install");

        assert_eq!(release_status(&dist), OpsStatus::Missing);
        assert_eq!(install_status(&prefix), OpsStatus::Missing);
        assert!(release_result(&dist).contains("release verify blocked"));
        assert!(release_output(&dist).contains("manifest=MISSING"));
        assert!(release_output(&dist).contains("launcher_app=MISSING"));
        assert!(install_result(&prefix).contains("install verify evidence 0/5"));
        assert!(install_output(&prefix).contains("std=MISSING"));
        assert!(install_output(&prefix).contains("studio_app=MISSING"));
    }
}
