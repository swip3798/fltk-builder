use fltk::prelude::{GroupExt, WidgetExt};

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