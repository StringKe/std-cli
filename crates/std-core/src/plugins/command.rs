use crate::CoreError;
use serde_json::Value;
use std::{
    io::Write,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PluginCommandOutput {
    pub(crate) runtime: String,
    pub(crate) exit_code: Option<i32>,
    pub(crate) stdout: String,
    pub(crate) stderr: String,
    pub(crate) timed_out: bool,
    pub(crate) duration_ms: u128,
}

pub(crate) fn run_shell_with_timeout(
    command: &str,
    timeout: Duration,
) -> Result<PluginCommandOutput, CoreError> {
    let mut process = Command::new("sh");
    process.arg("-c").arg(command);
    run_command_with_timeout(process, None, timeout, "shell")
}

pub(crate) fn run_command_with_timeout(
    mut command: Command,
    stdin_json: Option<&Value>,
    timeout: Duration,
    runtime: &str,
) -> Result<PluginCommandOutput, CoreError> {
    let started_at = Instant::now();
    let mut child = command
        .stdin(if stdin_json.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(args) = stdin_json {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(serde_json::to_string(args)?.as_bytes())?;
        }
    }

    loop {
        if child.try_wait()?.is_some() {
            let output = child.wait_with_output()?;
            return Ok(PluginCommandOutput {
                runtime: runtime.to_string(),
                exit_code: output.status.code(),
                stdout: String::from_utf8_lossy(&output.stdout).trim().to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
                timed_out: false,
                duration_ms: started_at.elapsed().as_millis(),
            });
        }

        if started_at.elapsed() >= timeout {
            let _ = child.kill();
            let output = child.wait_with_output()?;
            return Ok(PluginCommandOutput {
                runtime: runtime.to_string(),
                exit_code: None,
                stdout: String::from_utf8_lossy(&output.stdout).trim().to_string(),
                stderr: "plugin command timed out".to_string(),
                timed_out: true,
                duration_ms: started_at.elapsed().as_millis(),
            });
        }

        thread::sleep(Duration::from_millis(10));
    }
}
