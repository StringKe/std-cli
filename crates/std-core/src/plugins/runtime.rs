use crate::{
    plugins::{
        command::PluginCommandOutput, typescript::strip_typescript_source, PluginActionKind,
        PluginHostData, PluginManifest,
    },
    plugins::{
        runtime_http::{http_get, normalize_network_hosts, parse_http_url},
        runtime_paths::{
            manifest_allows_clipboard, manifest_allows_fs, manifest_allows_network,
            resolve_plugin_fs_scopes,
        },
    },
    CoreError,
};
use deno_core::{extension, op2, JsRuntime, OpState, RuntimeOptions};
use deno_error::JsErrorBox;
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};
use std_types::ClipboardRecord;

pub(crate) struct PluginScriptRun<'a> {
    pub(crate) script_path: &'a Path,
    pub(crate) args: &'a Value,
    pub(crate) manifest: &'a PluginManifest,
    pub(crate) kind: PluginActionKind,
    pub(crate) plugin_dir: &'a Path,
    pub(crate) host_data: &'a PluginHostData,
    pub(crate) timeout: Duration,
}

#[derive(Clone, Debug, Default)]
struct EmbeddedPluginState {
    args: Value,
    plugin_dir: PathBuf,
    stdout: Vec<String>,
    stderr: Vec<String>,
    fs_scopes: Vec<PathBuf>,
    fs_allowed: bool,
    network_hosts: Vec<String>,
    network_allowed: bool,
    clipboard: Vec<ClipboardRecord>,
    clipboard_allowed: bool,
}

#[derive(Clone, Debug, Default)]
struct EmbeddedPluginOutput {
    stdout: String,
    stderr: String,
    timed_out: bool,
}

struct EmbeddedPluginRun<'a> {
    script_path: &'a Path,
    source: String,
    args: &'a Value,
    manifest: &'a PluginManifest,
    plugin_dir: &'a Path,
    host_data: &'a PluginHostData,
    timeout: Duration,
}

#[op2]
#[string]
fn op_std_args_json(state: &mut OpState) -> String {
    serde_json::to_string(&state.borrow::<EmbeddedPluginState>().args).unwrap_or_default()
}

#[op2]
#[string]
fn op_std_plugin_dir(state: &mut OpState) -> String {
    state
        .borrow::<EmbeddedPluginState>()
        .plugin_dir
        .display()
        .to_string()
}

#[op2(fast)]
fn op_std_print(state: &mut OpState, #[string] value: String) {
    state.borrow_mut::<EmbeddedPluginState>().stdout.push(value);
}

#[op2(fast)]
fn op_std_error(state: &mut OpState, #[string] value: String) {
    state.borrow_mut::<EmbeddedPluginState>().stderr.push(value);
}

#[op2]
#[string]
fn op_std_read_text_file(
    state: &mut OpState,
    #[string] path: String,
) -> Result<String, JsErrorBox> {
    let state = state.borrow::<EmbeddedPluginState>();
    if !state.fs_allowed {
        return Err(JsErrorBox::generic("plugin requires fs_scoped permission"));
    }
    let requested = fs::canonicalize(PathBuf::from(path))
        .map_err(|error| JsErrorBox::generic(error.to_string()))?;
    if !state
        .fs_scopes
        .iter()
        .any(|scope| requested.starts_with(scope))
    {
        return Err(JsErrorBox::generic(format!(
            "file outside plugin fs scopes: {}",
            requested.display()
        )));
    }
    fs::read_to_string(requested).map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2(fast)]
fn op_std_write_text_file(
    state: &mut OpState,
    #[string] path: String,
    #[string] body: String,
) -> Result<(), JsErrorBox> {
    let state = state.borrow::<EmbeddedPluginState>();
    if !state.fs_allowed {
        return Err(JsErrorBox::generic("plugin requires fs_scoped permission"));
    }
    let requested = PathBuf::from(path);
    let parent = requested.parent().ok_or_else(|| {
        JsErrorBox::generic(format!("file path has no parent: {}", requested.display()))
    })?;
    let canonical_parent =
        fs::canonicalize(parent).map_err(|error| JsErrorBox::generic(error.to_string()))?;
    if !state
        .fs_scopes
        .iter()
        .any(|scope| canonical_parent.starts_with(scope))
    {
        return Err(JsErrorBox::generic(format!(
            "file outside plugin fs scopes: {}",
            requested.display()
        )));
    }
    fs::write(requested, body).map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2]
#[string]
fn op_std_http_get(state: &mut OpState, #[string] url: String) -> Result<String, JsErrorBox> {
    let state = state.borrow::<EmbeddedPluginState>();
    if !state.network_allowed {
        return Err(JsErrorBox::generic("plugin requires network permission"));
    }
    let parsed = parse_http_url(&url)?;
    let host_key = format!("{}:{}", parsed.host, parsed.port);
    if !state.network_hosts.iter().any(|host| host == &host_key) {
        return Err(JsErrorBox::generic(format!(
            "host outside plugin network scopes: {}",
            host_key
        )));
    }
    http_get(&parsed).map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2]
#[string]
fn op_std_clipboard_history(
    state: &mut OpState,
    #[string] limit: String,
) -> Result<String, JsErrorBox> {
    let state = state.borrow::<EmbeddedPluginState>();
    if !state.clipboard_allowed {
        return Err(JsErrorBox::generic("plugin requires clipboard permission"));
    }
    let limit = limit.parse::<usize>().unwrap_or(10).max(1);
    serde_json::to_string(
        &state
            .clipboard
            .iter()
            .take(limit)
            .cloned()
            .collect::<Vec<_>>(),
    )
    .map_err(|error| JsErrorBox::generic(error.to_string()))
}

extension!(
    std_plugin_runtime,
    ops = [
        op_std_args_json,
        op_std_plugin_dir,
        op_std_print,
        op_std_error,
        op_std_read_text_file,
        op_std_write_text_file,
        op_std_http_get,
        op_std_clipboard_history
    ]
);

pub(crate) fn run_script_with_timeout(
    request: PluginScriptRun<'_>,
) -> Result<PluginCommandOutput, CoreError> {
    let started_at = Instant::now();
    let source = prepare_plugin_source(request.script_path, request.kind)?;
    let output = run_embedded_deno_core(EmbeddedPluginRun {
        script_path: request.script_path,
        source,
        args: request.args,
        manifest: request.manifest,
        plugin_dir: request.plugin_dir,
        host_data: request.host_data,
        timeout: request.timeout,
    })?;
    let duration_ms = started_at.elapsed().as_millis();

    Ok(PluginCommandOutput {
        runtime: "deno_core".to_string(),
        exit_code: if output.timed_out { None } else { Some(0) },
        stdout: output.stdout.trim().to_string(),
        stderr: if output.timed_out {
            "plugin command timed out".to_string()
        } else {
            output.stderr.trim().to_string()
        },
        timed_out: output.timed_out,
        duration_ms,
    })
}

fn prepare_plugin_source(script_path: &Path, kind: PluginActionKind) -> Result<String, CoreError> {
    let source = fs::read_to_string(script_path)?;
    match kind {
        PluginActionKind::Javascript => Ok(source),
        PluginActionKind::Typescript => Ok(strip_typescript_source(&source)),
        PluginActionKind::Shell => Err(CoreError::PluginInvalid(format!(
            "shell plugin action cannot use script: {}",
            script_path.display()
        ))),
    }
}

fn run_embedded_deno_core(
    request: EmbeddedPluginRun<'_>,
) -> Result<EmbeddedPluginOutput, CoreError> {
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![std_plugin_runtime::init()],
        ..Default::default()
    });
    let finished = Arc::new(AtomicBool::new(false));
    let timed_out = Arc::new(AtomicBool::new(false));
    install_timeout(
        &mut runtime,
        request.timeout,
        finished.clone(),
        timed_out.clone(),
    );
    install_state(&mut runtime, &request)?;

    let execution_result = runtime
        .execute_script("<std-plugin-bootstrap>", bootstrap_source())
        .and_then(|_| {
            runtime.execute_script(request.script_path.display().to_string(), request.source)
        });

    finished.store(true, Ordering::SeqCst);
    if let Err(error) = execution_result {
        if !timed_out.load(Ordering::SeqCst) {
            return Err(CoreError::PluginInvalid(error.to_string()));
        }
    }

    let state = runtime.op_state();
    let state = state.borrow();
    let output = state.borrow::<EmbeddedPluginState>();
    Ok(EmbeddedPluginOutput {
        stdout: output.stdout.join("\n"),
        stderr: output.stderr.join("\n"),
        timed_out: timed_out.load(Ordering::SeqCst),
    })
}

fn install_timeout(
    runtime: &mut JsRuntime,
    timeout: Duration,
    finished: Arc<AtomicBool>,
    timed_out: Arc<AtomicBool>,
) {
    let isolate_handle = runtime.v8_isolate().thread_safe_handle();
    thread::spawn(move || {
        thread::sleep(timeout);
        if !finished.load(Ordering::SeqCst) {
            timed_out.store(true, Ordering::SeqCst);
            let _ = isolate_handle.terminate_execution();
        }
    });
}

fn install_state(
    runtime: &mut JsRuntime,
    request: &EmbeddedPluginRun<'_>,
) -> Result<(), CoreError> {
    let fs_scopes = resolve_plugin_fs_scopes(request.manifest, request.plugin_dir)?;
    let plugin_dir = fs::canonicalize(request.plugin_dir)?;
    runtime.op_state().borrow_mut().put(EmbeddedPluginState {
        args: request.args.clone(),
        plugin_dir,
        stdout: Vec::new(),
        stderr: Vec::new(),
        fs_scopes,
        fs_allowed: manifest_allows_fs(request.manifest),
        network_hosts: normalize_network_hosts(&request.manifest.network_hosts),
        network_allowed: manifest_allows_network(request.manifest),
        clipboard: request.host_data.clipboard.clone(),
        clipboard_allowed: manifest_allows_clipboard(request.manifest),
    });
    Ok(())
}

fn bootstrap_source() -> &'static str {
    r#"
globalThis.std = {
  args: () => JSON.parse(Deno.core.ops.op_std_args_json() || "{}"),
  pluginDir: () => Deno.core.ops.op_std_plugin_dir(),
  print: (value) => Deno.core.ops.op_std_print(String(value)),
  error: (value) => Deno.core.ops.op_std_error(String(value)),
  emit: (value) => Deno.core.ops.op_std_print(JSON.stringify(value)),
  readTextFile: (path) => Deno.core.ops.op_std_read_text_file(String(path)),
  writeTextFile: (path, body) => Deno.core.ops.op_std_write_text_file(String(path), String(body)),
  httpGet: (url) => Deno.core.ops.op_std_http_get(String(url)),
  clipboardHistory: (limit) => JSON.parse(Deno.core.ops.op_std_clipboard_history(String(Number(limit) || 10)))
};
globalThis.console = {
  log: (...values) => Deno.core.ops.op_std_print(values.map((value) => String(value)).join(" ")),
  error: (...values) => Deno.core.ops.op_std_error(values.map((value) => String(value)).join(" "))
};
"#
}
