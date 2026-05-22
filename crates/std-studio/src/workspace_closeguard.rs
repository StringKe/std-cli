use crate::{StudioApp, WorkspacePaneCloseGuard};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const CLOSEGUARD_FILE: &str = "studio-workspace-closeguard.json";

impl StudioApp {
    pub fn workspace_closeguard_path(&self) -> PathBuf {
        self.core.config.data_dir.join(CLOSEGUARD_FILE)
    }

    pub fn save_workspace_closeguard(
        &self,
        closeguard: &WorkspacePaneCloseGuard,
    ) -> io::Result<PathBuf> {
        let path = self.workspace_closeguard_path();
        write_workspace_closeguard(&path, closeguard)?;
        Ok(path)
    }

    pub fn load_workspace_closeguard(&self) -> io::Result<WorkspacePaneCloseGuard> {
        read_workspace_closeguard(&self.workspace_closeguard_path())
    }

    pub fn close_workspace_instance_to_disk(&mut self) -> io::Result<PathBuf> {
        let closeguard = self.close_workspace_instance();
        self.save_workspace_closeguard(&closeguard)
    }

    pub fn restore_workspace_closeguard_from_disk(&mut self) -> io::Result<()> {
        let closeguard = self.load_workspace_closeguard()?;
        self.restore_workspace_closeguard(&closeguard);
        Ok(())
    }
}

fn write_workspace_closeguard(path: &Path, closeguard: &WorkspacePaneCloseGuard) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = serde_json::to_string_pretty(closeguard).map_err(io::Error::other)?;
    fs::write(path, body)
}

fn read_workspace_closeguard(path: &Path) -> io::Result<WorkspacePaneCloseGuard> {
    let body = fs::read_to_string(path)?;
    serde_json::from_str(&body).map_err(io::Error::other)
}
