use crate::SymbolRelation;
use plist::{Dictionary, Value};
use std::{fs, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AppBundleMetadata {
    pub(crate) display_name: String,
    pub(crate) aliases: Vec<String>,
}

impl AppBundleMetadata {
    pub(crate) fn read(path: &Path) -> Option<Self> {
        let fallback_name = path.file_stem()?.to_str()?.to_string();
        let info = read_info_plist_dictionary(path);
        let mut display_names = info
            .as_ref()
            .map(bundle_display_name_fields)
            .unwrap_or_default();
        display_names.extend(read_localized_info_plist_names(path));
        display_names.push(fallback_name.clone());

        let mut aliases = display_names.clone();
        if let Some(dict) = &info {
            aliases.extend(bundle_alias_fields(dict));
        }
        aliases.push(fallback_name.clone());
        aliases.extend(derived_aliases(&aliases));
        let display_name = unique_non_empty(display_names)
            .first()
            .cloned()
            .unwrap_or(fallback_name);

        Some(Self {
            display_name,
            aliases: unique_non_empty(aliases),
        })
    }

    pub(crate) fn summary(&self) -> String {
        format!("macOS app bundle aliases: {}", self.aliases.join(", "))
    }

    pub(crate) fn alias_relations(&self, path: &Path) -> Vec<SymbolRelation> {
        self.aliases
            .iter()
            .map(|alias| SymbolRelation {
                symbol: alias.clone(),
                relation: "app_bundle_alias".to_string(),
                target: path.display().to_string(),
            })
            .collect()
    }
}

fn read_info_plist_dictionary(path: &Path) -> Option<Dictionary> {
    Value::from_file(path.join("Contents").join("Info.plist"))
        .ok()?
        .into_dictionary()
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

fn read_localized_info_plist_names(path: &Path) -> Vec<String> {
    let resources = path.join("Contents").join("Resources");
    let Ok(entries) = fs::read_dir(resources) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path().join("InfoPlist.strings"))
        .flat_map(|path| read_localized_names_file(&path))
        .collect()
}

fn read_localized_names_file(path: &Path) -> Vec<String> {
    read_localized_names_plist(path).unwrap_or_else(|| {
        read_text_file(path)
            .map(|body| localized_name_values(&body))
            .unwrap_or_default()
    })
}

fn read_localized_names_plist(path: &Path) -> Option<Vec<String>> {
    let value = Value::from_file(path).ok()?;
    let dict = value.as_dictionary()?;
    Some(bundle_display_name_fields(dict))
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
                .trim()
                .trim_matches('"')
                .trim();
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
    let has_wechat = names.iter().any(|name| {
        let normalized = normalize_ascii_alias(name);
        normalized == "wechat" || normalized == "weixin" || normalized.contains("xinwechat")
    });
    if names.iter().any(|name| name == "微信") || has_wechat {
        return vec![
            "微信".to_string(),
            "wechat".to_string(),
            "weixin".to_string(),
        ];
    }
    Vec::new()
}

fn normalize_ascii_alias(value: &str) -> String {
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
