use crate::ui;
use crate::window::apply_window_commands;
use eframe::egui;
use std::{
    process::Command,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use std_launcher::{
    GlobalHotkeyRuntime, HotkeyRegistrationPlan, LauncherHotkey, LauncherState,
    LauncherWindowCommand,
};

#[derive(Debug, Clone)]
pub(crate) struct GuiHotkeySmokeConfig {
    pub accelerator: String,
    pub timeout_ms: u64,
    pub trigger_delay_ms: u64,
    pub allow_system_events: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct GuiHotkeySmokeReport {
    pub status: &'static str,
    pub accelerator: String,
    pub registered: bool,
    pub input_sent: bool,
    pub event_received: bool,
    pub commands: Vec<LauncherWindowCommand>,
    pub close_commands: Vec<LauncherWindowCommand>,
    pub visible_after_close: bool,
    pub resident_after_close: bool,
    pub second_input_sent: bool,
    pub second_event_received: bool,
    pub second_commands: Vec<LauncherWindowCommand>,
    pub elapsed_ms: u128,
    pub timeout_ms: u64,
    pub error: Option<String>,
}

impl GuiHotkeySmokeReport {
    pub fn summary(&self) -> String {
        format!(
            "launcher_gui_hotkey_smoke {}\naccelerator={}\nregistered={}\ninput_sent={}\nevent_received={}\ncommands={}\nvisible_after_close={}\nresident_after_close={}\nsecond_input_sent={}\nsecond_event_received={}\nclose_commands={}\nsecond_commands={}\nelapsed_ms={}\ntimeout_ms={}\nerror={}",
            self.status,
            self.accelerator,
            self.registered,
            self.input_sent,
            self.event_received,
            format_window_commands(&self.commands),
            self.visible_after_close,
            self.resident_after_close,
            self.second_input_sent,
            self.second_event_received,
            format_window_commands(&self.close_commands),
            format_window_commands(&self.second_commands),
            self.elapsed_ms,
            self.timeout_ms,
            self.error.as_deref().unwrap_or("none")
        )
    }
}

pub(crate) fn run_gui_hotkey_smoke(
    config: GuiHotkeySmokeConfig,
) -> eframe::Result<GuiHotkeySmokeReport> {
    if !config.allow_system_events || gui_hotkey_smoke_blocked_by_test_mode() {
        return Ok(gui_hotkey_smoke_blocked_report(config));
    }
    let report = Arc::new(Mutex::new(None));
    let input_result = Arc::new(Mutex::new(Vec::new()));
    let app_report = Arc::clone(&report);
    let app_input_result = Arc::clone(&input_result);
    let accelerator = config.accelerator.clone();
    let trigger_delay_ms = config.trigger_delay_ms;
    let input_accelerator = config.accelerator.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(trigger_delay_ms));
        let result = send_macos_hotkey(&input_accelerator);
        if let Ok(mut slot) = input_result.lock() {
            slot.push(result);
        }
        thread::sleep(Duration::from_millis(850));
        let result = send_macos_hotkey(&input_accelerator);
        if let Ok(mut slot) = input_result.lock() {
            slot.push(result);
        }
    });

    let options = std_launcher::launcher_panel_native_options(
        ui::launcher_initial_window_inner_size(),
        false,
    );
    eframe::run_native(
        "std-cli Launcher GUI Hotkey Smoke",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(GuiHotkeySmokeApp::new(
                config,
                accelerator,
                app_report,
                app_input_result,
            )))
        }),
    )?;

    Ok(report
        .lock()
        .ok()
        .and_then(|mut slot| slot.take())
        .unwrap_or_else(|| GuiHotkeySmokeReport {
            status: "FAIL",
            accelerator: "UNKNOWN".to_string(),
            registered: false,
            input_sent: false,
            event_received: false,
            commands: Vec::new(),
            close_commands: Vec::new(),
            visible_after_close: false,
            resident_after_close: false,
            second_input_sent: false,
            second_event_received: false,
            second_commands: Vec::new(),
            elapsed_ms: 0,
            timeout_ms: 0,
            error: Some("GUI smoke exited without report".to_string()),
        }))
}

fn gui_hotkey_smoke_blocked_report(config: GuiHotkeySmokeConfig) -> GuiHotkeySmokeReport {
    GuiHotkeySmokeReport {
        status: "SKIP",
        accelerator: config.accelerator,
        registered: false,
        input_sent: false,
        event_received: false,
        commands: Vec::new(),
        close_commands: Vec::new(),
        visible_after_close: false,
        resident_after_close: false,
        second_input_sent: false,
        second_event_received: false,
        second_commands: Vec::new(),
        elapsed_ms: 0,
        timeout_ms: config.timeout_ms,
        error: Some(gui_hotkey_smoke_blocked_reason()),
    }
}

fn gui_hotkey_smoke_blocked_reason() -> String {
    if gui_hotkey_smoke_blocked_by_test_mode() {
        "STD_TEST_MODE blocked GUI hotkey smoke; use explicit desktop opt-in outside tests"
            .to_string()
    } else {
        "desktop automation requires STD_ALLOW_DESKTOP_AUTOMATION=1 explicit opt-in".to_string()
    }
}

fn gui_hotkey_smoke_blocked_by_test_mode() -> bool {
    cfg!(test) || std_core::std_test_mode_enabled()
}

struct GuiHotkeySmokeApp {
    state: LauncherState,
    hotkey_runtime: GlobalHotkeyRuntime,
    registered: bool,
    accelerator: String,
    started_at: Instant,
    timeout: Duration,
    timeout_ms: u64,
    input_result: Arc<Mutex<Vec<Result<(), String>>>>,
    report: Arc<Mutex<Option<GuiHotkeySmokeReport>>>,
    input_error: Option<String>,
    close_after: Option<Instant>,
    input_count: usize,
    first_commands: Vec<LauncherWindowCommand>,
    close_commands: Vec<LauncherWindowCommand>,
    second_commands: Vec<LauncherWindowCommand>,
    first_event_received: bool,
    second_event_received: bool,
    visible_after_close: bool,
    resident_after_close: bool,
}

impl GuiHotkeySmokeApp {
    fn new(
        config: GuiHotkeySmokeConfig,
        accelerator: String,
        report: Arc<Mutex<Option<GuiHotkeySmokeReport>>>,
        input_result: Arc<Mutex<Vec<Result<(), String>>>>,
    ) -> Self {
        let mut state = LauncherState::new();
        state.hide();
        let plan = HotkeyRegistrationPlan {
            accelerator: accelerator.clone(),
            enabled: true,
        };
        let (hotkey_runtime, registered, input_error) = match GlobalHotkeyRuntime::register(plan) {
            Ok(runtime) => (runtime, true, None),
            Err(error) => (
                GlobalHotkeyRuntime::disabled(HotkeyRegistrationPlan {
                    accelerator: accelerator.clone(),
                    enabled: false,
                }),
                false,
                Some(error),
            ),
        };
        Self {
            state,
            hotkey_runtime,
            registered,
            accelerator,
            started_at: Instant::now(),
            timeout: Duration::from_millis(config.timeout_ms),
            timeout_ms: config.timeout_ms,
            input_result,
            report,
            input_error,
            close_after: None,
            input_count: 0,
            first_commands: Vec::new(),
            close_commands: Vec::new(),
            second_commands: Vec::new(),
            first_event_received: false,
            second_event_received: false,
            visible_after_close: false,
            resident_after_close: false,
        }
    }

    fn finish(&self, report: GuiHotkeySmokeReport, ctx: &egui::Context) {
        if let Ok(mut slot) = self.report.lock() {
            if slot.is_none() {
                *slot = Some(report);
            }
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn collect_input(&mut self) {
        if let Ok(mut slot) = self.input_result.lock() {
            for result in slot.drain(..) {
                self.input_count += 1;
                if let Err(error) = result {
                    self.input_error = Some(error);
                }
            }
        }
    }

    fn passing_report(&self) -> GuiHotkeySmokeReport {
        GuiHotkeySmokeReport {
            status: "PASS",
            accelerator: self.accelerator.clone(),
            registered: self.registered,
            input_sent: self.input_count >= 1,
            event_received: self.first_event_received,
            commands: self.first_commands.clone(),
            close_commands: self.close_commands.clone(),
            visible_after_close: self.visible_after_close,
            resident_after_close: self.resident_after_close,
            second_input_sent: self.input_count >= 2,
            second_event_received: self.second_event_received,
            second_commands: self.second_commands.clone(),
            elapsed_ms: self.started_at.elapsed().as_millis(),
            timeout_ms: self.timeout_ms,
            error: self.input_error.clone(),
        }
    }
}

impl eframe::App for GuiHotkeySmokeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.collect_input();
        if self.hotkey_runtime.poll_toggle_event() {
            let commands = self.state.handle_hotkey_toggle();
            apply_window_commands(ctx, &commands, ui::launcher_window_inner_size(&self.state));
            if !self.first_event_received {
                self.first_event_received = true;
                self.first_commands = commands;
                self.close_commands = self.state.handle_escape_hide();
                apply_window_commands(
                    ctx,
                    &self.close_commands,
                    ui::launcher_window_inner_size(&self.state),
                );
                self.visible_after_close = self.state.controller.visible;
                self.resident_after_close = self.hotkey_runtime.is_registered();
            } else {
                self.second_event_received = true;
                self.second_commands = commands;
                if let Ok(mut slot) = self.report.lock() {
                    if slot.is_none() {
                        *slot = Some(self.passing_report());
                    }
                }
                self.close_after = Some(Instant::now() + Duration::from_millis(250));
            }
        }

        if self
            .close_after
            .map(|deadline| Instant::now() >= deadline)
            .unwrap_or(false)
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        if self.started_at.elapsed() >= self.timeout {
            self.finish(
                GuiHotkeySmokeReport {
                    status: "FAIL",
                    accelerator: self.accelerator.clone(),
                    registered: self.registered,
                    input_sent: self.input_count >= 1,
                    event_received: self.first_event_received,
                    commands: self.first_commands.clone(),
                    close_commands: self.close_commands.clone(),
                    visible_after_close: self.visible_after_close,
                    resident_after_close: self.resident_after_close,
                    second_input_sent: self.input_count >= 2,
                    second_event_received: self.second_event_received,
                    second_commands: self.second_commands.clone(),
                    elapsed_ms: self.started_at.elapsed().as_millis(),
                    timeout_ms: self.timeout_ms,
                    error: self.input_error.clone().or_else(|| {
                        Some("timed out waiting for complete hotkey sequence".to_string())
                    }),
                },
                ctx,
            );
        }
        ctx.request_repaint_after(Duration::from_millis(25));
    }
}

fn send_macos_hotkey(accelerator: &str) -> Result<(), String> {
    if !std_core::desktop_automation_allowed() {
        return Err(gui_hotkey_smoke_blocked_reason());
    }
    let hotkey = LauncherHotkey::parse(accelerator)
        .ok_or_else(|| format!("unsupported accelerator: {accelerator}"))?;
    let script = apple_script_for_hotkey(&hotkey)?;
    let output = Command::new("/usr/bin/osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if stderr.is_empty() {
        Err(format!("osascript exited with {}", output.status))
    } else {
        Err(stderr)
    }
}

fn apple_script_for_hotkey(hotkey: &LauncherHotkey) -> Result<String, String> {
    if hotkey.key.eq_ignore_ascii_case("Space") {
        return Ok(format!(
            "tell application \"System Events\" to key code 49 using {}",
            apple_script_modifiers(&hotkey.modifiers)?
        ));
    }
    if hotkey.key.chars().count() == 1 {
        return Ok(format!(
            "tell application \"System Events\" to keystroke \"{}\" using {}",
            hotkey.key,
            apple_script_modifiers(&hotkey.modifiers)?
        ));
    }
    Err(format!("unsupported key for GUI smoke: {}", hotkey.key))
}

fn apple_script_modifiers(modifiers: &[String]) -> Result<String, String> {
    let mapped = modifiers
        .iter()
        .map(|modifier| match modifier.as_str() {
            "Alt" => Ok("option down"),
            "Command" => Ok("command down"),
            "Control" => Ok("control down"),
            "Shift" => Ok("shift down"),
            other => Err(format!("unsupported modifier for GUI smoke: {other}")),
        })
        .collect::<Result<Vec<_>, _>>()?;
    if mapped.is_empty() {
        Ok("{}".to_string())
    } else {
        Ok(format!("{{{}}}", mapped.join(", ")))
    }
}

fn format_window_commands(commands: &[LauncherWindowCommand]) -> String {
    commands
        .iter()
        .map(|command| match command {
            LauncherWindowCommand::SetVisible(true) => "Visible(true)",
            LauncherWindowCommand::SetVisible(false) => "Visible(false)",
            LauncherWindowCommand::Focus => "Focus",
            LauncherWindowCommand::PositionForPanel => "PositionForPanel",
            LauncherWindowCommand::ResizeToPanel => "ResizeToPanel",
            LauncherWindowCommand::ResizeToHiddenHost => "ResizeToHiddenHost",
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macos_hotkey_sender_is_hard_blocked_in_tests() {
        let error = send_macos_hotkey("Alt+Space").unwrap_err();

        assert!(error.contains("STD_TEST_MODE blocked GUI hotkey smoke"));
    }
}
