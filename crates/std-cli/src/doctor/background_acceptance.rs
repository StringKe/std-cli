use crate::{
    doctor::{ui_capture_run::verify_capture_run_id, workspace::check_text},
    CliError,
};
use std::{env, fs};

pub(crate) fn check_background_acceptance_manifest() -> Result<(), CliError> {
    let Ok(path) = env::var("STD_BACKGROUND_UI_ACCEPTANCE_MANIFEST") else {
        return Ok(());
    };
    let body = fs::read_to_string(&path).map_err(|error| {
        CliError::Doctor(format!(
            "unable to read background UI acceptance manifest: {error}"
        ))
    })?;
    verify_background_acceptance_manifest(&body)
}

fn verify_background_acceptance_manifest(body: &str) -> Result<(), CliError> {
    for required in [
        "background-ui-acceptance manifest",
        "target=isolated_background_ui_harness_only",
        "identity_rule=pid+window-id+bundle-id+window-title+harness-token",
        "completion_rule=background-ui-smoke-PASS-and-frontmost-preserved",
        "default_gate=manual-opt-in-only",
        "forbidden_targets=frontmost_app,Terminal,1Password,WeChat,weixin,wechat,微信,System_Settings",
        "forbidden_route=global_HID,System_Events,frontmost_click,screen_coordinate_click",
        "fallback=never_frontmost_desktop_click",
        "frontmost_policy=previous_app_never_targeted",
        "bundle_id=dev.std-cli.background-ui-harness",
        "window_title=std-cli Background UI Harness ",
        "harness_token=run-",
        "harness_run_id=",
        "smoke_command=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke",
        "--bundle-id dev.std-cli.background-ui-harness",
        "--window-title \"std-cli Background UI Harness ",
        "--harness-token run-",
        "smoke_status=PASS",
        "smoke_run_id=",
        "driver_stdout=background_driver PASS",
        "driver_identity=target-pid-window-id-and-frontmost-pid",
        "event_route=postToPid_target_pid_only",
        "frontmost_preservation=required",
        "frontmost_preserved=true",
        "frontmost_before_equals_after=required",
        "frontmost_evidence_source=background_driver_stdout",
        "target_not_frontmost=required",
        "previous_app_policy=event_tap_only_no_input_delivery",
        "real_app_policy=deny_user_apps_by_bundle_pid_window_title_mismatch",
        "harness_origin=spawned_by_scripts_background_ui_harness_only",
    ] {
        check_text(body, required)?;
    }
    verify_positive_field(body, "harness_pid=")?;
    verify_positive_field(body, "window_id=")?;
    verify_driver_identity(body)?;
    verify_matching_run_id(body)?;
    verify_matching_token(body)
}

fn verify_positive_field(body: &str, key: &str) -> Result<(), CliError> {
    let value = manifest_field(body, key)?;
    if value
        .parse::<u32>()
        .ok()
        .filter(|value| *value > 0)
        .is_some()
    {
        return Ok(());
    }
    Err(CliError::Doctor(format!(
        "background UI acceptance manifest field must be positive: {key}"
    )))
}

fn verify_matching_token(body: &str) -> Result<(), CliError> {
    let token = manifest_field(body, "harness_token=")?;
    let title = manifest_field(body, "window_title=")?;
    let command = manifest_field(body, "smoke_command=")?;
    let expected_title = format!("std-cli Background UI Harness {token}");
    if title != expected_title {
        return Err(CliError::Doctor(
            "background UI acceptance window title token mismatch".to_string(),
        ));
    }
    if !command.contains(&format!("--harness-token {token}")) {
        return Err(CliError::Doctor(
            "background UI acceptance smoke command token mismatch".to_string(),
        ));
    }
    Ok(())
}

fn verify_matching_run_id(body: &str) -> Result<(), CliError> {
    let run_id = manifest_field(body, "run_id=")?;
    verify_capture_run_id(run_id)?;
    for key in ["harness_run_id=", "smoke_run_id="] {
        let actual = manifest_field(body, key)?;
        if actual != run_id {
            return Err(CliError::Doctor(format!(
                "background UI acceptance run_id mismatch for {key}"
            )));
        }
    }
    Ok(())
}

fn verify_driver_identity(body: &str) -> Result<(), CliError> {
    let harness_pid = manifest_field(body, "harness_pid=")?;
    let window_id = manifest_field(body, "window_id=")?;
    let driver = manifest_field(body, "driver_stdout=")?;
    for required in [
        format!("target_pid={harness_pid}"),
        format!("window_id={window_id}"),
        "event_route=postToPid_target_pid_only".to_string(),
        "frontmost_preserved=true".to_string(),
    ] {
        if !driver.contains(&required) {
            return Err(CliError::Doctor(format!(
                "background UI driver stdout missing identity: {required}"
            )));
        }
    }
    let before = driver_pid_field(driver, "frontmost_before=")?;
    let after = driver_pid_field(driver, "frontmost_after=")?;
    if before != after {
        return Err(CliError::Doctor(
            "background UI driver changed frontmost app".to_string(),
        ));
    }
    if before == harness_pid {
        return Err(CliError::Doctor(
            "background UI driver targeted frontmost harness".to_string(),
        ));
    }
    Ok(())
}

fn driver_pid_field<'a>(driver: &'a str, key: &str) -> Result<&'a str, CliError> {
    driver
        .split_whitespace()
        .find_map(|part| part.strip_prefix(key))
        .ok_or_else(|| CliError::Doctor(format!("background UI driver field missing: {key}")))
}

fn manifest_field<'a>(body: &'a str, key: &str) -> Result<&'a str, CliError> {
    body.lines()
        .find_map(|line| line.strip_prefix(key))
        .ok_or_else(|| CliError::Doctor(format!("background UI manifest field missing: {key}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_acceptance_manifest_requires_pass_and_frontmost_rule() {
        verify_background_acceptance_manifest(sample_manifest()).unwrap();
    }

    #[test]
    fn background_acceptance_manifest_rejects_non_pass_smoke() {
        let manifest = sample_manifest().replace("smoke_status=PASS", "smoke_status=FAIL");

        let error = verify_background_acceptance_manifest(&manifest).unwrap_err();
        assert!(error.to_string().contains("smoke_status=PASS"));
    }

    #[test]
    fn background_acceptance_manifest_rejects_title_token_mismatch() {
        let manifest = sample_manifest().replace(
            "window_title=std-cli Background UI Harness run-42",
            "window_title=std-cli Background UI Harness run-old",
        );

        let error = verify_background_acceptance_manifest(&manifest).unwrap_err();
        assert!(error.to_string().contains("window title token mismatch"));
    }

    #[test]
    fn background_acceptance_manifest_rejects_missing_frontmost_rule() {
        let manifest = sample_manifest().replace("frontmost_preservation=required\n", "");

        let error = verify_background_acceptance_manifest(&manifest).unwrap_err();
        assert!(error
            .to_string()
            .contains("frontmost_preservation=required"));
    }

    #[test]
    fn background_acceptance_manifest_rejects_driver_pid_mismatch() {
        let manifest = sample_manifest().replace("target_pid=42", "target_pid=43");

        let error = verify_background_acceptance_manifest(&manifest).unwrap_err();
        assert!(error.to_string().contains("target_pid=42"));
    }

    #[test]
    fn background_acceptance_manifest_rejects_frontmost_change() {
        let manifest = sample_manifest().replace("frontmost_after=777", "frontmost_after=778");

        let error = verify_background_acceptance_manifest(&manifest).unwrap_err();
        assert!(error.to_string().contains("changed frontmost app"));
    }

    #[test]
    fn background_acceptance_manifest_rejects_frontmost_harness_target() {
        let manifest = sample_manifest()
            .replace("harness_pid=42", "harness_pid=777")
            .replace("target_pid=42", "target_pid=777");

        let error = verify_background_acceptance_manifest(&manifest).unwrap_err();
        assert!(error.to_string().contains("targeted frontmost harness"));
    }

    #[test]
    fn background_acceptance_manifest_rejects_missing_forbidden_targets() {
        let manifest = sample_manifest().replace(
            "forbidden_targets=frontmost_app,Terminal,1Password,WeChat,weixin,wechat,微信,System_Settings\n",
            "",
        );

        let error = verify_background_acceptance_manifest(&manifest).unwrap_err();
        assert!(error.to_string().contains("forbidden_targets="));
    }

    #[test]
    fn background_acceptance_manifest_rejects_missing_no_frontmost_click_fallback() {
        let manifest = sample_manifest().replace("fallback=never_frontmost_desktop_click\n", "");

        let error = verify_background_acceptance_manifest(&manifest).unwrap_err();
        assert!(error
            .to_string()
            .contains("fallback=never_frontmost_desktop_click"));
    }

    #[test]
    fn background_acceptance_manifest_rejects_missing_run_id() {
        let manifest = sample_manifest().replace("run_id=20260522T000000Z-4242\n", "");

        let error = verify_background_acceptance_manifest(&manifest).unwrap_err();
        assert!(error.to_string().contains("run_id="));
    }

    #[test]
    fn background_acceptance_manifest_rejects_mismatched_run_id() {
        let manifest =
            sample_manifest().replace("smoke_run_id=20260522T000000Z-4242", "smoke_run_id=old");

        let error = verify_background_acceptance_manifest(&manifest).unwrap_err();
        assert!(error.to_string().contains("run_id mismatch"));
    }

    fn sample_manifest() -> &'static str {
        "background-ui-acceptance manifest\n\
created_at=2026-05-22T00:00:00Z\n\
run_id=20260522T000000Z-4242\n\
target=isolated_background_ui_harness_only\n\
identity_rule=pid+window-id+bundle-id+window-title+harness-token\n\
completion_rule=background-ui-smoke-PASS-and-frontmost-preserved\n\
default_gate=manual-opt-in-only\n\
forbidden_targets=frontmost_app,Terminal,1Password,WeChat,weixin,wechat,微信,System_Settings\n\
forbidden_route=global_HID,System_Events,frontmost_click,screen_coordinate_click\n\
fallback=never_frontmost_desktop_click\n\
frontmost_policy=previous_app_never_targeted\n\
harness_pid=42\n\
window_id=24\n\
bundle_id=dev.std-cli.background-ui-harness\n\
window_title=std-cli Background UI Harness run-42\n\
harness_token=run-42\n\
harness_run_id=20260522T000000Z-4242\n\
smoke_command=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke --harness-pid 42 --window-id 24 --bundle-id dev.std-cli.background-ui-harness --window-title \"std-cli Background UI Harness run-42\" --harness-token run-42\n\
smoke_status=PASS\n\
smoke_run_id=20260522T000000Z-4242\n\
driver_stdout=background_driver PASS target_pid=42 window_id=24 event_route=postToPid_target_pid_only frontmost_preserved=true frontmost_before=777 frontmost_after=777\n\
driver_identity=target-pid-window-id-and-frontmost-pid\n\
frontmost_preservation=required\n\
frontmost_preserved=true\n\
frontmost_before_equals_after=required\n\
frontmost_evidence_source=background_driver_stdout\n\
target_not_frontmost=required\n\
previous_app_policy=event_tap_only_no_input_delivery\n\
real_app_policy=deny_user_apps_by_bundle_pid_window_title_mismatch\n\
harness_origin=spawned_by_scripts_background_ui_harness_only\n\
manifest=artifacts/ui/background-acceptance/manifest.txt\n"
    }
}
