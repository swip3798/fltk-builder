# fltk-builder
A crate to enable building an FLTK UI in Rust with a builder pattern using [fltk-rs](https://github.com/fltk-rs/fltk-rs). 

This includes:
* Builder pattern friendly versions for setter functions, starting with a `with_` prefix
* A global map to store any widget based on an ID, which can be assigned using the `with_id` function and retrieved by the `fltk_builder::get_widget_by_id` function

## Usage
Just add the following to your project's Cargo.toml file:
```toml
[dependencies]
fltk = "1"
fltk-builder = "^0.1"
```
If you're not interested in the ID map functionality you can it off by disabling the default features: 
```toml
[dependencies]
fltk = "1"
fltk-builder = { version = "^0.1", default-features = false }
```

An example application based on the `hello_button` example from fltk-rs:
```rust
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk_builder::prelude::*;

fn main() {
    let mut app = fltk_builder::FltkBuilder::new(app::App::default()).window(
        Window::default()
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
            })),
    );
    app.show();
    app.app()
        .run()
        .unwrap();
}
```

You don't need to use the `FLTKBuilder` struct if you don't want to. You can still create the app and window normally and just use the builder pattern functions.

```rust
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
```