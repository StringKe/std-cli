#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherQueryMode {
    All,
    Command,
    Actions,
    Ask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherQueryRequest {
    pub display_query: String,
    pub search_query: String,
    pub mode: LauncherQueryMode,
}

impl LauncherQueryMode {
    pub fn from_query(query: &str) -> Self {
        match query.trim_start().chars().next() {
            Some('/') => Self::Command,
            Some('>') => Self::Actions,
            Some('?') => Self::Ask,
            _ => Self::All,
        }
    }

    pub fn tag_label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Command => "Command",
            Self::Actions => "Actions",
            Self::Ask => "Ask",
        }
    }

    pub fn contract_name(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Command => "command",
            Self::Actions => "actions",
            Self::Ask => "ask",
        }
    }
}

impl LauncherQueryRequest {
    pub fn parse(query: impl Into<String>) -> Self {
        let display_query = normalize_query(query.into());
        let mode = LauncherQueryMode::from_query(&display_query);
        let search_query = search_query_for_mode(&display_query, mode);
        Self {
            display_query,
            search_query,
            mode,
        }
    }

    pub fn action_only(&self) -> bool {
        self.mode == LauncherQueryMode::Actions
    }

    pub fn command_only(&self) -> bool {
        self.mode == LauncherQueryMode::Command
    }
}

fn normalize_query(query: String) -> String {
    query.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn search_query_for_mode(query: &str, mode: LauncherQueryMode) -> String {
    match mode {
        LauncherQueryMode::All => query.to_string(),
        LauncherQueryMode::Command | LauncherQueryMode::Actions | LauncherQueryMode::Ask => {
            query.chars().skip(1).collect::<String>().trim().to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_mode_tracks_launcher_prefixes() {
        assert_eq!(
            LauncherQueryMode::from_query("index"),
            LauncherQueryMode::All
        );
        assert_eq!(
            LauncherQueryMode::from_query("/workflow new"),
            LauncherQueryMode::Command
        );
        assert_eq!(
            LauncherQueryMode::from_query("> plugin"),
            LauncherQueryMode::Actions
        );
        assert_eq!(
            LauncherQueryMode::from_query("? rebuild"),
            LauncherQueryMode::Ask
        );
    }

    #[test]
    fn query_request_preserves_display_query_and_strips_prefix_for_search() {
        let command = LauncherQueryRequest::parse("  /workflow   new ");
        let actions = LauncherQueryRequest::parse("> plugin");
        let ask = LauncherQueryRequest::parse("? rebuild index");

        assert_eq!(command.display_query, "/workflow new");
        assert_eq!(command.search_query, "workflow new");
        assert!(!command.action_only());
        assert!(command.command_only());
        assert_eq!(actions.search_query, "plugin");
        assert!(actions.action_only());
        assert!(!actions.command_only());
        assert_eq!(ask.search_query, "rebuild index");
    }
}
