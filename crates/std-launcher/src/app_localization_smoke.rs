use crate::LauncherState;
use std::{fs, path::Path};
use std_core::{StdConfig, StdCore};
use std_types::{ActionExecutionStatus, ActionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherAppLocalizationSmokeReport {
    pub status: &'static str,
    pub app_title: String,
    pub queries: Vec<String>,
    pub action_ids_match: bool,
    pub enter_status: ActionExecutionStatus,
    pub deferred: bool,
    pub aliases: String,
    pub matched_fields: String,
}

impl LauncherAppLocalizationSmokeReport {
    pub fn run() -> Self {
        let config = smoke_config();
        let app = config.apps_dir().join("WeChat.app");
        write_wechat_app_bundle(&app);
        let core = StdCore::with_config(config);
        let mut state = LauncherState::with_core(core);
        let queries = vec![
            "wechat".to_string(),
            "weixin".to_string(),
            "\u{5fae}\u{4fe1}".to_string(),
        ];
        let mut action_ids = Vec::new();
        let mut aliases = String::new();
        let mut matched_fields = Vec::new();

        for query in &queries {
            state.update_query(query);
            let Some(result) = state
                .view
                .results
                .iter()
                .find(|result| result.action.name == "Open App: WeChat")
            else {
                return Self::fail(queries, action_ids);
            };
            matched_fields.extend(result.matched_fields.clone());
            action_ids.push(result.action.id);
            if aliases.is_empty() {
                aliases = state
                    .core
                    .preview_action(result.action.id)
                    .ok()
                    .and_then(|preview| preview.metadata.get("aliases").cloned())
                    .unwrap_or_default();
            }
        }

        state.view.selected = state
            .view
            .results
            .iter()
            .position(|result| result.action.id == action_ids[0])
            .unwrap_or_default();
        let execution = state
            .trigger_selected()
            .expect("fixture app should execute");
        let action_ids_match = same_action_ids(&action_ids);
        let deferred = execution
            .output
            .as_ref()
            .and_then(|output| output.get("deferred"))
            .and_then(|value| value.as_bool())
            == Some(true);
        let status = if action_ids_match
            && execution.status == ActionExecutionStatus::NeedsExternalRunner
            && deferred
            && aliases.contains("wechat")
            && aliases.contains("weixin")
            && aliases.contains("\u{5fae}\u{4fe1}")
        {
            "PASS"
        } else {
            "FAIL"
        };

        Self {
            status,
            app_title: "Open App: WeChat".to_string(),
            queries,
            action_ids_match,
            enter_status: execution.status,
            deferred,
            aliases,
            matched_fields: unique_join(matched_fields),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_app_localization_smoke {}\napp_title={}\nqueries={}\naction_ids_match={}\nenter_status={:?}\ndeferred={}\naliases={}\nmatched_fields={}\nfixture_scope=local_apps_dir_only\nsystem_apps_scanned=false",
            self.status,
            self.app_title,
            self.queries.join("|"),
            self.action_ids_match,
            self.enter_status,
            self.deferred,
            self.aliases,
            self.matched_fields
        )
    }

    fn fail(queries: Vec<String>, action_ids: Vec<ActionId>) -> Self {
        Self {
            status: "FAIL",
            app_title: "Open App: WeChat".to_string(),
            queries,
            action_ids_match: same_action_ids(&action_ids),
            enter_status: ActionExecutionStatus::Failed,
            deferred: false,
            aliases: String::new(),
            matched_fields: String::new(),
        }
    }
}

fn smoke_config() -> StdConfig {
    StdConfig {
        data_dir: std::env::temp_dir().join(format!(
            "std-launcher-app-localization-smoke-{}",
            std::process::id()
        )),
        ..StdConfig::default()
    }
}

fn same_action_ids(ids: &[ActionId]) -> bool {
    !ids.is_empty() && ids.windows(2).all(|pair| pair[0] == pair[1])
}

fn unique_join(values: Vec<String>) -> String {
    let mut unique = Vec::new();
    for value in values {
        if !unique.iter().any(|item| item == &value) {
            unique.push(value);
        }
    }
    unique.join("|")
}

fn write_wechat_app_bundle(app: &Path) {
    fs::create_dir_all(app.join("Contents").join("Resources").join("zh_CN.lproj")).unwrap();
    fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>WeChat</string>
<key>CFBundleName</key><string>Weixin</string>
<key>CFBundleIdentifier</key><string>com.tencent.xinWeChat</string>
</dict></plist>"#,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_localization_smoke_matches_wechat_aliases_without_launching() {
        let report = LauncherAppLocalizationSmokeReport::run();

        assert_eq!(report.status, "PASS");
        assert!(report.action_ids_match);
        assert_eq!(
            report.enter_status,
            ActionExecutionStatus::NeedsExternalRunner
        );
        assert!(report.deferred);
        assert!(report.aliases.contains("wechat"));
        assert!(report.aliases.contains("weixin"));
        assert!(report.aliases.contains("\u{5fae}\u{4fe1}"));
        assert!(report.matched_fields.contains("tags"));
        assert!(report.summary().contains("system_apps_scanned=false"));
    }
}
