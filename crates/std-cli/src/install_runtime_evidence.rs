use crate::CliError;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const SAFE_ENV: [(&str, &str); 4] = [
    ("STD_TEST_MODE", "1"),
    ("STD_ALLOW_DESKTOP_AUTOMATION", "0"),
    ("STD_ALLOW_UI_PREVIEW", "0"),
    ("STD_ALLOW_BACKGROUND_UI_AUTOMATION", "0"),
];

pub(crate) fn install_runtime_evidence(prefix: &Path) -> Result<String, CliError> {
    let bin = prefix.join("bin").join("std");
    if !bin.is_file() {
        return Err(CliError::Install(format!(
            "installed std binary missing: {}",
            bin.display()
        )));
    }
    let config_path = prepare_config(prefix)?;
    let commands = runtime_commands();
    let mut outputs = Vec::new();
    for command in &commands {
        outputs.push(run_installed_std(&bin, command, &config_path)?);
    }
    let report = runtime_report(prefix, &commands, &outputs);
    let report_path = prefix.join("runtime-evidence.txt");
    fs::write(&report_path, &report)?;
    Ok(format!(
        "{report}\nruntime_evidence_artifact={}",
        report_path.display()
    ))
}

fn prepare_config(prefix: &Path) -> Result<PathBuf, CliError> {
    let data_dir = prefix.join("runtime-evidence-data");
    let plugins_dir = data_dir.join("plugins");
    fs::create_dir_all(&plugins_dir)?;
    copy_example_plugin("hello-js", &plugins_dir.join("hello-js"))?;
    copy_example_plugin("typed-ts", &plugins_dir.join("typed-ts"))?;
    let config_path = prefix.join("install-runtime-evidence.json");
    fs::write(
        &config_path,
        serde_json::json!({ "data_dir": data_dir }).to_string(),
    )?;
    Ok(config_path)
}

fn copy_example_plugin(name: &str, target: &Path) -> Result<(), CliError> {
    let source = Path::new("examples").join("plugins").join(name);
    if !source.join("plugin.json").is_file() {
        return Err(CliError::Install(format!(
            "example plugin missing: {}",
            source.display()
        )));
    }
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(&source)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::copy(&path, target.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn runtime_commands() -> Vec<Vec<&'static str>> {
    vec![
        vec!["plugin", "check", "examples/plugins/hello-js"],
        vec!["plugin", "check", "examples/plugins/typed-ts"],
        vec!["plugin", "run", "hello-js"],
        vec!["plugin", "run", "plugin-typed-ts"],
        vec!["index", "rebuild", "."],
        vec!["index", "coverage"],
    ]
}

fn run_installed_std(
    bin: &Path,
    args: &[&str],
    config_path: &Path,
) -> Result<RuntimeOutput, CliError> {
    let output = Command::new(bin)
        .args(args)
        .env("STDCLI_CONFIG", config_path)
        .env("STD_TEST_MODE", "1")
        .env("STD_ALLOW_DESKTOP_AUTOMATION", "0")
        .env("STD_ALLOW_UI_PREVIEW", "0")
        .env("STD_ALLOW_BACKGROUND_UI_AUTOMATION", "0")
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() {
        return Err(CliError::Install(format!(
            "installed runtime evidence command failed: std {}\nstdout={stdout}\nstderr={stderr}",
            args.join(" ")
        )));
    }
    Ok(RuntimeOutput { stdout, stderr })
}

struct RuntimeOutput {
    stdout: String,
    stderr: String,
}

fn runtime_report(prefix: &Path, commands: &[Vec<&str>], outputs: &[RuntimeOutput]) -> String {
    let mut lines = vec![
        "install runtime evidence PASS".to_string(),
        format!("prefix={}", prefix.display()),
    ];
    for (key, value) in SAFE_ENV {
        lines.push(format!("safe_env={key}={value}"));
    }
    lines.push(format!(
        "safe_env=STDCLI_CONFIG={}",
        prefix.join("install-runtime-evidence.json").display()
    ));
    for command in commands {
        lines.push(format!(
            "command=.std-cli/install-check/bin/std {}",
            command.join(" ")
        ));
    }
    let joined = outputs
        .iter()
        .map(|output| output.stdout.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    lines.push(format!(
        "plugin_js={}",
        status_contains(&joined, "Plugin Hello JS")
    ));
    lines.push(format!(
        "plugin_ts={}",
        status_contains(&joined, "Plugin Typed TS")
    ));
    lines.push(format!(
        "plugin_runtime={}",
        status_contains(&joined, "deno_core")
    ));
    lines.push(format!(
        "plugin_exit={}",
        status_contains(&joined, "\"exit_code\": 0")
    ));
    let coverage = coverage_summary(outputs);
    lines.push(format!("index_total={}", pass_bool(coverage.total >= 1)));
    lines.push(format!(
        "index_complete={}",
        pass_bool(coverage.complete >= 1)
    ));
    lines.push(format!(
        "index_incomplete={}",
        pass_bool(coverage.incomplete == 0)
    ));
    lines.push(format!("index_layers={}", pass_bool(coverage.layers)));
    lines.push(format!(
        "stderr_empty={}",
        outputs.iter().all(|output| output.stderr.is_empty())
    ));
    lines.join("\n")
}

fn status_contains(body: &str, needle: &str) -> &'static str {
    if body.contains(needle) {
        "PASS"
    } else {
        "MISSING"
    }
}

fn pass_bool(pass: bool) -> &'static str {
    if pass {
        "PASS"
    } else {
        "MISSING"
    }
}

#[derive(Default)]
struct CoverageSummary {
    total: u64,
    complete: u64,
    incomplete: u64,
    layers: bool,
}

fn coverage_summary(outputs: &[RuntimeOutput]) -> CoverageSummary {
    outputs
        .iter()
        .filter_map(|output| serde_json::from_str::<serde_json::Value>(&output.stdout).ok())
        .find_map(|value| {
            let total = value.get("total")?.as_u64()?;
            let complete = value.get("complete")?.as_u64()?;
            let incomplete = value.get("incomplete")?.as_u64()?;
            let layers = value
                .get("items")
                .and_then(|items| items.as_array())
                .is_some_and(|items| items.iter().all(item_has_four_layers));
            Some(CoverageSummary {
                total,
                complete,
                incomplete,
                layers,
            })
        })
        .unwrap_or_default()
}

fn item_has_four_layers(item: &serde_json::Value) -> bool {
    let Some(coverage) = item.get("coverage") else {
        return false;
    };
    [
        "entity_overview",
        "component_digest",
        "symbol_relation_index",
        "historical_context",
    ]
    .iter()
    .all(|key| coverage.get(key).and_then(|value| value.as_bool()) == Some(true))
}
