use crate::config::{
    paths::{home_config_path, project_config_candidates},
    ConfigError, StdConfig,
};
use figment::{
    providers::{Format, Json, Serialized, Toml, Yaml},
    Figment,
};
use std::{
    env,
    path::{Path, PathBuf},
};

impl StdConfig {
    pub fn load() -> Self {
        Self::try_load().unwrap_or_default()
    }

    pub fn try_load() -> Result<Self, ConfigError> {
        Self::try_load_from(env::current_dir().ok().as_deref())
    }

    pub fn load_from(start_dir: Option<&Path>) -> Self {
        Self::try_load_from(start_dir).unwrap_or_default()
    }

    pub fn try_load_from(start_dir: Option<&Path>) -> Result<Self, ConfigError> {
        let mut figment = Figment::from(Serialized::defaults(Self::default()));
        let explicit_config = env::var_os("STDCLI_CONFIG").map(PathBuf::from);

        if let Some(path) = explicit_config.as_ref() {
            figment = merge_config_file(figment, path);
        } else if !runtime_test_config_isolated() {
            figment = merge_config_file(figment, &home_config_path(".std-cli/config.toml"));
            figment = merge_config_file(figment, &home_config_path(".std-cli/config.yaml"));
            figment = merge_config_file(figment, &home_config_path(".config/std-cli/config.toml"));
            figment = merge_config_file(figment, &home_config_path(".config/std-cli/config.yaml"));

            if let Some(dir) = start_dir {
                for path in project_config_candidates(dir) {
                    figment = merge_config_file(figment, &path);
                }
            }
        }

        let mut config: StdConfig = figment
            .extract()
            .map_err(|error| ConfigError::new(format!("config parse failed: {error}")))?;
        if !runtime_test_config_isolated() || explicit_config.is_some() {
            apply_env_overrides(&mut config)?;
        }
        Ok(config)
    }
}

fn runtime_test_config_isolated() -> bool {
    !cfg!(test) && crate::std_test_mode_enabled()
}

fn merge_config_file(figment: Figment, path: &Path) -> Figment {
    if !path.is_file() {
        return figment;
    }

    match path.extension().and_then(|ext| ext.to_str()) {
        Some("toml") => figment.merge(Toml::file(path)),
        Some("json") => figment.merge(Json::file(path)),
        Some("yaml") | Some("yml") => figment.merge(Yaml::file(path)),
        _ => figment,
    }
}

fn apply_env_overrides(config: &mut StdConfig) -> Result<(), ConfigError> {
    for (env_key, field) in [
        ("STDCLI_LAUNCHER_HOTKEY", "launcher_hotkey"),
        ("STD_LAUNCHER_HOTKEY", "launcher_hotkey"),
        ("STDCLI_DATA_DIR", "data_dir"),
        ("STD_DATA_DIR", "data_dir"),
        ("STDCLI_ENABLE_AI", "enable_ai"),
        ("STD_ENABLE_AI", "enable_ai"),
        ("STDCLI_THEME", "theme"),
        ("STD_THEME", "theme"),
        ("STDCLI_REDUCE_MOTION", "appearance.reduce_motion"),
        ("STD_REDUCE_MOTION", "appearance.reduce_motion"),
    ] {
        if let Ok(value) = env::var(env_key) {
            config.set_field(field, &value).map_err(|error| {
                ConfigError::new(format!("{env_key} invalid for {field}: {error}"))
            })?;
        }
    }
    Ok(())
}
