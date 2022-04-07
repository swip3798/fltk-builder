use fltk::{prelude::WidgetExt, enums::CallbackTrigger};

pub trait CallbackExt {
    fn with_callback<F>(self, cb: F) -> Self
    where
        F: FnMut(&mut Self) + 'static;
    fn with_trigger(self, trigger: CallbackTrigger) -> Self;
}

impl<W> CallbackExt for W
where
    W: WidgetExt,
{
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
}
