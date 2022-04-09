use fltk::{prelude::*, app::App};
#[macro_use]
extern crate lazy_static;

/// Holds several extension traits to enable a builder pattern
pub mod extensions;
mod id_map;

pub use id_map::{get_widget_by_id, IdMapError};

/// Starting point for the UI
#[derive(Debug)]
pub struct FltkBuilder<W> 
where W: WindowExt {
    app: App,
    window: Option<W>
}

impl<W> FltkBuilder<W>
where W: WindowExt
{
    /// Creates a new FltkBuilder struct
    pub fn new(app: App) -> Self { Self { app, window: None } }

    /// Get the fltk builder's app
    #[must_use]
    pub fn app(&self) -> App {
        self.app
    }

    /// Set the main window of the fltk app
    pub fn window(mut self, window: W) -> Self{
        window.end();
        self.window = Some(window);
        self
    }

    /// Call show on the window if it's available
    pub fn show(&mut self) {
        if self.window.is_some() {
            let win = self.window.as_mut().unwrap();
            win.show();
        }
    }

    /// Get a mutable reference to the fltk builder's window.
    #[must_use]
    pub fn get_window_mut(&mut self) -> &mut Option<W> {
        &mut self.window
    }
}

/// Reexports of all fltk_builder traits 
pub mod prelude;