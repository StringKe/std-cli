use crate::gui_smoke::{run_gui_hotkey_smoke, GuiHotkeySmokeConfig};
use std_launcher::{
    hotkey_smoke, HotkeySmokeReport, LauncherKeyboardReport, LauncherSmokeReport, LauncherState,
    LauncherUiSemanticsReport, LauncherWindowSmokeReport,
};

enum LauncherCliSmoke {
    Launcher(LauncherSmokeReport),
    Hotkey(HotkeySmokeReport),
    Window(LauncherWindowSmokeReport),
    Keyboard(LauncherKeyboardReport),
    UiSemantics(LauncherUiSemanticsReport),
    GuiHotkey(GuiHotkeySmokeConfig),
}

pub(crate) fn run_smoke_from_args(args: Vec<String>) -> eframe::Result<bool> {
    match smoke_from_args(args) {
        Some(LauncherCliSmoke::Launcher(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::Hotkey(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::Window(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::Keyboard(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::UiSemantics(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::GuiHotkey(config)) => {
            let report = run_gui_hotkey_smoke(config)?;
            println!("{}", report.summary());
            Ok(true)
        }
        None => Ok(false),
    }
}

fn smoke_from_args(args: Vec<String>) -> Option<LauncherCliSmoke> {
    match args.get(1).map(String::as_str) {
        Some("--smoke") => {
            let query = args
                .get(2)
                .map(String::as_str)
                .filter(|query| !query.trim().is_empty())
                .unwrap_or("rebuild index");
            LauncherState::smoke(query).map(LauncherCliSmoke::Launcher)
        }
        Some("--hotkey-smoke") => {
            let accelerator = args
                .get(2)
                .map(String::as_str)
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("Alt+Space");
            Some(LauncherCliSmoke::Hotkey(hotkey_smoke(accelerator)))
        }
        Some("--window-smoke") => Some(LauncherCliSmoke::Window(LauncherState::window_smoke())),
        Some("--keyboard-smoke") => {
            let query = args
                .get(2)
                .map(String::as_str)
                .filter(|query| !query.trim().is_empty())
                .unwrap_or("index");
            Some(LauncherCliSmoke::Keyboard(LauncherState::keyboard_smoke(
                query,
            )))
        }
        Some("--ui-semantics-smoke") => {
            let query = args
                .get(2)
                .map(String::as_str)
                .filter(|query| !query.trim().is_empty())
                .unwrap_or("index");
            Some(LauncherCliSmoke::UiSemantics(
                LauncherState::ui_semantics_smoke(query),
            ))
        }
        Some("--gui-hotkey-smoke") => Some(LauncherCliSmoke::GuiHotkey(GuiHotkeySmokeConfig {
            accelerator: args
                .get(2)
                .map(String::as_str)
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("Alt+Space")
                .to_string(),
            timeout_ms: args
                .get(3)
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(5_000),
            trigger_delay_ms: 500,
            allow_system_events: std::env::var("STD_ALLOW_DESKTOP_AUTOMATION")
                .map(|value| value == "1")
                .unwrap_or(false),
        })),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_preview_args_are_not_smoke_args() {
        let args = vec![
            "std-launcher".to_string(),
            "--ui-preview".to_string(),
            "light".to_string(),
            "defer".to_string(),
            "1200".to_string(),
        ];

        assert!(smoke_from_args(args).is_none());
    }

    #[test]
    fn gui_hotkey_smoke_requires_desktop_automation_opt_in() {
        let args = vec![
            "std-launcher".to_string(),
            "--gui-hotkey-smoke".to_string(),
            "Alt+Space".to_string(),
        ];
        let Some(LauncherCliSmoke::GuiHotkey(config)) = smoke_from_args(args) else {
            panic!("expected GUI hotkey smoke config");
        };

        assert!(!config.allow_system_events);
    }

    #[test]
    fn gui_hotkey_smoke_skips_when_test_mode_is_active() {
        let report = run_gui_hotkey_smoke(GuiHotkeySmokeConfig {
            accelerator: "Alt+Space".to_string(),
            timeout_ms: 5_000,
            trigger_delay_ms: 500,
            allow_system_events: true,
        })
        .unwrap();

        assert_eq!(report.status, "SKIP");
        assert!(!report.registered);
        assert!(!report.input_sent);
        assert!(report
            .error
            .as_deref()
            .unwrap()
            .contains("STD_TEST_MODE blocked GUI hotkey smoke"));
    }
}
