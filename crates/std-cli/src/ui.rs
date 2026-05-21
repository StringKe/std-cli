use clap::Subcommand;

const HARNESS_BUNDLE_ID: &str = "dev.std-cli.background-ui-harness";
const HARNESS_WINDOW_TITLE: &str = "std-cli Background UI Harness";

#[derive(Debug, Subcommand)]
pub enum UiCommand {
    /// Run the opt-in-only background UI acceptance smoke gate.
    BackgroundSmoke {
        #[arg(long)]
        harness_pid: Option<u32>,
        #[arg(long)]
        window_id: Option<u32>,
        #[arg(long)]
        bundle_id: Option<String>,
        #[arg(long)]
        window_title: Option<String>,
    },
}

pub(crate) fn handle_ui(command: UiCommand) -> String {
    match command {
        UiCommand::BackgroundSmoke {
            harness_pid,
            window_id,
            bundle_id,
            window_title,
        } => background_smoke(BackgroundSmokeConfig {
            harness_pid,
            window_id,
            bundle_id,
            window_title,
        }),
    }
}

struct BackgroundSmokeConfig {
    harness_pid: Option<u32>,
    window_id: Option<u32>,
    bundle_id: Option<String>,
    window_title: Option<String>,
}

fn background_smoke(config: BackgroundSmokeConfig) -> String {
    let automation = std::env::var("STD_ALLOW_BACKGROUND_UI_AUTOMATION").ok();
    if std_core::std_test_mode_enabled() {
        return background_smoke_report(
            "SKIP",
            "STD_TEST_MODE blocks background UI automation",
            &config,
        );
    }
    if automation.as_deref() != Some("1") {
        return background_smoke_report(
            "SKIP",
            "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required",
            &config,
        );
    }
    if let Some(reason) = invalid_harness_reason(&config) {
        return background_smoke_report("SKIP", reason, &config);
    }
    background_smoke_report(
        "SKIP",
        "background UI driver pending isolated harness implementation",
        &config,
    )
}

fn invalid_harness_reason(config: &BackgroundSmokeConfig) -> Option<&'static str> {
    match (
        config.harness_pid,
        config.window_id,
        config.bundle_id.as_deref(),
        config.window_title.as_deref(),
    ) {
        (None, _, _, _) => Some("harness_pid required"),
        (_, None, _, _) => Some("window_id required"),
        (_, _, None, _) => Some("bundle_id required"),
        (_, _, _, None) => Some("window_title required"),
        (Some(0), _, _, _) => Some("harness_pid must be nonzero"),
        (_, Some(0), _, _) => Some("window_id must be nonzero"),
        (_, _, Some(bundle), _) if bundle != HARNESS_BUNDLE_ID => {
            Some("bundle_id outside whitelist")
        }
        (_, _, _, Some(title)) if title != HARNESS_WINDOW_TITLE => {
            Some("window_title outside whitelist")
        }
        _ => None,
    }
}

fn background_smoke_report(status: &str, reason: &str, config: &BackgroundSmokeConfig) -> String {
    [
        format!("background_ui_smoke {status}"),
        format!("reason={reason}"),
        "target=isolated_background_ui_harness_only".to_string(),
        format!("required_bundle_id={HARNESS_BUNDLE_ID}"),
        format!("required_window_title={HARNESS_WINDOW_TITLE}"),
        format!("harness_pid={}", opt_u32(config.harness_pid)),
        format!("window_id={}", opt_u32(config.window_id)),
        format!("bundle_id={}", opt_str(config.bundle_id.as_deref())),
        format!("window_title={}", opt_str(config.window_title.as_deref())),
        "driver=AX_or_CGEvent_postToPid_after_explicit_opt_in".to_string(),
        "activation=event_tap_then_appkit_defined_primer_then_center_primer".to_string(),
        "event_route=postToPid_target_pid_only".to_string(),
        "focus_guard=drop_previous_app_deactivation".to_string(),
        "focus_messages=raw_13_19_20".to_string(),
        "primer_start=appKitDefined_subtype_1_applicationActivated".to_string(),
        "primer_end=appKitDefined_subtype_2_applicationDeactivated".to_string(),
        "center_primer=window_center_activation_only_no_user_action".to_string(),
        "window_addressing=windowUnderMouse_windowThatCanHandle_fields_51_58".to_string(),
        "forbidden_targets=frontmost_app,Terminal,1Password,WeChat,System_Settings".to_string(),
        "forbidden_route=global_HID,System_Events,frontmost_click,screen_coordinate_click"
            .to_string(),
        "fallback=never_frontmost_desktop_click".to_string(),
    ]
    .join("\n")
}

fn opt_u32(value: Option<u32>) -> String {
    value
        .map(|number| number.to_string())
        .unwrap_or_else(|| "MISSING".to_string())
}

fn opt_str(value: Option<&str>) -> &str {
    value.unwrap_or("MISSING")
}
