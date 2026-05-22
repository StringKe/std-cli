use crate::StudioEguiApp;
use std_core::{StdConfig, StdCore};
use std_egui::tokens::{EffectiveTheme, ThemeMode, ThemeProfile};
use std_studio::StudioApp;

#[test]
fn settings_hotkeys_render_registry_source_and_reset() {
    let settings = include_str!("views/settings.rs");
    let rows = include_str!("views/settings_rows.rs");
    let binding = include_str!("views/settings_binding.rs");
    let registry = std_core::shortcuts::shortcut_registry(&StdConfig {
        launcher_hotkey: "Cmd+Space".to_string(),
        ..StdConfig::default()
    });
    let launcher = registry
        .iter()
        .find(|shortcut| shortcut.id == "launcher.global.toggle")
        .unwrap();

    assert_eq!(launcher.source.label(), "user");
    assert!(launcher.resettable);
    assert!(settings.contains("std_core::shortcuts::shortcut_registry"));
    assert!(settings.contains("settings_binding::binding_editor_row"));
    assert!(settings.contains("self.save_setting(\"launcher_hotkey\""));
    assert!(settings.contains("ShortcutRowEvent::Reset(\"launcher.global.toggle\")"));
    assert!(!settings.contains("ui.text_edit_singleline(&mut self.settings_hotkey)"));
    assert!(binding.contains("WidgetType::TextEdit"));
    assert!(binding.contains("row_paint::paint_row_frame"));
    assert!(binding.contains("Color::accent_weak"));
    assert!(rows.contains("shortcut.source.label()"));
    assert!(rows.contains("shortcut.default_binding"));
    assert!(rows.contains("shortcut_a11y_label"));
}

#[test]
fn settings_theme_control_uses_tokens_and_saves_theme_modes() {
    let settings = include_str!("views/settings.rs");
    let rows = include_str!("views/settings_rows.rs");

    assert!(settings.contains("settings_rows::theme_mode_control"));
    assert!(settings.contains("self.save_setting(\"theme\", self.settings_theme.clone())"));
    assert!(settings.contains("self.render_theme_profile(ui)"));
    assert!(!settings.contains("ui.text_edit_singleline(&mut self.settings_theme)"));
    assert!(rows.contains("[\"system\", \"dark\", \"light\"]"));
    assert!(rows.contains("Color::accent_weak"));
    assert!(rows.contains("Color::accent_base"));
}

#[test]
fn settings_theme_save_updates_config_state_and_theme_profile() {
    let mut app = test_app();
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("std.toml");
    let ctx = egui::Context::default();

    assert_theme_save(&mut app, &path, "dark", ThemeMode::Dark);
    let profile = ThemeProfile::apply_with_accessibility(
        &ctx,
        ThemeMode::resolve(&app.settings_theme),
        false,
        false,
        false,
        1.0,
    );
    assert_eq!(profile.requested, ThemeMode::Dark);
    assert_eq!(profile.effective, EffectiveTheme::Dark);

    assert_theme_save(&mut app, &path, "light", ThemeMode::Light);
    let profile = ThemeProfile::apply_with_accessibility(
        &ctx,
        ThemeMode::resolve(&app.settings_theme),
        false,
        false,
        false,
        1.0,
    );
    assert_eq!(profile.requested, ThemeMode::Light);
    assert_eq!(profile.effective, EffectiveTheme::Light);

    assert_theme_save(&mut app, &path, "system", ThemeMode::System);
}

#[test]
fn settings_appearance_exposes_reduce_motion_toggle() {
    let settings = include_str!("views/settings.rs");
    let model = include_str!("views/settings_model.rs");

    assert!(settings.contains("studio.settings.motion.reduce"));
    assert!(settings.contains("studio.settings.motion.reduce.detail"));
    assert!(settings.contains("self.settings_reduce_motion"));
    assert!(settings.contains("settings_toggle::toggle_row"));
    assert!(settings.contains("self.save_setting("));
    assert!(settings.contains("\"appearance.reduce_motion\""));
    assert!(!settings.contains("ui.checkbox("));
    assert!(model.contains("motion_control: \"token-toggle-row\""));
}

#[test]
fn settings_appearance_exposes_high_contrast_toggle() {
    let settings = include_str!("views/settings.rs");
    let model = include_str!("views/settings_model.rs");

    assert!(settings.contains("studio.settings.contrast.high"));
    assert!(settings.contains("studio.settings.contrast.high.detail"));
    assert!(settings.contains("self.settings_high_contrast"));
    assert!(settings.contains("\"appearance.high_contrast\""));
    assert!(model.contains("contrast_control: \"token-toggle-row\""));
}

#[test]
fn settings_appearance_exposes_reduce_transparency_toggle() {
    let settings = include_str!("views/settings.rs");
    let model = include_str!("views/settings_model.rs");

    assert!(settings.contains("studio.settings.transparency.reduce"));
    assert!(settings.contains("studio.settings.transparency.reduce.detail"));
    assert!(settings.contains("self.settings_reduce_transparency"));
    assert!(settings.contains("\"appearance.reduce_transparency\""));
    assert!(model.contains("transparency_control: \"token-toggle-row\""));
}

#[test]
fn settings_appearance_exposes_ui_zoom_control() {
    let settings = include_str!("views/settings.rs");
    let rows = include_str!("views/settings_rows.rs");
    let model = include_str!("views/settings_model.rs");

    assert!(settings.contains("studio.settings.zoom.label"));
    assert!(settings.contains("settings_rows::ui_scale_control"));
    assert!(settings.contains("\"appearance.ui_scale\""));
    assert!(rows.contains("[\"0.85\", \"1.00\", \"1.25\", \"1.50\"]"));
    assert!(model.contains("zoom_control: \"segmented-control\""));
}

#[test]
fn settings_ai_provider_uses_token_toggle_row() {
    let settings = include_str!("views/settings.rs");
    let toggle = include_str!("views/settings_toggle.rs");

    assert!(settings.contains("settings_toggle::toggle_row"));
    assert!(settings.contains("self.save_setting(\"enable_ai\""));
    assert!(!settings.contains("ui.checkbox("));
    assert!(toggle.contains("paint_toggle"));
    assert!(toggle.contains("ToggleRowEvent::Toggle"));
    assert!(toggle.contains("WidgetType::Checkbox"));
}

#[test]
fn settings_storage_uses_token_path_row() {
    let settings = include_str!("views/settings.rs");
    let binding = include_str!("views/settings_binding.rs");

    assert!(settings.contains("settings_binding::binding_editor_row"));
    assert!(settings.contains("self.save_setting(\"data_dir\""));
    assert!(!settings.contains("ui.text_edit_singleline(&mut self.settings_data_dir)"));
    assert!(binding.contains("save_label"));
    assert!(binding.contains("WidgetType::TextEdit"));
    assert!(binding.contains("row_paint::paint_row_frame"));
    assert!(binding.contains("Color::accent_weak"));
}

fn assert_theme_save(
    app: &mut StudioEguiApp,
    path: &std::path::Path,
    value: &str,
    expected_mode: ThemeMode,
) {
    app.app.save_config_field_to(path, "theme", value).unwrap();
    assert_eq!(app.app.core.config.theme, value);
    app.sync_settings_from_app();
    assert_eq!(app.settings_theme, value);
    assert_eq!(ThemeMode::resolve(&app.settings_theme), expected_mode);
}

fn test_app() -> StudioEguiApp {
    let mut app = StudioEguiApp::default();
    let temp = tempfile::tempdir().unwrap();
    app.app = StudioApp::with_core(StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    }));
    app.sync_settings_from_app();
    app
}
