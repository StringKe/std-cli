use super::*;

#[test]
fn coverage_report_summarizes_four_layer_indexes() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join(".git").join("logs")).unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(
        temp.path().join("src").join("lib.rs"),
        "pub struct CoverageReport {}\n",
    )
    .unwrap();
    fs::write(
        temp.path().join(".git").join("logs").join("HEAD"),
        "0000000 1111111 User <u@example.com> 1 +0000\tcommit: coverage report\n",
    )
    .unwrap();
    let doc = Indexer::analyze(temp.path()).unwrap();
    let index_dir = temp.path().join("index");
    Indexer::write_document(&index_dir, &doc).unwrap();

    let report = Indexer::coverage_report(&index_dir).unwrap();

    assert_eq!(report.total, 1);
    assert_eq!(report.complete, 1);
    assert_eq!(report.incomplete, 0);
    assert_eq!(report.items[0].name, doc.overview.name);
    assert!(report.items[0].coverage.entity_overview);
    assert!(report.items[0].coverage.component_digest);
    assert!(report.items[0].coverage.symbol_relation_index);
    assert!(report.items[0].coverage.historical_context);
    assert!(report.items[0].coverage.complete());
}
