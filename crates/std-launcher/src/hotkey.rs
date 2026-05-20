use std::time::Instant;

const HOTKEY_BUDGET_MS: u128 = 80;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeyRegistrationPlan {
    pub accelerator: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherHotkey {
    pub modifiers: Vec<String>,
    pub key: String,
}

impl LauncherHotkey {
    pub fn parse(value: &str) -> Option<Self> {
        let parts = value
            .split('+')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();
        let key = parts.last()?.to_string();
        let modifiers = parts[..parts.len().saturating_sub(1)]
            .iter()
            .map(|part| normalize_modifier(part))
            .collect::<Option<Vec<_>>>()?;
        Some(Self { modifiers, key })
    }

    pub fn display(&self) -> String {
        if self.modifiers.is_empty() {
            return self.key.clone();
        }
        format!("{}+{}", self.modifiers.join("+"), self.key)
    }

    pub fn accelerator(&self) -> String {
        self.display()
    }
}

pub struct GlobalHotkeyRuntime {
    manager: Option<global_hotkey::GlobalHotKeyManager>,
    hotkey: Option<global_hotkey::hotkey::HotKey>,
    hotkey_id: Option<u32>,
    pub plan: HotkeyRegistrationPlan,
}

impl GlobalHotkeyRuntime {
    pub fn register(plan: HotkeyRegistrationPlan) -> Result<Self, String> {
        let hotkey = global_hotkey::hotkey::HotKey::try_from(plan.accelerator.as_str())
            .map_err(|error| error.to_string())?;
        let manager =
            global_hotkey::GlobalHotKeyManager::new().map_err(|error| error.to_string())?;
        manager
            .register(hotkey)
            .map_err(|error| error.to_string())?;
        let hotkey_id = hotkey.id();
        Ok(Self {
            manager: Some(manager),
            hotkey: Some(hotkey),
            hotkey_id: Some(hotkey_id),
            plan,
        })
    }

    pub fn disabled(plan: HotkeyRegistrationPlan) -> Self {
        Self {
            manager: None,
            hotkey: None,
            hotkey_id: None,
            plan,
        }
    }

    pub fn is_registered(&self) -> bool {
        self.manager.is_some() && self.hotkey.is_some()
    }

    pub fn should_toggle_for_event(&self, event: global_hotkey::GlobalHotKeyEvent) -> bool {
        self.hotkey_id == Some(event.id()) && event.state() == global_hotkey::HotKeyState::Pressed
    }

    pub fn poll_toggle_event(&self) -> bool {
        while let Ok(event) = global_hotkey::GlobalHotKeyEvent::receiver().try_recv() {
            if self.should_toggle_for_event(event) {
                return true;
            }
        }
        false
    }

    #[cfg(test)]
    pub(crate) fn set_hotkey_id_for_test(&mut self, hotkey_id: u32) {
        self.hotkey_id = Some(hotkey_id);
    }
}

impl Drop for GlobalHotkeyRuntime {
    fn drop(&mut self) {
        if let (Some(manager), Some(hotkey)) = (&self.manager, self.hotkey) {
            let _ = manager.unregister(hotkey);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeySmokeReport {
    pub status: &'static str,
    pub accelerator: String,
    pub registered: bool,
    pub register_ms: u128,
    pub budget_ms: u128,
    pub error: Option<String>,
}

impl HotkeySmokeReport {
    pub fn pass(&self) -> bool {
        self.status == "PASS"
            && self.registered
            && self.register_ms <= self.budget_ms
            && self.error.is_none()
    }

    pub fn summary(&self) -> String {
        let error = self.error.as_deref().unwrap_or("none");
        format!(
            "launcher_hotkey_smoke {}\naccelerator={}\nregistered={}\nregister_ms={}\nbudget_hotkey_ms={}\nerror={error}",
            self.status, self.accelerator, self.registered, self.register_ms, self.budget_ms
        )
    }
}

pub fn hotkey_smoke(accelerator: &str) -> HotkeySmokeReport {
    if hotkey_smoke_blocked_by_test_mode() {
        return HotkeySmokeReport {
            status: "SKIP",
            accelerator: accelerator.to_string(),
            registered: false,
            register_ms: 0,
            budget_ms: HOTKEY_BUDGET_MS,
            error: Some(
                "STD_TEST_MODE blocked global hotkey registration; use explicit desktop opt-in"
                    .to_string(),
            ),
        };
    }
    let plan = HotkeyRegistrationPlan {
        accelerator: accelerator.to_string(),
        enabled: true,
    };
    let started_at = Instant::now();
    match GlobalHotkeyRuntime::register(plan) {
        Ok(runtime) => {
            let register_ms = started_at.elapsed().as_millis();
            let registered = runtime.is_registered();
            drop(runtime);
            HotkeySmokeReport {
                status: "PASS",
                accelerator: accelerator.to_string(),
                registered,
                register_ms,
                budget_ms: HOTKEY_BUDGET_MS,
                error: None,
            }
        }
        Err(error) => HotkeySmokeReport {
            status: "FAIL",
            accelerator: accelerator.to_string(),
            registered: false,
            register_ms: started_at.elapsed().as_millis(),
            budget_ms: HOTKEY_BUDGET_MS,
            error: Some(error),
        },
    }
}

fn hotkey_smoke_blocked_by_test_mode() -> bool {
    cfg!(test) || std_core::std_test_mode_enabled()
}

pub(crate) fn normalize_modifier(value: &str) -> Option<String> {
    match value.to_ascii_lowercase().as_str() {
        "alt" | "option" => Some("Alt".to_string()),
        "cmd" | "command" | "meta" | "super" => Some("Command".to_string()),
        "ctrl" | "control" => Some("Control".to_string()),
        "shift" => Some("Shift".to_string()),
        _ => None,
    }
}
