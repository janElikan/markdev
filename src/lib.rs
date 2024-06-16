use std::{collections::HashMap, path::PathBuf};

pub mod obsidian;

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
