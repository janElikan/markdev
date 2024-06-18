use std::{collections::BTreeMap, path::PathBuf};

use markdev::{App, AppError};
use zellij_tile::{
    prelude::{Event, EventType, PermissionType},
    shim::{report_panic, request_permission, subscribe},
    ZellijPlugin,
};

struct Plugin {
    app: Option<App>,
    ui: Ui,
}

struct Ui {
    input: String,
    state: UiState,
    error: Option<AppError>,
}

enum UiState {
    InquiringPath,
    InputConfirmed,
    AppCreated,
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            app: None,
            ui: Ui::new(),
        }
    }
}

impl Plugin {
    /// Creates a valid Some(App) or prints the error to the UI.
    /// uses the path from self.ui.input
    fn open_vault(&mut self) {
        let path = PathBuf::from(&self.ui.input);

        match App::new(path) {
            Ok(app) => {
                self.app = Some(app);
                self.ui.state = UiState::AppCreated;
            }
            Err(reason) => self.ui.error = Some(reason),
        }
    }
}

#[allow(unused_variables)]
impl ZellijPlugin for Plugin {
    fn load(&mut self, _config: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::OpenFiles,
            PermissionType::RunCommands,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        match self.ui.state {
            UiState::InquiringPath => {
                todo!("handle keypress");
                true
            }
            UiState::InputConfirmed => {
                todo!("create the app");
                // this is blocking further execution because it's gonna be watching files
                true
            }
            UiState::AppCreated => {
                todo!("main app logic");
                false
            }
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        todo!();
    }
}

zellij_tile::register_plugin!(Plugin);

impl Ui {
    fn new() -> Self {
        Self {
            input: String::new(),
            state: UiState::InquiringPath,
            error: None,
        }
    }
}
