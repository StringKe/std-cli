mod dashboard;
mod dashboard_rows;
mod history;
mod history_rows;
pub(crate) mod history_timeline;
mod memory;
mod memory_rows;
mod plugin_rows;
mod plugin_status_bar;
mod plugins;
pub(crate) mod row_metrics;
pub(crate) mod row_paint;
mod settings;
pub(crate) mod settings_model;
mod settings_rows;
mod workflow_builder;
mod workflow_builder_actions;
pub(crate) mod workflow_builder_ai;
mod workflow_builder_metrics;
mod workflow_builder_properties;
mod workflow_builder_status;
mod workflow_builder_toolbar;
pub(crate) mod workflow_builder_trace;
mod workflow_rows;
mod workflows;

pub(crate) fn schema_label(schema: Option<&serde_json::Value>) -> String {
    match schema {
        Some(serde_json::Value::Object(object)) => object
            .get("title")
            .and_then(|value| value.as_str())
            .map(ToString::to_string)
            .unwrap_or_else(|| format!("object({})", object.len())),
        Some(value) => value.to_string(),
        None => "none".to_string(),
    }
}
