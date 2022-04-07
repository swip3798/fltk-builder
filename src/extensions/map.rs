use fltk::prelude::WidgetExt;

pub trait MapExt {
    fn with_key(self, key: &'static str) -> Self;
}

impl<W> MapExt for W 
where W: WidgetExt + Clone + Send + Sync + 'static
{
    fn with_key(self, key: &'static str) -> Self {
        crate::label_map::set_widget_to_id(key, self.clone()).unwrap();
        self
    }
}