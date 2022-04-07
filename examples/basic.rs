use fltk::{prelude::*, *};
use fltk_builder::{prelude::*, FltkBuilder};

fn main() {
    let app =
        FltkBuilder::new(app::App::default()).window(window::Window::default().with_size(400, 300));
}
