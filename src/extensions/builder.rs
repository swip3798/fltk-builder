use fltk::{prelude::{GroupExt, WidgetExt}, group::Flex};

pub trait BuilderExt {
    fn group(self, group: impl GroupExt) -> Self;
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

pub trait FlexBuilderExt {
    fn widget_with_size(self, size: i32, widget: impl WidgetExt) -> Self;
}

impl FlexBuilderExt for Flex {
    fn widget_with_size(mut self, size: i32, widget: impl WidgetExt) -> Self {
        self.set_size(&widget, size);
        self.widget(widget)
    }
}