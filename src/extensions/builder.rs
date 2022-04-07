use fltk::prelude::{GroupExt, WidgetExt};

pub trait BuilderExt {
    fn group(self, group: impl GroupExt) -> Self;
    fn widget(self, widget: impl WidgetExt) -> Self;
}

pub trait FlexBuilderTrait {
    
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
