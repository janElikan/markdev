use std::{collections::HashMap, path::PathBuf};

use obsidian::ObsidianError;

mod obsidian;

pub struct App {
    vault: obsidian::Vault,

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
        let vault = obsidian::Vault::new(path_to_vault)?;
        let tab = vault.get_focused_tab()?;
        let workspaces = match Workspace::from(tab, 0) {
            Some(workspace) => vec![workspace],
            None => Vec::new(),
        };

        Ok(Self {
            vault,
            workspaces,
            active_workspace_id: 0,
        })
    }

    /// Updates the state by scanning the vault and editing the workspaces if needed
    pub fn update(&mut self) {
        todo!()
    }
}

impl Workspace {
    /// Returns `None` if the tab is a plugin or does not contain markdev metadata
    pub fn from(tab: &obsidian::Tab, workspace_id: usize) -> Option<Self> {
        match tab {
            obsidian::Tab::File(file) => {
                todo!("read frontmatter");
            }
            obsidian::Tab::Plugin => None,
        }
    }
}

pub enum AppError {
    InvalidVaultPath,
    ObsidianBug,
}

impl From<ObsidianError> for AppError {
    fn from(value: ObsidianError) -> Self {
        match value {
            ObsidianError::InvalidPath => AppError::InvalidVaultPath,
            ObsidianError::InvalidTabId => AppError::ObsidianBug,
        }
    }
}
