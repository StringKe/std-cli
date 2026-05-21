use std_types::{ActionPreview, ActionType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionBarPreviewSummary {
    pub breadcrumb: String,
    pub primary: String,
}

impl ActionBarPreviewSummary {
    pub fn from_preview(preview: &ActionPreview) -> Self {
        Self {
            breadcrumb: format!(
                "{} > {}",
                action_type_label(&preview.action_type),
                preview.title
            ),
            primary: action_primary_text(preview),
        }
    }

    pub fn contract(&self) -> String {
        format!("breadcrumb={},primary={}", self.breadcrumb, self.primary)
    }
}

fn action_primary_text(preview: &ActionPreview) -> String {
    if preview.primary_command.trim().is_empty() {
        preview.subtitle.clone()
    } else {
        preview.primary_command.clone()
    }
}

fn action_type_label(action_type: &ActionType) -> &'static str {
    match action_type {
        ActionType::AppLaunch => std_egui::i18n::t("launcher.results.kind.app"),
        ActionType::Workflow => std_egui::i18n::t("launcher.results.kind.workflow"),
        ActionType::Command => std_egui::i18n::t("launcher.results.kind.command"),
        ActionType::Memory => std_egui::i18n::t("launcher.results.kind.memory"),
        ActionType::Skill => std_egui::i18n::t("launcher.results.kind.skill"),
        ActionType::Clipboard => std_egui::i18n::t("launcher.results.kind.clipboard"),
        ActionType::Custom(kind) if kind == "file" => {
            std_egui::i18n::t("launcher.results.kind.file")
        }
        ActionType::Custom(_) => std_egui::i18n::t("launcher.results.kind.custom"),
    }
}
