use std_studio::StudioPane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StudioNavSectionKind {
    Workspace,
    Tools,
    Recent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioNavSection {
    pub(crate) kind: StudioNavSectionKind,
    pub(crate) title_key: &'static str,
    pub(crate) detail_key: &'static str,
    pub(crate) items: Vec<StudioNavItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioNavItem {
    pub(crate) pane: StudioPane,
    pub(crate) title: &'static str,
    pub(crate) opens_workspace_pane: bool,
}

pub(crate) fn studio_nav_sections() -> Vec<StudioNavSection> {
    vec![
        StudioNavSection {
            kind: StudioNavSectionKind::Workspace,
            title_key: "studio.shell.workspace.title",
            detail_key: "studio.shell.workspace.detail",
            items: vec![
                nav_item(StudioPane::Workflows, false),
                nav_item(StudioPane::Apps, false),
                nav_item(StudioPane::Memory, true),
            ],
        },
        StudioNavSection {
            kind: StudioNavSectionKind::Tools,
            title_key: "studio.shell.tools.title",
            detail_key: "studio.shell.tools.detail",
            items: vec![
                nav_item(StudioPane::Plugins, true),
                nav_item(StudioPane::Analysis, true),
                nav_item(StudioPane::Settings, true),
                nav_item(StudioPane::Operations, false),
            ],
        },
        StudioNavSection {
            kind: StudioNavSectionKind::Recent,
            title_key: "studio.shell.recent.title",
            detail_key: "studio.shell.recent.detail",
            items: vec![
                nav_item(StudioPane::Dashboard, false),
                nav_item(StudioPane::History, true),
            ],
        },
    ]
}

fn nav_item(pane: StudioPane, opens_workspace_pane: bool) -> StudioNavItem {
    StudioNavItem {
        pane,
        title: pane.label(),
        opens_workspace_pane,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_nav_sections_match_docs_22_sidebar_groups() {
        let sections = studio_nav_sections();

        assert_eq!(
            sections
                .iter()
                .map(|section| section.kind)
                .collect::<Vec<_>>(),
            vec![
                StudioNavSectionKind::Workspace,
                StudioNavSectionKind::Tools,
                StudioNavSectionKind::Recent
            ]
        );
        assert_eq!(
            section_titles(&sections[0]),
            vec!["Workflows", "Apps", "Memory"]
        );
        assert_eq!(
            section_titles(&sections[1]),
            vec!["Plugins", "Analysis", "Settings", "Operations"]
        );
        assert_eq!(section_titles(&sections[2]), vec!["Dashboard", "History"]);
    }

    #[test]
    fn nav_model_marks_deep_tools_as_workspace_panes() {
        let sections = studio_nav_sections();
        let tools = &sections[1];

        assert!(tools.items[0].opens_workspace_pane);
        assert!(tools.items[1].opens_workspace_pane);
        assert!(tools.items[2].opens_workspace_pane);
        assert!(!tools.items[3].opens_workspace_pane);
    }

    fn section_titles(section: &StudioNavSection) -> Vec<&'static str> {
        section.items.iter().map(|item| item.title).collect()
    }
}
