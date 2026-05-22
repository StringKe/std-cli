//! Configuration system for std-cli (full product foundation)

mod fields;
mod load;
mod paths;

use paths::default_data_dir;
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    message: String,
}

impl ConfigError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ConfigError {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StdConfig {
    pub launcher_hotkey: String,
    pub data_dir: PathBuf,
    pub enable_ai: bool,
    pub theme: String,
    #[serde(default)]
    pub appearance: AppearanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppearanceConfig {
    pub reduce_motion: bool,
    pub high_contrast: bool,
    pub reduce_transparency: bool,
    pub ui_scale: f32,
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            reduce_motion: false,
            high_contrast: false,
            reduce_transparency: false,
            ui_scale: 1.0,
        }
    }
}

impl StdConfig {
    pub fn workflows_dir(&self) -> PathBuf {
        self.data_dir.join("workflows")
    }

    pub fn index_dir(&self) -> PathBuf {
        self.data_dir.join("index")
    }

    pub fn memory_dir(&self) -> PathBuf {
        self.data_dir.join("memory")
    }

    pub fn history_dir(&self) -> PathBuf {
        self.data_dir.join("history")
    }

    pub fn plugins_dir(&self) -> PathBuf {
        self.data_dir.join("plugins")
    }

    pub fn apps_dir(&self) -> PathBuf {
        self.data_dir.join("Applications")
    }

    pub fn save_to(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let body = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        fs::write(path, body)
    }

    pub fn reduce_motion(&self) -> bool {
        self.appearance.reduce_motion
    }

    pub fn high_contrast(&self) -> bool {
        self.appearance.high_contrast
    }

    pub fn reduce_transparency(&self) -> bool {
        self.appearance.reduce_transparency
    }

    pub fn ui_scale(&self) -> f32 {
        self.appearance.ui_scale
    }

    pub fn writable_config_path() -> PathBuf {
        env::var_os("STDCLI_CONFIG")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("std-cli.json"))
    }
}

impl Default for StdConfig {
    fn default() -> Self {
        Self {
            launcher_hotkey: "Alt+Space".to_string(),
            data_dir: default_data_dir(),
            enable_ai: false,
            theme: "system".to_string(),
            appearance: AppearanceConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn config_loads_without_panic() {
        let _cfg = StdConfig::load();
    }

    #[test]
    fn config_has_product_defaults() {
        let cfg = StdConfig::default();

        assert_eq!(cfg.launcher_hotkey, "Alt+Space");
        assert!(!cfg.enable_ai);
        assert_eq!(cfg.theme, "system");
        assert!(!cfg.reduce_motion());
        assert!(!cfg.high_contrast());
        assert!(!cfg.reduce_transparency());
        assert_eq!(cfg.ui_scale(), 1.0);
        assert!(cfg.workflows_dir().ends_with("workflows"));
        assert!(cfg.plugins_dir().ends_with("plugins"));
        assert!(cfg.apps_dir().ends_with("Applications"));
    }

    #[test]
    fn test_default_data_dir_is_isolated_from_user_profile() {
        let cfg = StdConfig::default();

        assert!(cfg.data_dir.starts_with(env::temp_dir()));
        assert!(cfg.data_dir.ends_with(std::process::id().to_string()));
        assert!(!cfg.data_dir.ends_with(".std-cli"));
    }

    #[test]
    fn config_can_get_set_and_save_fields() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("std-cli.json");
        let mut config = StdConfig::default();

        config.set_field("launcher_hotkey", "Cmd+Space").unwrap();
        config.set_field("enable_ai", "true").unwrap();
        config
            .set_field("appearance.reduce_motion", "true")
            .unwrap();
        config
            .set_field("appearance.high_contrast", "true")
            .unwrap();
        config
            .set_field("appearance.reduce_transparency", "true")
            .unwrap();
        config.set_field("appearance.ui_scale", "1.25").unwrap();
        config.set_field("data_dir", "/tmp/std-data").unwrap();
        config.save_to(&path).unwrap();

        let loaded: StdConfig = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();

        assert_eq!(
            config.get_field("launcher_hotkey").as_deref(),
            Some("Cmd+Space")
        );
        assert!(loaded.enable_ai);
        assert!(loaded.appearance.reduce_motion);
        assert!(loaded.appearance.high_contrast);
        assert!(loaded.appearance.reduce_transparency);
        assert_eq!(loaded.appearance.ui_scale, 1.25);
        assert_eq!(loaded.data_dir, PathBuf::from("/tmp/std-data"));
        assert!(config.set_field("missing", "value").is_err());
        assert!(config.set_field("enable_ai", "yes").is_err());
        assert!(config.set_field("appearance.reduce_motion", "yes").is_err());
        assert!(config.set_field("appearance.high_contrast", "yes").is_err());
        assert!(config
            .set_field("appearance.reduce_transparency", "yes")
            .is_err());
        assert!(config.set_field("appearance.ui_scale", "3.0").is_err());
    }

    #[test]
    fn config_loads_yaml_from_explicit_env_path() {
        let _guard = env_lock();
        let temp = tempfile::tempdir().unwrap();
        let config_path = temp.path().join("std-cli.yaml");
        let data_dir = temp.path().join("yaml-data");
        fs::write(
            &config_path,
            format!(
                "launcher_hotkey: Cmd+K\ndata_dir: {}\nenable_ai: true\ntheme: dark\nappearance:\n  reduce_motion: true\n  high_contrast: true\n  reduce_transparency: true\n  ui_scale: 1.25\n",
                data_dir.display()
            ),
        )
        .unwrap();
        env::set_var("STDCLI_CONFIG", &config_path);

        let config = StdConfig::load_from(Some(temp.path()));

        env::remove_var("STDCLI_CONFIG");

        assert_eq!(config.launcher_hotkey, "Cmd+K");
        assert_eq!(config.data_dir, data_dir);
        assert!(config.enable_ai);
        assert_eq!(config.theme, "dark");
        assert!(config.reduce_motion());
        assert!(config.high_contrast());
        assert!(config.reduce_transparency());
        assert_eq!(config.ui_scale(), 1.25);
    }

    #[test]
    fn config_discovers_project_yaml_from_ancestor() {
        let temp = tempfile::tempdir().unwrap();
        let project = temp.path().join("project");
        let nested = project.join("src").join("bin");
        let data_dir = temp.path().join("project-yaml-data");
        fs::create_dir_all(&nested).unwrap();
        fs::write(
            project.join("std-cli.yaml"),
            format!(
                "launcher_hotkey: Ctrl+Space\ndata_dir: {}\n",
                data_dir.display()
            ),
        )
        .unwrap();

        let config = StdConfig::load_from(Some(&nested));

        assert_eq!(config.launcher_hotkey, "Ctrl+Space");
        assert_eq!(config.data_dir, data_dir);
    }

    #[test]
    fn explicit_config_path_overrides_project_config() {
        let _guard = env_lock();
        let temp = tempfile::tempdir().unwrap();
        let project = temp.path().join("project");
        let nested = project.join("nested");
        let explicit_path = temp.path().join("explicit.yaml");
        let explicit_data_dir = temp.path().join("explicit-data");
        let project_data_dir = temp.path().join("project-data");
        fs::create_dir_all(&nested).unwrap();
        fs::write(
            project.join("std-cli.yaml"),
            format!(
                "launcher_hotkey: Ctrl+Space\ndata_dir: {}\ntheme: project\n",
                project_data_dir.display()
            ),
        )
        .unwrap();
        fs::write(
            &explicit_path,
            format!(
                "launcher_hotkey: Cmd+K\ndata_dir: {}\ntheme: explicit\n",
                explicit_data_dir.display()
            ),
        )
        .unwrap();
        env::set_var("STDCLI_CONFIG", &explicit_path);

        let config = StdConfig::load_from(Some(&nested));

        env::remove_var("STDCLI_CONFIG");

        assert_eq!(config.launcher_hotkey, "Cmd+K");
        assert_eq!(config.data_dir, explicit_data_dir);
        assert_eq!(config.theme, "explicit");
    }

    #[test]
    fn nearest_project_config_overrides_parent_config() {
        let temp = tempfile::tempdir().unwrap();
        let project = temp.path().join("project");
        let nested = project.join("nested");
        let leaf = nested.join("src");
        let parent_data_dir = temp.path().join("parent-data");
        let nearest_data_dir = temp.path().join("nearest-data");
        fs::create_dir_all(&leaf).unwrap();
        fs::write(
            project.join("std-cli.yaml"),
            format!(
                "launcher_hotkey: Ctrl+Space\ndata_dir: {}\ntheme: parent\n",
                parent_data_dir.display()
            ),
        )
        .unwrap();
        fs::write(
            nested.join("std-cli.yaml"),
            format!(
                "launcher_hotkey: Cmd+Space\ndata_dir: {}\ntheme: nearest\n",
                nearest_data_dir.display()
            ),
        )
        .unwrap();

        let config = StdConfig::load_from(Some(&leaf));

        assert_eq!(config.launcher_hotkey, "Cmd+Space");
        assert_eq!(config.data_dir, nearest_data_dir);
        assert_eq!(config.theme, "nearest");
    }

    #[test]
    fn explicit_environment_fields_override_config_files() {
        let _guard = env_lock();
        let temp = tempfile::tempdir().unwrap();
        let config_path = temp.path().join("std-cli.yaml");
        let file_data_dir = temp.path().join("file-data");
        let env_data_dir = temp.path().join("env-data");
        fs::write(
            &config_path,
            format!(
                "launcher_hotkey: Ctrl+Space\ndata_dir: {}\nenable_ai: false\ntheme: file\nappearance:\n  reduce_motion: false\n",
                file_data_dir.display()
            ),
        )
        .unwrap();
        env::set_var("STDCLI_CONFIG", &config_path);
        env::set_var("STDCLI_LAUNCHER_HOTKEY", "Cmd+K");
        env::set_var("STDCLI_DATA_DIR", &env_data_dir);
        env::set_var("STDCLI_ENABLE_AI", "true");
        env::set_var("STDCLI_THEME", "env");
        env::set_var("STD_REDUCE_MOTION", "true");

        let config = StdConfig::load_from(Some(temp.path()));

        env::remove_var("STDCLI_CONFIG");
        env::remove_var("STDCLI_LAUNCHER_HOTKEY");
        env::remove_var("STDCLI_DATA_DIR");
        env::remove_var("STDCLI_ENABLE_AI");
        env::remove_var("STDCLI_THEME");
        env::remove_var("STD_REDUCE_MOTION");

        assert_eq!(config.launcher_hotkey, "Cmd+K");
        assert_eq!(config.data_dir, env_data_dir);
        assert!(config.enable_ai);
        assert_eq!(config.theme, "env");
        assert!(config.reduce_motion());
    }

    #[test]
    fn std_prefixed_environment_fields_are_supported() {
        let _guard = env_lock();
        let temp = tempfile::tempdir().unwrap();
        let env_data_dir = temp.path().join("std-env-data");
        env::set_var("STD_LAUNCHER_HOTKEY", "Alt+K");
        env::set_var("STD_DATA_DIR", &env_data_dir);
        env::set_var("STD_ENABLE_AI", "true");
        env::set_var("STD_THEME", "std-env");
        env::set_var("STDCLI_REDUCE_MOTION", "true");

        let config = StdConfig::load_from(Some(temp.path()));

        env::remove_var("STD_LAUNCHER_HOTKEY");
        env::remove_var("STD_DATA_DIR");
        env::remove_var("STD_ENABLE_AI");
        env::remove_var("STD_THEME");
        env::remove_var("STDCLI_REDUCE_MOTION");

        assert_eq!(config.launcher_hotkey, "Alt+K");
        assert_eq!(config.data_dir, env_data_dir);
        assert!(config.enable_ai);
        assert_eq!(config.theme, "std-env");
        assert!(config.reduce_motion());
    }

    #[test]
    fn invalid_environment_field_returns_error() {
        let _guard = env_lock();
        let temp = tempfile::tempdir().unwrap();
        env::set_var("STDCLI_ENABLE_AI", "yes");

        let error = StdConfig::try_load_from(Some(temp.path())).unwrap_err();

        env::remove_var("STDCLI_ENABLE_AI");

        assert!(error.to_string().contains("STDCLI_ENABLE_AI invalid"));
        assert!(error
            .to_string()
            .contains("enable_ai must be true or false: yes"));
    }

    #[test]
    fn runtime_test_mode_still_allows_explicit_fixture_config() {
        let _guard = env_lock();
        let temp = tempfile::tempdir().unwrap();
        let config_path = temp.path().join("std-cli.yaml");
        let fixture_data = temp.path().join("fixture-data");
        fs::write(
            &config_path,
            format!(
                "launcher_hotkey: Cmd+K\ndata_dir: {}\ntheme: dark\n",
                fixture_data.display()
            ),
        )
        .unwrap();
        env::set_var("STDCLI_CONFIG", &config_path);

        let config = StdConfig::try_load_from(Some(temp.path())).unwrap();

        env::remove_var("STDCLI_CONFIG");

        assert_eq!(config.launcher_hotkey, "Cmd+K");
        assert_eq!(config.data_dir, fixture_data);
        assert_eq!(config.theme, "dark");
    }
}
