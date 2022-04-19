use fltk::{enums::{Font, Color}, prelude::InputExt};

/// Adds builder pattern friendly versions of several setter functions for Input widgets
pub trait InputBuilderExt {
    /// Sets the value inside an input/output widget
    fn with_value(self, val: &str) -> Self;
    /// Sets the maximum size (in bytes) accepted by an input/output widget
    fn with_maximum_size(self, val: i32) -> Self;
    /// Sets the text font
    fn with_text_font(self, font: Font) -> Self;
    /// Sets the text color
    fn with_text_color(self, color: Color) -> Self;
    /// Sets the text size
    fn with_text_size(self, sz: i32) -> Self;
    /// Set readonly status of the input/output widget
    fn as_readonly(self, val: bool) -> Self;
    /// Set whether text is wrapped inside an input/output widget
    fn as_wrap(self, val: bool) -> Self;
}

impl<I> InputBuilderExt for I 
where I: InputExt {
    fn with_value(mut self, val: &str) -> Self {
        self.set_value(val);
        self
    }

    fn with_maximum_size(mut self, val: i32) -> Self {
        self.set_maximum_size(val);
        self
    }

    fn with_text_font(mut self, font: Font) -> Self {
        self.set_text_font(font);
        self
    }

    fn with_text_color(mut self, color: Color) -> Self {
        self.set_text_color(color);
        self
    }

    fn with_text_size(mut self, sz: i32) -> Self {
        self.set_text_size(sz);
        self
    }

    fn as_readonly(mut self, val: bool) -> Self {
        self.set_readonly(val);
        self
    }

    fn as_wrap(mut self, val: bool) -> Self {
        self.set_wrap(val);
        self
    }
}