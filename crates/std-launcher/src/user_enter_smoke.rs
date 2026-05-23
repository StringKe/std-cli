use crate::{LauncherKey, LauncherState};
use std_core::{StdConfig, StdCore};
use std_types::ActionExecutionStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherUserEnterSmokeReport {
    pub status: ActionExecutionStatus,
    pub route: &'static str,
    pub mode: &'static str,
    pub deferred: bool,
    pub reason: String,
    pub feedback_visible: bool,
    pub feedback_title: String,
    pub window_policy: &'static str,
    pub app_scope: &'static str,
    pub real_execution_gate: &'static str,
    pub localized_app_enter_contract: String,
}

impl LauncherUserEnterSmokeReport {
    pub fn run() -> Self {
        let root = std::env::temp_dir().join(format!(
            "std-launcher-user-enter-smoke-{}",
            std::process::id()
        ));
        let config = StdConfig {
            data_dir: root.join("data"),
            ..StdConfig::default()
        };
        write_fixture_app(&config);
        write_localized_wechat_fixture_app(&config);
        let core = StdCore::with_config(config);
        let mut state = LauncherState::with_core(core);
        state.controller.show();
        state.update_query("Launcher User Enter App");
        let execution = state
            .handle_keyboard_input_by_user(LauncherKey::Enter, false)
            .expect("fixture app must be searchable");
        let reason = execution
            .output
            .as_ref()
            .and_then(|output| output.get("reason"))
            .and_then(|value| value.as_str())
            .unwrap_or("none")
            .to_string();
        let status = execution.status.clone();
        let report = Self {
            status: status.clone(),
            route: "Enter>handle_keyboard_input_by_user",
            mode: "ReviewFirst",
            deferred: status == ActionExecutionStatus::NeedsExternalRunner,
            reason,
            feedback_visible: state.view.feedback.is_some(),
            feedback_title: state
                .view
                .feedback
                .as_ref()
                .map(|feedback| feedback.title.clone())
                .unwrap_or_else(|| "none".to_string()),
            window_policy: "NeedsExternalRunner->keep-open",
            app_scope: "local_fixture_app_only",
            real_execution_gate: "installed-hotkey-or-background-ui-acceptance",
            localized_app_enter_contract: localized_app_enter_contract(&state.core),
        };
        let _ = std::fs::remove_dir_all(root);
        report
    }

    pub fn pass(&self) -> bool {
        self.status == ActionExecutionStatus::NeedsExternalRunner
            && self.route == "Enter>handle_keyboard_input_by_user"
            && self.mode == "ReviewFirst"
            && self.deferred
            && self.reason == "review command before running external action"
            && self.feedback_visible
            && self.feedback_title == std_egui::i18n::t("launcher.feedback.deferred")
            && self.window_policy == "NeedsExternalRunner->keep-open"
            && self.app_scope == "local_fixture_app_only"
            && self.real_execution_gate == "installed-hotkey-or-background-ui-acceptance"
            && self
                .localized_app_enter_contract
                .contains("same_action=true")
            && self.localized_app_enter_contract.contains(
                "queries=wechat:NeedsExternalRunner|weixin:NeedsExternalRunner|微信:NeedsExternalRunner",
            )
            && self
                .localized_app_enter_contract
                .contains("desktop_open=default_review_first")
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_user_enter_smoke {}\nstatus={:?}\nroute={}\nmode={}\ndeferred={}\nreason={}\nfeedback_visible={}\nfeedback_title={}\nwindow_policy={}\napp_scope={}\nreal_execution_gate={}\nlocalized_app_enter_contract={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.status,
            self.route,
            self.mode,
            self.deferred,
            self.reason,
            self.feedback_visible,
            self.feedback_title,
            self.window_policy,
            self.app_scope,
            self.real_execution_gate,
            self.localized_app_enter_contract
        )
    }
}

fn write_fixture_app(config: &StdConfig) {
    let app = config.apps_dir().join("LauncherUserEnterApp.app");
    let contents = app.join("Contents");
    let _ = std::fs::create_dir_all(&contents);
    let _ = std::fs::write(
        contents.join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>Launcher User Enter App</string>
<key>CFBundleName</key><string>LauncherUserEnterApp</string>
</dict></plist>"#,
    );
}

fn write_localized_wechat_fixture_app(config: &StdConfig) {
    let app = config.apps_dir().join("LauncherWeChatFixture.app");
    let localized = app.join("Contents").join("Resources").join("zh_CN.lproj");
    let _ = std::fs::create_dir_all(&localized);
    let _ = std::fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>WeChat</string>
<key>CFBundleName</key><string>Weixin</string>
<key>CFBundleExecutable</key><string>WeChat</string>
<key>CFBundleIdentifier</key><string>com.tencent.xinWeChat</string>
</dict></plist>"#,
    );
    let _ = std::fs::write(
        localized.join("InfoPlist.strings"),
        "\"CFBundleDisplayName\" = \"\\U5fae\\U4fe1\";",
    );
}

fn localized_app_enter_contract(core: &StdCore) -> String {
    let queries = ["wechat", "weixin", "微信"];
    let mut action_ids = Vec::new();
    let mut statuses = Vec::new();
    for query in queries {
        let mut state = LauncherState::with_core(core.clone());
        state.controller.show();
        state.update_query(query);
        let action_id = state.view.selected_result().map(|result| result.action.id);
        if let Some(action_id) = action_id {
            action_ids.push(action_id);
        }
        let status = state
            .handle_keyboard_input_by_user(LauncherKey::Enter, false)
            .map(|execution| format!("{:?}", execution.status))
            .unwrap_or_else(|| "Missing".to_string());
        statuses.push(format!("{query}:{status}"));
    }
    let same_action = action_ids.len() == queries.len()
        && action_ids
            .first()
            .is_some_and(|first| action_ids.iter().all(|id| id == first));
    format!(
        "same_action={same_action};queries={};desktop_open=default_review_first;aliases=wechat|weixin|微信",
        statuses.join("|")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_enter_smoke_proves_launcher_user_route_without_desktop_open() {
        let report = LauncherUserEnterSmokeReport::run();

        assert!(report.pass(), "{}", report.summary());
        assert!(report
            .summary()
            .contains("route=Enter>handle_keyboard_input_by_user"));
        assert!(report.summary().contains("mode=ReviewFirst"));
        assert!(report.summary().contains("status=NeedsExternalRunner"));
        assert!(report.summary().contains("local_fixture_app_only"));
        assert!(report
            .summary()
            .contains("localized_app_enter_contract=same_action=true"));
        assert!(report.summary().contains("aliases=wechat|weixin|微信"));
    }
}
