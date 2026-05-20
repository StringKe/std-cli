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
