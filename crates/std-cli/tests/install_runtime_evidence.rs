use std::{os::unix::fs::PermissionsExt, path::Path, process::Command};

#[test]
fn install_runtime_evidence_reports_installed_plugin_and_index_commands() {
    let temp = tempfile::tempdir().unwrap();
    let prefix = temp.path().join("install-check");
    let bin_dir = prefix.join("bin");
    std::fs::create_dir_all(&bin_dir).unwrap();
    let std_bin = bin_dir.join("std");
    std::fs::copy(env!("CARGO_BIN_EXE_std"), &std_bin).unwrap();
    let mut permissions = std::fs::metadata(&std_bin).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&std_bin, permissions).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_std"))
        .args([
            "install",
            "runtime-evidence",
            "--prefix",
            prefix.to_str().unwrap(),
        ])
        .env("STD_TEST_MODE", "1")
        .env("STD_ALLOW_DESKTOP_AUTOMATION", "0")
        .env("STD_ALLOW_UI_PREVIEW", "0")
        .env("STD_ALLOW_BACKGROUND_UI_AUTOMATION", "0")
        .env("STDCLI_CONFIG", temp.path().join("std-cli.json"))
        .current_dir(workspace_root())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let artifact = prefix.join("runtime-evidence.txt");

    assert!(output.status.success(), "{stdout}\n{stderr}");
    let report = std::fs::read_to_string(&artifact).unwrap();
    assert_runtime_evidence_output(&stdout);
    assert_runtime_evidence_output(&report);
    assert!(stdout.contains(&format!("runtime_evidence_artifact={}", artifact.display())));
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}

fn assert_runtime_evidence_output(output: &str) {
    for expected in [
        "install runtime evidence PASS",
        "safe_env=STD_TEST_MODE=1",
        "safe_env=STD_ALLOW_DESKTOP_AUTOMATION=0",
        "safe_env=STD_ALLOW_UI_PREVIEW=0",
        "safe_env=STD_ALLOW_BACKGROUND_UI_AUTOMATION=0",
        "safe_env=STDCLI_CONFIG=",
        "command=.std-cli/install-check/bin/std plugin check examples/plugins/hello-js",
        "command=.std-cli/install-check/bin/std plugin check examples/plugins/typed-ts",
        "command=.std-cli/install-check/bin/std plugin run hello-js",
        "command=.std-cli/install-check/bin/std plugin run plugin-typed-ts",
        "command=.std-cli/install-check/bin/std index rebuild .",
        "command=.std-cli/install-check/bin/std index coverage",
        "plugin_js=PASS",
        "plugin_ts=PASS",
        "plugin_runtime=PASS",
        "plugin_exit=PASS",
        "index_total=PASS",
        "index_complete=PASS",
        "index_incomplete=PASS",
        "index_layers=PASS",
        "stderr_empty=true",
    ] {
        assert!(output.contains(expected), "missing {expected}: {output}");
    }
}
