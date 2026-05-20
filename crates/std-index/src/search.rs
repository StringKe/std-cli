use crate::{
    coverage::index_coverage, ComponentDigest, HistoricalContext, IndexAnswer, IndexAnswerSource,
    IndexDocument, IndexInspection, IndexSearchResult, SymbolRelation,
};

pub(crate) fn search_documents(
    documents: Vec<IndexDocument>,
    query: &str,
    limit: usize,
) -> Vec<IndexSearchResult> {
    let query = query.trim().to_lowercase();
    let mut results = documents
        .into_iter()
        .filter_map(|document| search_document(document, &query))
        .collect::<Vec<_>>();

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results.truncate(limit);
    results
}

fn search_document(document: IndexDocument, query: &str) -> Option<IndexSearchResult> {
    let mut score = 0.0;
    let mut matched_fields = Vec::new();
    let name = document.overview.name.to_lowercase();
    let path = document.overview.path.display().to_string().to_lowercase();
    let summary = document.overview.summary.to_lowercase();

    if query.is_empty() || name.contains(query) {
        score += 12.0;
        matched_fields.push("overview.name".to_string());
    }
    if !query.is_empty() && path.contains(query) {
        score += 8.0;
        matched_fields.push("overview.path".to_string());
    }
    if !query.is_empty() && summary.contains(query) {
        score += 6.0;
        matched_fields.push("overview.summary".to_string());
    }

    score += component_score(&document.components, query, &mut matched_fields);
    score += relation_score(&document.relations, query, &mut matched_fields);
    score += history_score(&document.history, query, &mut matched_fields);

    (score > 0.0).then_some(IndexSearchResult {
        document,
        score,
        matched_fields,
    })
}

fn component_score(
    components: &[ComponentDigest],
    query: &str,
    matched_fields: &mut Vec<String>,
) -> f32 {
    let matches = components
        .iter()
        .filter(|component| component_matches(component, query))
        .count();
    if !query.is_empty() && matches > 0 {
        matched_fields.push("components".to_string());
        matches as f32 * 4.0
    } else {
        0.0
    }
}

fn relation_score(
    relations: &[SymbolRelation],
    query: &str,
    matched_fields: &mut Vec<String>,
) -> f32 {
    let matches = relations
        .iter()
        .filter(|relation| relation_matches(relation, query))
        .count();
    if !query.is_empty() && matches > 0 {
        matched_fields.push("relations".to_string());
        matches as f32 * 5.0
    } else {
        0.0
    }
}

fn history_score(
    history_entries: &[HistoricalContext],
    query: &str,
    matched_fields: &mut Vec<String>,
) -> f32 {
    let matches = history_entries
        .iter()
        .filter(|history| {
            format!("{} {}", history.source, history.summary)
                .to_lowercase()
                .contains(query)
        })
        .count();
    if !query.is_empty() && matches > 0 {
        matched_fields.push("history".to_string());
        matches as f32 * 3.0
    } else {
        0.0
    }
}

pub(crate) fn build_index_answer(query: &str, results: Vec<IndexSearchResult>) -> IndexAnswer {
    if results.is_empty() {
        return IndexAnswer {
            query: query.to_string(),
            answer: "未找到匹配的索引文档。先运行 `std index rebuild <path>` 建立结构索引。"
                .to_string(),
            sources: Vec::new(),
        };
    }

    let mut lines = Vec::new();
    let mut sources = Vec::new();
    for result in results {
        let document = result.document;
        lines.push(answer_line(&document));
        let evidence = answer_evidence(&document, query, &result.matched_fields);
        sources.push(IndexAnswerSource {
            entity: document.overview.name,
            path: document.overview.path,
            reason: result.matched_fields.join(","),
            matched_fields: result.matched_fields,
            evidence,
        });
    }

    IndexAnswer {
        query: query.to_string(),
        answer: lines.join("\n"),
        sources,
    }
}

pub(crate) fn inspect_document(document: IndexDocument, limit: usize) -> IndexInspection {
    let limit = limit.max(1);
    let coverage = index_coverage(&document);
    IndexInspection {
        overview: document.overview,
        coverage,
        component_count: document.components.len(),
        relation_count: document.relations.len(),
        history_count: document.history.len(),
        key_components: document.components.into_iter().take(limit).collect(),
        key_relations: document.relations.into_iter().take(limit).collect(),
        key_history: document.history.into_iter().take(limit).collect(),
    }
}

fn answer_evidence(
    document: &IndexDocument,
    query: &str,
    matched_fields: &[String],
) -> Vec<String> {
    let query = query.trim().to_lowercase();
    let mut evidence = Vec::new();
    if matched_fields
        .iter()
        .any(|field| field.starts_with("overview"))
    {
        evidence.push(format!("overview: {}", document.overview.summary));
    }
    if matched_fields.iter().any(|field| field == "components") {
        evidence.extend(matching_components(&document.components, &query));
    }
    if matched_fields.iter().any(|field| field == "relations") {
        evidence.extend(matching_relations(&document.relations, &query));
    }
    if matched_fields.iter().any(|field| field == "history") {
        evidence.extend(matching_history(&document.history, &query));
    }
    evidence.truncate(8);
    evidence
}

fn matching_components(components: &[ComponentDigest], query: &str) -> Vec<String> {
    components
        .iter()
        .filter(|component| query.is_empty() || component_matches(component, query))
        .take(4)
        .map(|component| {
            format!(
                "component: {} ({}, {}, {} bytes, symbols: {})",
                component.path.display(),
                component.purpose,
                component.language,
                component.size_bytes,
                component.symbols.join(", ")
            )
        })
        .collect()
}

fn matching_relations(relations: &[SymbolRelation], query: &str) -> Vec<String> {
    relations
        .iter()
        .filter(|relation| query.is_empty() || relation_matches(relation, query))
        .take(4)
        .map(|relation| {
            format!(
                "relation: {} {} {}",
                relation.symbol, relation.relation, relation.target
            )
        })
        .collect()
}

fn matching_history(history_entries: &[HistoricalContext], query: &str) -> Vec<String> {
    history_entries
        .iter()
        .filter(|history| {
            query.is_empty()
                || format!("{} {}", history.source, history.summary)
                    .to_lowercase()
                    .contains(query)
        })
        .take(4)
        .map(|history| format!("history: {}: {}", history.source, history.summary))
        .collect()
}

fn answer_line(document: &IndexDocument) -> String {
    let key_components = document
        .components
        .iter()
        .take(5)
        .map(|component| format!("{} ({})", component.path.display(), component.purpose))
        .collect::<Vec<_>>()
        .join("; ");
    let key_relations = document
        .relations
        .iter()
        .take(5)
        .map(|relation| {
            format!(
                "{} {} {}",
                relation.symbol, relation.relation, relation.target
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    let key_history = document
        .history
        .iter()
        .take(5)
        .map(|history| format!("{}: {}", history.source, history.summary))
        .collect::<Vec<_>>()
        .join("; ");
    format!(
        "{}: {}. 关键组件: {}. 关键关系: {}. 历史上下文: {}.",
        document.overview.name,
        document.overview.summary,
        key_components,
        key_relations,
        key_history
    )
}

fn searchable_component_text(component: &ComponentDigest) -> String {
    format!(
        "{} {} {}",
        component.path.display(),
        component.purpose,
        component.size_bytes
    ) + &format!(
        " {} {} {}",
        component.language,
        component.snippet,
        component.symbols.join(" ")
    )
}

fn searchable_relation_text(relation: &SymbolRelation) -> String {
    format!(
        "{} {} {}",
        relation.symbol, relation.relation, relation.target
    )
}

fn component_matches(component: &ComponentDigest, query: &str) -> bool {
    searchable_component_text(component)
        .to_lowercase()
        .contains(query)
}

fn relation_matches(relation: &SymbolRelation, query: &str) -> bool {
    searchable_relation_text(relation)
        .to_lowercase()
        .contains(query)
}
