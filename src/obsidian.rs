use std::{fs, path::PathBuf, str::FromStr};

use serde_json::Value;

pub struct Vault {
    path: PathBuf,
    tab_groups: Vec<TabGroup>,
    focused_group: String,
}

impl Vault {
    pub fn new(path: PathBuf) -> Result<Self, Error> {
        let mut path = path;
        path.push(PathBuf::from_str(".obsidian/workspace.json").unwrap());
        dbg!(&path);

        let workspace_file = fs::File::open(path)?;
        let nodes: Value = serde_json::from_reader(workspace_file)?;
        dbg!(nodes);

        todo!();
    }

    pub fn current_tab(&self) -> Option<&Tab> {
        todo!();
    }
}

struct TabGroup {
    id: String,
    tabs: Vec<Tab>,
    active_tab: usize,
}

pub enum Tab {
    File(File),
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

#[derive(Debug)]
pub enum Error {
    ParseFailed,
    NotVault,
    NoFileOrDirectory,
}

impl From<std::io::Error> for Error {
    fn from(_value: std::io::Error) -> Self {
        return Self::NoFileOrDirectory;
    }
}

impl From<serde_json::Error> for Error {
    fn from(_value: serde_json::Error) -> Self {
        return Self::ParseFailed;
    }
}
