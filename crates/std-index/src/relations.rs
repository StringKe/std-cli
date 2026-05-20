use crate::{
    workflow::{
        json_workflow_relations, json_workflow_symbols, markdown_workflow_relations,
        markdown_workflow_symbols,
    },
    ComponentDigest, SymbolRelation,
};
use std::{fs, path::Path};

pub(crate) fn infer_relations(components: &[ComponentDigest]) -> Vec<SymbolRelation> {
    let mut relations = Vec::new();
    for component in components {
        if let Some(name) = component.path.file_name().and_then(|name| name.to_str()) {
            relations.push(SymbolRelation {
                symbol: name.to_string(),
                relation: "contains".to_string(),
                target: component.path.display().to_string(),
            });
        }
        relations.extend(extract_symbol_relations(component));
    }
    relations
}

pub(crate) fn extract_component_symbols(path: &Path, snippet: &str) -> Vec<String> {
    let Some(ext) = path.extension().and_then(|ext| ext.to_str()) else {
        return Vec::new();
    };
    match ext {
        "rs" => extract_rust_symbol_names(snippet),
        "md" => markdown_workflow_symbols(snippet),
        "json" => json_workflow_symbols(snippet),
        "toml" => extract_toml_symbol_names(snippet),
        "plist" => extract_plist_symbol_names(snippet),
        _ => Vec::new(),
    }
}

pub(crate) fn infer_component_purpose(name: &str, snippet: &str) -> String {
    let lower_name = name.to_lowercase();
    let lower_snippet = snippet.to_lowercase();
    if lower_name == "cargo.toml" {
        "rust package manifest".to_string()
    } else if lower_name.ends_with(".rs") && lower_snippet.contains("fn main") {
        "rust binary entrypoint".to_string()
    } else if lower_name.ends_with(".rs") {
        "rust source module".to_string()
    } else if lower_name == "workflow.md" || lower_name.ends_with(".workflow.json") {
        "workflow definition".to_string()
    } else if lower_name.ends_with(".md") {
        "markdown documentation".to_string()
    } else if lower_name.ends_with(".json") {
        "json configuration".to_string()
    } else if lower_name == "info.plist" {
        "mac app bundle metadata".to_string()
    } else if lower_name.ends_with(".plist") {
        "plist configuration".to_string()
    } else if lower_name.ends_with(".toml") {
        "toml configuration".to_string()
    } else {
        "file".to_string()
    }
}

fn extract_symbol_relations(component: &ComponentDigest) -> Vec<SymbolRelation> {
    let Ok(body) = fs::read_to_string(&component.path) else {
        return Vec::new();
    };
    let Some(ext) = component.path.extension().and_then(|ext| ext.to_str()) else {
        return Vec::new();
    };
    match ext {
        "rs" => extract_rust_relations(&component.path, &body),
        "md" => markdown_workflow_relations(&component.path, &body),
        "json" => json_workflow_relations(&component.path, &body),
        "toml" => extract_toml_relations(&component.path, &body),
        "plist" => extract_plist_relations(&component.path, &body),
        _ => Vec::new(),
    }
}

fn extract_rust_relations(path: &Path, body: &str) -> Vec<SymbolRelation> {
    let mut relations = Vec::new();
    let mut current_fn: Option<String> = None;
    for line in body.lines().map(str::trim) {
        for prefix in ["fn ", "pub fn ", "async fn ", "pub async fn "] {
            if let Some(name) = symbol_after_prefix(line, prefix) {
                relations.push(symbol_relation(name, "defines_fn", path));
                current_fn = Some(name.to_string());
            }
        }
        for prefix in [
            "struct ",
            "pub struct ",
            "enum ",
            "pub enum ",
            "trait ",
            "pub trait ",
        ] {
            if let Some(name) = symbol_after_prefix(line, prefix) {
                relations.push(symbol_relation(name, "defines_type", path));
            }
        }
        if let Some(name) = extract_impl_target(line) {
            relations.push(symbol_relation(name, "implements_type", path));
        }
        if let Some(target) = line.strip_prefix("use ") {
            relations.push(symbol_relation(
                target.trim_end_matches(';').trim(),
                "uses",
                path,
            ));
        }
        if let Some(target) = line.strip_prefix("mod ") {
            relations.push(symbol_relation(
                target.trim_end_matches(';').trim(),
                "declares_module",
                path,
            ));
        }
        if let Some(caller) = current_fn.as_deref() {
            relations.extend(extract_call_relations(path, caller, line));
        }
    }
    relations
}

fn extract_rust_symbol_names(body: &str) -> Vec<String> {
    let mut symbols = Vec::new();
    for line in body.lines().map(str::trim) {
        for prefix in ["fn ", "pub fn ", "async fn ", "pub async fn "] {
            if let Some(name) = symbol_after_prefix(line, prefix) {
                symbols.push(format!("fn {name}"));
            }
        }
        for prefix in [
            "struct ",
            "pub struct ",
            "enum ",
            "pub enum ",
            "trait ",
            "pub trait ",
        ] {
            if let Some(name) = symbol_after_prefix(line, prefix) {
                symbols.push(format!("type {name}"));
            }
        }
        if let Some(name) = extract_impl_target(line) {
            symbols.push(format!("impl {name}"));
        }
    }
    symbols.truncate(12);
    symbols
}

fn extract_toml_relations(path: &Path, body: &str) -> Vec<SymbolRelation> {
    let mut relations = Vec::new();
    let mut section = String::new();
    for line in body.lines().map(str::trim) {
        if line.starts_with('[') && line.ends_with(']') {
            section = line.trim_matches(&['[', ']'][..]).to_string();
            relations.push(symbol_relation(&section, "defines_config_section", path));
            continue;
        }
        if section == "dependencies" || section == "dev-dependencies" {
            if let Some((name, _)) = line.split_once('=') {
                let name = name.trim();
                if !name.is_empty() {
                    relations.push(symbol_relation(name, "depends_on_crate", path));
                }
            }
        }
    }
    relations
}

fn extract_toml_symbol_names(body: &str) -> Vec<String> {
    body.lines()
        .map(str::trim)
        .filter(|line| line.starts_with('[') && line.ends_with(']'))
        .map(|line| format!("section {}", line.trim_matches(&['[', ']'][..])))
        .take(12)
        .collect()
}

fn extract_plist_relations(path: &Path, body: &str) -> Vec<SymbolRelation> {
    let mut relations = Vec::new();
    for (key, relation) in [
        ("CFBundleIdentifier", "app_bundle_identifier"),
        ("CFBundleExecutable", "app_bundle_executable"),
        ("CFBundleName", "app_bundle_name"),
        ("CFBundleDisplayName", "app_bundle_display_name"),
        ("CFBundleShortVersionString", "app_bundle_version"),
    ] {
        if let Some(value) = plist_string_value(body, key) {
            relations.push(symbol_relation(&value, relation, path));
        }
    }
    relations
}

fn extract_plist_symbol_names(body: &str) -> Vec<String> {
    [
        "CFBundleIdentifier",
        "CFBundleExecutable",
        "CFBundleName",
        "CFBundleDisplayName",
        "CFBundleShortVersionString",
    ]
    .into_iter()
    .filter_map(|key| plist_string_value(body, key).map(|value| format!("{key} {value}")))
    .collect()
}

fn plist_string_value(body: &str, key: &str) -> Option<String> {
    let (_, tail) = body.split_once(&format!("<key>{key}</key>"))?;
    let (_, tail) = tail.split_once("<string>")?;
    let (value, _) = tail.split_once("</string>")?;
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn symbol_relation(symbol: &str, relation: &str, path: &Path) -> SymbolRelation {
    SymbolRelation {
        symbol: symbol.to_string(),
        relation: relation.to_string(),
        target: path.display().to_string(),
    }
}

fn symbol_after_prefix<'a>(line: &'a str, prefix: &str) -> Option<&'a str> {
    let tail = line.strip_prefix(prefix)?;
    let end = tail
        .find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .unwrap_or(tail.len());
    if end == 0 {
        None
    } else {
        Some(&tail[..end])
    }
}

fn extract_impl_target(line: &str) -> Option<&str> {
    let tail = line.strip_prefix("impl ")?;
    let tail = tail.strip_prefix('<').map_or(tail, |generic_tail| {
        generic_tail
            .split_once('>')
            .map(|(_, rest)| rest.trim_start())
            .unwrap_or(generic_tail)
    });
    let before_body = tail.split_once('{').map(|(head, _)| head).unwrap_or(tail);
    let target = before_body
        .split_once(" for ")
        .map(|(_, type_name)| type_name)
        .unwrap_or(before_body)
        .trim();
    if target.is_empty() || target == "for" {
        None
    } else {
        Some(target)
    }
}

fn extract_call_relations(path: &Path, caller: &str, line: &str) -> Vec<SymbolRelation> {
    let ignored = [
        "if",
        "for",
        "while",
        "match",
        "return",
        "Some",
        "Ok",
        "Err",
        "format",
        "vec",
        "println",
        "assert",
        "assert_eq",
    ];
    let mut relations = Vec::new();
    for paren_index in line.match_indices('(').map(|(index, _)| index) {
        let name = call_name_before(line, paren_index);
        if name.is_empty()
            || name == caller
            || ignored.contains(&name.as_str())
            || name
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_uppercase())
        {
            continue;
        }
        relations.push(SymbolRelation {
            symbol: caller.to_string(),
            relation: "calls_fn".to_string(),
            target: format!("{}#{}", path.display(), name),
        });
    }
    relations
}

fn call_name_before(line: &str, paren_index: usize) -> String {
    let prefix = &line[..paren_index];
    let start = prefix
        .char_indices()
        .rev()
        .find(|(_, ch)| !(ch.is_ascii_alphanumeric() || *ch == '_' || *ch == ':' || *ch == '.'))
        .map(|(index, ch)| index + ch.len_utf8())
        .unwrap_or(0);
    prefix[start..]
        .rsplit([':', '.'])
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}
