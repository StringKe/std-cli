use std::{fs, path::Path};

pub(crate) fn entity_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(ToString::to_string)
        .or_else(|| {
            fs::canonicalize(path)
                .ok()
                .and_then(|canonical| canonical.file_name().map(|name| name.to_os_string()))
                .and_then(|name| name.into_string().ok())
        })
        .unwrap_or_else(|| "root".to_string())
}

pub(crate) fn slug(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

pub(crate) fn compact_snippet(body: &str, max_chars: usize) -> String {
    body.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(max_chars)
        .collect()
}
