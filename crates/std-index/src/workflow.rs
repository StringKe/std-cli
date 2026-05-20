use crate::SymbolRelation;
use std::path::Path;

pub(crate) fn markdown_workflow_relations(path: &Path, body: &str) -> Vec<SymbolRelation> {
    let mut relations = Vec::new();
    for line in body.lines().map(str::trim) {
        if let Some(name) = line.strip_prefix("name:") {
            relations.push(symbol_relation(
                trim_frontmatter_scalar(name),
                "workflow_name",
                path,
            ));
        }
        if let Some(description) = line.strip_prefix("description:") {
            relations.push(symbol_relation(
                trim_frontmatter_scalar(description),
                "workflow_description",
                path,
            ));
        }
        if let Some(steps_json) = line.strip_prefix("steps_json:") {
            let steps_json = trim_frontmatter_scalar(steps_json).replace("\\'", "'");
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&steps_json) {
                relations.extend(workflow_step_relations(path, &value));
            }
        }
    }
    relations
}

pub(crate) fn markdown_workflow_symbols(body: &str) -> Vec<String> {
    let mut symbols = Vec::new();
    for line in body.lines().map(str::trim) {
        if let Some(name) = line.strip_prefix("name:") {
            symbols.push(format!("workflow {}", trim_frontmatter_scalar(name)));
        }
        if let Some(steps_json) = line.strip_prefix("steps_json:") {
            let steps_json = trim_frontmatter_scalar(steps_json).replace("\\'", "'");
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&steps_json) {
                symbols.extend(workflow_step_symbols(&value));
            }
        }
    }
    symbols.truncate(12);
    symbols
}

pub(crate) fn json_workflow_relations(path: &Path, body: &str) -> Vec<SymbolRelation> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(body) else {
        return Vec::new();
    };
    let mut relations = Vec::new();
    if let Some(name) = value.get("name").and_then(|name| name.as_str()) {
        relations.push(symbol_relation(name, "workflow_name", path));
    }
    if let Some(description) = value.get("description").and_then(|value| value.as_str()) {
        relations.push(symbol_relation(description, "workflow_description", path));
    }
    if let Some(steps) = value.get("steps") {
        relations.extend(workflow_step_relations(path, steps));
    }
    relations
}

pub(crate) fn json_workflow_symbols(body: &str) -> Vec<String> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(body) else {
        return Vec::new();
    };
    let mut symbols = Vec::new();
    if let Some(name) = value.get("name").and_then(|name| name.as_str()) {
        symbols.push(format!("workflow {name}"));
    }
    if let Some(steps) = value.get("steps") {
        symbols.extend(workflow_step_symbols(steps));
    }
    symbols.truncate(12);
    symbols
}

fn workflow_step_relations(path: &Path, steps: &serde_json::Value) -> Vec<SymbolRelation> {
    let mut relations = Vec::new();
    for step in steps.as_array().into_iter().flatten() {
        let Some(name) = step.get("name").and_then(|value| value.as_str()) else {
            continue;
        };
        relations.push(symbol_relation(name, "defines_workflow_step", path));
        if let Some(step_type) = step.get("step_type").and_then(|value| value.as_str()) {
            relations.push(SymbolRelation {
                symbol: name.to_string(),
                relation: "workflow_step_type".to_string(),
                target: step_type.to_string(),
            });
        }
        if let Some(action_id) = step.get("action_id").and_then(|value| value.as_str()) {
            relations.push(SymbolRelation {
                symbol: name.to_string(),
                relation: "workflow_step_action_id".to_string(),
                target: action_id.to_string(),
            });
        }
        relations.extend(parameter_target_relations(
            path,
            name,
            step.get("parameters"),
        ));
    }
    relations
}

fn parameter_target_relations(
    path: &Path,
    step_name: &str,
    parameters: Option<&serde_json::Value>,
) -> Vec<SymbolRelation> {
    let Some(serde_json::Value::Object(parameters)) = parameters else {
        return Vec::new();
    };
    let mut relations = Vec::new();
    for key in ["action", "action_name", "target", "workflow", "command"] {
        if let Some(value) = parameters.get(key).and_then(|value| value.as_str()) {
            relations.push(SymbolRelation {
                symbol: step_name.to_string(),
                relation: format!("workflow_step_parameter_{key}"),
                target: value.to_string(),
            });
        }
    }
    if relations.is_empty() && !parameters.is_empty() {
        relations.push(SymbolRelation {
            symbol: step_name.to_string(),
            relation: "workflow_step_parameters".to_string(),
            target: path.display().to_string(),
        });
    }
    relations
}

fn workflow_step_symbols(steps: &serde_json::Value) -> Vec<String> {
    steps
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|step| step.get("name").and_then(|name| name.as_str()))
        .map(|name| format!("step {name}"))
        .collect()
}

fn symbol_relation(symbol: &str, relation: &str, path: &Path) -> SymbolRelation {
    SymbolRelation {
        symbol: symbol.to_string(),
        relation: relation.to_string(),
        target: path.display().to_string(),
    }
}

fn trim_frontmatter_scalar(value: &str) -> &str {
    value.trim().trim_matches('"').trim_matches('\'').trim()
}
