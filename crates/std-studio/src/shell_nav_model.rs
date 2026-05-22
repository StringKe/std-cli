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
}

pub(crate) fn studio_nav_sections() -> Vec<StudioNavSection> {
    vec![
        StudioNavSection {
            kind: StudioNavSectionKind::Workspace,
            title_key: "studio.shell.workspace.title",
            detail_key: "studio.shell.workspace.detail",
            items: vec![
                nav_item(StudioPane::Workflows),
                nav_item(StudioPane::Apps),
                nav_item(StudioPane::Memory),
            ],
        },
        StudioNavSection {
            kind: StudioNavSectionKind::Tools,
            title_key: "studio.shell.tools.title",
            detail_key: "studio.shell.tools.detail",
            items: vec![
                nav_item(StudioPane::Plugins),
                nav_item(StudioPane::Analysis),
                nav_item(StudioPane::Settings),
                nav_item(StudioPane::Operations),
            ],
        },
        StudioNavSection {
            kind: StudioNavSectionKind::Recent,
            title_key: "studio.shell.recent.title",
            detail_key: "studio.shell.recent.detail",
            items: vec![
                nav_item(StudioPane::Dashboard),
                nav_item(StudioPane::History),
            ],
        },
    ]
}

fn nav_item(pane: StudioPane) -> StudioNavItem {
    StudioNavItem {
        pane,
        title: pane.label(),
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
            vec!["Workflow Workbench", "Apps", "Memory Browser"]
        );
        assert_eq!(
            section_titles(&sections[1]),
            vec!["Plugin Manager", "Index Analysis", "设置", "Operations"]
        );
        assert_eq!(section_titles(&sections[2]), vec!["Dashboard", "History"]);
    }

    #[test]
    fn nav_model_routes_every_docs22_item_through_workspace_navigation() {
        let sections = studio_nav_sections();
        let item_count = sections
            .iter()
            .map(|section| section.items.len())
            .sum::<usize>();

        assert_eq!(item_count, 9);
    }

    fn section_titles(section: &StudioNavSection) -> Vec<&'static str> {
        section.items.iter().map(|item| item.title).collect()
    }
}
