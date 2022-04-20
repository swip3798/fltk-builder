use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk_builder::prelude::*;

fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .widget(
            Frame::default()
                .with_size(200, 100)
                .center_of_parent()
                .with_id("frame"),
        )
        .widget(Button::new(160, 210, 80, 40, "Click me").with_callback({
            // Retrieve the frame once and not every click for better performance
            let mut frame: Frame = fltk_builder::get_widget_by_id("frame").unwrap();
            move |_| frame.set_label("Hello World")
        }));
    wind.end();
    wind.show();
    app.run()
        .unwrap();
}
