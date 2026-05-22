use super::*;

#[test]
fn studio_reports_saved_analysis_coverage() {
    let mut studio = test_studio();
    let project_dir = studio.core.config.data_dir.join("coverage-project");
    std::fs::create_dir_all(project_dir.join(".git").join("logs")).unwrap();
    std::fs::create_dir_all(project_dir.join("src")).unwrap();
    std::fs::write(
        project_dir.join("src").join("lib.rs"),
        "pub struct StudioCoverage {}\n",
    )
    .unwrap();
    std::fs::write(
        project_dir.join(".git").join("logs").join("HEAD"),
        "0000000 1111111 User <u@example.com> 1 +0000\tcommit: studio coverage\n",
    )
    .unwrap();

    studio.analyze_entity(&project_dir).unwrap();
    let report = studio.analysis_coverage_report().unwrap();

    assert_eq!(report.total, 1);
    assert_eq!(report.complete, 1);
    assert_eq!(report.incomplete, 0);
    assert_eq!(report.items[0].name, "coverage-project");
    assert!(report.items[0].coverage.complete());
}

#[test]
fn analysis_workbench_model_exposes_docs_22_tabs_and_evidence() {
    let mut studio = test_studio();
    let project_dir = studio.core.config.data_dir.join("workbench-project");
    seed_analysis_project(&project_dir);

    let document = studio.analyze_entity(&project_dir).unwrap().clone();
    let coverage = studio.analysis_coverage_report().unwrap();
    let search = studio.search_analyses("StudioWorkbench", 8).unwrap();
    let answer = studio.ask_analyses("StudioWorkbench", 5).unwrap();
    let inspection = studio
        .inspect_analysis("workbench-project", 8)
        .unwrap()
        .unwrap();
    let model = AnalysisWorkbenchViewModel::build(
        Some(&document),
        Some(&coverage),
        Some(&answer),
        &search,
        Some(&inspection),
    );

    assert_eq!(
        model
            .tabs
            .iter()
            .map(|tab| tab.tab.key())
            .collect::<Vec<_>>(),
        ["overview", "components", "symbols", "relations", "qa"]
    );
    assert_eq!(
        model
            .coverage_layers
            .iter()
            .map(|layer| layer.key)
            .collect::<Vec<_>>(),
        ["overview", "components", "relations", "history"]
    );
    assert!(model.coverage_layers.iter().all(|layer| layer.complete));
    assert!(model
        .coverage_layers
        .iter()
        .all(|layer| layer.status_label() == "PASS"));
    assert!(model
        .tabs
        .iter()
        .any(|tab| tab.tab == AnalysisWorkbenchTab::Components && tab.count >= 1));
    assert!(model
        .tabs
        .iter()
        .any(|tab| tab.tab == AnalysisWorkbenchTab::Relations && tab.count >= 1));
    assert!(model
        .tabs
        .iter()
        .any(|tab| tab.tab == AnalysisWorkbenchTab::Qa && tab.count >= 1));
    assert_eq!(model.overview_cards.len(), 3);
    assert!(!model.search_hits.is_empty());
    assert!(model
        .answer_sources
        .iter()
        .any(|source| source.evidence_count >= 1));
    assert!(model
        .answer_sources
        .iter()
        .any(|source| source.jump_target.starts_with("analysis-source://")));
    assert_eq!(
        model.inspection_summary.as_ref().unwrap().entity,
        "workbench-project"
    );
}

#[test]
fn analysis_coverage_layers_expose_fail_text_for_incomplete_index() {
    let model = AnalysisWorkbenchViewModel::build(None, None, None, &[], None);

    assert_eq!(
        model
            .coverage_layers
            .iter()
            .map(|layer| (layer.key, layer.status_label()))
            .collect::<Vec<_>>(),
        [
            ("overview", "FAIL"),
            ("components", "FAIL"),
            ("relations", "FAIL"),
            ("history", "FAIL")
        ]
    );
}

#[test]
fn analysis_query_panel_contract_surfaces_evidence_and_search_hits() {
    let body = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("analysis_query_panel.rs"),
    )
    .unwrap();

    assert!(body.contains("AnalysisQueryAction"));
    assert!(body.contains("WidgetType::TextEdit"));
    assert!(body.contains("analysis_query_a11y_label"));
    assert!(body.contains("AnalysisFocusArea::Query.focus_id()"));
    assert!(body.contains("Analysis query, text box, value"));
    assert!(body.contains("query.trim().is_empty()"));

    let tabs = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("analysis_tab_content.rs"),
    )
    .unwrap();
    assert!(tabs.contains("Sources"));
    assert!(tabs.contains("jump_target"));
    assert!(tabs.contains("model.answer_sources"));
    assert!(tabs.contains("model.search_hits"));
}

#[test]
fn analysis_toolbar_query_contract_matches_docs_22_workbench() {
    let body = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("analysis_query_panel.rs"),
    )
    .unwrap();

    assert!(body.contains("pub(crate) fn render_toolbar_query"));
    assert!(body.contains("studio.analysis.qa.hint"));
    assert!(body.contains("AnalysisFocusArea::Query.focus_id()"));
    assert!(body.contains("AnalysisQueryAction::Ask"));
    assert!(body.contains("AnalysisQueryAction::Search"));
    assert!(body.contains("AnalysisQueryAction::Inspect"));
    assert!(body.contains("WidgetType::TextEdit"));
}

#[test]
fn analysis_workbench_tabs_have_stable_default_and_order() {
    assert_eq!(
        AnalysisWorkbenchTab::default(),
        AnalysisWorkbenchTab::Overview
    );
    assert_eq!(
        AnalysisWorkbenchTab::all()
            .into_iter()
            .map(AnalysisWorkbenchTab::key)
            .collect::<Vec<_>>(),
        ["overview", "components", "symbols", "relations", "qa"]
    );
}

fn seed_analysis_project(project_dir: &std::path::Path) {
    std::fs::create_dir_all(project_dir.join(".git").join("logs")).unwrap();
    std::fs::create_dir_all(project_dir.join("src")).unwrap();
    std::fs::write(
        project_dir.join("src").join("lib.rs"),
        "pub struct StudioWorkbench {}\nimpl StudioWorkbench { pub fn analyze() {} }\n",
    )
    .unwrap();
    std::fs::write(
        project_dir.join(".git").join("logs").join("HEAD"),
        "0000000 1111111 User <u@example.com> 1 +0000\tcommit: StudioWorkbench relation\n",
    )
    .unwrap();
}
