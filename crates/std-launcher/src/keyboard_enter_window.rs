use crate::{
    format_window_commands, launcher_execution_hides_window, LauncherEnterWindowReport,
    LauncherKey, LauncherState,
};
use std_core::{StdConfig, StdCore};
use std_types::ActionExecutionStatus;

pub(crate) fn enter_window_evidence() -> LauncherEnterWindowReport {
    let completed = enter_window_case("index");
    let deferred = enter_window_case("Keyboard Smoke App");
    LauncherEnterWindowReport {
        completed_status: completed.status,
        completed_hide_requested: completed.hide_requested,
        completed_window_commands: completed.window_commands,
        deferred_status: deferred.status,
        deferred_hide_requested: deferred.hide_requested,
        deferred_window_commands: deferred.window_commands,
    }
}

struct EnterWindowCase {
    status: Option<ActionExecutionStatus>,
    hide_requested: bool,
    window_commands: String,
}

fn enter_window_case(query: &str) -> EnterWindowCase {
    let root = std::env::temp_dir().join(format!(
        "std-launcher-enter-window-{}-{query}",
        std::process::id()
    ));
    let config = StdConfig {
        data_dir: root.join("data"),
        ..StdConfig::default()
    };
    write_keyboard_smoke_app(&config);
    let core = StdCore::with_config(config);
    let mut state = LauncherState::with_core(core);
    state.controller.show();
    state.update_query(query);
    let previous_visible = state.controller.visible;
    let execution = state.handle_keyboard_input_by_user(LauncherKey::Enter, false);
    let hide_requested = execution
        .as_ref()
        .map(launcher_execution_hides_window)
        .unwrap_or(false);
    if hide_requested {
        state.hide();
    }
    let commands = LauncherState::enter_window_commands(previous_visible, state.controller.visible);
    let _ = std::fs::remove_dir_all(root);
    EnterWindowCase {
        status: execution.map(|execution| execution.status),
        hide_requested,
        window_commands: if commands.is_empty() {
            "none".to_string()
        } else {
            format_window_commands(&commands)
        },
    }
}

fn write_keyboard_smoke_app(config: &StdConfig) {
    let app = config.apps_dir().join("KeyboardSmokeApp.app");
    let contents = app.join("Contents");
    let _ = std::fs::create_dir_all(&contents);
    let _ = std::fs::write(
        contents.join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>Keyboard Smoke App</string>
<key>CFBundleName</key><string>KeyboardSmokeApp</string>
</dict></plist>"#,
    );
}
