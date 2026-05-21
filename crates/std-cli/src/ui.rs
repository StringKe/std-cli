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
        "target=isolated_background_ui_harness_only".to_string(),
        "driver=AX_or_CGEvent_postToPid_after_explicit_opt_in".to_string(),
        "activation=event_tap_then_appkit_defined_primer_then_center_primer".to_string(),
        "forbidden_targets=frontmost_app,Terminal,1Password,WeChat,System_Settings".to_string(),
        "fallback=never_frontmost_desktop_click".to_string(),
    ]
    .join("\n")
}
