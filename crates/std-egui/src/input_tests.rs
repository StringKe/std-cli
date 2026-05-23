use super::*;

#[test]
fn keybinding_labels_use_platform_primary_modifier() {
    let label = launcher_action_panel().label();

    if cfg!(target_os = "macos") {
        assert_eq!(label, "⌘+K");
    } else {
        assert_eq!(label, "Ctrl+K");
    }
}

#[test]
fn named_key_labels_follow_platform_conventions() {
    assert_eq!(arrow_up().label(), "↑");
    assert_eq!(arrow_down().label(), "↓");
    assert_eq!(escape().label(), "Esc");
    if cfg!(target_os = "macos") {
        assert_eq!(enter().label(), "↵");
        assert_eq!(tab().label(), "⇥");
        assert_eq!(launcher_defer().label(), "⌘+⇧+↵");
        assert_eq!(launcher_delete_previous_token().label(), "⌘+⌫");
    } else {
        assert_eq!(enter().label(), "Enter");
        assert_eq!(tab().label(), "Tab");
        assert_eq!(launcher_defer().label(), "Ctrl+Shift+Enter");
        assert_eq!(launcher_delete_previous_token().label(), "Ctrl+Backspace");
    }
}

#[test]
fn studio_palette_binding_matches_docs() {
    assert!(studio_command_palette().label().ends_with("+P"));
    assert!(studio_command_palette_slash().label().ends_with("+/"));
    assert!(studio_new_workflow().label().ends_with("+N"));
    assert!(studio_zoom_reset().label().ends_with("+0"));
    assert!(studio_zoom_in().label().ends_with("+="));
    assert!(studio_zoom_out().label().ends_with("+-"));
    assert!(studio_quick_open().label().ends_with("+P"));
    assert!(studio_settings().label().ends_with("+,"));
    assert!(studio_analysis_relation_toggle().label().ends_with("+L"));
    assert_eq!(studio_analysis_qa_focus().label(), "?");
    assert_studio_workflow_bindings();
    assert_launcher_bindings();
}

fn assert_studio_workflow_bindings() {
    let alt = alt_modifier_label();
    let enter = named_key_label(egui::Key::Enter);
    let shift = shift_modifier_label();
    let up = named_key_label(egui::Key::ArrowUp);
    let down = named_key_label(egui::Key::ArrowDown);
    assert_eq!(
        studio_workflow_step_move_up().label(),
        format!("{alt}+{up}")
    );
    assert_eq!(
        studio_workflow_step_move_down().label(),
        format!("{alt}+{down}")
    );
    assert!(studio_workflow_test()
        .label()
        .ends_with(&format!("+{enter}")));
    assert!(studio_workflow_simulate()
        .label()
        .ends_with(&format!("+{shift}+{enter}")));
    assert!(studio_workflow_save().label().ends_with("+S"));
    assert!(studio_workflow_history()
        .label()
        .ends_with(&format!("+{shift}+H")));
    assert!(studio_previous_workspace_pane()
        .label()
        .ends_with(&format!("+{shift}+{up}")));
    assert!(studio_next_workspace_pane()
        .label()
        .ends_with(&format!("+{shift}+{down}")));
    assert!(studio_previous_bottom_panel_tab()
        .label()
        .ends_with("+Left"));
    assert!(studio_next_bottom_panel_tab().label().ends_with("+Right"));
    assert!(studio_close_tab().label().ends_with("+W"));
}

fn assert_launcher_bindings() {
    let backspace = named_key_label(egui::Key::Backspace);
    let enter_label = named_key_label(egui::Key::Enter);
    let shift = shift_modifier_label();
    let tab_label = named_key_label(egui::Key::Tab);
    assert!(launcher_delete_previous_token()
        .label()
        .ends_with(&format!("+{backspace}")));
    assert_eq!(enter().label(), enter_label);
    assert_eq!(
        launcher_defer().label(),
        format!("{}+{shift}+{enter_label}", primary_modifier_label())
    );
    assert_eq!(KeyBinding::Ctrl('C').label(), "Ctrl+C");
    assert_eq!(launcher_cancel().label(), "Ctrl+C");
    assert!(launcher_open_studio().label().ends_with("+O"));
    assert!(launcher_copy_command().label().ends_with("+C"));
    assert!(launcher_result_keycap(0).unwrap().ends_with("+1"));
    assert!(launcher_result_keycap(8).unwrap().ends_with("+9"));
    if cfg!(target_os = "macos") {
        assert_eq!(launcher_result_keycap(0).unwrap(), "⌘+1");
    }
    assert!(launcher_result_keycap(9).is_none());
    assert_eq!(tab().label(), tab_label);
    assert_eq!(shift_tab().label(), format!("{shift}+{tab_label}"));
}

#[test]
fn ime_guard_api_is_available_to_ui_surfaces() {
    let ctx = egui::Context::default();

    assert!(!ime_composing(&ctx));
    assert!(ime_action_guard(&ctx).action_allowed);
    assert!(!ime_action_guard(&ctx).blocks_actions());
    assert_eq!(ime_action_guard(&ctx).frame_event, None);
}

#[test]
fn ime_frame_event_exposes_current_preedit_or_commit() {
    let ctx = egui::Context::default();

    let _ = ctx.run(
        egui::RawInput {
            events: vec![egui::Event::Ime(egui::ImeEvent::Preedit(
                "zhong".to_string(),
            ))],
            ..Default::default()
        },
        |ctx| {
            assert_eq!(
                ime_frame_event(ctx),
                Some(egui::ImeEvent::Preedit("zhong".to_string()))
            );
        },
    );

    let _ = ctx.run(
        egui::RawInput {
            events: vec![egui::Event::Ime(egui::ImeEvent::Commit("中".to_string()))],
            ..Default::default()
        },
        |ctx| {
            assert_eq!(
                ime_frame_event(ctx),
                Some(egui::ImeEvent::Commit("中".to_string()))
            );
        },
    );
}

#[test]
fn ime_composing_persists_until_commit_or_disabled() {
    let ctx = egui::Context::default();

    run_with_ime_event(&ctx, egui::ImeEvent::Enabled);
    assert!(!ime_composing(&ctx));

    run_with_ime_event(&ctx, egui::ImeEvent::Preedit("zhong".to_string()));
    assert!(ime_composing(&ctx));

    let _ = ctx.run(Default::default(), |_| {});
    assert!(ime_composing(&ctx));

    run_with_ime_event(&ctx, egui::ImeEvent::Commit("中".to_string()));
    assert!(!ime_composing(&ctx));

    run_with_ime_event(&ctx, egui::ImeEvent::Preedit("zhong".to_string()));
    assert!(ime_composing(&ctx));

    run_with_ime_event(&ctx, egui::ImeEvent::Disabled);
    assert!(!ime_composing(&ctx));
}

#[test]
fn ime_action_guard_blocks_action_keys_until_commit() {
    let ctx = egui::Context::default();

    let _ = ctx.run(
        egui::RawInput {
            events: vec![
                egui::Event::Ime(egui::ImeEvent::Preedit("zhong".to_string())),
                egui::Event::Key {
                    key: egui::Key::Enter,
                    physical_key: Some(egui::Key::Enter),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            ..Default::default()
        },
        |ctx| {
            let guard = ime_action_guard(ctx);
            assert!(guard.composing);
            assert!(guard.blocks_actions());
            assert_eq!(guard.frame_event.as_deref(), Some("preedit:zhong"));
            assert_eq!(
                guard.contract,
                "ime-action-guard=preedit-blocks-enter-escape-arrows-shortcuts;commit-restores-actions"
            );
        },
    );

    let _ = ctx.run(
        egui::RawInput {
            events: vec![egui::Event::Ime(egui::ImeEvent::Commit("中".to_string()))],
            ..Default::default()
        },
        |ctx| {
            let guard = ime_action_guard(ctx);
            assert!(!guard.composing);
            assert!(guard.action_allowed);
            assert_eq!(guard.frame_event.as_deref(), Some("commit:中"));
        },
    );
}

fn run_with_ime_event(ctx: &egui::Context, event: egui::ImeEvent) {
    let _ = ctx.run(
        egui::RawInput {
            events: vec![egui::Event::Ime(event)],
            ..Default::default()
        },
        |_| {},
    );
}
