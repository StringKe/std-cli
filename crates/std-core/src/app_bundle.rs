use plist::{Dictionary, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};
use std_types::{Action, ActionType, RegistryEntry};

pub(crate) fn discover_app_actions(local_apps_dir: &Path) -> Vec<RegistryEntry> {
    app_discovery_dirs(local_apps_dir)
        .into_iter()
        .flat_map(|dir| discover_apps_in_dir(&dir))
        .collect()
}

fn app_discovery_dirs(local_apps_dir: &Path) -> Vec<PathBuf> {
    if crate::std_test_mode_enabled() {
        return vec![local_apps_dir.to_path_buf()];
    }
    vec![
        local_apps_dir.to_path_buf(),
        default_apps_dir(),
        system_apps_dir(),
    ]
}

fn discover_apps_in_dir(dir: &Path) -> Vec<RegistryEntry> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("app"))
        .filter_map(app_registry_entry)
        .collect()
}

fn app_registry_entry(path: PathBuf) -> Option<RegistryEntry> {
    let profile = AppProfile::read(&path)?;
    let description = app_description(&path, &profile.names);
    let mut registry_entry = RegistryEntry::from_action(
        Action::new(
            format!("Open App: {}", profile.display_name),
            description,
            "When opening this local macOS application",
            ActionType::AppLaunch,
        ),
        app_tags(&profile.names),
    );
    registry_entry
        .metadata
        .insert("path".to_string(), path.display().to_string());
    registry_entry
        .metadata
        .insert("bundle_name".to_string(), profile.bundle_name);
    registry_entry
        .metadata
        .insert("aliases".to_string(), profile.names.join(","));
    Some(registry_entry)
}

fn app_description(path: &Path, names: &[String]) -> String {
    if names.is_empty() {
        return format!("Launch macOS app at {}", path.display());
    }
    format!("Aliases: {} / Path: {}", names.join(", "), path.display())
}

struct AppProfile {
    display_name: String,
    bundle_name: String,
    names: Vec<String>,
}

impl AppProfile {
    fn read(path: &Path) -> Option<Self> {
        let bundle_name = path.file_stem()?.to_str()?.to_string();
        let mut display_names = read_display_info_plist_names(path);
        display_names.extend(read_localized_info_plist_names(path));
        display_names.push(bundle_name.clone());
        let mut names = display_names.clone();
        names.extend(read_alias_info_plist_names(path));
        names.push(bundle_name.clone());
        names.extend(derived_aliases(&names));
        names = unique_non_empty(names);
        let display_name = unique_non_empty(display_names)
            .first()
            .cloned()
            .unwrap_or_else(|| bundle_name.clone());
        Some(Self {
            display_name,
            bundle_name,
            names,
        })
    }
}

fn read_display_info_plist_names(path: &Path) -> Vec<String> {
    let Some(value) = read_info_plist_value(path) else {
        return Vec::new();
    };
    let Some(dict) = value.as_dictionary() else {
        return Vec::new();
    };
    bundle_display_name_fields(dict)
}

fn read_alias_info_plist_names(path: &Path) -> Vec<String> {
    let Some(value) = read_info_plist_value(path) else {
        return Vec::new();
    };
    let Some(dict) = value.as_dictionary() else {
        return Vec::new();
    };
    bundle_alias_fields(dict)
        .into_iter()
        .chain(bundle_url_schemes(dict))
        .collect()
}

fn read_info_plist_value(path: &Path) -> Option<Value> {
    let plist_path = path.join("Contents").join("Info.plist");
    Value::from_file(plist_path).ok()
}

fn bundle_display_name_fields(dict: &Dictionary) -> Vec<String> {
    ["CFBundleDisplayName", "CFBundleName", "CFBundleExecutable"]
        .into_iter()
        .filter_map(|key| dict.get(key).and_then(Value::as_string))
        .map(ToString::to_string)
        .collect()
}

fn bundle_alias_fields(dict: &Dictionary) -> Vec<String> {
    [
        "CFBundleDisplayName",
        "CFBundleName",
        "CFBundleExecutable",
        "CFBundleIdentifier",
    ]
    .into_iter()
    .filter_map(|key| dict.get(key).and_then(Value::as_string))
    .map(ToString::to_string)
    .collect()
}

fn bundle_url_schemes(dict: &Dictionary) -> Vec<String> {
    let Some(Value::Array(types)) = dict.get("CFBundleURLTypes") else {
        return Vec::new();
    };
    types
        .iter()
        .filter_map(Value::as_dictionary)
        .filter_map(|url_type| url_type.get("CFBundleURLSchemes"))
        .filter_map(Value::as_array)
        .flat_map(|schemes| schemes.iter().filter_map(Value::as_string))
        .map(ToString::to_string)
        .collect()
}

fn read_localized_info_plist_names(path: &Path) -> Vec<String> {
    let resources = path.join("Contents").join("Resources");
    let Ok(entries) = fs::read_dir(resources) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path().join("InfoPlist.strings"))
        .filter_map(|path| read_text_file(&path).ok())
        .flat_map(|body| localized_name_values(&body))
        .collect()
}

fn read_text_file(path: &Path) -> std::io::Result<String> {
    let bytes = fs::read(path)?;
    if bytes.starts_with(&[0xff, 0xfe]) {
        return Ok(decode_utf16(&bytes[2..], false));
    }
    if bytes.starts_with(&[0xfe, 0xff]) {
        return Ok(decode_utf16(&bytes[2..], true));
    }
    String::from_utf8(bytes).map_err(std::io::Error::other)
}

fn decode_utf16(bytes: &[u8], big_endian: bool) -> String {
    let units = bytes.chunks_exact(2).map(|chunk| {
        if big_endian {
            u16::from_be_bytes([chunk[0], chunk[1]])
        } else {
            u16::from_le_bytes([chunk[0], chunk[1]])
        }
    });
    String::from_utf16_lossy(&units.collect::<Vec<_>>())
}

fn localized_name_values(body: &str) -> Vec<String> {
    body.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            let key = localized_key(trimmed)?;
            if !localized_name_key(key) {
                return None;
            }
            let value = trimmed
                .split_once('=')?
                .1
                .trim()
                .trim_end_matches(';')
                .trim();
            let value = value.trim_matches('"').trim();
            (!value.is_empty()).then(|| unescape_strings(value))
        })
        .collect()
}

fn localized_key(line: &str) -> Option<&str> {
    let (key, _) = line.split_once('=')?;
    Some(key.trim().trim_matches('"').trim())
}

fn localized_name_key(key: &str) -> bool {
    matches!(
        key,
        "CFBundleDisplayName" | "CFBundleName" | "CFBundleExecutable"
    )
}

fn unescape_strings(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }
        match chars.next() {
            Some('"') => output.push('"'),
            Some('\\') => output.push('\\'),
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some('t') => output.push('\t'),
            Some('U') | Some('u') => {
                let hex = chars.by_ref().take(4).collect::<String>();
                if let Ok(code) = u16::from_str_radix(&hex, 16) {
                    output.push(char::from_u32(u32::from(code)).unwrap_or('\u{fffd}'));
                }
            }
            Some(other) => output.push(other),
            None => output.push('\\'),
        }
    }
    output
}

fn derived_aliases(names: &[String]) -> Vec<String> {
    let mut aliases = Vec::new();
    let has_wechat = names.iter().any(|name| {
        let normalized = normalize_alias(name);
        normalized == "wechat" || normalized == "weixin" || normalized.contains("xinwechat")
    });
    if names.iter().any(|name| name == "微信") || has_wechat {
        aliases.extend([
            "微信".to_string(),
            "wechat".to_string(),
            "weixin".to_string(),
        ]);
    }
    aliases
}

fn normalize_alias(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn unique_non_empty(values: Vec<String>) -> Vec<String> {
    let mut unique = Vec::new();
    for value in values {
        let value = value.trim().to_string();
        if !value.is_empty() && !unique.iter().any(|item| item == &value) {
            unique.push(value);
        }
    }
    unique
}

fn app_tags(names: &[String]) -> Vec<String> {
    let mut tags = vec!["app".to_string(), "macos".to_string()];
    tags.extend(names.iter().cloned());
    unique_non_empty(tags)
}

fn default_apps_dir() -> PathBuf {
    PathBuf::from("/Applications")
}

fn system_apps_dir() -> PathBuf {
    PathBuf::from("/System/Applications")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_app_discovery_uses_only_local_fixture_dir() {
        let local = PathBuf::from("/tmp/std-cli-fixture-apps");
        let dirs = app_discovery_dirs(&local);

        assert_eq!(dirs, vec![local]);
    }
}
