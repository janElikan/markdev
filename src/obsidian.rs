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

        let workspace_file = fs::File::open(&path)?;
        let nodes: Value = serde_json::from_reader(workspace_file)?;
        let main = nodes.get("main").unwrap();

        let tab_groups = get_tab_groups(&main);
        let focused_group = nodes.get("active").unwrap().to_string();

        Ok(Self {
            path,
            tab_groups,
            focused_group,
        })
    }

    pub fn current_tab(&self) -> Option<&Tab> {
        let focused_tab_group = self
            .tab_groups
            .iter()
            .find(|group| group.id == self.focused_group)?;

        focused_tab_group.tabs.get(focused_tab_group.active_tab)
    }
}

/// this should eventually become a const function
fn get_tab_groups(node: &Value) -> Vec<TabGroup> {
    if node.is_array() {
        let node = node.as_array().unwrap();
        return node.into_iter().flat_map(get_tab_groups).collect();
    }

    let node_type = node.get("type").unwrap().as_str().unwrap();

    return match node_type {
        "tabs" => {
            vec![TabGroup::from(node)]
        }
        _ => {
            let children = node.get("children").unwrap();
            get_tab_groups(children)
        }
    };
}

struct TabGroup {
    pub id: String,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
}

impl From<&Value> for TabGroup {
    // ## Breaks at runtime
    // if you have more than 18_446_744_073_709_551_615 tabs open!
    fn from(value: &Value) -> Self {
        let id = value.get("id").unwrap().to_string();

        let active_tab = match value.get("currentTab").unwrap().as_number() {
            Some(number) => number.as_u64().unwrap() as usize,
            None => 0,
        };

        let tabs = value
            .get("children")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(Tab::from)
            .collect();

        Self {
            id,
            tabs,
            active_tab,
        }
    }
}

pub enum Tab {
    File(File),
    Plugin,
}

impl From<&Value> for Tab {
    /// This function currently has uncomfortably many `.unwrap()`s
    fn from(value: &Value) -> Self {
        let value = value.get("state").unwrap(); // throw away the leaf ID

        match value.get("type").unwrap().as_str().unwrap() {
            "markdown" => {
                let state = value.get("state").unwrap();
                dbg!(state);

                Tab::File(File {
                    path: state.get("file").unwrap().as_str().unwrap().into(),
                    mode: FileMode::Unimplemented,
                })
            }
            _ => Tab::Plugin,
        }
    }
}

pub struct File {
    pub path: PathBuf,
    pub mode: FileMode,
}

/// this is not yet used
pub enum FileMode {
    Unimplemented,
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
