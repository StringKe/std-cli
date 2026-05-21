use std::path::Path;

#[test]
fn studio_memory_toolbar_search_has_textbox_semantics() {
    let body = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("views")
            .join("memory.rs"),
    )
    .unwrap();

    assert!(body.contains("WidgetType::TextEdit"));
    assert!(body.contains("memory_query_a11y_label"));
    assert!(body.contains("Memory search, text box, value"));
    assert!(body.contains("query.trim().is_empty()"));
}

#[test]
fn studio_memory_writer_fields_have_textbox_semantics() {
    let body = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("views")
            .join("memory.rs"),
    )
    .unwrap();

    assert!(body.contains("memory_text_input"));
    assert!(body.contains("memory_body_input"));
    assert!(body.contains("memory_field_a11y_label"));
    assert!(body.contains("TextEdit::singleline"));
    assert!(body.contains("TextEdit::multiline"));
    assert!(body.contains("text box, value"));
    assert!(!body.contains("ui.text_edit_singleline"));
}
