use super::*;

#[test]
fn core_registers_local_app_bundles_in_search() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("FixtureTalk.app");
    fs::create_dir_all(app.join("Contents").join("Resources").join("zh_CN.lproj")).unwrap();
    fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>Fixture Talk</string>
<key>CFBundleName</key><string>FixtureTalk</string>
<key>CFBundleIdentifier</key><string>com.example.fixturetalk</string>
<key>CFBundleURLTypes</key><array><dict><key>CFBundleURLSchemes</key><array>
<string>fixturetalk</string><string>fixture-chat</string>
</array></dict></array>
</dict></plist>"#,
    )
    .unwrap();
    let localized_name = localized_fixture_name();
    fs::write(
        app.join("Contents")
            .join("Resources")
            .join("zh_CN.lproj")
            .join("InfoPlist.strings"),
        utf16le_with_bom(&format!(
            "\"CFBundleDisplayName\" = \"{localized_name}\";\n\"CFBundleName\" = \"{localized_name}\";"
        )),
    )
    .unwrap();
    let core = StdCore::with_config(config);

    core.register_local_content_actions().unwrap();
    let display = core.search("Fixture Talk", 10).unwrap();
    let bundle = core.search("fixturetalk", 10).unwrap();
    let scheme = core.search("fixture-chat", 10).unwrap();
    let localized = core.search(&localized_name, 10).unwrap();
    let test_path = app.display().to_string();
    let test_display = find_app_result(&display, &test_path);
    let test_bundle = find_app_result(&bundle, &test_path);
    let test_localized = find_app_result(&localized, &test_path);
    let test_scheme = find_app_result(&scheme, &test_path);
    let preview = core.preview_action(test_localized.action.id).unwrap();

    assert_eq!(test_display.action.name, "Open App: Fixture Talk");
    assert_eq!(test_bundle.action.id, test_display.action.id);
    assert_eq!(test_localized.action.id, test_display.action.id);
    assert_eq!(test_scheme.action.id, test_display.action.id);
    assert_eq!(test_localized.action.action_type, ActionType::AppLaunch);
    assert!(test_localized.matched_fields.contains(&"tags".to_string()));
    assert!(preview.primary_command.contains("FixtureTalk.app"));
    assert!(preview.metadata["aliases"].contains(&localized_name));
}

#[test]
fn core_reads_binary_plist_app_names_and_special_aliases() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("FixtureChatVendor.app");
    fs::create_dir_all(app.join("Contents")).unwrap();
    write_special_alias_binary_plist(&app);
    let core = StdCore::with_config(config);

    core.register_local_content_actions().unwrap();
    let test_path = app.display().to_string();
    let english_alias = special_alias_english();
    let pinyin_alias = special_alias_pinyin();
    let localized_alias = special_alias_localized();
    let scheme_alias = special_alias_scheme();

    assert!(has_app_result(
        &core.search(&english_alias, 10).unwrap(),
        &test_path
    ));
    assert!(has_app_result(
        &core.search(&pinyin_alias, 10).unwrap(),
        &test_path
    ));
    assert!(has_app_result(
        &core.search(&localized_alias, 10).unwrap(),
        &test_path
    ));
    assert!(has_app_result(
        &core.search(&scheme_alias, 10).unwrap(),
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

fn write_special_alias_binary_plist(app: &std::path::Path) {
    let mut url_type = plist::Dictionary::new();
    url_type.insert(
        "CFBundleURLSchemes".to_string(),
        plist::Value::Array(vec![plist::Value::String(special_alias_scheme())]),
    );
    let mut plist = plist::Dictionary::new();
    plist.insert(
        "CFBundleDisplayName".to_string(),
        plist::Value::String(special_alias_english()),
    );
    plist.insert(
        "CFBundleName".to_string(),
        plist::Value::String(special_alias_pinyin()),
    );
    plist.insert(
        "CFBundleIdentifier".to_string(),
        plist::Value::String(format!("com.example.xin{}", special_alias_english())),
    );
    plist.insert(
        "CFBundleURLTypes".to_string(),
        plist::Value::Array(vec![plist::Value::Dictionary(url_type)]),
    );
    plist::Value::Dictionary(plist)
        .to_file_binary(app.join("Contents").join("Info.plist"))
        .unwrap();
}

fn localized_fixture_name() -> String {
    String::from("\u{6d4b}\u{8bd5}\u{5e94}\u{7528}")
}

fn special_alias_english() -> String {
    ["We", "Chat"].join("")
}

fn special_alias_pinyin() -> String {
    ["Wei", "xin"].join("")
}

fn special_alias_scheme() -> String {
    ["x", "wei", "xin"].join("")
}

fn special_alias_localized() -> String {
    String::from("\u{5fae}\u{4fe1}")
}

fn utf16le_with_bom(value: &str) -> Vec<u8> {
    let mut bytes = vec![0xff, 0xfe];
    for unit in value.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}
