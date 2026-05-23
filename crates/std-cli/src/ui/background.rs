const HARNESS_BUNDLE_ID: &str = "dev.std-cli.background-ui-harness";
const HARNESS_WINDOW_TITLE_PREFIX: &str = "std-cli Background UI Harness";
const BACKGROUND_HARNESS_HELPER: &str = "scripts/background-ui-harness.sh";
const BACKGROUND_RUNNER: &str = "scripts/background-ui-smoke.swift";
const BACKGROUND_DRIVER: [&str; 4] = [
    "per-process-event-tap",
    "appKitDefined-activation-primer",
    "window-center-primer",
    "postToPid-target-pid-input",
];

pub(super) struct BackgroundSmokeConfig {
    pub(super) harness_pid: Option<u32>,
    pub(super) window_id: Option<u32>,
    pub(super) bundle_id: Option<String>,
    pub(super) window_title: Option<String>,
    pub(super) harness_token: Option<String>,
}

pub(super) fn background_smoke(config: BackgroundSmokeConfig) -> String {
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
    run_background_smoke_driver(&config)
}

fn invalid_harness_reason(config: &BackgroundSmokeConfig) -> Option<&'static str> {
    match (
        config.harness_pid,
        config.window_id,
        config.bundle_id.as_deref(),
        config.window_title.as_deref(),
        config.harness_token.as_deref(),
    ) {
        (None, _, _, _, _) => Some("harness_pid required"),
        (_, None, _, _, _) => Some("window_id required"),
        (_, _, None, _, _) => Some("bundle_id required"),
        (_, _, _, None, _) => Some("window_title required"),
        (_, _, _, _, None) => Some("harness_token required"),
        (Some(0), _, _, _, _) => Some("harness_pid must be nonzero"),
        (_, Some(0), _, _, _) => Some("window_id must be nonzero"),
        (_, _, Some(bundle), _, _) if bundle != HARNESS_BUNDLE_ID => {
            Some("bundle_id outside whitelist")
        }
        (_, _, _, Some(title), Some(token)) if title != harness_window_title(token) => {
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
        format!("required_window_title_prefix={HARNESS_WINDOW_TITLE_PREFIX}"),
        format!("harness_helper={BACKGROUND_HARNESS_HELPER}"),
        format!("harness_pid={}", opt_u32(config.harness_pid)),
        format!("window_id={}", opt_u32(config.window_id)),
        format!("bundle_id={}", opt_str(config.bundle_id.as_deref())),
        format!("window_title={}", opt_str(config.window_title.as_deref())),
        format!("harness_token={}", opt_str(config.harness_token.as_deref())),
        format!("driver_sequence={}", BACKGROUND_DRIVER.join(",")),
        "cursor_visual=floating_cursor_not_required_for_event_delivery".to_string(),
        "harness_origin=spawned_by_scripts_background_ui_harness_only".to_string(),
        "target_identity=fixed_bundle_pid_window_title_quadruple".to_string(),
        "run_identity=harness_token_required_to_reject_stale_windows".to_string(),
        "tap_order=install_previous_and_target_taps_before_primer".to_string(),
        "trust_gate=AXIsProcessTrusted_before_event_tap".to_string(),
        "tap_failure=identify_previous_or_target_and_fail_before_primer".to_string(),
        "activation=event_tap_then_appkit_defined_primer_then_center_primer".to_string(),
        "event_route=postToPid_target_pid_only".to_string(),
        "key_smoke=Enter_to_isolated_echo_result".to_string(),
        "frontmost_policy=previous_app_never_targeted".to_string(),
        "frontmost_user_app_policy=identify_and_preserve_current_frontmost_app".to_string(),
        "frontmost_preservation=frontmost_before_equals_frontmost_after_required".to_string(),
        "frontmost_sensitive_app_policy=fail_before_event_tap".to_string(),
        "real_app_policy=deny_user_apps_by_bundle_pid_window_title_mismatch".to_string(),
        "focus_guard=drop_previous_app_deactivation".to_string(),
        "focus_policy=allow_target_activation_only".to_string(),
        "focus_messages=raw_13_19_20".to_string(),
        "tap_mask=focus_raw_13_19_20_only".to_string(),
        "primer_start=appKitDefined_subtype_1_applicationActivated".to_string(),
        "primer_end=appKitDefined_subtype_2_applicationDeactivated".to_string(),
        "center_primer=window_center_activation_only_no_user_action".to_string(),
        "window_addressing=windowUnderMouse_windowThatCanHandle_fields_51_58".to_string(),
        "forbidden_targets=frontmost_app,Terminal,1Password,WeChat,weixin,wechat,微信,System_Settings"
            .to_string(),
        "forbidden_route=global_HID,System_Events,frontmost_click,screen_coordinate_click"
            .to_string(),
        "fallback=never_frontmost_desktop_click".to_string(),
        "manual_only=excluded_from_default_quality_and_release_smoke".to_string(),
    ]
    .join("\n")
}

fn run_background_smoke_driver(config: &BackgroundSmokeConfig) -> String {
    #[cfg(target_os = "macos")]
    {
        run_macos_background_smoke_driver(config)
    }
    #[cfg(not(target_os = "macos"))]
    {
        background_smoke_report("SKIP", "background UI driver requires macOS", config)
    }
}

#[cfg(target_os = "macos")]
fn run_macos_background_smoke_driver(config: &BackgroundSmokeConfig) -> String {
    let Some(runner) = background_runner_path() else {
        return background_smoke_report("SKIP", "background UI runner script missing", config);
    };
    let Some(harness_pid) = config.harness_pid else {
        return background_smoke_report("SKIP", "harness_pid required", config);
    };
    let Some(window_id) = config.window_id else {
        return background_smoke_report("SKIP", "window_id required", config);
    };
    let output = std::process::Command::new("/usr/bin/swift")
        .arg(runner)
        .arg("--harness-pid")
        .arg(harness_pid.to_string())
        .arg("--window-id")
        .arg(window_id.to_string())
        .arg("--bundle-id")
        .arg(HARNESS_BUNDLE_ID)
        .arg("--window-title")
        .arg(harness_window_title(
            config.harness_token.as_deref().unwrap_or_default(),
        ))
        .arg("--harness-token")
        .arg(config.harness_token.as_deref().unwrap_or_default())
        .output();
    match output {
        Ok(output) => background_driver_report(config, output),
        Err(error) => background_smoke_report(
            "SKIP",
            &format!("background UI runner failed to start: {error}"),
            config,
        ),
    }
}

#[cfg(target_os = "macos")]
fn background_runner_path() -> Option<std::path::PathBuf> {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.parent()?.parent()?;
    let runner = root.join(BACKGROUND_RUNNER);
    runner.exists().then_some(runner)
}

#[cfg(target_os = "macos")]
fn background_driver_report(
    config: &BackgroundSmokeConfig,
    output: std::process::Output,
) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let status = if output.status.success() {
        "PASS"
    } else {
        "FAIL"
    };
    [
        background_smoke_report(
            status,
            "background UI driver executed isolated harness",
            config,
        ),
        format!("runner={BACKGROUND_RUNNER}"),
        format!("runner_exit={}", output.status.code().unwrap_or(-1)),
        format!("runner_stdout={}", single_line(stdout.trim())),
        format!("runner_stderr={}", single_line(stderr.trim())),
    ]
    .join("\n")
}

#[cfg(target_os = "macos")]
fn single_line(value: &str) -> String {
    if value.is_empty() {
        return "EMPTY".to_string();
    }
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" | ")
}

fn opt_u32(value: Option<u32>) -> String {
    value
        .map(|number| number.to_string())
        .unwrap_or_else(|| "MISSING".to_string())
}

fn opt_str(value: Option<&str>) -> &str {
    value.unwrap_or("MISSING")
}

fn harness_window_title(token: &str) -> String {
    format!("{HARNESS_WINDOW_TITLE_PREFIX} {token}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn harness_config(bundle_id: &str, window_title: &str) -> BackgroundSmokeConfig {
        BackgroundSmokeConfig {
            harness_pid: Some(42),
            window_id: Some(24),
            bundle_id: Some(bundle_id.to_string()),
            window_title: Some(window_title.to_string()),
            harness_token: Some("run-42".to_string()),
        }
    }

    #[test]
    fn background_harness_identity_rejects_real_app_bundle_ids() {
        for bundle_id in [
            "com.apple.Terminal",
            "com.1password.1password",
            "com.tencent.xinWeChat",
            "com.tencent.WeChat",
        ] {
            let config = harness_config(bundle_id, &harness_window_title("run-42"));
            assert_eq!(
                invalid_harness_reason(&config),
                Some("bundle_id outside whitelist")
            );
        }
    }

    #[test]
    fn background_harness_identity_rejects_real_app_titles() {
        for title in [
            "Terminal",
            "1Password",
            "WeChat",
            "weixin",
            "wechat",
            "微信",
        ] {
            let config = harness_config(HARNESS_BUNDLE_ID, title);
            assert_eq!(
                invalid_harness_reason(&config),
                Some("window_title outside whitelist")
            );
        }
    }

    #[test]
    fn background_report_names_multilingual_forbidden_apps() {
        let report = background_smoke_report(
            "SKIP",
            "test",
            &harness_config(HARNESS_BUNDLE_ID, &harness_window_title("run-42")),
        );
        assert!(report
            .contains("frontmost_user_app_policy=identify_and_preserve_current_frontmost_app"));
        for name in [
            "WeChat",
            "weixin",
            "wechat",
            "微信",
            "Terminal",
            "1Password",
        ] {
            assert!(report.contains(name));
        }
    }

    #[test]
    fn background_harness_identity_requires_run_token() {
        let mut config = harness_config(HARNESS_BUNDLE_ID, &harness_window_title("run-42"));
        config.harness_token = None;

        assert_eq!(
            invalid_harness_reason(&config),
            Some("harness_token required")
        );
    }

    #[test]
    fn background_harness_identity_rejects_stale_window_token() {
        let config = harness_config(HARNESS_BUNDLE_ID, &harness_window_title("run-old"));

        assert_eq!(
            invalid_harness_reason(&config),
            Some("window_title outside whitelist")
        );
    }

    #[test]
    fn background_runner_uses_background_event_delivery_contract() {
        let runner = include_str!("../../../../scripts/background-ui-smoke.swift");
        let tap_index = runner.find("session.start()").unwrap();
        let activation_index = runner.find("sendAppKitActivation(to:").unwrap();
        let center_primer_index = runner.find("postCenterPrimer(to:").unwrap();
        let key_smoke_index = runner.find("postKeySmoke(to:").unwrap();

        assert!(tap_index < activation_index);
        assert!(activation_index < center_primer_index);
        assert!(center_primer_index < key_smoke_index);
        assert!(runner.contains("CGEvent.tapCreateForPid"));
        assert!(runner.contains("NSEvent.otherEvent"));
        assert!(runner.contains("with: .appKitDefined"));
        assert!(runner.contains("subtype: subtype"));
        assert!(runner.contains("postToPid(pid)"));
        assert!(runner.contains("frontmost_before"));
        assert!(runner.contains("frontmost_after"));
    }

    #[test]
    fn background_runner_forbids_frontmost_and_global_event_routes() {
        let runner = include_str!("../../../../scripts/background-ui-smoke.swift");

        for forbidden in [
            "System Events",
            "osascript",
            "CGWarpMouseCursorPosition",
            ".post(tap:",
            ".cghidEventTap",
            "NSWorkspace.shared.open",
            "activate(options:",
        ] {
            assert!(!runner.contains(forbidden), "{forbidden}");
        }
        assert!(runner.contains("isForbiddenFrontmostApp"));
        assert!(runner.contains("frontmost app is forbidden for event tap"));
        assert!(runner.contains("frontmost app changed"));
    }

    #[test]
    fn background_runner_names_multilingual_sensitive_apps() {
        let runner = include_str!("../../../../scripts/background-ui-smoke.swift");

        for name in [
            "1password",
            "terminal",
            "iterm",
            "wechat",
            "weixin",
            "微信",
            "system settings",
            "system preferences",
        ] {
            assert!(runner.contains(name), "{name}");
        }
    }
}
