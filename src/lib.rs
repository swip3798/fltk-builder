use fltk::{prelude::*, app::App};
#[macro_use]
extern crate lazy_static;

mod extensions;
mod label_map;

pub use label_map::{get_widget_by_id, IdMapError};


#[derive(Debug)]
pub struct FltkBuilder<W> 
where W: WindowExt {
    app: App,
    window: Option<W>
}

impl<W> FltkBuilder<W>
where W: WindowExt
{
    pub fn new(app: App) -> Self { Self { app, window: None } }

    /// Get the fltk builder's app.
    #[must_use]
    pub fn app(&self) -> App {
        self.app
    }

    pub fn window(mut self, window: W) -> Self{
        window.end();
        self.window = Some(window);
        self
    }

    pub fn show(&mut self) {
        if self.window.is_some() {
            let win = self.window.as_mut().unwrap();
            win.show();
        }
    }
}

pub mod prelude;