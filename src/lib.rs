use std::{collections::HashMap, path::PathBuf};

mod obsidian;

pub struct App {
    vault: Option<obsidian::Vault>,

    pub workspaces: Vec<Workspace>,
    pub active_workspace_id: usize,
}

pub struct Workspace {
    id: usize,
    working_directory: PathBuf,
    branch: String,
    processes: Vec<Process>,
}

pub struct Process {
    pub command: String,
    pub environment: HashMap<String, String>,
}

impl App {
    pub fn new(path_to_vault: PathBuf) -> Result<Self, AppError> {
        todo!()
    }

    /// Updates the state by scanning the vault and editing the workspaces if needed
    pub fn update() {
        todo!()
    }
}

pub enum AppError {
    InvalidVaultPath,
}

