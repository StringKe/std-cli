use super::*;

#[test]
fn core_registers_local_app_bundles_in_search() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("Weixin.app");
    fs::create_dir_all(app.join("Contents").join("Resources").join("zh_CN.lproj")).unwrap();
    fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>WeChat</string>
<key>CFBundleName</key><string>Weixin</string>
<key>CFBundleIdentifier</key><string>com.tencent.xinWeChat</string>
<key>CFBundleURLTypes</key><array><dict><key>CFBundleURLSchemes</key><array>
<string>xweixin</string><string>weixin</string><string>wechat</string>
</array></dict></array>
</dict></plist>"#,
    )
    .unwrap();
    fs::write(
        app.join("Contents")
            .join("Resources")
            .join("zh_CN.lproj")
            .join("InfoPlist.strings"),
        utf16le_with_bom(
            r#""CFBundleDisplayName" = "微信";
"CFBundleName" = "微信";"#,
        ),
    )
    .unwrap();
    let core = StdCore::with_config(config);

    core.register_local_content_actions().unwrap();
    let wechat = core.search("WeChat", 10).unwrap();
    let weixin = core.search("weixin", 10).unwrap();
    let scheme = core.search("xweixin", 10).unwrap();
    let chinese = core.search("微信", 10).unwrap();
    let test_path = app.display().to_string();
    let test_wechat = find_app_result(&wechat, &test_path);
    let test_weixin = find_app_result(&weixin, &test_path);
    let test_chinese = find_app_result(&chinese, &test_path);
    let test_scheme = find_app_result(&scheme, &test_path);
    let preview = core.preview_action(test_chinese.action.id).unwrap();

    assert_eq!(test_wechat.action.name, "Open App: WeChat");
    assert_eq!(test_weixin.action.id, test_wechat.action.id);
    assert_eq!(test_chinese.action.id, test_wechat.action.id);
    assert_eq!(test_scheme.action.id, test_wechat.action.id);
    assert_eq!(test_chinese.action.action_type, ActionType::AppLaunch);
    assert!(test_chinese.matched_fields.contains(&"tags".to_string()));
    assert!(preview.primary_command.contains("Weixin.app"));
    assert!(preview.metadata["aliases"].contains("微信"));
}

#[test]
fn core_reads_binary_plist_app_names_and_wechat_aliases() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("Tencent.app");
    fs::create_dir_all(app.join("Contents")).unwrap();
    write_wechat_binary_plist(&app);
    let core = StdCore::with_config(config);

    core.register_local_content_actions().unwrap();
    let test_path = app.display().to_string();

    assert!(has_app_result(
        &core.search("wechat", 10).unwrap(),
        &test_path
    ));
    assert!(has_app_result(
        &core.search("weixin", 10).unwrap(),
        &test_path
    ));
    assert!(has_app_result(
        &core.search("微信", 10).unwrap(),
        &test_path
    ));
    assert!(has_app_result(
        &core.search("xweixin", 10).unwrap(),
        &test_path
    ));
}

#[test]
fn core_registers_app_bundle_from_source() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let source_app = temp.path().join("Workbench.app");
    fs::create_dir_all(source_app.join("Contents").join("MacOS")).unwrap();
    fs::write(
        source_app.join("Contents").join("MacOS").join("workbench"),
        "bin",
    )
    .unwrap();
    let core = StdCore::with_config(config);

    let registered = core.register_app_bundle(&source_app).unwrap();
    let apps = core.list_registered_apps().unwrap();
    let results = core.search("Workbench", 10).unwrap();
    let execution = core.execute_action(results[0].action.id).unwrap();

    assert!(registered.ends_with("Applications/Workbench.app"));
    assert_eq!(apps, vec![registered]);
    assert_eq!(results[0].action.name, "Open App: Workbench");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
}

#[test]
fn test_mode_does_not_discover_system_app_bundles() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let core = StdCore::with_config(config);

    let entries = core.discover_local_content_actions().unwrap();

    assert!(entries.iter().all(|entry| {
        entry.action.action_type != ActionType::AppLaunch
            || entry
                .metadata
                .get("path")
                .map(|path| path.contains(temp.path().to_str().unwrap()))
                .unwrap_or(false)
    }));
}

fn find_app_result<'a>(
    results: &'a [std_types::SearchResult],
    path: &str,
) -> &'a std_types::SearchResult {
    results
        .iter()
        .find(|result| result.action.description.contains(path))
        .unwrap()
}

fn has_app_result(results: &[std_types::SearchResult], path: &str) -> bool {
    results
        .iter()
        .any(|result| result.action.description.contains(path))
}

fn write_wechat_binary_plist(app: &std::path::Path) {
    let mut url_type = plist::Dictionary::new();
    url_type.insert(
        "CFBundleURLSchemes".to_string(),
        plist::Value::Array(vec![plist::Value::String("xweixin".to_string())]),
    );
    let mut plist = plist::Dictionary::new();
    plist.insert(
        "CFBundleDisplayName".to_string(),
        plist::Value::String("WeChat".to_string()),
    );
    plist.insert(
        "CFBundleName".to_string(),
        plist::Value::String("Weixin".to_string()),
    );
    plist.insert(
        "CFBundleIdentifier".to_string(),
        plist::Value::String("com.tencent.xinWeChat".to_string()),
    );
    plist.insert(
        "CFBundleURLTypes".to_string(),
        plist::Value::Array(vec![plist::Value::Dictionary(url_type)]),
    );
    plist::Value::Dictionary(plist)
        .to_file_binary(app.join("Contents").join("Info.plist"))
        .unwrap();
}

fn utf16le_with_bom(value: &str) -> Vec<u8> {
    let mut bytes = vec![0xff, 0xfe];
    for unit in value.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}
