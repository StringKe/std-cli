use crate::CliError;
use std::{
    fs,
    path::{Path, PathBuf},
};

const LAUNCHER_MIN_WIDTH: u32 = 720;
const LAUNCHER_MIN_HEIGHT: u32 = 64;
const STUDIO_MIN_WIDTH: u32 = 1080;
const STUDIO_MIN_HEIGHT: u32 = 640;

pub(crate) struct CaptureManifestEntry<'a> {
    pub(crate) path: &'a str,
    pub(crate) declared_bytes: usize,
    pub(crate) declared_width: u32,
    pub(crate) declared_height: u32,
    pub(crate) surface: &'a str,
    pub(crate) theme: &'a str,
    pub(crate) scenario: &'a str,
}

pub(crate) fn verify_capture_png(
    root: &Path,
    entry: &CaptureManifestEntry<'_>,
) -> Result<(), CliError> {
    let png_path = capture_path(root, entry.path);
    let bytes = fs::read(&png_path).map_err(|error| {
        CliError::Doctor(format!(
            "unable to read capture png for {} {} {}: {error}",
            entry.surface, entry.theme, entry.scenario
        ))
    })?;
    if bytes.len() != entry.declared_bytes {
        return Err(CliError::Doctor(format!(
            "capture manifest byte count mismatch for {} {} {}: declared={} actual={}",
            entry.surface,
            entry.theme,
            entry.scenario,
            entry.declared_bytes,
            bytes.len()
        )));
    }
    if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Err(CliError::Doctor(format!(
            "capture file must be PNG for {} {} {}",
            entry.surface, entry.theme, entry.scenario
        )));
    }
    let (actual_width, actual_height) = png_dimensions(&bytes)?;
    if actual_width != entry.declared_width || actual_height != entry.declared_height {
        return Err(CliError::Doctor(format!(
            "capture manifest dimensions mismatch for {} {} {}: declared={}x{} actual={}x{}",
            entry.surface,
            entry.theme,
            entry.scenario,
            entry.declared_width,
            entry.declared_height,
            actual_width,
            actual_height
        )));
    }
    verify_minimum_dimensions(entry, actual_width, actual_height)
}

fn verify_minimum_dimensions(
    entry: &CaptureManifestEntry<'_>,
    width: u32,
    height: u32,
) -> Result<(), CliError> {
    let (min_width, min_height) = minimum_dimensions(entry.surface)?;
    if width >= min_width && height >= min_height {
        return Ok(());
    }
    Err(CliError::Doctor(format!(
        "capture PNG too small for {} {} {}: actual={}x{} minimum={}x{}",
        entry.surface, entry.theme, entry.scenario, width, height, min_width, min_height
    )))
}

fn minimum_dimensions(surface: &str) -> Result<(u32, u32), CliError> {
    match surface {
        "launcher" => Ok((LAUNCHER_MIN_WIDTH, LAUNCHER_MIN_HEIGHT)),
        "studio" => Ok((STUDIO_MIN_WIDTH, STUDIO_MIN_HEIGHT)),
        _ => Err(CliError::Doctor(format!(
            "unknown capture surface: {surface}"
        ))),
    }
}

fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32), CliError> {
    if bytes.len() < 24 || !bytes.starts_with(b"\x89PNG\r\n\x1a\n") || &bytes[12..16] != b"IHDR" {
        return Err(CliError::Doctor(
            "capture file must contain PNG IHDR dimensions".to_string(),
        ));
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    if width == 0 || height == 0 {
        return Err(CliError::Doctor(
            "capture PNG dimensions must be positive".to_string(),
        ));
    }
    Ok((width, height))
}

fn capture_path(root: &Path, path: &str) -> PathBuf {
    let path = Path::new(path);
    if path.is_absolute() {
        path.to_path_buf()
    } else if let Some(name) = path.file_name() {
        root.join(name)
    } else {
        root.join(path)
    }
}
