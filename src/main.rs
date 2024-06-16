use std::collections::BTreeMap;

use zellij_tile::{prelude::Event, ZellijPlugin, shim::report_panic};

pub struct App {
    vault: Option<markdev::obsidian::Vault>,

    workspaces: Vec<markdev::Workspace>,
    active_workspace_id: usize,
}

impl Default for App {
    fn default() -> Self {
        Self { vault: None, workspaces: Vec::new(), active_workspace_id: 0 }
    }
}

#[allow(unused_variables)]
impl ZellijPlugin for App {
    fn load(&mut self, _config: BTreeMap<String, String>) {
        todo!();
    }

    fn update(&mut self, event: Event) -> bool {
        todo!();
    }

    fn render(&mut self, rows: usize, cols: usize) {
        todo!();
    }
}

zellij_tile::register_plugin!(App);
