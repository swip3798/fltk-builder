use fltk::{button::Button, group::Flex, prelude::*, *};
use fltk_builder::{prelude::*, FltkBuilder};

fn main() {
    let mut app = FltkBuilder::new(app::App::default()).window(
        window::Window::default()
            .with_size(400, 300)
            .group(
                Flex::default_fill()
                    .column()
                    .widget_with_size(
                        30,
                        Button::default()
                            .with_label("Test Button")
                            .with_id("btn1"),
                    ),
            ),
    );

    app.show();
    let mut btn1: Button = fltk_builder::get_widget_by_id("btn1").unwrap();
    btn1.set_callback(|_| {
        println!("Btn1 was clicked");
    });
    app.app()
        .run()
        .unwrap();
}
