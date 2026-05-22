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
            coverage_status(item.coverage.complete()),
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
            coverage_status(inspection.coverage.entity_overview),
            coverage_status(inspection.coverage.component_digest),
            coverage_status(inspection.coverage.symbol_relation_index),
            coverage_status(inspection.coverage.historical_context),
            coverage_status(inspection.coverage.complete())
        ),
    ]
}

fn coverage_status(pass: bool) -> &'static str {
    if pass {
        "PASS"
    } else {
        "FAIL"
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use std_index::{
        EntityKind, EntityOverview, IndexCoverage, IndexCoverageItem, IndexCoverageReport,
        IndexInspection,
    };
    use uuid::Uuid;

    #[test]
    fn coverage_report_uses_text_status() {
        let report = IndexCoverageReport {
            total: 1,
            complete: 0,
            incomplete: 1,
            items: vec![IndexCoverageItem {
                name: "project".to_string(),
                kind: EntityKind::Project,
                path: "project".into(),
                coverage: coverage(false),
                component_count: 1,
                relation_count: 0,
                history_count: 0,
            }],
        };

        let output = super::format_coverage_report(&report);

        assert!(output.contains("complete=FAIL"));
        assert!(!output.contains("complete=false"));
    }

    #[test]
    fn inspection_uses_layer_text_status() {
        let inspection = IndexInspection {
            overview: overview(),
            coverage: coverage(false),
            component_count: 1,
            relation_count: 0,
            history_count: 0,
            key_components: vec![],
            key_relations: vec![],
            key_history: vec![],
        };

        let output = super::format_inspection(&inspection);

        assert!(output.contains("overview=PASS"));
        assert!(output.contains("relations=FAIL"));
        assert!(output.contains("complete=FAIL"));
    }

    fn coverage(complete: bool) -> IndexCoverage {
        IndexCoverage {
            entity_overview: true,
            component_digest: true,
            symbol_relation_index: complete,
            historical_context: complete,
        }
    }

    fn overview() -> EntityOverview {
        EntityOverview {
            id: Uuid::nil(),
            kind: EntityKind::Project,
            path: "project".into(),
            name: "project".to_string(),
            summary: "summary".to_string(),
            created_at: Utc::now(),
        }
    }
}
