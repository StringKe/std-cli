use super::*;
use std_core::{StdConfig, StdCore};
use std_types::ActionExecutionStatus;

#[test]
fn executing_search_bar_shows_running_action_text() {
    let mut state = LauncherState::new();
    state.update_query("index");
    state.view.preview_executing();
    let text = search_bar_text(&state);
    let a11y = AccessibilityContext::from_env();

    assert!(text.starts_with(i18n::t("launcher.search.running")));
    assert!(text.contains("Rebuild Index"));
    assert_eq!(
        search_a11y_label(&state, &a11y),
        i18n::t("launcher.a11y.running").replace("{action}", "Rebuild Index")
    );
    assert_eq!(
        search_placeholder(&state),
        i18n::t("launcher.action.executing")
    );
}

#[test]
fn search_focus_ring_is_tied_to_search_section() {
    let mut state = LauncherState::new();
    assert_eq!(state.focus_section, LauncherFocusSection::Search);
    assert!(state.keyboard_focus_visible(LauncherFocusSection::Search));

    state.focus_section = LauncherFocusSection::Results;

    assert_ne!(state.focus_section, LauncherFocusSection::Search);
    assert!(!state.keyboard_focus_visible(LauncherFocusSection::Search));
}

#[test]
fn search_input_releases_egui_focus_to_action_panel_and_feedback() {
    let mut state = LauncherState::new();
    assert!(search_input_owns_egui_focus(&state));

    state.update_query("index");
    state.open_action_panel();

    assert!(!search_input_owns_egui_focus(&state));

    state.close_action_panel();
    state.focus_section = LauncherFocusSection::Feedback;

    assert!(!search_input_owns_egui_focus(&state));
}

#[test]
fn search_input_releases_egui_focus_to_voice_input() {
    let mut state = LauncherState::new();
    state.start_voice_input();

    assert!(!search_input_owns_egui_focus(&state));
}

#[test]
fn search_shortcuts_pause_while_voice_input_is_active() {
    let source = include_str!("ui_search.rs");
    let production_source = source.split("#[cfg(test)]").next().unwrap();

    assert!(production_source.contains("if !executing && !state.controller.voice_active"));
    assert!(production_source.contains("ui_keyboard::handle_search_shortcuts"));
}

#[test]
fn search_input_focus_request_is_conditional() {
    let source = include_str!("ui_search.rs");
    let production_source = source.split("#[cfg(test)]").next().unwrap();
    let request_branch = production_source
        .split("if search_input_owns_egui_focus(state)")
        .nth(1)
        .and_then(|body| {
            body.split("let a11y = AccessibilityContext::from_env();")
                .next()
        })
        .unwrap();

    assert!(request_branch.contains("response.request_focus();"));
    assert!(production_source.contains("response.changed()"));
    assert!(production_source.contains("search_input_owns_egui_focus(state)"));
}

#[test]
fn launcher_search_mode_tag_tracks_query_prefix() {
    let mut state = LauncherState::new();

    assert_eq!(search_mode_tag_label(&state), None);

    state.update_query("? rebuild");

    assert_eq!(search_mode_tag_label(&state), Some("Ask"));
}

#[test]
fn collapsed_launcher_does_not_nest_search_surface_inside_panel() {
    let source = include_str!("ui_search.rs");
    let collapsed_branch = source
        .split("if collapsed")
        .nth(1)
        .and_then(|body| body.split("let ctx = ui.ctx().clone();").next())
        .unwrap();

    assert!(collapsed_branch.contains("render_search_bar_contents"));
    assert!(!collapsed_branch.contains("egui::Frame::new()"));
    assert!(source.contains("fn render_search_bar_contents"));
}

#[test]
fn search_indicator_tracks_loading_and_executing_phases() {
    assert_eq!(
        search_indicator_for_state(LauncherPhase::Empty, LauncherLoadingState::Idle),
        SearchIndicator::Search
    );
    assert_eq!(
        search_indicator_for_state(
            LauncherPhase::Searching,
            LauncherLoadingState::UpdatingResults
        ),
        SearchIndicator::Search
    );
    assert_eq!(
        search_indicator_for_state(
            LauncherPhase::Searching,
            LauncherLoadingState::SlowEmptyResults
        ),
        SearchIndicator::Loading
    );
    assert_eq!(
        search_indicator_for_state(LauncherPhase::Executing, LauncherLoadingState::Idle),
        SearchIndicator::Executing
    );
}

#[test]
fn search_loading_and_executing_indicators_expose_status_semantics() {
    let source = include_str!("ui_search.rs");

    assert!(source.contains("WidgetType::ProgressIndicator"));
    assert!(source.contains("launcher.search.loading"));
    assert!(source.contains("launcher.results.executing.title"));
    assert!(source.contains("Color::accent_weak"));
    assert!(source.contains("circle_stroke"));
}

#[test]
fn search_ui_contract_requires_visible_ime_state() {
    assert_eq!(
        search_ime_visible_state_contract(),
        "ime-visible-state=search-preedit-visible,preedit-not-query,commit-clears-preedit,enter-owned-by-ime"
    );
    assert!(
        search_input_width(420.0, true) < search_input_width(420.0, false),
        "IME state chip must reserve stable width in the search row"
    );
    assert_eq!(
        ime_composing_label(None),
        i18n::t("launcher.search.ime_composing")
    );
    assert_eq!(
        ime_composing_label(Some("zhong")),
        format!("{} zhong", i18n::t("launcher.search.ime_composing"))
    );
}

#[test]
fn search_input_uses_outer_token_surface_not_inner_textedit_frame() {
    let source = include_str!("ui_search.rs");
    let input_body = source
        .split("egui::TextEdit::singleline")
        .nth(1)
        .and_then(|body| body.split(".interactive(!executing)").next())
        .unwrap();

    assert!(input_body.contains(".frame(false)"));
    assert!(source.contains("Color::bg_surface_1"));
    assert!(source.contains("Color::stroke_border"));
}

#[test]
fn search_ime_focus_contract_uses_visible_chip_and_keeps_query_stable() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let before = state.view.query.clone();

    state.handle_ime_preedit("zhong");

    assert_eq!(state.view.query, before);
    assert_eq!(state.ime_preedit.as_deref(), Some("zhong"));
    assert_eq!(state.focus_section, LauncherFocusSection::Search);
    assert!(state.keyboard_focus_visible(LauncherFocusSection::Search));
    assert!(search_input_owns_egui_focus(&state));
    assert!(search_input_width(560.0, true) < search_input_width(560.0, false));

    state.handle_ime_commit("重建索引");

    assert!(state.ime_preedit.is_none());
    assert_eq!(state.view.query, "重建索引");
}

#[test]
fn search_render_syncs_ime_events_into_launcher_state() {
    let ctx = egui::Context::default();
    let mut state = LauncherState::new();
    state.update_query("index");
    let mut hide_requested = false;

    let _ = ctx.run(ime_preedit_input("zhong"), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            render_search_bar(ui, &mut state, false, &mut hide_requested);
        });
    });

    assert_eq!(state.view.query, "index");
    assert_eq!(state.ime_preedit.as_deref(), Some("zhong"));
    assert_eq!(state.focus_section, LauncherFocusSection::Search);
    assert!(!hide_requested);

    let _ = ctx.run(ime_commit_input("重建索引"), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            render_search_bar(ui, &mut state, false, &mut hide_requested);
        });
    });

    assert!(state.ime_preedit.is_none());
    assert_eq!(state.view.query, "重建索引");
}

#[test]
fn search_render_enter_executes_selected_app_through_user_route() {
    let ctx = egui::Context::default();
    let mut state = launcher_with_fixture_app();
    state.controller.show();
    state.update_query("Search Render Enter App");
    let mut hide_requested = false;

    let _ = ctx.run(enter_input(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            render_search_bar(ui, &mut state, false, &mut hide_requested);
        });
    });

    let execution = state
        .view
        .last_execution
        .as_ref()
        .expect("Enter in rendered search bar must execute selected fixture app");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(
        execution
            .output
            .as_ref()
            .and_then(|output| output.get("reason"))
            .and_then(|value| value.as_str()),
        Some("STD_TEST_MODE blocked desktop open")
    );
    assert!(state.view.feedback.is_some());
    assert!(!hide_requested);
    assert!(state.controller.visible);
}

fn ime_preedit_input(preedit: &str) -> egui::RawInput {
    egui::RawInput {
        events: vec![egui::Event::Ime(egui::ImeEvent::Preedit(
            preedit.to_string(),
        ))],
        ..Default::default()
    }
}

fn ime_commit_input(committed: &str) -> egui::RawInput {
    egui::RawInput {
        events: vec![egui::Event::Ime(egui::ImeEvent::Commit(
            committed.to_string(),
        ))],
        ..Default::default()
    }
}

fn enter_input() -> egui::RawInput {
    egui::RawInput {
        events: vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: Some(egui::Key::Enter),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        ..Default::default()
    }
}

fn launcher_with_fixture_app() -> LauncherState {
    let root = std::env::temp_dir().join(format!(
        "std-launcher-search-render-enter-fixture-{}",
        std::process::id()
    ));
    let config = StdConfig {
        data_dir: root.join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("SearchRenderEnterApp.app");
    let contents = app.join("Contents");
    let _ = std::fs::create_dir_all(&contents);
    let _ = std::fs::write(
        contents.join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>Search Render Enter App</string>
<key>CFBundleName</key><string>SearchRenderEnterApp</string>
</dict></plist>"#,
    );
    LauncherState::with_core(StdCore::with_config(config))
}

fn search_mode_tag_label(state: &LauncherState) -> Option<&'static str> {
    let mode = LauncherQueryMode::from_query(&state.view.query);
    (mode != LauncherQueryMode::All).then_some(mode.tag_label())
}
