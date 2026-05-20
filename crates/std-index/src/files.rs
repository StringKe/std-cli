use crate::{
    infer_component_purpose, relations::extract_component_symbols, util::compact_snippet,
    util::entity_name, util::slug, ComponentDigest, FileIndex, FileIndexEntry, FileIndexOptions,
    FileSearchResult, IndexError,
};
use chrono::{DateTime, Utc};
use std::{fs, io, path::Path};

pub(crate) fn index_files(root: &Path, options: FileIndexOptions) -> Result<FileIndex, IndexError> {
    let mut entries = Vec::new();
    collect_files(root, 0, &options, &mut entries)?;
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(FileIndex {
        root: root.to_path_buf(),
        entries,
        created_at: Utc::now(),
    })
}

pub(crate) fn write_file_index(
    index_dir: &Path,
    index: &FileIndex,
) -> Result<std::path::PathBuf, IndexError> {
    fs::create_dir_all(index_dir)?;
    let file_name = format!("files-{}.json", slug(&entity_name(&index.root)));
    let path = index_dir.join(file_name);
    fs::write(&path, serde_json::to_string_pretty(index)?)?;
    Ok(path)
}

pub(crate) fn read_file_indexes(index_dir: &Path) -> Result<Vec<FileIndex>, IndexError> {
    if !index_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut indexes: Vec<FileIndex> = Vec::new();
    for entry in fs::read_dir(index_dir)? {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !name.starts_with("files-")
            || path.extension().and_then(|ext| ext.to_str()) != Some("json")
        {
            continue;
        }
        let body = fs::read_to_string(path)?;
        indexes.push(serde_json::from_str(&body)?);
    }
    indexes.sort_by(|a, b| a.root.cmp(&b.root));
    Ok(indexes)
}

pub(crate) fn digest_directory(
    path: &Path,
    options: &FileIndexOptions,
) -> Result<Vec<ComponentDigest>, IndexError> {
    let mut file_entries = Vec::new();
    collect_files(path, 0, options, &mut file_entries)?;
    let mut components = file_entries
        .into_iter()
        .map(component_from_entry)
        .collect::<Vec<_>>();
    components.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(components)
}

pub(crate) fn digest_file(path: &Path) -> Result<ComponentDigest, IndexError> {
    let metadata = fs::metadata(path)?;
    let body = fs::read_to_string(path).unwrap_or_default();
    let snippet = compact_snippet(&body, 240);
    Ok(ComponentDigest {
        path: path.to_path_buf(),
        purpose: infer_component_purpose(&entity_name(path), &snippet),
        size_bytes: metadata.len(),
        language: language_for_path(path),
        symbols: extract_component_symbols(path, &body),
        snippet,
    })
}

fn component_from_entry(entry: FileIndexEntry) -> ComponentDigest {
    let body = fs::read_to_string(&entry.path).unwrap_or_default();
    ComponentDigest {
        path: entry.path.clone(),
        purpose: infer_component_purpose(&entry.name, &entry.snippet),
        size_bytes: entry.size_bytes,
        language: language_for_path(&entry.path),
        symbols: extract_component_symbols(&entry.path, &body),
        snippet: entry.snippet,
    }
}

fn collect_files(
    path: &Path,
    depth: usize,
    options: &FileIndexOptions,
    entries: &mut Vec<FileIndexEntry>,
) -> Result<(), IndexError> {
    if entries.len() >= options.max_files {
        return Ok(());
    }

    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::PermissionDenied => return Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };

    if metadata.is_file() {
        if should_skip_file(path, metadata.len()) {
            return Ok(());
        }
        entries.push(file_index_entry(path, &metadata, options.max_file_bytes)?);
        return Ok(());
    }

    if !metadata.is_dir() || depth >= options.max_depth {
        return Ok(());
    }

    let mut children = Vec::new();
    for entry in fs::read_dir(path)? {
        let child_path = entry?.path();
        if !should_skip_path(&child_path) {
            children.push(child_path);
        }
    }
    children.sort();

    for child in children {
        if entries.len() >= options.max_files {
            break;
        }
        collect_files(&child, depth + 1, options, entries)?;
    }
    Ok(())
}

fn file_index_entry(
    path: &Path,
    metadata: &fs::Metadata,
    max_file_bytes: u64,
) -> Result<FileIndexEntry, IndexError> {
    let modified_at = metadata.modified().ok().map(DateTime::<Utc>::from);
    let snippet = if metadata.len() <= max_file_bytes {
        fs::read_to_string(path)
            .ok()
            .map(|body| compact_snippet(&body, 240))
            .unwrap_or_default()
    } else {
        String::new()
    };

    Ok(FileIndexEntry {
        path: path.to_path_buf(),
        name: entity_name(path),
        size_bytes: metadata.len(),
        modified_at,
        snippet,
    })
}

fn should_skip_path(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    name.starts_with('.')
        || matches!(
            name,
            "target"
                | "node_modules"
                | "vendor"
                | "dist"
                | "build"
                | ".git"
                | ".DS_Store"
                | "__pycache__"
        )
}

fn should_skip_file(path: &Path, size_bytes: u64) -> bool {
    if should_skip_path(path) {
        return true;
    }
    let Some(ext) = path.extension().and_then(|ext| ext.to_str()) else {
        return false;
    };
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "png"
            | "jpg"
            | "jpeg"
            | "gif"
            | "webp"
            | "ico"
            | "pdf"
            | "zip"
            | "gz"
            | "tar"
            | "mp3"
            | "mp4"
            | "mov"
            | "sqlite"
            | "db"
            | "lock"
    ) || size_bytes == 0
}

fn language_for_path(path: &Path) -> String {
    let Some(ext) = path.extension().and_then(|ext| ext.to_str()) else {
        return "text".to_string();
    };
    match ext.to_ascii_lowercase().as_str() {
        "rs" => "rust",
        "toml" => "toml",
        "json" => "json",
        "md" => "markdown",
        "plist" => "plist",
        "yaml" | "yml" => "yaml",
        "js" => "javascript",
        "ts" => "typescript",
        "sh" => "shell",
        _ => "text",
    }
    .to_string()
}

pub(crate) fn search_file_entries(
    entries: Vec<FileIndexEntry>,
    query: &str,
    limit: usize,
) -> Vec<FileSearchResult> {
    let query = query.trim().to_lowercase();
    let mut results = entries
        .into_iter()
        .filter_map(|entry| search_file_entry(entry, &query))
        .collect::<Vec<_>>();

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results.truncate(limit);
    results
}

fn search_file_entry(entry: FileIndexEntry, query: &str) -> Option<FileSearchResult> {
    let mut score = 0.0;
    let mut matched_fields = Vec::new();
    let name = entry.name.to_lowercase();
    let path = entry.path.display().to_string().to_lowercase();
    let snippet = entry.snippet.to_lowercase();

    if query.is_empty() || name.contains(query) {
        score += 10.0;
        matched_fields.push("name".to_string());
    }
    if !query.is_empty() && path.contains(query) {
        score += 6.0;
        matched_fields.push("path".to_string());
    }
    if !query.is_empty() && snippet.contains(query) {
        score += 4.0;
        matched_fields.push("snippet".to_string());
    }

    (score > 0.0).then_some(FileSearchResult {
        entry,
        score,
        matched_fields,
    })
}
