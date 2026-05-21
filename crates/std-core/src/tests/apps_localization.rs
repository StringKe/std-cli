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
