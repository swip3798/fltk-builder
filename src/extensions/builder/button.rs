use fltk::{enums::{Shortcut, FrameType}, prelude::ButtonExt};

/// Adds builder pattern friendly versions of several setter functions for Button widgets
pub trait ButtonBuilderExt {
    /// Sets the shortcut associated with a button
    fn with_shortcut(self, shortcut: Shortcut) -> Self;
    /// Set the `down_box` of the widget
    fn with_down_frame(self, f: FrameType) -> Self;
}

impl<B> ButtonBuilderExt for B 
where B: ButtonExt {
    fn with_shortcut(mut self, shortcut: Shortcut) -> Self {
        self.set_shortcut(shortcut);
        self
    }

    fn with_down_frame(mut self, f: FrameType) -> Self {
        self.set_down_frame(f);
        self
    }
}