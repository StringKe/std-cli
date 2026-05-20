use crate::config::StdConfig;
use std::path::PathBuf;

impl StdConfig {
    pub fn get_field(&self, key: &str) -> Option<String> {
        match key {
            "launcher_hotkey" => Some(self.launcher_hotkey.clone()),
            "data_dir" => Some(self.data_dir.display().to_string()),
            "enable_ai" => Some(self.enable_ai.to_string()),
            "theme" => Some(self.theme.clone()),
            _ => None,
        }
    }

    pub fn set_field(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "launcher_hotkey" => self.launcher_hotkey = value.to_string(),
            "data_dir" => self.data_dir = PathBuf::from(value),
            "enable_ai" => {
                self.enable_ai = value
                    .parse::<bool>()
                    .map_err(|_| format!("enable_ai must be true or false: {value}"))?;
            }
            "theme" => self.theme = value.to_string(),
            _ => return Err(format!("unknown config key: {key}")),
        }
        Ok(())
    }
}
