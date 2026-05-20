use crate::CliError;
use std_core::StdConfig;

pub(crate) fn format_config(config: &StdConfig) -> String {
    format!(
        "launcher_hotkey={}\ndata_dir={}\nworkflows_dir={}\nindex_dir={}\nmemory_dir={}\nhistory_dir={}\nenable_ai={}\ntheme={}",
        config.launcher_hotkey,
        config.data_dir.display(),
        config.workflows_dir().display(),
        config.index_dir().display(),
        config.memory_dir().display(),
        config.history_dir().display(),
        config.enable_ai,
        config.theme
    )
}

pub(crate) fn config_get(config: &StdConfig, key: &str) -> Result<String, CliError> {
    config
        .get_field(key)
        .ok_or_else(|| CliError::Config(format!("unknown config key: {key}")))
}

pub(crate) fn config_set(
    mut config: StdConfig,
    key: &str,
    value: &str,
) -> Result<String, CliError> {
    config.set_field(key, value).map_err(CliError::Config)?;
    let path = StdConfig::writable_config_path();
    config.save_to(&path)?;
    Ok(format!(
        "config set\npath={}\n{}={}",
        path.display(),
        key,
        config
            .get_field(key)
            .ok_or_else(|| CliError::Config(format!("unknown config key: {key}")))?
    ))
}
