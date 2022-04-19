use fltk::{
    enums::{Color, Font, FrameType},
    prelude::MenuExt,
};

pub trait MenuBuilderExt {
    /// Sets the text font
    fn with_text_font(self, c: Font) -> Self;
    /// Sets the text size
    fn with_text_size(self, c: i32) -> Self;
    /// Sets the text color
    fn with_text_color(self, c: Color) -> Self;
    /// Set the `down_box` of the widget
    fn with_down_frame(self, f: FrameType) -> Self;
}

impl<M> MenuBuilderExt for M
where
    M: MenuExt,
{
    fn with_text_font(mut self, c: Font) -> Self {
        self.set_text_font(c);
        self
    }

    fn with_text_size(mut self, c: i32) -> Self {
        self.set_text_size(c);
        self
    }

    fn with_text_color(mut self, c: Color) -> Self {
        self.set_text_color(c);
        self
    }

    fn with_down_frame(mut self, f: FrameType) -> Self {
        self.set_down_frame(f);
        self
    }
}
