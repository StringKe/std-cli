#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherQueryMode {
    All,
    Command,
    Actions,
    Ask,
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
}
