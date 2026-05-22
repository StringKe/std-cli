use super::*;

mod coverage;

#[test]
fn indexer_analyzes_directory_and_writes_document() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("README.md"), "hello").unwrap();

    let doc = Indexer::analyze(temp.path()).unwrap();
    let written = Indexer::write_document(&temp.path().join("index"), &doc).unwrap();

    assert_eq!(doc.overview.kind, EntityKind::Project);
    assert_eq!(doc.components.len(), 1);
    assert!(written.is_file());
}

#[test]
fn indexer_reads_git_head_history_context() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join(".git").join("logs")).unwrap();
    fs::write(temp.path().join("README.md"), "hello").unwrap();
    fs::write(
        temp.path().join(".git").join("logs").join("HEAD"),
        "0000000 1111111 User <u@example.com> 1 +0000\tcommit: bootstrap project\n1111111 2222222 User <u@example.com> 2 +0000\tcommit: add launcher workflow\n",
    )
    .unwrap();

    let doc = Indexer::analyze(temp.path()).unwrap();
    let index_dir = temp.path().join("index");
    Indexer::write_document(&index_dir, &doc).unwrap();
    let searched = Indexer::search_documents(&index_dir, "launcher workflow", 10).unwrap();
    let answer = Indexer::answer_from_documents(&index_dir, "launcher workflow", 10).unwrap();

    assert!(doc
        .history
        .iter()
        .any(|history| history.source == "git HEAD"
            && history.summary == "commit: add launcher workflow"));
    assert!(searched[0].matched_fields.contains(&"history".to_string()));
    assert!(answer.answer.contains("commit: add launcher workflow"));
    assert!(answer.sources[0]
        .evidence
        .contains(&"history: git HEAD: commit: add launcher workflow".to_string()));
}

#[test]
fn indexer_extracts_mac_app_bundle_metadata() {
    let temp = tempfile::tempdir().unwrap();
    let app = temp.path().join("std Studio.app");
    fs::create_dir_all(app.join("Contents").join("MacOS")).unwrap();
    fs::write(
        app.join("Contents").join("MacOS").join("std-studio"),
        "binary",
    )
    .unwrap();
    fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
  <key>CFBundleIdentifier</key>
  <string>com.stringke.std-cli.studio</string>
  <key>CFBundleExecutable</key>
  <string>std-studio</string>
  <key>CFBundleName</key>
  <string>std Studio</string>
  <key>CFBundleShortVersionString</key>
  <string>1.0.0</string>
</dict>
</plist>
"#,
    )
    .unwrap();

    let doc = Indexer::analyze(&app).unwrap();

    assert_eq!(doc.overview.kind, EntityKind::AppBundle);
    assert!(doc
        .components
        .iter()
        .any(|component| component.path.ends_with("Info.plist")
            && component.purpose == "mac app bundle metadata"));
    assert!(doc
        .relations
        .iter()
        .any(|relation| relation.relation == "app_bundle_identifier"
            && relation.symbol == "com.stringke.std-cli.studio"));
    assert!(doc
        .relations
        .iter()
        .any(|relation| relation.relation == "app_bundle_executable"
            && relation.symbol == "std-studio"));
    assert!(doc
        .relations
        .iter()
        .any(|relation| relation.relation == "app_bundle_version" && relation.symbol == "1.0.0"));
}

#[test]
fn indexer_searches_mac_app_bundle_by_multilingual_aliases() {
    let temp = tempfile::tempdir().unwrap();
    let app = temp.path().join("Weixin.app");
    fs::create_dir_all(app.join("Contents").join("Resources").join("zh_CN.lproj")).unwrap();
    fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
  <key>CFBundleIdentifier</key>
  <string>com.tencent.xinWeChat</string>
  <key>CFBundleExecutable</key>
  <string>WeChat</string>
</dict>
</plist>
"#,
    )
    .unwrap();
    fs::write(
        app.join("Contents")
            .join("Resources")
            .join("zh_CN.lproj")
            .join("InfoPlist.strings"),
        "\"CFBundleDisplayName\" = \"\\U5fae\\U4fe1\";",
    )
    .unwrap();

    let doc = Indexer::analyze(&app).unwrap();
    let index_dir = temp.path().join("index");
    Indexer::write_document(&index_dir, &doc).unwrap();
    let chinese = Indexer::search_documents(&index_dir, "微信", 10).unwrap();
    let english = Indexer::search_documents(&index_dir, "wechat", 10).unwrap();
    let pinyin = Indexer::search_documents(&index_dir, "weixin", 10).unwrap();

    assert_eq!(doc.overview.name, "WeChat");
    assert!(doc.overview.summary.contains("wechat"));
    assert!(doc.overview.summary.contains("weixin"));
    assert!(doc.overview.summary.contains("微信"));
    assert!(doc
        .relations
        .iter()
        .any(|relation| relation.relation == "app_bundle_alias" && relation.symbol == "wechat"));
    assert_eq!(chinese[0].document.overview.id, doc.overview.id);
    assert_eq!(english[0].document.overview.id, doc.overview.id);
    assert_eq!(pinyin[0].document.overview.id, doc.overview.id);
    assert!(
        english[0]
            .matched_fields
            .contains(&"overview.summary".to_string())
            || english[0].matched_fields.contains(&"relations".to_string())
    );
}

#[test]
fn indexer_recurses_components_and_extracts_symbols() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(
        temp.path().join("src").join("main.rs"),
        "use std::path::Path;\nfn main() {}\npub struct AppState {}\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\n[dependencies]\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("release.workflow.json"),
        serde_json::json!({
            "name": "Release",
            "description": "Ship the build",
            "steps": [{
                "name": "Build",
                "action_id": "00000000-0000-4000-8000-000000000101",
                "step_type": "Action",
                "parameters": {"target": "cargo build --release"},
            }],
        })
        .to_string(),
    )
    .unwrap();

    let doc = Indexer::analyze(temp.path()).unwrap();

    let main_component = doc
        .components
        .iter()
        .find(|component| component.path.ends_with("src/main.rs"))
        .unwrap();
    assert_eq!(main_component.language, "rust");
    assert!(main_component.snippet.contains("AppState"));
    assert!(main_component.symbols.contains(&"fn main".to_string()));
    assert!(main_component
        .symbols
        .contains(&"type AppState".to_string()));
    assert!(doc
        .components
        .iter()
        .any(|component| component.purpose == "rust binary entrypoint"));
    assert!(doc
        .relations
        .iter()
        .any(|relation| relation.symbol == "main" && relation.relation == "defines_fn"));
    assert!(doc
        .relations
        .iter()
        .any(|relation| relation.symbol == "AppState" && relation.relation == "defines_type"));
    assert!(
        doc.relations
            .iter()
            .any(|relation| relation.symbol == "Build"
                && relation.relation == "defines_workflow_step")
    );
    assert!(doc.relations.iter().any(|relation| {
        relation.symbol == "Build"
            && relation.relation == "workflow_step_type"
            && relation.target == "Action"
    }));
    assert!(doc.relations.iter().any(|relation| {
        relation.symbol == "Build"
            && relation.relation == "workflow_step_action_id"
            && relation.target == "00000000-0000-4000-8000-000000000101"
    }));
    assert!(doc.relations.iter().any(|relation| {
        relation.symbol == "Build"
            && relation.relation == "workflow_step_parameter_target"
            && relation.target == "cargo build --release"
    }));
    assert!(doc
        .relations
        .iter()
        .any(|relation| relation.symbol == "package"
            && relation.relation == "defines_config_section"));
}

#[test]
fn indexer_extracts_markdown_workflow_structure() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(
        temp.path().join("workflow.md"),
        r#"---
name: "Daily Smoke"
description: "Run daily smoke checks"
steps_json: '[{"id":"00000000-0000-4000-8000-000000000201","name":"Run tests","action_id":null,"step_type":"Action","parameters":{"command":"cargo test --workspace"}}]'
---

Run daily smoke checks.
"#,
    )
    .unwrap();

    let doc = Indexer::analyze(&temp.path().join("workflow.md")).unwrap();
    let search_dir = temp.path().join("index");
    Indexer::write_document(&search_dir, &doc).unwrap();
    let answer = Indexer::answer_from_documents(&search_dir, "cargo test --workspace", 5).unwrap();

    assert_eq!(doc.overview.kind, EntityKind::Workflow);
    assert!(doc.components[0]
        .symbols
        .contains(&"workflow Daily Smoke".to_string()));
    assert!(doc.components[0]
        .symbols
        .contains(&"step Run tests".to_string()));
    assert!(doc.relations.iter().any(|relation| {
        relation.symbol == "Run tests"
            && relation.relation == "workflow_step_parameter_command"
            && relation.target == "cargo test --workspace"
    }));
    assert!(answer.answer.contains("Run tests"));
    assert!(answer.sources[0]
        .evidence
        .iter()
        .any(|item| item.contains("Run tests workflow_step_parameter_command cargo test")));
}

#[test]
fn indexer_extracts_impl_calls_and_cargo_dependencies() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(
        temp.path().join("src").join("lib.rs"),
        r#"pub struct Runner {}

impl Runner {
    pub fn run(&self) {
        self.prepare();
        execute_plan();
    }

    fn prepare(&self) {}
}

fn execute_plan() {}
"#,
    )
    .unwrap();
    fs::write(
        temp.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\n[dependencies]\nserde = \"1\"\n[dev-dependencies]\ntempfile = \"3\"\n",
    )
    .unwrap();

    let doc = Indexer::analyze(temp.path()).unwrap();

    assert!(doc
        .relations
        .iter()
        .any(|relation| relation.symbol == "Runner" && relation.relation == "implements_type"));
    assert!(doc.relations.iter().any(|relation| {
        relation.symbol == "run"
            && relation.relation == "calls_fn"
            && relation.target.ends_with("#prepare")
    }));
    assert!(doc.relations.iter().any(|relation| {
        relation.symbol == "run"
            && relation.relation == "calls_fn"
            && relation.target.ends_with("#execute_plan")
    }));
    assert!(doc
        .relations
        .iter()
        .any(|relation| relation.symbol == "serde" && relation.relation == "depends_on_crate"));
    assert!(doc
        .relations
        .iter()
        .any(|relation| relation.symbol == "tempfile" && relation.relation == "depends_on_crate"));
}

#[test]
fn current_directory_gets_stable_entity_name() {
    let doc = Indexer::analyze(Path::new(".")).unwrap();

    assert!(!doc.overview.name.is_empty());
    assert_ne!(doc.overview.name, ".");
}

#[test]
fn file_index_recurses_and_searches_content() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(
        temp.path().join("src").join("main.rs"),
        "fn main() { println!(\"workflow\"); }",
    )
    .unwrap();
    fs::write(temp.path().join("target").join("ignored.rs"), "workflow").unwrap_err();
    fs::create_dir_all(temp.path().join("target")).unwrap();
    fs::write(temp.path().join("target").join("ignored.rs"), "workflow").unwrap();

    let index = Indexer::index_files(temp.path(), FileIndexOptions::default()).unwrap();
    let results = search_file_entries(index.entries.clone(), "workflow", 10);

    assert_eq!(index.entries.len(), 1);
    assert_eq!(index.entries[0].name, "main.rs");
    assert_eq!(results.len(), 1);
    assert!(results[0].matched_fields.contains(&"snippet".to_string()));
}

#[test]
fn file_index_can_be_written_and_read() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("README.md"), "std-cli file search").unwrap();

    let index = Indexer::index_files(temp.path(), FileIndexOptions::default()).unwrap();
    let written = Indexer::write_file_index(&temp.path().join("index"), &index).unwrap();
    let indexes = Indexer::read_file_indexes(&temp.path().join("index")).unwrap();
    let results = Indexer::search_indexed_files(&temp.path().join("index"), "std-cli", 10).unwrap();

    assert!(written.is_file());
    assert_eq!(indexes.len(), 1);
    assert_eq!(results.len(), 1);
}

#[test]
fn index_documents_can_be_read_searched_and_answered() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(
        temp.path().join("src").join("main.rs"),
        "fn main() {}\npub struct Launcher {}\n",
    )
    .unwrap();

    let doc = Indexer::analyze(temp.path()).unwrap();
    let index_dir = temp.path().join("index");
    let written = Indexer::write_document(&index_dir, &doc).unwrap();
    let read = Indexer::read_document(&written).unwrap();
    let documents = Indexer::read_documents(&index_dir).unwrap();
    let found = Indexer::find_document(&index_dir, &doc.overview.name)
        .unwrap()
        .unwrap();
    let searched = Indexer::search_documents(&index_dir, "Launcher", 10).unwrap();
    let answer = Indexer::answer_from_documents(&index_dir, "Launcher", 10).unwrap();
    let component_search = Indexer::search_documents(&index_dir, "type Launcher", 10).unwrap();

    assert_eq!(read.overview.name, doc.overview.name);
    assert_eq!(documents.len(), 1);
    assert_eq!(found.overview.path, doc.overview.path);
    assert_eq!(searched.len(), 1);
    assert!(searched[0]
        .matched_fields
        .contains(&"relations".to_string()));
    assert!(answer.answer.contains("Launcher"));
    assert_eq!(answer.sources.len(), 1);
    assert!(answer.sources[0]
        .matched_fields
        .contains(&"relations".to_string()));
    assert!(answer.sources[0]
        .evidence
        .iter()
        .any(|item| item.contains("Launcher defines_type")));
    assert!(component_search[0]
        .matched_fields
        .contains(&"components".to_string()));
    assert!(component_search[0]
        .document
        .components
        .iter()
        .any(|component| component.symbols.contains(&"type Launcher".to_string())));
}

#[test]
fn index_inspection_reports_four_layer_coverage() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join(".git").join("logs")).unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(
        temp.path().join("src").join("lib.rs"),
        "pub struct Coverage {}\n",
    )
    .unwrap();
    fs::write(
        temp.path().join(".git").join("logs").join("HEAD"),
        "0000000 1111111 User <u@example.com> 1 +0000\tcommit: add coverage\n",
    )
    .unwrap();

    let doc = Indexer::analyze(temp.path()).unwrap();
    let index_dir = temp.path().join("index");
    Indexer::write_document(&index_dir, &doc).unwrap();
    let inspection = Indexer::inspect_document(&index_dir, &doc.overview.name, 3)
        .unwrap()
        .unwrap();

    assert!(inspection.coverage.entity_overview);
    assert!(inspection.coverage.component_digest);
    assert!(inspection.coverage.symbol_relation_index);
    assert!(inspection.coverage.historical_context);
    assert!(inspection.coverage.complete());
    assert_eq!(inspection.component_count, doc.components.len());
    assert_eq!(inspection.relation_count, doc.relations.len());
    assert_eq!(inspection.history_count, doc.history.len());
}

#[test]
fn index_document_reads_legacy_component_digest_json() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("legacy.json");
    fs::write(
        &path,
        serde_json::json!({
            "overview": {
                "id": uuid::Uuid::new_v4(),
                "kind": "Project",
                "path": temp.path(),
                "name": "legacy",
                "summary": "legacy index",
                "created_at": chrono::Utc::now(),
            },
            "components": [{
                "path": temp.path().join("src/lib.rs"),
                "purpose": "rust source module",
                "size_bytes": 42
            }],
            "relations": [],
            "history": []
        })
        .to_string(),
    )
    .unwrap();

    let doc = Indexer::read_document(&path).unwrap();

    assert_eq!(doc.components[0].language, "");
    assert_eq!(doc.components[0].snippet, "");
    assert!(doc.components[0].symbols.is_empty());
}
