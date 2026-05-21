use crate::{
    events::EventBus,
    execution::{action_preview, execute_registry_entry, ExternalExecutionMode},
    CoreError, StdCore,
};
use std_types::{
    ActionExecution, ActionId, ActionPreview, RegistryEntry, SearchResult, StdEvent, StdEventType,
};

impl StdCore {
    pub fn register_action(&self, entry: RegistryEntry) -> Result<(), CoreError> {
        self.registry
            .write()
            .map_err(|_| CoreError::RegistryLockPoisoned)?
            .register(entry.clone())?;
        self.publish(StdEvent::new(
            StdEventType::RegistryChanged,
            "std-core",
            serde_json::json!({
                "action_id": entry.action.id,
                "name": entry.action.name,
            }),
        ))
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, CoreError> {
        Ok(self
            .registry
            .read()
            .map_err(|_| CoreError::RegistryLockPoisoned)?
            .search(query, limit))
    }

    pub fn preview_action(&self, id: ActionId) -> Result<ActionPreview, CoreError> {
        let entry = self
            .registry
            .read()
            .map_err(|_| CoreError::RegistryLockPoisoned)?
            .get(id)
            .cloned()
            .ok_or(CoreError::ActionNotFound(id))?;
        let preview = action_preview(&entry);
        self.publish(StdEvent::new(
            StdEventType::ActionPreviewed,
            "std-core",
            serde_json::json!({
                "action_id": preview.action_id,
                "title": preview.title,
                "action_type": preview.action_type,
            }),
        ))?;
        Ok(preview)
    }

    pub fn execute_action(&self, id: ActionId) -> Result<ActionExecution, CoreError> {
        self.execute_action_with_mode(id, ExternalExecutionMode::Disabled)
    }

    pub fn execute_action_with_external_runner(
        &self,
        id: ActionId,
        allow_external_runner: bool,
    ) -> Result<ActionExecution, CoreError> {
        let mode = if allow_external_runner && crate::desktop_automation_allowed() {
            ExternalExecutionMode::DesktopAutomation
        } else {
            ExternalExecutionMode::Disabled
        };
        self.execute_action_with_mode(id, mode)
    }

    pub fn execute_action_from_launcher_user(
        &self,
        id: ActionId,
    ) -> Result<ActionExecution, CoreError> {
        self.execute_action_with_mode(id, ExternalExecutionMode::LauncherUser)
    }

    fn execute_action_with_mode(
        &self,
        id: ActionId,
        external_mode: ExternalExecutionMode,
    ) -> Result<ActionExecution, CoreError> {
        let entry = self
            .registry
            .read()
            .map_err(|_| CoreError::RegistryLockPoisoned)?
            .get(id)
            .cloned()
            .ok_or(CoreError::ActionNotFound(id))?;
        let execution = execute_registry_entry(self, &entry, external_mode)?;
        self.publish(StdEvent::new(
            StdEventType::ActionExecuted,
            "std-core",
            serde_json::json!({
                "action_id": execution.action_id,
                "action_name": execution.action_name,
                "status": execution.status,
                "message": execution.message,
            }),
        ))?;
        Ok(execution)
    }

    pub(crate) fn register_action_if_missing(&self, entry: RegistryEntry) -> Result<(), CoreError> {
        let exists = self
            .registry
            .read()
            .map_err(|_| CoreError::RegistryLockPoisoned)?
            .get_by_name(&entry.action.name)
            .is_some();
        if exists {
            return Ok(());
        }
        self.register_action(entry)
    }
}
