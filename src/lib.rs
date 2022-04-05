use fltk::{prelude::*, app::App};


#[derive(Debug)]
pub struct FltkBuilder<W> 
where W: WindowExt {
    app: App,
    window: W
}

impl<W> FltkBuilder<W>
where W: WindowExt
{
    pub fn new(app: App, window: W) -> Self { Self { app, window } }

    /// Get the fltk builder's app.
    #[must_use]
    pub fn app(&self) -> App {
        self.app
    }

    /// Get a reference to the fltk builder's window.
    #[must_use]
    pub fn window(&self) -> &W {
        &self.window
    }

    /// Get a mutable reference to the fltk builder's window.
    #[must_use]
    pub fn window_mut(&mut self) -> &mut W {
        &mut self.window
    }

    pub fn widget(&self, widget: impl WidgetExt) {
        
    }
}