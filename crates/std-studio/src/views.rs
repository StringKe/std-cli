mod dashboard;
mod history;
mod memory;
mod plugins;
mod settings;
mod workflow_builder;
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
