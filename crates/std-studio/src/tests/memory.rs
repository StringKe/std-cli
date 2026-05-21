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
