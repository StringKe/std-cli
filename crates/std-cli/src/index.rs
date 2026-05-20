use crate::CliError;
use clap::Subcommand;
use std::path::{Path, PathBuf};
use std_core::StdCore;
use std_index::{FileIndexOptions, IndexDocument, Indexer};

#[derive(Debug, Subcommand)]
pub enum IndexCommand {
    Rebuild {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    List,
    Show {
        name: String,
    },
    Inspect {
        name: String,
        #[arg(short, long, default_value_t = 8)]
        limit: usize,
    },
    Coverage,
    Search {
        query: String,
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    Ask {
        query: String,
        #[arg(short, long, default_value_t = 3)]
        limit: usize,
    },
}

#[derive(Debug, Subcommand)]
pub enum FilesCommand {
    Index {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long, default_value_t = 6)]
        max_depth: usize,
        #[arg(long, default_value_t = 5_000)]
        max_files: usize,
    },
    Search {
        query: String,
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
}

pub(crate) fn handle_index(core: &StdCore, command: IndexCommand) -> Result<String, CliError> {
    match command {
        IndexCommand::Rebuild { path } => rebuild_index(core, &path),
        IndexCommand::List => list_indexes(core),
        IndexCommand::Show { name } => show_index(core, &name),
        IndexCommand::Inspect { name, limit } => inspect_index(core, &name, limit),
        IndexCommand::Coverage => coverage_index(core),
        IndexCommand::Search { query, limit } => search_indexes(core, &query, limit),
        IndexCommand::Ask { query, limit } => ask_indexes(core, &query, limit),
    }
}

pub(crate) fn handle_files(core: &StdCore, command: FilesCommand) -> Result<String, CliError> {
    match command {
        FilesCommand::Index {
            path,
            max_depth,
            max_files,
        } => index_files(core, &path, max_depth, max_files),
        FilesCommand::Search { query, limit } => search_files(core, &query, limit),
    }
}

fn rebuild_index(core: &StdCore, path: &Path) -> Result<String, CliError> {
    let (doc, written) = core.analyze_and_store_entity(path)?;
    Ok(format!(
        "index rebuilt\nentity={}\ncomponents={}\noutput={}",
        doc.overview.path.display(),
        doc.components.len(),
        written.display()
    ))
}

fn list_indexes(core: &StdCore) -> Result<String, CliError> {
    let documents = Indexer::read_documents(&core.config.index_dir())?;
    let lines = documents
        .into_iter()
        .map(|document| {
            format!(
                "{}\t{:?}\t{}\tcomponents={}\trelations={}",
                document.overview.name,
                document.overview.kind,
                document.overview.path.display(),
                document.components.len(),
                document.relations.len()
            )
        })
        .collect::<Vec<_>>();
    Ok(lines.join("\n"))
}

fn show_index(core: &StdCore, name: &str) -> Result<String, CliError> {
    let document = Indexer::find_document(&core.config.index_dir(), name)?
        .ok_or_else(|| CliError::IndexNotFound(name.to_string()))?;
    format_index_document(&document)
}

fn inspect_index(core: &StdCore, name: &str, limit: usize) -> Result<String, CliError> {
    let inspection = Indexer::inspect_document(&core.config.index_dir(), name, limit)?
        .ok_or_else(|| CliError::IndexNotFound(name.to_string()))?;
    Ok(serde_json::to_string_pretty(&inspection)?)
}

fn coverage_index(core: &StdCore) -> Result<String, CliError> {
    let report = Indexer::coverage_report(&core.config.index_dir())?;
    Ok(serde_json::to_string_pretty(&report)?)
}

fn search_indexes(core: &StdCore, query: &str, limit: usize) -> Result<String, CliError> {
    let results = Indexer::search_documents(&core.config.index_dir(), query, limit)?;
    let lines = results
        .into_iter()
        .map(|result| {
            format!(
                "{}\t{}\t{}\t{}",
                result.document.overview.name,
                result.score,
                result.matched_fields.join(","),
                result.document.overview.path.display()
            )
        })
        .collect::<Vec<_>>();
    Ok(lines.join("\n"))
}

fn ask_indexes(core: &StdCore, query: &str, limit: usize) -> Result<String, CliError> {
    let answer = Indexer::answer_from_documents(&core.config.index_dir(), query, limit)?;
    Ok(serde_json::to_string_pretty(&answer)?)
}

fn index_files(
    core: &StdCore,
    path: &Path,
    max_depth: usize,
    max_files: usize,
) -> Result<String, CliError> {
    core.ensure_storage()?;
    let options = FileIndexOptions {
        max_depth,
        max_files,
        ..FileIndexOptions::default()
    };
    let (index, written) = core.index_and_store_files(path, options)?;
    Ok(format!(
        "files indexed\nroot={}\nentries={}\noutput={}",
        index.root.display(),
        index.entries.len(),
        written.display()
    ))
}

fn search_files(core: &StdCore, query: &str, limit: usize) -> Result<String, CliError> {
    let results = Indexer::search_indexed_files(&core.config.index_dir(), query, limit)?;
    let lines = results
        .into_iter()
        .map(|result| {
            format!(
                "{}\t{}\t{}",
                result.entry.path.display(),
                result.score,
                result.matched_fields.join(",")
            )
        })
        .collect::<Vec<_>>();
    Ok(lines.join("\n"))
}

pub(crate) fn format_index_document(doc: &IndexDocument) -> Result<String, CliError> {
    Ok(serde_json::to_string_pretty(doc)?)
}
