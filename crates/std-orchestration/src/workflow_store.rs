use crate::{OrchestrationError, Workflow, WorkflowExecution};
use chrono::Utc;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

pub fn load_workflow(path: &Path) -> Result<Workflow, OrchestrationError> {
    let workflow_path = resolve_workflow_path(path);
    let body = fs::read_to_string(&workflow_path)?;

    match workflow_path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => Ok(serde_json::from_str(&body)?),
        Some("md") => parse_markdown_workflow(&body),
        _ => Err(OrchestrationError::InvalidWorkflow(format!(
            "unsupported workflow file extension: {}",
            workflow_path.display()
        ))),
    }
}

pub fn list_workflows(workflows_dir: &Path) -> Result<Vec<PathBuf>, OrchestrationError> {
    if !workflows_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut workflows = Vec::new();
    for entry in fs::read_dir(workflows_dir)? {
        let path = entry?.path();
        if path.is_dir() {
            for candidate in [path.join("workflow.md"), path.join("workflow.json")] {
                if candidate.is_file() {
                    workflows.push(candidate);
                    break;
                }
            }
        } else if matches!(
            path.extension().and_then(|ext| ext.to_str()),
            Some("md") | Some("json")
        ) {
            workflows.push(path);
        }
    }
    workflows.sort();
    Ok(workflows)
}

pub fn write_workflow_markdown(
    workflows_dir: &Path,
    name: &str,
    description: &str,
) -> Result<PathBuf, OrchestrationError> {
    let slug = workflow_slug(name);
    if slug.is_empty() {
        return Err(OrchestrationError::InvalidWorkflow(
            "workflow name must contain ascii letters or digits".to_string(),
        ));
    }

    let workflow_dir = workflows_dir.join(&slug);
    fs::create_dir_all(&workflow_dir)?;
    let workflow_path = workflow_dir.join("workflow.md");
    if workflow_path.exists() {
        return Err(OrchestrationError::InvalidWorkflow(format!(
            "workflow already exists: {}",
            workflow_path.display()
        )));
    }

    let workflow = Workflow::simple(name, description);
    let body = format_workflow_markdown(&workflow);
    fs::write(&workflow_path, body)?;
    Ok(workflow_path)
}

pub fn write_workflow(path: &Path, workflow: &Workflow) -> Result<(), OrchestrationError> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => fs::write(path, serde_json::to_string_pretty(workflow)?)?,
        Some("md") => fs::write(path, format_workflow_markdown(workflow))?,
        _ => {
            return Err(OrchestrationError::InvalidWorkflow(format!(
                "unsupported workflow file extension: {}",
                path.display()
            )));
        }
    }
    Ok(())
}

pub fn append_workflow_execution(
    history_dir: &Path,
    execution: &WorkflowExecution,
) -> Result<PathBuf, OrchestrationError> {
    fs::create_dir_all(history_dir)?;
    let path = workflow_history_path(history_dir);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    let line = serde_json::to_string(execution)?;
    writeln!(file, "{line}")?;
    Ok(path)
}

pub fn read_workflow_executions(
    history_dir: &Path,
    limit: usize,
) -> Result<Vec<WorkflowExecution>, OrchestrationError> {
    let path = workflow_history_path(history_dir);
    if !path.is_file() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut executions: Vec<WorkflowExecution> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        executions.push(serde_json::from_str(&line)?);
    }
    executions.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    executions.truncate(limit);
    Ok(executions)
}

pub fn workflow_history_path(history_dir: &Path) -> PathBuf {
    history_dir.join("workflow-executions.jsonl")
}

pub fn resolve_workflow_input(config: &std_core::StdConfig, workflow: &str) -> Option<PathBuf> {
    let direct = Path::new(workflow);
    if direct.exists() {
        return Some(direct.to_path_buf());
    }

    [
        config.workflows_dir().join(workflow),
        config.workflows_dir().join(format!("{workflow}.json")),
        config.workflows_dir().join(workflow).join("workflow.json"),
        config.workflows_dir().join(workflow).join("workflow.md"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
}

pub fn format_workflow_markdown(workflow: &Workflow) -> String {
    let steps_json = serde_json::to_string(&workflow.steps).unwrap_or_else(|_| "[]".to_string());
    format!(
        "---\nname: \"{}\"\ndescription: \"{}\"\nsteps_json: '{}'\n---\n\n{}\n",
        escape_frontmatter_value(&workflow.name),
        escape_frontmatter_value(&workflow.description),
        steps_json.replace('\'', "\\'"),
        workflow.description
    )
}

pub fn resolve_workflow_path(path: &Path) -> PathBuf {
    if path.is_dir() {
        let markdown = path.join("workflow.md");
        if markdown.is_file() {
            return markdown;
        }
        let json = path.join("workflow.json");
        if json.is_file() {
            return json;
        }
    }
    path.to_path_buf()
}

pub fn invalid_step_index(step_index: usize) -> OrchestrationError {
    OrchestrationError::InvalidWorkflow(format!("step index out of range: {step_index}"))
}

pub fn update_workflow_timestamp(workflow: &mut Workflow) {
    workflow.updated_at = Utc::now();
}

fn workflow_slug(name: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }
    if slug.ends_with('-') {
        slug.pop();
    }
    slug
}

fn escape_frontmatter_value(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn parse_markdown_workflow(body: &str) -> Result<Workflow, OrchestrationError> {
    let trimmed = body.trim_start();
    if !trimmed.starts_with("---") {
        return Err(OrchestrationError::InvalidWorkflow(
            "workflow.md must start with frontmatter".to_string(),
        ));
    }

    let mut parts = trimmed.splitn(3, "---");
    parts.next();
    let frontmatter = parts.next().ok_or_else(|| {
        OrchestrationError::InvalidWorkflow("missing workflow frontmatter".to_string())
    })?;
    let markdown = parts.next().unwrap_or_default().trim();

    let mut name = None;
    let mut description = None;
    let mut steps = Vec::new();

    for raw_line in frontmatter.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            let value = parse_frontmatter_value(value);
            match key.trim() {
                "name" => name = Some(value),
                "description" => description = Some(value),
                "steps_json" => steps = serde_json::from_str(&value)?,
                _ => {}
            }
        }
    }

    let mut workflow = Workflow::simple(
        name.ok_or_else(|| OrchestrationError::InvalidWorkflow("missing name".to_string()))?,
        description.unwrap_or_else(|| first_markdown_paragraph(markdown)),
    );
    workflow.steps = steps;
    Ok(workflow)
}

fn parse_frontmatter_value(value: &str) -> String {
    let value = value.trim();
    if value.len() >= 2 {
        let first = value.as_bytes()[0];
        let last = value.as_bytes()[value.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return value[1..value.len() - 1].to_string();
        }
    }
    value.to_string()
}

fn first_markdown_paragraph(markdown: &str) -> String {
    markdown
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
        .unwrap_or("")
        .to_string()
}
