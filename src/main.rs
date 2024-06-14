use std::collections::BTreeMap;

use zellij_tile::{prelude::Event, register_plugin, ZellijPlugin, shim::report_panic};

#[derive(Default)]
struct Plugin {
    config: BTreeMap<String, String>,
}

impl ZellijPlugin for Plugin {
    fn load(&mut self, config: BTreeMap<String, String>) {
        self.config = config;
    }

    fn update(&mut self, event: Event) -> bool {
        true
    }

    fn render(&mut self, rows: usize, cols: usize) {
        println!("Hello, world!");
    }
}

register_plugin!(Plugin);
