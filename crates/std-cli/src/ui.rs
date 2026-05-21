use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum UiCommand {
    /// Run the opt-in-only background UI acceptance smoke gate.
    BackgroundSmoke,
}

pub(crate) fn handle_ui(command: UiCommand) -> String {
    match command {
        UiCommand::BackgroundSmoke => background_smoke(),
    }
}

fn background_smoke() -> String {
    let automation = std::env::var("STD_ALLOW_BACKGROUND_UI_AUTOMATION").ok();
    if std_core::std_test_mode_enabled() {
        return background_smoke_skip("STD_TEST_MODE blocks background UI automation");
    }
    if automation.as_deref() != Some("1") {
        return background_smoke_skip("STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required");
    }
    background_smoke_skip("background UI harness not implemented")
}

fn background_smoke_skip(reason: &str) -> String {
    [
        "background_ui_smoke SKIP".to_string(),
        format!("reason={reason}"),
        "target=installed_std_launcher_or_std_studio_only".to_string(),
        "forbidden_targets=frontmost_app,Terminal,1Password,WeChat,System_Settings".to_string(),
        "fallback=never_frontmost_desktop_click".to_string(),
    ]
    .join("\n")
}
