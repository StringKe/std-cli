use super::*;

#[test]
fn core_searches_app_bundle_by_escaped_unicode_localized_name() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("FixtureMessenger.app");
    write_escaped_unicode_app_bundle(&app);
    let core = StdCore::with_config(config);

    core.register_local_content_actions().unwrap();
    let test_path = app.display().to_string();
    let localized_results = core.search(localized_name(), 10).unwrap();
    let english_results = core.search("Fixture Messenger", 10).unwrap();
    let pinyin_results = core.search("fixturexin", 10).unwrap();
    let localized = find_app_result(&localized_results, &test_path);
    let english = find_app_result(&english_results, &test_path);
    let pinyin = find_app_result(&pinyin_results, &test_path);
    let preview = core.preview_action(localized.action.id).unwrap();

    assert_eq!(localized.action.id, english.action.id);
    assert_eq!(localized.action.id, pinyin.action.id);
    assert_eq!(localized.action.name, "Open App: Fixture Messenger");
    assert!(localized.matched_fields.contains(&"tags".to_string()));
    assert!(preview.metadata["aliases"].contains(localized_name()));
}

#[test]
fn core_searches_wechat_app_by_english_pinyin_and_chinese_names() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("WeChat.app");
    write_wechat_bundle(&app);
    let core = StdCore::with_config(config);

    core.register_local_content_actions().unwrap();
    let test_path = app.display().to_string();
    let english_results = core.search("wechat", 10).unwrap();
    let pinyin_results = core.search("weixin", 10).unwrap();
    let chinese_results = core.search(wechat_chinese_name(), 10).unwrap();
    let english = find_app_result(&english_results, &test_path);
    let pinyin = find_app_result(&pinyin_results, &test_path);
    let chinese = find_app_result(&chinese_results, &test_path);
    let preview = core.preview_action(chinese.action.id).unwrap();

    assert_eq!(english.action.id, pinyin.action.id);
    assert_eq!(english.action.id, chinese.action.id);
    assert_eq!(chinese.action.name, "Open App: WeChat");
    assert!(chinese.action.description.contains("Aliases:"));
    assert!(chinese.action.description.contains("wechat"));
    assert!(chinese.action.description.contains("weixin"));
    assert!(chinese.action.description.contains(wechat_chinese_name()));
    assert!(preview.subtitle.contains("Aliases:"));
    assert!(preview.metadata["aliases"].contains("wechat"));
    assert!(preview.metadata["aliases"].contains("weixin"));
    assert!(preview.metadata["aliases"].contains(wechat_chinese_name()));
    assert!(preview.subtitle.contains("wechat"));
    assert!(preview.subtitle.contains("weixin"));
    assert!(preview.subtitle.contains(wechat_chinese_name()));
}

#[test]
fn core_prefers_localized_app_name_over_bundle_identifier() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("Weixin.app");
    write_localized_only_wechat_bundle(&app);
    let core = StdCore::with_config(config);

    core.register_local_content_actions().unwrap();
    let test_path = app.display().to_string();
    let chinese_results = core.search(wechat_chinese_name(), 10).unwrap();
    let english_results = core.search("wechat", 10).unwrap();
    let bundle_results = core.search("com.tencent.xinwechat", 10).unwrap();
    let chinese = find_app_result(&chinese_results, &test_path);
    let english = find_app_result(&english_results, &test_path);
    let bundle = find_app_result(&bundle_results, &test_path);
    let preview = core.preview_action(chinese.action.id).unwrap();

    assert_eq!(chinese.action.id, english.action.id);
    assert_eq!(chinese.action.id, bundle.action.id);
    assert_eq!(
        chinese.action.name,
        format!("Open App: {}", wechat_chinese_name())
    );
    assert_ne!(chinese.action.name, "Open App: com.tencent.xinWeChat");
    assert!(preview.metadata["aliases"].contains("com.tencent.xinWeChat"));
    assert!(preview.metadata["aliases"].contains("wechat"));
    assert!(preview.metadata["aliases"].contains("weixin"));
}

#[test]
fn core_searches_app_bundle_by_binary_localized_strings_plist() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("BinaryLocalizedWeixin.app");
    write_binary_localized_strings_wechat_bundle(&app);
    let core = StdCore::with_config(config);

    core.register_local_content_actions().unwrap();
    let test_path = app.display().to_string();
    let chinese_results = core.search(wechat_chinese_name(), 10).unwrap();
    let english_results = core.search("wechat", 10).unwrap();
    let pinyin_results = core.search("weixin", 10).unwrap();
    let chinese = find_app_result(&chinese_results, &test_path);
    let english = find_app_result(&english_results, &test_path);
    let pinyin = find_app_result(&pinyin_results, &test_path);

    assert_eq!(chinese.action.id, english.action.id);
    assert_eq!(chinese.action.id, pinyin.action.id);
    assert_eq!(
        chinese.action.name,
        format!("Open App: {}", wechat_chinese_name())
    );
    assert!(chinese.matched_fields.contains(&"tags".to_string()));
}

#[test]
fn core_reads_zh_hans_localized_app_names_and_ignores_non_lproj_files() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("ZhHansWeChat.app");
    write_zh_hans_wechat_bundle(&app);
    let core = StdCore::with_config(config);

    core.register_local_content_actions().unwrap();
    let test_path = app.display().to_string();
    let chinese_results = core.search(wechat_chinese_name(), 10).unwrap();
    let english_results = core.search("wechat", 10).unwrap();
    let pinyin_results = core.search("weixin", 10).unwrap();
    let chinese = find_app_result(&chinese_results, &test_path);
    let english = find_app_result(&english_results, &test_path);
    let pinyin = find_app_result(&pinyin_results, &test_path);
    let preview = core.preview_action(chinese.action.id).unwrap();

    assert_eq!(chinese.action.id, english.action.id);
    assert_eq!(chinese.action.id, pinyin.action.id);
    assert_eq!(
        chinese.action.name,
        format!("Open App: {}", wechat_chinese_name())
    );
    assert!(preview.metadata["aliases"].contains(wechat_chinese_name()));
    assert!(!preview.metadata["aliases"].contains("Not A Locale"));
}

fn write_escaped_unicode_app_bundle(app: &std::path::Path) {
    fs::create_dir_all(app.join("Contents").join("Resources").join("zh_CN.lproj")).unwrap();
    fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>Fixture Messenger</string>
<key>CFBundleName</key><string>FixtureXin</string>
<key>CFBundleIdentifier</key><string>com.example.fixturemessenger</string>
</dict></plist>"#,
    )
    .unwrap();
    fs::write(
        app.join("Contents")
            .join("Resources")
            .join("zh_CN.lproj")
            .join("InfoPlist.strings"),
        "\"CFBundleDisplayName\" = \"\\U6d4b\\U8bd5\\U5e94\\U7528\";",
    )
    .unwrap();
}

fn write_binary_localized_strings_wechat_bundle(app: &std::path::Path) {
    let localized_dir = app.join("Contents").join("Resources").join("zh_CN.lproj");
    fs::create_dir_all(&localized_dir).unwrap();
    fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleIdentifier</key><string>com.tencent.xinWeChat</string>
</dict></plist>"#,
    )
    .unwrap();
    let mut plist = plist::Dictionary::new();
    plist.insert(
        "CFBundleDisplayName".to_string(),
        plist::Value::String(wechat_chinese_name().to_string()),
    );
    plist::Value::Dictionary(plist)
        .to_file_binary(localized_dir.join("InfoPlist.strings"))
        .unwrap();
}

fn write_wechat_bundle(app: &std::path::Path) {
    fs::create_dir_all(app.join("Contents").join("Resources").join("zh_CN.lproj")).unwrap();
    fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>WeChat</string>
<key>CFBundleName</key><string>Weixin</string>
<key>CFBundleExecutable</key><string>WeChat</string>
<key>CFBundleIdentifier</key><string>com.tencent.xinWeChat</string>
</dict></plist>"#,
    )
    .unwrap();
    fs::write(
        app.join("Contents")
            .join("Resources")
            .join("zh_CN.lproj")
            .join("InfoPlist.strings"),
        "\"CFBundleDisplayName\" = \"\\U5fae\\U4fe1\";",
    )
    .unwrap();
}

fn write_localized_only_wechat_bundle(app: &std::path::Path) {
    fs::create_dir_all(app.join("Contents").join("Resources").join("zh_CN.lproj")).unwrap();
    fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleIdentifier</key><string>com.tencent.xinWeChat</string>
</dict></plist>"#,
    )
    .unwrap();
    fs::write(
        app.join("Contents")
            .join("Resources")
            .join("zh_CN.lproj")
            .join("InfoPlist.strings"),
        "\"CFBundleDisplayName\" = \"\\U5fae\\U4fe1\";",
    )
    .unwrap();
}

fn write_zh_hans_wechat_bundle(app: &std::path::Path) {
    let resources = app.join("Contents").join("Resources");
    let localized = resources.join("zh-Hans.lproj");
    fs::create_dir_all(&localized).unwrap();
    fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleIdentifier</key><string>com.tencent.xinWeChat</string>
</dict></plist>"#,
    )
    .unwrap();
    fs::write(
        localized.join("InfoPlist.strings"),
        "\"CFBundleDisplayName\" = \"\\U5fae\\U4fe1\";",
    )
    .unwrap();
    fs::create_dir_all(resources.join("NotALocale")).unwrap();
    fs::write(
        resources.join("NotALocale").join("InfoPlist.strings"),
        "\"CFBundleDisplayName\" = \"Not A Locale\";",
    )
    .unwrap();
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

fn localized_name() -> &'static str {
    "\u{6d4b}\u{8bd5}\u{5e94}\u{7528}"
}

fn wechat_chinese_name() -> &'static str {
    "\u{5fae}\u{4fe1}"
}
