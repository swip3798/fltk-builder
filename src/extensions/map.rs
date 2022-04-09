use fltk::prelude::WidgetExt;

/// Add the with_id function to assign an ID to the widget for later retrieval
pub trait MapExt {
    /// Adds an ID to the widget. The widget can be at any point retrieved using its ID.
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