use crate::CoreError;
use std::collections::HashMap;
use std_types::{ActionId, RegistryEntry, SearchResult};
use tracing::info;

#[derive(Clone, Default)]
pub struct ActionRegistry {
    actions: HashMap<ActionId, RegistryEntry>,
    name_index: HashMap<String, ActionId>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, entry: RegistryEntry) -> Result<(), CoreError> {
        let normalized_name = normalize_name(&entry.action.name);
        if self.name_index.contains_key(&normalized_name) {
            return Err(CoreError::DuplicateAction(entry.action.name.clone()));
        }

        let id = entry.action.id;
        self.name_index.insert(normalized_name, id);
        self.actions.insert(id, entry);
        info!("Registered action: {}", id);
        Ok(())
    }

    pub fn get(&self, id: ActionId) -> Option<&RegistryEntry> {
        self.actions.get(&id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&RegistryEntry> {
        self.name_index
            .get(&normalize_name(name))
            .and_then(|id| self.actions.get(id))
    }

    pub fn entries(&self) -> Vec<RegistryEntry> {
        let mut entries: Vec<_> = self.actions.values().cloned().collect();
        entries.sort_by(|a, b| a.action.name.cmp(&b.action.name));
        entries
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return self.empty_query_results(limit);
        }

        let mut results: Vec<SearchResult> = self
            .actions
            .values()
            .filter_map(|entry| search_entry(entry, &query))
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(limit);
        results
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    fn empty_query_results(&self, limit: usize) -> Vec<SearchResult> {
        self.entries()
            .into_iter()
            .take(limit)
            .map(|entry| SearchResult {
                action: entry.action,
                score: 0.5,
                matched_fields: vec![],
            })
            .collect()
    }
}

fn search_entry(entry: &RegistryEntry, query: &str) -> Option<SearchResult> {
    let mut score = 0.0;
    let mut matched_fields = Vec::new();
    score += score_text_field(
        &entry.action.name,
        query,
        "name",
        10.0,
        1.0,
        &mut matched_fields,
    );
    score += score_text_field(
        &entry.action.description,
        query,
        "description",
        4.0,
        0.4,
        &mut matched_fields,
    );
    score += score_text_field(
        &entry.action.when_to_use,
        query,
        "when_to_use",
        2.0,
        0.2,
        &mut matched_fields,
    );
    score += score_tags(&entry.tags, query, &mut matched_fields);

    (score > 0.0).then(|| SearchResult {
        action: entry.action.clone(),
        score,
        matched_fields,
    })
}

fn score_text_field(
    value: &str,
    query: &str,
    field: &str,
    exact_score: f32,
    fuzzy_multiplier: f32,
    matched_fields: &mut Vec<String>,
) -> f32 {
    let value = value.to_lowercase();
    if value.contains(query) {
        matched_fields.push(field.to_string());
        exact_score
    } else if let Some(fuzzy) = fuzzy_score(&value, query) {
        matched_fields.push(format!("{field}:fuzzy"));
        fuzzy * fuzzy_multiplier
    } else {
        0.0
    }
}

fn score_tags(tags: &[String], query: &str, matched_fields: &mut Vec<String>) -> f32 {
    for tag in tags {
        let tag = tag.to_lowercase();
        if tag.contains(query) {
            matched_fields.push("tags".to_string());
            return 3.0;
        }
        if let Some(fuzzy) = fuzzy_score(&tag, query) {
            matched_fields.push("tags:fuzzy".to_string());
            return fuzzy * 0.3;
        }
    }
    0.0
}

fn normalize_name(name: &str) -> String {
    name.trim().to_lowercase()
}

fn fuzzy_score(haystack: &str, query: &str) -> Option<f32> {
    if query.is_empty() {
        return Some(0.0);
    }
    let mut query_chars = query.chars().filter(|ch| !ch.is_whitespace()).peekable();
    query_chars.peek()?;
    let mut last_match = None;
    let mut matched = 0_usize;
    let mut contiguous = 0_usize;
    let mut word_start = 0_usize;
    for (index, ch) in haystack.chars().enumerate() {
        let Some(&target) = query_chars.peek() else {
            break;
        };
        if ch.is_whitespace() || ch == '-' || ch == '_' || ch == '/' {
            word_start = index + 1;
            continue;
        }
        if ch == target {
            matched += 1;
            if last_match == Some(index.saturating_sub(1)) {
                contiguous += 1;
            }
            if index == word_start {
                contiguous += 1;
            }
            last_match = Some(index);
            query_chars.next();
        }
    }
    if query_chars.peek().is_some() {
        return None;
    }
    let searchable_len = haystack.chars().filter(|ch| !ch.is_whitespace()).count();
    let density = matched as f32 / searchable_len.max(1) as f32;
    Some(1.0 + density * 3.0 + contiguous as f32)
}
