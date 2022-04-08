use fltk::prelude::WidgetExt;

pub trait MapExt {
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