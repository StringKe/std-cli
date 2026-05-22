use crate::CliError;
use std::{fs, path::Path};

pub(crate) fn check_ui_token_usage(root: &Path) -> Result<(), CliError> {
    for crate_dir in [
        "crates/std-egui/src",
        "crates/std-launcher/src",
        "crates/std-studio/src",
    ] {
        scan_ui_token_usage(&root.join(crate_dir))?;
    }
    check_studio_row_helpers(root)?;
    Ok(())
}

fn scan_ui_token_usage(dir: &Path) -> Result<(), CliError> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            scan_ui_token_usage(&path)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            check_ui_source_tokens(&path)?;
        }
    }
    Ok(())
}

fn check_ui_source_tokens(path: &Path) -> Result<(), CliError> {
    let text = fs::read_to_string(path)?;
    for line in text.lines() {
        for rule in visual_rules() {
            if rule.matches(path, line) {
                return Err(CliError::Doctor(format!(
                    "{} contains UI token bypass: {}",
                    path.display(),
                    rule.term
                )));
            }
        }
    }
    Ok(())
}

struct VisualRule {
    term: &'static str,
    mode: VisualRuleMode,
}

impl VisualRule {
    fn matches(&self, path: &Path, line: &str) -> bool {
        if ui_token_exception(path, self.term) {
            return false;
        }
        match self.mode {
            VisualRuleMode::Any => line.contains(self.term),
            VisualRuleMode::NumericCall => line_contains_numeric_call(line, self.term),
            VisualRuleMode::RawColor => {
                line.contains(self.term) && !line.contains("Color32::TRANSPARENT")
            }
            VisualRuleMode::ShellNumericCall => {
                ui_shell_path(path) && line_contains_numeric_call(line, self.term)
            }
        }
    }
}

enum VisualRuleMode {
    Any,
    NumericCall,
    RawColor,
    ShellNumericCall,
}

fn visual_rules() -> Vec<VisualRule> {
    vec![
        any(".size("),
        any("FontId::new("),
        any("Color32::from_rgb("),
        any("Color32::from_rgba("),
        raw_color("Color32::"),
        numeric("add_space("),
        numeric("Margin::same("),
        numeric("Margin::symmetric("),
        numeric("CornerRadius::same("),
        shell_numeric("egui::vec2("),
    ]
}

fn any(term: &'static str) -> VisualRule {
    VisualRule {
        term,
        mode: VisualRuleMode::Any,
    }
}

fn numeric(term: &'static str) -> VisualRule {
    VisualRule {
        term,
        mode: VisualRuleMode::NumericCall,
    }
}

fn raw_color(term: &'static str) -> VisualRule {
    VisualRule {
        term,
        mode: VisualRuleMode::RawColor,
    }
}

fn shell_numeric(term: &'static str) -> VisualRule {
    VisualRule {
        term,
        mode: VisualRuleMode::ShellNumericCall,
    }
}

fn ui_shell_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| {
            matches!(
                name,
                "ui.rs"
                    | "ui_results.rs"
                    | "ui_empty.rs"
                    | "ui_action_bar.rs"
                    | "ui_action_panel.rs"
                    | "ui_feedback.rs"
                    | "shell.rs"
                    | "shell_navigation.rs"
                    | "shell_overlays.rs"
                    | "context_help.rs"
                    | "host_chrome.rs"
                    | "bottom_panel.rs"
                    | "workspace_tabs.rs"
                    | "workflow_rows.rs"
                    | "plugin_rows.rs"
                    | "analysis_rows.rs"
                    | "app_rows.rs"
                    | "dashboard_rows.rs"
                    | "history_rows.rs"
                    | "memory_rows.rs"
                    | "settings_toggle.rs"
                    | "settings_rows.rs"
                    | "operations_rows.rs"
            )
        })
        .unwrap_or(false)
}

fn line_contains_numeric_call(line: &str, call: &str) -> bool {
    let mut rest = line;
    while let Some(index) = rest.find(call) {
        let after = rest[index + call.len()..].trim_start();
        if after
            .chars()
            .next()
            .is_some_and(|value| value.is_ascii_digit())
        {
            return true;
        }
        rest = &after[after.len().min(1)..];
    }
    false
}

fn ui_token_exception(path: &Path, term: &str) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let Some(parent) = path
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
    else {
        return false;
    };
    if parent == "tokens" || path_has_component(path, "i18n") || file_name == "i18n.rs" {
        return true;
    }
    if file_name == "shell_icons.rs" {
        return true;
    }
    matches!(
        (parent, file_name, term),
        ("src", "main.rs", "egui::vec2(")
            | ("src", "preview.rs", "egui::vec2(")
            | ("src", "window.rs", "egui::vec2(")
            | (_, "ui_empty.rs", "egui::vec2(")
            | (_, "ui_metrics.rs", "egui::vec2(")
            | (_, "ui_metrics_empty.rs", "egui::vec2(")
            | (_, "ui_metrics_results.rs", "egui::vec2(")
            | (_, "ui_metrics_action_panel.rs", "egui::vec2(")
    )
}

fn path_has_component(path: &Path, component: &str) -> bool {
    path.components().any(|part| part.as_os_str() == component)
}

fn check_studio_row_helpers(root: &Path) -> Result<(), CliError> {
    for path in [
        "crates/std-studio/src/views/workflow_rows.rs",
        "crates/std-studio/src/views/plugin_rows.rs",
        "crates/std-studio/src/views/memory_rows.rs",
        "crates/std-studio/src/views/history_rows.rs",
        "crates/std-studio/src/views/settings_rows.rs",
        "crates/std-studio/src/analysis_rows.rs",
        "crates/std-studio/src/app_rows.rs",
        "crates/std-studio/src/operations_rows.rs",
    ] {
        let source = fs::read_to_string(root.join(path))?;
        if !source.contains("row_paint::paint_row_frame") {
            return Err(CliError::Doctor(format!(
                "{path} must use shared Studio row_paint frame helper"
            )));
        }
        if source.contains("fn paint_row_frame") {
            return Err(CliError::Doctor(format!(
                "{path} must not define a local paint_row_frame helper"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn numeric_visual_calls_are_detected() {
        assert!(line_contains_numeric_call(
            "let size = egui::vec2(96.0, 22.0);",
            "egui::vec2("
        ));
        assert!(line_contains_numeric_call(
            ".corner_radius(egui::CornerRadius::same(4))",
            "CornerRadius::same("
        ));
        assert!(!line_contains_numeric_call(
            ".inner_margin(egui::Margin::same(Space::SM))",
            "Margin::same("
        ));
    }

    #[test]
    fn visual_token_gate_covers_launcher_and_studio_sources() {
        let root = super::super::workspace::find_workspace_root().unwrap();

        check_ui_token_usage(&root).unwrap();
    }

    #[test]
    fn visual_token_gate_covers_shared_std_egui_sources() {
        let root = super::super::workspace::find_workspace_root().unwrap();

        check_ui_source_tokens(&root.join("crates/std-egui/src/dashboard.rs")).unwrap();
        check_ui_source_tokens(&root.join("crates/std-egui/src/plugin.rs")).unwrap();
        assert!(ui_token_exception(
            &root.join("crates/std-egui/src/tokens/color.rs"),
            "Color32::from_rgb("
        ));
        assert!(ui_token_exception(
            &root.join("crates/std-egui/src/i18n/catalog/launcher.rs"),
            ".size("
        ));
    }

    #[test]
    fn ui_metric_modules_are_allowed_to_hold_geometry_constants() {
        let path = PathBuf::from("crates/std-launcher/src/ui_metrics.rs");

        assert!(ui_token_exception(&path, "egui::vec2("));
    }

    #[test]
    fn launcher_shared_ui_parts_must_not_bypass_color_tokens() {
        let path = PathBuf::from("crates/std-launcher/src/ui_parts.rs");

        assert!(!ui_token_exception(&path, "Color32::"));
    }

    #[test]
    fn studio_core_rows_use_shared_paint_helpers() {
        let root = super::super::workspace::find_workspace_root().unwrap();

        check_studio_row_helpers(&root).unwrap();
    }
}
