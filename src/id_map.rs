use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, PoisonError, RwLock},
};

lazy_static! {
    // WTF is this type?!
    pub static ref ID_MAP: Arc<RwLock<HashMap<&'static str, Box<dyn Any + Send + Sync>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

/// Represents all possible outcomes when using the `get_widget_by_id` function
#[derive(Debug)]
pub enum IdMapError {
    PoisonErrorLock,
    IdNotFound,
    WrongType,
}

impl<T> From<PoisonError<T>> for IdMapError {
    fn from(_: PoisonError<T>) -> Self {
        IdMapError::PoisonErrorLock
    }
}

/**
    Retrieves a widget by its ID   

    If the ID could not be found, the type is wrong or the RwLock used for threadsafety is poisoned, the Result returns an error.   
    The returned widget needs to be typed explicitly:
    ```rust,no_run
    use fltk::{prelude::*, *, button::Button};
    use fltk_builder::{prelude::*, FltkBuilder};

    fn main() {
        let mut app =
            FltkBuilder::new(app::App::default())
                .window(window::Window::default()
                    .with_size(400, 300)
                    .widget(Button::new(10, 10, 50, 50, "Click me!")
                        .with_id("button")));
        // Unwrapping is fine, if it runs once, it will run everytime
        let mut btn: Button = fltk_builder::get_widget_by_id("button").unwrap();
        btn.set_callback(|_| {
            println!("Button was clicked");
        });
        app.show();
        app.app().run().unwrap();
    }
    ```
 */
pub fn get_widget_by_id<F>(id: &str) -> Result<F, IdMapError>
where
    F: Clone + Send + Sync + 'static,
{
    let id_map = ID_MAP.clone();
    let map = id_map.read()?;
    let cast = map
        .get(id)
        .as_ref()
        .unwrap()
        .downcast_ref::<F>()
        .unwrap()
        .clone();
    Ok(cast)
}

pub fn set_widget_to_id<F>(id: &'static str, value: F) -> Result<(), IdMapError>
where
    F: Any + Clone + Send + Sync + 'static,
{
    let id_map = ID_MAP.clone();
    let mut map = id_map.write()?;
    map.insert(id, Box::new(value));
    Ok(())
}


#[cfg(test)]
mod test {
    use fltk::{frame::Frame, prelude::WidgetExt, button::Button};

    use super::*;
    #[test]
    fn test() {
        let frame = Frame::default().with_label("TestFrame");
        set_widget_to_id("frame", frame).expect("Set should have worked");
        let btn = Button::default().with_label("TestButton");
        set_widget_to_id("button", btn).expect("Set should have worked");
        let re_frame: Frame = get_widget_by_id("frame").expect("Get should have worked");
        let re_btn: Button = get_widget_by_id("button").expect("Get should have worked");
        assert_eq!(re_frame.label(), "TestFrame");
        assert_eq!(re_btn.label(), "TestButton");
    }
}