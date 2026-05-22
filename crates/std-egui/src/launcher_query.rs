#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LauncherQueryMode {
    All,
    Command,
    Actions,
    Ask,
}

pub(crate) struct LauncherQueryRequest {
    pub(crate) display_query: String,
    pub(crate) search_query: String,
    mode: LauncherQueryMode,
}

impl LauncherQueryRequest {
    pub(crate) fn parse(query: impl Into<String>) -> Self {
        let display_query = normalize_query(query.into());
        let mode = match display_query.trim_start().chars().next() {
            Some('/') => LauncherQueryMode::Command,
            Some('>') => LauncherQueryMode::Actions,
            Some('?') => LauncherQueryMode::Ask,
            _ => LauncherQueryMode::All,
        };
        let search_query = match mode {
            LauncherQueryMode::All => display_query.clone(),
            LauncherQueryMode::Command | LauncherQueryMode::Actions | LauncherQueryMode::Ask => {
                display_query
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .trim()
                    .to_string()
            }
        };
        Self {
            display_query,
            search_query,
            mode,
        }
    }

    pub(crate) fn action_only(&self) -> bool {
        self.mode == LauncherQueryMode::Actions
    }

    pub(crate) fn command_only(&self) -> bool {
        self.mode == LauncherQueryMode::Command
    }

    pub(crate) fn ask_mode(&self) -> bool {
        self.mode == LauncherQueryMode::Ask
    }
}

pub(crate) fn normalize_query(query: String) -> String {
    query.split_whitespace().collect::<Vec<_>>().join(" ")
}
