use fltk::{prelude::{GroupExt, WidgetExt, ImageExt, ButtonExt}, group::Flex, enums::{LabelType, Shortcut}};
use fltk::{enums::{Color, FrameType, Font, CallbackTrigger}};

/// Defines all necessary functions for group widgets to add widgets and other groups as children, as well as builder friendly versions of several setter functions
pub trait GroupBuilderExt {
    // Functions for adding groups and widgets
    /// Adds a group widget as a child
    fn group(self, group: impl GroupExt) -> Self;
    /// Adds a widget as a child
    fn widget(self, widget: impl WidgetExt) -> Self;

    // Builder Pattern friendly functions
    /// Make the group itself resizable
    fn as_resizeable(self, val: bool) -> Self;
    /// Clips children outside the group boundaries
    fn with_clip_children(self, flag: bool) -> Self;
}

impl<G> GroupBuilderExt for G where G: GroupExt {
    fn group(self, group: impl GroupExt) -> Self {
        group.end();
        self
    }

    fn widget(self, _widget: impl WidgetExt) -> Self {
        self
    }

    fn as_resizeable(mut self, val: bool) -> Self {
        self.make_resizable(val);
        self
    }

    fn with_clip_children(mut self, flag: bool) -> Self {
        self.set_clip_children(flag);
        self
    }
}

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

/// Adds helper function to add a widget to a flex with an already set size
pub trait FlexBuilderExt {
    /// Adds a widget and sets its size within the flex layout
    fn widget_with_size(self, size: i32, widget: impl WidgetExt) -> Self;
}

impl FlexBuilderExt for Flex {
    fn widget_with_size(mut self, size: i32, widget: impl WidgetExt) -> Self {
        self.set_size(&widget, size);
        self.widget(widget)
    }
}

/// Adds builder pattern friendly versions of several setter functions 
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

