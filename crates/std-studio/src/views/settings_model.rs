#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SettingsCategory {
    Appearance,
    Hotkeys,
    AiProvider,
    Index,
    Plugins,
    Privacy,
    About,
}

impl SettingsCategory {
    pub(crate) const ALL: [Self; 7] = [
        Self::Appearance,
        Self::Hotkeys,
        Self::AiProvider,
        Self::Index,
        Self::Plugins,
        Self::Privacy,
        Self::About,
    ];

    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Appearance => "appearance",
            Self::Hotkeys => "hotkeys",
            Self::AiProvider => "ai-provider",
            Self::Index => "index",
            Self::Plugins => "plugins",
            Self::Privacy => "privacy",
            Self::About => "about",
        }
    }

    pub(crate) fn title_key(self) -> &'static str {
        match self {
            Self::Appearance => "studio.settings.category.appearance",
            Self::Hotkeys => "studio.settings.category.hotkeys",
            Self::AiProvider => "studio.settings.category.ai_provider",
            Self::Index => "studio.settings.category.index",
            Self::Plugins => "studio.settings.category.plugins",
            Self::Privacy => "studio.settings.category.privacy",
            Self::About => "studio.settings.category.about",
        }
    }

    pub(crate) fn detail_key(self) -> &'static str {
        match self {
            Self::Appearance => "studio.settings.category.appearance.detail",
            Self::Hotkeys => "studio.settings.category.hotkeys.detail",
            Self::AiProvider => "studio.settings.category.ai_provider.detail",
            Self::Index => "studio.settings.category.index.detail",
            Self::Plugins => "studio.settings.category.plugins.detail",
            Self::Privacy => "studio.settings.category.privacy.detail",
            Self::About => "studio.settings.category.about.detail",
        }
    }
}

pub(crate) struct SettingsContract {
    pub(crate) categories: Vec<&'static str>,
    pub(crate) surface: &'static str,
    pub(crate) navigation: &'static str,
    pub(crate) hotkey_source: &'static str,
    pub(crate) hotkey_reset: &'static str,
    pub(crate) hotkey_control: &'static str,
    pub(crate) theme_modes: Vec<&'static str>,
    pub(crate) theme_control: &'static str,
    pub(crate) ai_control: &'static str,
    pub(crate) storage_control: &'static str,
}

pub(crate) fn settings_contract() -> SettingsContract {
    SettingsContract {
        categories: SettingsCategory::ALL
            .iter()
            .map(|category| category.key())
            .collect(),
        surface: "internal-workspace-pane",
        navigation: "left-category-rail",
        hotkey_source: "default-or-user",
        hotkey_reset: "reset-to-default",
        hotkey_control: "token-binding-row",
        theme_modes: vec!["system", "dark", "light"],
        theme_control: "segmented-control",
        ai_control: "token-toggle-row",
        storage_control: "token-path-row",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_contract_matches_studio_spec_categories() {
        let contract = settings_contract();

        assert_eq!(
            contract.categories,
            vec![
                "appearance",
                "hotkeys",
                "ai-provider",
                "index",
                "plugins",
                "privacy",
                "about",
            ]
        );
        assert_eq!(contract.surface, "internal-workspace-pane");
        assert_eq!(contract.navigation, "left-category-rail");
        assert_eq!(contract.hotkey_source, "default-or-user");
        assert_eq!(contract.hotkey_reset, "reset-to-default");
        assert_eq!(contract.hotkey_control, "token-binding-row");
        assert_eq!(contract.theme_modes, vec!["system", "dark", "light"]);
        assert_eq!(contract.theme_control, "segmented-control");
        assert_eq!(contract.ai_control, "token-toggle-row");
        assert_eq!(contract.storage_control, "token-path-row");
    }
}
