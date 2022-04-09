use fltk::{prelude::{GroupExt, WidgetExt}, group::Flex};
use fltk::{enums::{Color, FrameType, Font, CallbackTrigger}};

/// Defines all necessary functions for group widgets to add widgets and other groups as children
pub trait BuilderExt {
    /// Adds a group widget as a child
    fn group(self, group: impl GroupExt) -> Self;
    /// Adds a widget as a child
    fn widget(self, widget: impl WidgetExt) -> Self;
}

impl<G> BuilderExt for G where G: GroupExt {
    fn group(self, group: impl GroupExt) -> Self {
        group.end();
        self
    }

    fn widget(self, _widget: impl WidgetExt) -> Self {
        self
    }
}

/// Adds builder pattern friendly versions of several functions
/// TODO: Add documentation from the original setter functions
pub trait WidgetBuilderExt {
    fn with_color(self, color: Color) -> Self;
    fn with_frame(self, frame: FrameType) -> Self;
    fn with_label_size(self, size: i32) -> Self;
    fn with_label_color(self, color: Color) -> Self;
    fn with_selection_color(self, color: Color) -> Self;
    fn with_label_font(self, font: Font) -> Self;
    fn with_callback<F>(self, cb: F) -> Self
    where
        F: FnMut(&mut Self) + 'static;
    fn with_trigger(self, trigger: CallbackTrigger) -> Self;
}

impl<W> WidgetBuilderExt for W 
where W: WidgetExt{
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

    fn with_callback<F>(mut self, cb: F) -> Self
    where
        F: FnMut(&mut Self) + 'static,
    {
        self.set_callback(cb);
        self
    }

    fn with_trigger(mut self, trigger: CallbackTrigger) -> Self {
        self.set_trigger(trigger);
        self
    }
}

/// Defines helper function to add a widget to a flex with an already set size
pub trait FlexBuilderExt {
    fn widget_with_size(self, size: i32, widget: impl WidgetExt) -> Self;
}

impl FlexBuilderExt for Flex {
    fn widget_with_size(mut self, size: i32, widget: impl WidgetExt) -> Self {
        self.set_size(&widget, size);
        self.widget(widget)
    }
}