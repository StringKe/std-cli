use crate::LauncherState;

impl LauncherState {
    pub fn complete_query_from_selection(&mut self) -> bool {
        let Some(result) = self.view.selected_result() else {
            return false;
        };
        let query = result.action.name.to_lowercase();
        self.update_query(query);
        true
    }
}
