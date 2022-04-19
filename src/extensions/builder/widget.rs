use fltk::{enums::{Color, FrameType, LabelType, CallbackTrigger, Font}, prelude::{ImageExt, WidgetExt}};

/// Adds builder pattern friendly versions of several setter functions
pub trait WidgetBuilderExt {
    /// Sets the widget's color
    fn with_color(self, color: Color) -> Self;
    /// Sets the widget's frame type
    fn with_frame(self, frame: FrameType) -> Self;
    /// Sets the widget label's size
    fn with_label_size(self, size: i32) -> Self;
    /// Sets the widget label's color
    fn with_label_color(self, color: Color) -> Self;
    /// Sets the widget label's type
    fn with_label_type(self, typ: LabelType) -> Self;
    /// Sets the selection color of the widget
    fn with_selection_color(self, color: Color) -> Self;
    /// Sets the widget label's font
    fn with_label_font(self, font: Font) -> Self;
    /// Sets the callback when the widget is triggered (clicks for example)
    /// takes the widget as a closure argument
    fn with_callback<F>(self, cb: F) -> Self
    where
        F: FnMut(&mut Self) + 'static;
    /// Sets the default callback trigger for a widget, equivalent to `when()`
    fn with_trigger(self, trigger: CallbackTrigger) -> Self;
    /// Emits a message on callback using a sender
    fn with_emit<T: 'static + Clone + Send + Sync>(self, sender: fltk::app::Sender<T>, msg: T) -> Self;
    /// Sets the image of the widget
    fn with_image<I: ImageExt>(self, image: Option<I>) -> Self;
    /// Sets the image of the widget scaled to the widget's size
    fn with_image_scaled<I: ImageExt>(self, image: Option<I>) -> Self
    where
        Self: Sized;
    /// Sets the deactivated image of the widget
    fn with_deimage<I: ImageExt>(self, image: Option<I>) -> Self
    where
        Self: Sized;
    /// Sets the deactivated image of the widget scaled to the widget's size
    fn with_deimage_scaled<I: ImageExt>(self, image: Option<I>) -> Self
    where
        Self: Sized;
    /// Sets the tooltip text
    fn with_tooltip(self, txt: &str) -> Self;
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

    fn with_label_type(mut self, typ: LabelType) -> Self {
        self.set_label_type(typ);
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

    fn with_emit<T: 'static + Clone + Send + Sync>(mut self, sender: fltk::app::Sender<T>, msg: T) -> Self {
        self.emit(sender, msg);
        self
    }

    fn with_image<I: ImageExt>(mut self, image: Option<I>) -> Self {
        self.set_image(image);
        self
    }

    fn with_image_scaled<I: ImageExt>(mut self, image: Option<I>) -> Self
    where
        Self: Sized {
        self.set_image_scaled(image);
        self
    }

    fn with_deimage<I: ImageExt>(mut self, image: Option<I>) -> Self
    where
        Self: Sized {
        self.set_deimage(image);
        self
    }

    fn with_deimage_scaled<I: ImageExt>(mut self, image: Option<I>) -> Self
    where
        Self: Sized {
        self.set_deimage_scaled(image);
        self
    }

    fn with_tooltip(mut self, txt: &str) -> Self {
        self.set_tooltip(txt);
        self
    }
}