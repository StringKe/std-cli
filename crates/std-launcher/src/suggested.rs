#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SuggestedWorkflowRow {
    pub title_key: &'static str,
    pub detail_key: &'static str,
    pub shortcut: &'static str,
    pub query: &'static str,
}

pub fn suggested_workflow_rows() -> [SuggestedWorkflowRow; 3] {
    [
        SuggestedWorkflowRow {
            title_key: "launcher.empty.suggestion.rebuild.title",
            detail_key: "launcher.empty.suggestion.rebuild.detail",
            shortcut: "/",
            query: "/rebuild index",
        },
        SuggestedWorkflowRow {
            title_key: "launcher.empty.suggestion.ask.title",
            detail_key: "launcher.empty.suggestion.ask.detail",
            shortcut: "?",
            query: "? ",
        },
        SuggestedWorkflowRow {
            title_key: "launcher.empty.suggestion.studio.title",
            detail_key: "launcher.empty.suggestion.studio.detail",
            shortcut: ">",
            query: "> studio",
        },
    ]
}
