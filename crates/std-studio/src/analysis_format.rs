use std_index::{IndexAnswer, IndexCoverageReport, IndexInspection};

pub(crate) fn format_analysis_answer(answer: &IndexAnswer) -> String {
    let mut lines = vec![answer.answer.clone()];
    for source in &answer.sources {
        lines.push(format!(
            "source: {} {} {}",
            source.entity,
            source.path.display(),
            source.reason
        ));
        lines.extend(
            source
                .evidence
                .iter()
                .map(|evidence| format!("evidence: {evidence}")),
        );
    }
    lines.join("\n")
}

pub(crate) fn format_inspection(inspection: &IndexInspection) -> String {
    let mut lines = inspection_header(inspection);
    for component in &inspection.key_components {
        lines.push(format!(
            "component: {} [{}] {}",
            component.path.display(),
            component.language,
            component.purpose
        ));
    }
    for relation in &inspection.key_relations {
        lines.push(format!(
            "relation: {} {} {}",
            relation.symbol, relation.relation, relation.target
        ));
    }
    for history in &inspection.key_history {
        lines.push(format!("history: {} {}", history.source, history.summary));
    }
    lines.join("\n")
}

pub(crate) fn format_coverage_report(report: &IndexCoverageReport) -> String {
    let mut lines = vec![format!(
        "coverage: total={} complete={} incomplete={}",
        report.total, report.complete, report.incomplete
    )];
    for item in &report.items {
        lines.push(format!(
            "entity: {} complete={} components={} relations={} history={}",
            item.name,
            item.coverage.complete(),
            item.component_count,
            item.relation_count,
            item.history_count
        ));
    }
    lines.join("\n")
}

fn inspection_header(inspection: &IndexInspection) -> Vec<String> {
    vec![
        format!("entity: {}", inspection.overview.name),
        format!("kind: {:?}", inspection.overview.kind),
        format!("path: {}", inspection.overview.path.display()),
        format!("summary: {}", inspection.overview.summary),
        format!("components: {}", inspection.component_count),
        format!("relations: {}", inspection.relation_count),
        format!("history: {}", inspection.history_count),
        format!(
            "coverage: overview={} components={} relations={} history={} complete={}",
            inspection.coverage.entity_overview,
            inspection.coverage.component_digest,
            inspection.coverage.symbol_relation_index,
            inspection.coverage.historical_context,
            inspection.coverage.complete()
        ),
    ]
}
