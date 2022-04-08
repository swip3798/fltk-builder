use fltk::{prelude::*, *, group::Flex, button::Button};
use fltk_builder::{prelude::*, FltkBuilder};
use fltk_theme::{ColorTheme, color_themes};

fn main() {
    let mut app =
        FltkBuilder::new(app::App::default())
            .window(window::Window::default()
                .with_size(400, 300)
                .group(Flex::default_fill()
                    .column()
                    .widget_with_size(30, Button::default()
                        .with_label("Test Button")
                        .with_id("btn1"))));
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    app.show();
    let mut btn1: Button = fltk_builder::get_widget_by_id("btn1").unwrap();
    btn1.set_color(btn1.color().lighter());
    btn1.set_callback(|_| {
        println!("Btn1 was clicked");
    });
    app.app().run().unwrap();
}
