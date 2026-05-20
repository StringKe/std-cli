use super::*;

#[test]
fn analyze_command_outputs_index_document() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("README.md"), "hello").unwrap();

    let output = run_cli(["std", "analyze", temp.path().to_str().unwrap()]).unwrap();

    assert!(output.contains("\"components\""));
    assert!(output.contains("\"relations\""));
    assert!(output.contains("\"history\""));
}

#[test]
fn index_commands_rebuild_list_show_search_and_ask() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let project_dir = temp.path().join("project");
    std::fs::create_dir_all(project_dir.join("src")).unwrap();
    std::fs::write(
        project_dir.join("src").join("main.rs"),
        "fn main() {}\npub struct StudioAnalysis {}\n",
    )
    .unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let rebuilt = run_cli(["std", "index", "rebuild", project_dir.to_str().unwrap()]).unwrap();
    let listed = run_cli(["std", "index", "list"]).unwrap();
    let shown = run_cli(["std", "index", "show", "project"]).unwrap();
    let inspected = run_cli(["std", "index", "inspect", "project", "--limit", "2"]).unwrap();
    let coverage = run_cli(["std", "index", "coverage"]).unwrap();
    let searched = run_cli(["std", "index", "search", "StudioAnalysis"]).unwrap();
    let answer = run_cli(["std", "index", "ask", "StudioAnalysis"]).unwrap();
    let events = run_cli(["std", "events", "--audit"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert_index_command_outputs(IndexCommandOutputs {
        rebuilt,
        listed,
        shown,
        inspected,
        coverage,
        searched,
        answer,
        events,
    });
}

struct IndexCommandOutputs {
    rebuilt: String,
    listed: String,
    shown: String,
    inspected: String,
    coverage: String,
    searched: String,
    answer: String,
    events: String,
}

fn assert_index_command_outputs(outputs: IndexCommandOutputs) {
    assert!(outputs.rebuilt.contains("index rebuilt"));
    assert!(outputs.listed.contains("project"));
    assert_show_output(&outputs.shown);
    assert_inspect_output(&outputs.inspected);
    assert_coverage_output(&outputs.coverage);
    assert!(outputs.searched.contains("relations"));
    assert_answer_output(&outputs.answer);
    assert!(outputs.events.contains("IndexUpdated"));
}

fn assert_show_output(output: &str) {
    for expected in [
        "\"components\"",
        "\"language\"",
        "\"symbols\"",
        "type StudioAnalysis",
    ] {
        assert!(output.contains(expected));
    }
}

fn assert_inspect_output(output: &str) {
    for expected in [
        "\"component_count\"",
        "\"coverage\"",
        "\"entity_overview\": true",
        "\"component_digest\": true",
        "\"symbol_relation_index\": true",
        "\"relation_count\"",
        "\"history_count\"",
        "\"key_components\"",
        "\"key_relations\"",
        "type StudioAnalysis",
    ] {
        assert!(output.contains(expected));
    }
}

fn assert_coverage_output(output: &str) {
    for expected in [
        "\"total\": 1",
        "\"complete\": 1",
        "\"incomplete\": 0",
        "\"symbol_relation_index\": true",
    ] {
        assert!(output.contains(expected));
    }
}

fn assert_answer_output(output: &str) {
    for expected in [
        "StudioAnalysis",
        "\"sources\"",
        "\"evidence\"",
        "StudioAnalysis defines_type",
    ] {
        assert!(output.contains(expected));
    }
}

#[test]
fn files_commands_index_search_and_feed_action_search() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let project_dir = temp.path().join("project");
    std::fs::create_dir_all(project_dir.join("src")).unwrap();
    std::fs::write(
        project_dir.join("src").join("main.rs"),
        "workflow file lookup",
    )
    .unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let indexed = run_cli(["std", "files", "index", project_dir.to_str().unwrap()]).unwrap();
    let file_results = run_cli(["std", "files", "search", "workflow"]).unwrap();
    let action_results = run_cli(["std", "search", "workflow file"]).unwrap();
    let events = run_cli(["std", "events", "--audit"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(indexed.contains("files indexed"));
    assert!(file_results.contains("main.rs"));
    assert!(action_results.contains("Open File: main.rs"));
    assert!(events.contains("IndexUpdated"));
}
