//! Shared surface view models for Launcher and Studio.

pub mod a11y;
mod dashboard;
pub mod i18n;
pub mod input;
mod launcher;
mod memory;
pub mod motion;
mod plugin;
pub mod tokens;

pub use dashboard::StudioDashboardViewModel;
pub use launcher::{
    LauncherFeedback, LauncherNlSuggestion, LauncherPhase, LauncherResultMode, LauncherTelemetry,
    LauncherViewModel,
};
pub use memory::MemoryBrowserViewModel;
pub use plugin::PluginManagerViewModel;

pub fn summarize_json(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(value) => value.to_string(),
        serde_json::Value::Number(value) => value.to_string(),
        serde_json::Value::String(value) => value.clone(),
        serde_json::Value::Array(items) => format!("array({})", items.len()),
        serde_json::Value::Object(items) => format!("object({})", items.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_core::{StdConfig, StdCore};
    use std_types::ActionType;

    fn test_core() -> StdCore {
        let temp = tempfile::tempdir().unwrap();
        let core = StdCore::with_config(StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        });
        core.seed_builtin_actions().unwrap();
        core
    }

    #[test]
    fn launcher_view_model_searches_and_selects() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        model.update_query(&core, "index");
        assert_eq!(model.phase, LauncherPhase::WithResults);
        let execution = model.trigger_selected(&core).unwrap();

        assert_eq!(execution.action_name, "Rebuild Index");
        assert_eq!(model.phase, LauncherPhase::Feedback);
        assert_eq!(model.preview.as_ref().unwrap().title, "Rebuild Index");
        assert_eq!(model.last_triggered.as_deref(), Some("Rebuild Index"));
        assert_eq!(
            model.last_execution.as_ref().unwrap().action_name,
            "Rebuild Index"
        );
        assert_eq!(model.feedback.as_ref().unwrap().title, "Completed");
        assert_eq!(model.telemetry.last_result_count, model.results.len());
    }

    #[test]
    fn launcher_updates_preview_on_selection_move() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        model.update_query(&core, "");
        let first_title = model.preview.as_ref().unwrap().title.clone();
        model.move_selection_with_preview(&core, 1);
        let second_title = model.preview.as_ref().unwrap().title.clone();

        assert_ne!(first_title, second_title);
        assert_eq!(model.result_mode, LauncherResultMode::SuggestedWorkflows);
    }

    #[test]
    fn launcher_selection_stops_at_list_edges() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        model.update_query(&core, "");
        model.move_selection(-1);
        assert_eq!(model.selected, 0);

        let last = model.results.len() - 1;
        for _ in 0..(model.results.len() + 2) {
            model.move_selection(1);
        }
        assert_eq!(model.selected, last);
        model.move_selection(1);
        assert_eq!(model.selected, last);
    }

    #[test]
    fn launcher_view_model_distinguishes_result_modes() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        assert_eq!(model.result_mode, LauncherResultMode::SuggestedWorkflows);
        assert_eq!(model.phase, LauncherPhase::Empty);
        model.update_query(&core, "index");
        assert_eq!(model.result_mode, LauncherResultMode::Matches);
        assert_eq!(model.phase, LauncherPhase::WithResults);
        model.update_query(&core, "zzzz-no-launcher-match");
        assert_eq!(model.result_mode, LauncherResultMode::NoMatches);
        assert_eq!(model.phase, LauncherPhase::NoMatches);
    }

    #[test]
    fn launcher_preview_phases_cover_searching_and_executing() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        model.preview_searching("slow query");
        assert_eq!(model.phase, LauncherPhase::Searching);
        assert_eq!(model.query, "slow query");
        assert!(model.results.is_empty());

        model.preview_executing();
        assert_eq!(model.phase, LauncherPhase::Executing);
    }

    #[test]
    fn launcher_results_follow_documented_group_order() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        model.update_query(&core, "");
        let ranks = model
            .results
            .iter()
            .map(|result| match &result.action.action_type {
                ActionType::Workflow | ActionType::Command => 0,
                ActionType::AppLaunch => 1,
                ActionType::Custom(kind) if kind == "file" => 1,
                ActionType::Clipboard => 2,
                ActionType::Skill => 3,
                ActionType::Custom(_) => 4,
            })
            .collect::<Vec<_>>();

        assert!(!ranks.is_empty());
        assert!(ranks.windows(2).all(|window| window[0] <= window[1]));
    }

    #[test]
    fn launcher_query_is_normalized_and_deletes_previous_token() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        model.update_query(&core, "  rebuild   index  now ");
        assert_eq!(model.query, "rebuild index now");
        model.delete_previous_query_token(&core);
        assert_eq!(model.query, "rebuild index");
        model.delete_previous_query_token(&core);
        assert_eq!(model.query, "rebuild");
    }

    #[test]
    fn launcher_prefix_queries_strip_prefix_before_searching() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        model.update_query(&core, "/rebuild index");
        assert_eq!(model.query, "/rebuild index");
        assert!(model
            .results
            .iter()
            .all(|result| result.action.action_type == ActionType::Command));
        assert_eq!(model.preview.as_ref().unwrap().title, "Rebuild Index");

        model.update_query(&core, ">rebuild index");
        assert!(model.results.iter().all(|result| matches!(
            result.action.action_type,
            ActionType::Command | ActionType::Workflow
        )));
        assert_eq!(model.preview.as_ref().unwrap().title, "Rebuild Index");
    }

    #[test]
    fn launcher_question_prefix_uses_natural_language_suggestion_state() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        model.update_query(&core, "? rebuild index");

        assert_eq!(model.result_mode, LauncherResultMode::NaturalLanguage);
        assert_eq!(model.phase, LauncherPhase::WithResults);
        assert!(model.results.is_empty());
        assert!(model.preview.is_none());
        let suggestion = model.nl_suggestion.as_ref().unwrap();
        assert_eq!(suggestion.intent, "ask");
        assert_eq!(suggestion.query, "rebuild index");
        assert_eq!(suggestion.actions, vec!["Ask AI", "Search Actions"]);
    }

    #[test]
    fn launcher_records_search_preview_and_trigger_telemetry() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        model.update_query(&core, "index");
        model.trigger_selected(&core).unwrap();

        assert_eq!(model.telemetry.last_result_count, model.results.len());
        assert_eq!(model.telemetry.last_total_matches, model.results.len());
        assert!(!model.telemetry.last_overflowed);
        assert!(model.telemetry.last_result_count >= 1);
        assert!(model.telemetry.last_search_ms < 1_000);
        assert!(model.telemetry.last_preview_ms < 1_000);
        assert!(model.telemetry.last_trigger_ms < 1_000);
    }

    #[test]
    fn launcher_non_empty_query_caps_visible_results_at_docs_limit() {
        let temp = tempfile::tempdir().unwrap();
        let core = StdCore::with_config(StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        });
        for index in 0..205 {
            core.register_action(std_types::RegistryEntry::from_action(
                std_types::Action::new(
                    format!("Bulk Action {index:03}"),
                    "Bulk searchable action",
                    "bulk-search-overflow",
                    ActionType::Command,
                ),
                vec!["bulk-search-overflow".to_string()],
            ))
            .unwrap();
        }
        let mut model = LauncherViewModel::new(&core);

        model.update_query(&core, "bulk-search-overflow");

        assert_eq!(model.results.len(), 200);
        assert_eq!(model.telemetry.last_result_count, 200);
        assert_eq!(model.telemetry.last_total_matches, 201);
        assert!(model.telemetry.last_overflowed);
        assert!(model.result_overflowed());
    }

    #[test]
    fn launcher_view_model_defers_external_runner_actions() {
        let core = test_core();
        let mut model = LauncherViewModel::new(&core);

        model.update_query(&core, "terminal");
        let execution = model.trigger_selected(&core).unwrap();

        assert_eq!(execution.action_name, "Open Terminal");
        assert_eq!(
            execution.status,
            std_types::ActionExecutionStatus::NeedsExternalRunner
        );
        assert_eq!(
            execution
                .output
                .as_ref()
                .unwrap()
                .get("deferred")
                .and_then(|value| value.as_bool()),
            Some(true)
        );
        let feedback = model.feedback.as_ref().unwrap();
        assert_eq!(feedback.title, "Needs external runner");
        assert!(feedback.deferred);
    }

    #[test]
    fn studio_dashboard_loads_from_core() {
        let core = test_core();
        core.remember("project", "Memory", "Body", vec!["tag".to_string()])
            .unwrap();

        let dashboard = StudioDashboardViewModel::load(&core);

        assert!(dashboard.action_count >= 3);
        assert_eq!(dashboard.memory_count, 1);
        assert!(dashboard.audit_event_count >= 1);
    }

    #[test]
    fn plugin_manager_loads_searches_and_runs_plugin_action() {
        let temp = tempfile::tempdir().unwrap();
        let core = StdCore::with_config(StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        });
        let plugin_dir = core.config.plugins_dir().join("smoke");
        std::fs::create_dir_all(&plugin_dir).unwrap();
        std::fs::write(
            plugin_dir.join("plugin.json"),
            serde_json::json!({
                "name": "smoke",
                "description": "Smoke plugin",
                "permissions": ["shell"],
                "actions": [{
                    "name": "Plugin Smoke",
                    "description": "Run plugin smoke",
                    "when_to_use": "When validating Studio plugin manager",
                    "kind": "shell",
                    "command": "printf studio-plugin-smoke",
                    "tags": ["studio-plugin-smoke"]
                }]
            })
            .to_string(),
        )
        .unwrap();
        core.seed_builtin_actions().unwrap();
        let mut manager = PluginManagerViewModel::load(&core);

        manager.search(&core, "studio-plugin-smoke");
        let execution = manager.run_selected(&core).unwrap();

        assert_eq!(manager.manifest_paths.len(), 1);
        assert_eq!(manager.preview.as_ref().unwrap().title, "Plugin Smoke");
        assert_eq!(execution.action_name, "Plugin Smoke");
        assert_eq!(execution.status, std_types::ActionExecutionStatus::Failed);
        assert!(execution
            .message
            .contains("STD_TEST_MODE blocked shell plugin command"));
    }

    #[test]
    fn memory_browser_searches_selects_and_writes_memory() {
        let core = test_core();
        core.remember(
            "project",
            "Workflow storage",
            "Workflow definitions live under workflows",
            vec!["workflow".to_string()],
        )
        .unwrap();
        let mut browser = MemoryBrowserViewModel::load(&core);

        browser.search(&core, "workflow");
        browser.select(0);
        let written = browser
            .remember(
                &core,
                "studio",
                "Studio note",
                "Memory Browser writes through std-core",
                vec!["studio".to_string()],
            )
            .unwrap();

        assert_eq!(browser.selected_memory().unwrap().title, "Studio note");
        assert_eq!(written.scope, "studio");
        assert_eq!(browser.last_written.as_ref().unwrap().title, "Studio note");
        assert!(core
            .search("Studio note", 10)
            .unwrap()
            .iter()
            .any(|result| result.action.name.contains("Memory: Studio note")));
    }
}
