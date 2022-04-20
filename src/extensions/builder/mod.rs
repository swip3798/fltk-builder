// All builder pattern friendly setter functions

mod browser;
mod button;
mod display;
mod flex;
mod group;
mod image;
mod input;
mod menu;
mod table;
mod valuator;
mod widget;
mod window;

pub use browser::BrowserBuilderExt;
pub use button::ButtonBuilderExt;
pub use display::DisplayBuilderExt;
pub use flex::FlexBuilderExt;
pub use group::GroupBuilderExt;
pub use image::ImageBuilderExt;
pub use input::InputBuilderExt;
pub use menu::MenuBuilderExt;
pub use table::TableBuilderExt;
pub use valuator::ValuatorBuilderExt;
pub use widget::WidgetBuilderExt;
pub use window::WindowBuilderExt;