use std_types::{ActionType, SearchResult};

pub(crate) fn is_action_result(result: &SearchResult) -> bool {
    matches!(
        result.action.action_type,
        ActionType::Command | ActionType::Workflow
    )
}

pub(crate) fn is_command_result(result: &SearchResult) -> bool {
    result.action.action_type == ActionType::Command
}

pub(crate) fn sort_launcher_results(results: &mut [SearchResult]) {
    results.sort_by(|left, right| {
        group_rank(&left.action.action_type)
            .cmp(&group_rank(&right.action.action_type))
            .then_with(|| right.score.total_cmp(&left.score))
            .then_with(|| left.action.name.cmp(&right.action.name))
    });
}

pub(crate) fn exact_app_alias_index(results: &[SearchResult], query: &str) -> Option<usize> {
    let query = normalize_alias(query);
    if query.is_empty() {
        return None;
    }
    results.iter().position(|result| {
        result.action.action_type == ActionType::AppLaunch && exact_app_alias_match(result, &query)
    })
}

fn group_rank(action_type: &ActionType) -> u8 {
    match action_type {
        ActionType::Workflow | ActionType::Command => 0,
        ActionType::AppLaunch => 1,
        ActionType::Custom(kind) if kind == "file" => 1,
        ActionType::Clipboard => 2,
        ActionType::Memory => 3,
        ActionType::Skill => 4,
        ActionType::Custom(_) => 5,
    }
}

fn exact_app_alias_match(result: &SearchResult, query: &str) -> bool {
    let display_name = result
        .action
        .name
        .strip_prefix("Open App: ")
        .unwrap_or(&result.action.name);
    normalize_alias(display_name) == query
        || result
            .action
            .description
            .split("Aliases:")
            .nth(1)
            .and_then(|aliases| aliases.split(" / Path:").next())
            .map(|aliases| {
                aliases
                    .split(',')
                    .any(|alias| normalize_alias(alias.trim()) == query)
            })
            .unwrap_or(false)
}

fn normalize_alias(value: &str) -> String {
    value
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_types::Action;

    #[test]
    fn launcher_results_follow_documented_group_order_before_score() {
        let mut results = vec![
            result("High score app", ActionType::AppLaunch, 99.0),
            result("Lower score action", ActionType::Command, 1.0),
            result(
                "Highest file",
                ActionType::Custom("file".to_string()),
                120.0,
            ),
            result("Workflow", ActionType::Workflow, 0.5),
        ];

        sort_launcher_results(&mut results);

        let names: Vec<&str> = results
            .iter()
            .map(|item| item.action.name.as_str())
            .collect();
        assert_eq!(
            names,
            vec![
                "Lower score action",
                "Workflow",
                "Highest file",
                "High score app"
            ]
        );
    }

    #[test]
    fn launcher_results_sort_by_score_inside_group() {
        let mut results = vec![
            result("App B", ActionType::AppLaunch, 2.0),
            result("App A", ActionType::AppLaunch, 8.0),
            result("App C", ActionType::AppLaunch, 8.0),
        ];

        sort_launcher_results(&mut results);

        let names: Vec<&str> = results
            .iter()
            .map(|item| item.action.name.as_str())
            .collect();
        assert_eq!(names, vec!["App A", "App C", "App B"]);
    }

    #[test]
    fn exact_app_alias_index_promotes_direct_app_queries_only() {
        let results = vec![
            result("Rebuild Index", ActionType::Command, 99.0),
            app_result(
                "Open App: WeChat",
                "Aliases: WeChat, weixin, 微信 / Path: /tmp/WeChat.app",
            ),
        ];

        assert_eq!(exact_app_alias_index(&results, "wechat"), Some(1));
        assert_eq!(exact_app_alias_index(&results, "weixin"), Some(1));
        assert_eq!(exact_app_alias_index(&results, "微信"), Some(1));
        assert_eq!(exact_app_alias_index(&results, "we"), None);
    }

    fn result(name: &str, action_type: ActionType, score: f32) -> SearchResult {
        SearchResult {
            action: Action::new(name, "description", "use", action_type),
            score,
            matched_fields: vec!["name".to_string()],
        }
    }

    fn app_result(name: &str, description: &str) -> SearchResult {
        SearchResult {
            action: Action::new(name, description, "use", ActionType::AppLaunch),
            score: 1.0,
            matched_fields: vec!["tags".to_string()],
        }
    }
}
