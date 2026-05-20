use crate::{HistoricalContext, IndexError};
use chrono::Utc;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn historical_context(path: &Path) -> Result<Vec<HistoricalContext>, IndexError> {
    let mut history = vec![HistoricalContext {
        source: "std-index".to_string(),
        summary: "Local structural analysis snapshot".to_string(),
        observed_at: Utc::now(),
    }];
    if let Some(git_dir) = find_git_dir(path) {
        history.extend(read_git_head_history(&git_dir)?);
    }
    Ok(history)
}

fn find_git_dir(path: &Path) -> Option<PathBuf> {
    let start = if path.is_dir() { path } else { path.parent()? };
    start
        .ancestors()
        .map(|ancestor| ancestor.join(".git"))
        .find(|candidate| candidate.is_dir())
}

fn read_git_head_history(git_dir: &Path) -> Result<Vec<HistoricalContext>, IndexError> {
    let path = git_dir.join("logs").join("HEAD");
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let body = fs::read_to_string(path)?;
    let now = Utc::now();
    let mut entries = body
        .lines()
        .rev()
        .filter_map(git_log_summary)
        .take(5)
        .map(|summary| HistoricalContext {
            source: "git HEAD".to_string(),
            summary,
            observed_at: now,
        })
        .collect::<Vec<_>>();
    entries.reverse();
    Ok(entries)
}

fn git_log_summary(line: &str) -> Option<String> {
    let (_, message) = line.split_once('\t')?;
    let message = message.trim();
    if message.is_empty() {
        None
    } else {
        Some(message.to_string())
    }
}
