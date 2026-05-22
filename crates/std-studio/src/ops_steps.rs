use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpsStep {
    pub name: &'static str,
    pub command: String,
    pub result: String,
}

impl OpsStep {
    pub fn summary(&self) -> String {
        format!("{}:{}:{}", self.name, self.command, self.result)
    }
}

pub(crate) fn quality_steps() -> Vec<OpsStep> {
    [
        ("fmt", "mise run fmt"),
        ("clippy", "mise run clippy"),
        ("dylint", "mise run dylint"),
        ("dylint-test", "mise run dylint-test"),
        ("file-limits", "mise run file-limits"),
        ("a11y-audit", "mise run a11y-audit"),
        ("test", "mise run test"),
        ("deny", "mise run deny"),
        ("machete", "mise run machete"),
        ("quality", "mise run quality"),
    ]
    .into_iter()
    .map(|(name, command)| OpsStep {
        name,
        command: command.to_string(),
        result: "configured".to_string(),
    })
    .collect()
}

pub(crate) fn release_steps(dist_dir: &Path) -> Vec<OpsStep> {
    vec![
        OpsStep {
            name: "release-build",
            command: "cargo build --release --workspace".to_string(),
            result: "required before package".to_string(),
        },
        OpsStep {
            name: "release-package",
            command: format!(
                "std release package --version 1.0.0 --from target/release --dist {}",
                dist_dir.display()
            ),
            result: path_result(&dist_dir.join("release-manifest.json")),
        },
        OpsStep {
            name: "release-verify",
            command: format!("std release verify --dist {}", dist_dir.display()),
            result: path_result(&dist_dir.join("quality").join("quality-report.txt")),
        },
    ]
}

pub(crate) fn install_steps(prefix: &Path, dist_dir: &Path) -> Vec<OpsStep> {
    vec![
        OpsStep {
            name: "install-run",
            command: format!(
                "std install run --prefix {} --from {}",
                prefix.display(),
                dist_dir.join("bin").display()
            ),
            result: path_result(&prefix.join("bin").join("std")),
        },
        OpsStep {
            name: "install-verify",
            command: format!("std install verify --prefix {}", prefix.display()),
            result: path_result(&prefix.join("Applications").join("std Studio.app")),
        },
    ]
}

fn path_result(path: &Path) -> String {
    if path.exists() {
        format!("present {}", path.display())
    } else {
        format!("missing {}", path.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ops_steps_expose_release_and_install_verification_commands() {
        let dist_dir = Path::new("dist/1.0.0-current");
        let prefix = Path::new(".std-cli/install-check");
        let release = release_steps(dist_dir);
        let install = install_steps(prefix, dist_dir);

        assert!(release.iter().any(|step| step.name == "release-build"
            && step.command == "cargo build --release --workspace"));
        assert!(release
            .iter()
            .any(|step| step.name == "release-package"
                && step.command.contains("std release package")));
        assert!(release.iter().any(
            |step| step.name == "release-verify" && step.command.contains("std release verify")
        ));
        assert!(install
            .iter()
            .any(|step| step.name == "install-run" && step.command.contains("std install run")));
        assert!(install.iter().any(
            |step| step.name == "install-verify" && step.command.contains("std install verify")
        ));
    }
}
