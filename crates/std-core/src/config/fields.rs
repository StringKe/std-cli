use crate::config::StdConfig;
use std::path::PathBuf;

impl StdConfig {
    pub fn get_field(&self, key: &str) -> Option<String> {
        match key {
            "launcher_hotkey" => Some(self.launcher_hotkey.clone()),
            "data_dir" => Some(self.data_dir.display().to_string()),
            "enable_ai" => Some(self.enable_ai.to_string()),
            "theme" => Some(self.theme.clone()),
            "appearance.reduce_motion" => Some(self.appearance.reduce_motion.to_string()),
            "appearance.high_contrast" => Some(self.appearance.high_contrast.to_string()),
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
            "appearance.reduce_motion" => {
                self.appearance.reduce_motion = value.parse::<bool>().map_err(|_| {
                    format!("appearance.reduce_motion must be true or false: {value}")
                })?;
            }
            "appearance.high_contrast" => {
                self.appearance.high_contrast = value.parse::<bool>().map_err(|_| {
                    format!("appearance.high_contrast must be true or false: {value}")
                })?;
            }
            _ => return Err(format!("unknown config key: {key}")),
        }
        Ok(())
    }
}
