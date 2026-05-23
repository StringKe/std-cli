use crate::{
    doctor::workspace::{check_text, read_required},
    CliError,
};
use std::path::Path;

pub(crate) fn check_launcher_keyboard_ime_evidence(root: &Path) -> Result<(), CliError> {
    let keyboard = read_required(&root.join("crates/std-launcher/src/keyboard.rs"))?;
    let smoke = read_required(&root.join("crates/std-launcher/src/keyboard_smoke.rs"))?;
    let user_enter = read_required(&root.join("crates/std-launcher/src/user_enter_smoke.rs"))?;
    let evidence = format!("{keyboard}\n{smoke}\n{user_enter}");
    for required in [
        "if ime_composing",
        "ime_composition_path",
        "zh-preedit({query_before_preedit})>blocked>commit({commit_query})>enter",
        "ime_commit_trigger_status",
        "user_enter_status",
        "user_enter_route",
        "Enter>handle_keyboard_input_by_user>ReviewFirst",
        "user_enter_deferred",
        "user_enter_open_contract",
        "ui_enter=handle_keyboard_input_by_user",
        "mode=ReviewFirst",
        "default=review-command",
        "run=ActionPanel>Run",
        "desktop_open=default_review_first",
        "explicit_run_status=NeedsExternalRunner",
        "explicit_run_reason=STD_TEST_MODE blocked desktop open",
        "hide_policy=Completed->hide,NeedsExternalRunner->keep-open",
    ] {
        check_text(&evidence, required)?;
    }
    Ok(())
}

pub(crate) fn check_launcher_app_localization_evidence(root: &Path) -> Result<(), CliError> {
    let smoke = read_required(&root.join("crates/std-launcher/src/app_localization_smoke.rs"))?;
    let cli = read_required(&root.join("crates/std-launcher/src/cli.rs"))?;
    let core = read_required(&root.join("crates/std-core/src/app_bundle.rs"))?;
    let core_profile = read_required(&root.join("crates/std-core/src/app_bundle_profile.rs"))?;
    let tests = read_required(&root.join("crates/std-launcher/src/app_tests.rs"))?;
    let evidence = format!("{smoke}\n{cli}\n{core}\n{core_profile}\n{tests}");
    for required in [
        "--app-localization-smoke",
        "launcher_app_localization_smoke",
        "queries=wechat|weixin|",
        "action_ids_match=true",
        "selected_titles={}",
        "preview_titles={}",
        "enter_statuses={}",
        "enter_status=NeedsExternalRunner",
        "deferred=true",
        "fixture_scope=local_apps_dir_only",
        "system_apps_scanned=false",
        "CFBundleDisplayName",
        "CFBundleName",
        "CFBundleIdentifier",
        "InfoPlist.strings",
        "read_localized_info_plist_names",
        "read_localized_names_plist",
        "decode_utf16",
        "derived_aliases",
        "zh_CN.lproj",
        "zh-Hans.lproj",
        "wechat",
        "weixin",
        "\\\\U5fae\\\\U4fe1",
        "ActionExecutionStatus::NeedsExternalRunner",
        "launcher_searches_wechat_by_macos_multilingual_names_without_launching",
        "launcher_searches_zh_hans_wechat_and_enter_keeps_desktop_blocked",
    ] {
        check_text(&evidence, required)?;
    }
    Ok(())
}
