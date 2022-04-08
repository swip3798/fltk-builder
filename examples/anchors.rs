use fltk::{prelude::*, *};
use fltk_builder::{prelude::*, FltkBuilder};
use fltk_anchor::{Anchor, Anchored};

const PADDING: i32 = 8;

fn main() {
    let mut app =
        FltkBuilder::new(app::App::default())
            .window(window::Window::default()
                .with_size(400, 300)
                .widget(button::Button::new(PADDING, PADDING, 80, 40, "Click me!").with_anchor(Anchor::Left | Anchor::Top).with_id("btn1"))
                .widget(input::MultilineInput::new(
                    PADDING,
                    PADDING * 2 + 40,
                    400 - PADDING * 2,
                    300 - 40 - PADDING * 3,
                    "",
                )
                    .with_anchor(Anchor::Left | Anchor::Right | Anchor::Top | Anchor::Bottom)
                    .with_id("inp1")));
    let win = app.get_window_mut().as_mut().unwrap();
    win.make_resizable(true);
    app.show();
    // Callbacks
    let mut btn1: button::Button = fltk_builder::get_widget_by_id("btn1").unwrap();
    btn1.set_callback(|_| {
        let inp: input::MultilineInput = fltk_builder::get_widget_by_id("inp1").unwrap();
        println!("Btn1 was clicked; Input contains: {}", inp.value());
    });
    app.app().run().unwrap();
}
