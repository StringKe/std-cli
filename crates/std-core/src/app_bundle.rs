use plist::{Dictionary, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};
use std_types::{Action, ActionType, RegistryEntry};

pub(crate) fn discover_app_actions(local_apps_dir: &Path) -> Vec<RegistryEntry> {
    if crate::std_test_mode_enabled() {
        return discover_apps_in_dir(local_apps_dir);
    }
    [
        local_apps_dir.to_path_buf(),
        default_apps_dir(),
        system_apps_dir(),
    ]
    .into_iter()
    .flat_map(|dir| discover_apps_in_dir(&dir))
    .collect()
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
    let mut registry_entry = RegistryEntry::from_action(
        Action::new(
            format!("Open App: {}", profile.display_name),
            format!("Launch macOS app at {}", path.display()),
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

struct AppProfile {
    display_name: String,
    bundle_name: String,
    names: Vec<String>,
}

impl AppProfile {
    fn read(path: &Path) -> Option<Self> {
        let bundle_name = path.file_stem()?.to_str()?.to_string();
        let mut names = read_info_plist_names(path);
        names.extend(read_localized_info_plist_names(path));
        names.push(bundle_name.clone());
        names.extend(derived_aliases(&names));
        names = unique_non_empty(names);
        let display_name = names
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

fn read_info_plist_names(path: &Path) -> Vec<String> {
    let plist_path = path.join("Contents").join("Info.plist");
    let Ok(value) = Value::from_file(plist_path) else {
        return Vec::new();
    };
    let Some(dict) = value.as_dictionary() else {
        return Vec::new();
    };
    bundle_name_fields(dict)
        .into_iter()
        .chain(bundle_url_schemes(dict))
        .collect()
}

fn bundle_name_fields(dict: &Dictionary) -> Vec<String> {
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
    value.replace("\\\"", "\"").replace("\\\\", "\\")
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
