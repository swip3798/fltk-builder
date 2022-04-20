use fltk::prelude::WidgetExt;

/// Add the with_id function to assign an ID to the widget for later retrieval
pub trait MapExt {
    /// Adds an ID to the widget. The widget can be at any point retrieved using its ID.
    /// Panics if RWLock used internally is poisoned
    fn with_id(self, id: &'static str) -> Self;
}

impl<W> MapExt for W 
where W: WidgetExt + Clone + Send + Sync + 'static
{
    fn with_id(self, id: &'static str) -> Self {
        crate::id_map::set_widget_to_id(id, self.clone()).unwrap();
        self
    }
}

#[cfg(test)]
mod test {
    use fltk::{frame::Frame, button::Button};
    use crate::get_widget_by_id;
    use super::*;

    #[test]
    fn test() {
        Frame::default().with_label("TestFrame").with_id("frame");
        Button::default().with_label("TestButton").with_id("button");
        let re_frame: Frame = get_widget_by_id("frame").expect("Get should have worked");
        let re_btn: Button = get_widget_by_id("button").expect("Get should have worked");
        assert_eq!(re_frame.label(), "TestFrame");
        assert_eq!(re_btn.label(), "TestButton");
    }
}