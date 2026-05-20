use super::*;
use crate::EventBus;
use std_index::Indexer;

#[test]
fn core_registers_indexed_files_in_search() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let core = StdCore::with_config(config.clone());
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    fs::write(project_dir.join("README.md"), "Workflow search source").unwrap();
    let index = Indexer::index_files(&project_dir, std_index::FileIndexOptions::default()).unwrap();
    Indexer::write_file_index(&config.index_dir(), &index).unwrap();

    core.register_local_content_actions().unwrap();
    let results = core.search("Workflow search", 10).unwrap();

    assert!(results
        .iter()
        .any(|result| result.action.name.contains("Open File: README.md")));
}

#[test]
fn core_indexes_entities_and_files_with_audit_events() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let core = StdCore::with_config(config);
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    fs::write(project_dir.join("README.md"), "Workflow search source").unwrap();

    let (document, _) = core.analyze_and_store_entity(&project_dir).unwrap();
    let (file_index, _) = core
        .index_and_store_files(&project_dir, std_index::FileIndexOptions::default())
        .unwrap();
    let events = core.events().unwrap();

    assert_eq!(document.overview.name, "project");
    assert_eq!(file_index.entries.len(), 1);
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == StdEventType::IndexUpdated)
            .count(),
        2
    );
}
