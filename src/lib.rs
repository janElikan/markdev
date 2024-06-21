use std::path::PathBuf;

use obsidian::{Tab, Vault};

mod obsidian;

pub fn run(vault_path: PathBuf) {
    let vault = Vault::new(vault_path).expect("Failed to open vault");
    let tab = vault.current_tab().expect("No open tab");
    let file = match tab {
        Tab::File(file) => file,
        Tab::Plugin => panic!("The open tab is not a file"),
    };

    let workspace = Workspace::from(tab);
}

pub struct Workspace {
    path: PathBuf,
    branch: Option<String>,
    edit: Option<PathBuf>,
    procs: Vec<Process>,
}

struct Process {
    command: String,
}

impl From<&Tab> for Workspace {
    fn from(value: &Tab) -> Self {
        todo!()
    }
}
