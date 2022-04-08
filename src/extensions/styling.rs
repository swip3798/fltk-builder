use fltk::{enums::{Color, FrameType, Font}, prelude::WidgetExt};

pub trait StylingExt {
    fn with_color(self, color: Color) -> Self;
    fn with_frame(self, frame: FrameType) -> Self;
    fn with_label_size(self, size: i32) -> Self;
    fn with_label_color(self, color: Color) -> Self;
    fn with_selection_color(self, color: Color) -> Self;
    fn with_label_font(self, font: Font) -> Self;
}

impl<W> StylingExt for W 
where W: WidgetExt
{
    fn with_color(mut self, color: Color) -> Self {
        self.set_color(color);
        self
    }

    fn with_frame(mut self, frame: FrameType) -> Self {
        self.set_frame(frame);
        self
    }

    fn with_label_size(mut self, size: i32) -> Self {
        self.set_label_size(size);
        self
    }

    fn with_label_color(mut self, color: Color) -> Self {
        self.set_label_color(color);
        self
    }

    fn with_selection_color(mut self, color: Color) -> Self {
        self.set_selection_color(color);
        self
    }

    fn with_label_font(mut self, font: Font) -> Self {
        self.set_label_font(font);
        self
    }
}