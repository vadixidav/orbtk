//! This module contains the base elements of an OrbTk application (Application, WindowBuilder and Window).

use crate::{Rect, Theme};

pub use self::global::*;
pub use self::tree::*;
pub use self::window::*;

mod global;
mod tree;
mod window;

#[derive(Default)]
/// The `Application` represents the entry point of an OrbTk based application.
pub struct Application {
    windows: Vec<Window>,
}

impl Application {
    /// Creates a new application.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a `WindowBuilder.
    pub fn create_window(&mut self) -> WindowBuilder {
        WindowBuilder {
            application: self,
            bounds: Rect::default(),
            title: String::from(""),
            theme: Theme::new(),
            root: None,
            debug_flag: false,
        }
    }

    /// Starts the application and run it until quit is requested.
    pub fn run(&mut self) {
        for window in &mut self.windows {
            window.run();
        }
    }
}
