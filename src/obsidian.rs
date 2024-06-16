use std::path::PathBuf;

pub struct Vault {
    path: PathBuf,
    tab_groups: Vec<TabGroup>,
    focused_group: String,
}

impl Vault {
    pub fn new(path: PathBuf) -> Result<Self, ObsidianError> {
        todo!();
    }

    pub fn get_focused_tab(&self) -> Result<&Tab, ObsidianError> {
        todo!();
    }
}

pub struct TabGroup {
    id: String,
    tabs: Vec<Tab>,
    active_tab: usize,
}

pub enum Tab {
    File,
    Plugin,
}

pub struct File {
    pub path: PathBuf,
    pub mode: FileMode,
}

pub enum FileMode {
    Source,
    Edit,
    Preview,
}

pub enum ObsidianError {
    /// does not exist or is not an obsidian vault
    InvalidPath,
    InvalidTabId,
}
