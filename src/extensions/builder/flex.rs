use fltk::{prelude::WidgetExt, group::Flex};

use super::GroupBuilderExt;

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