// All builder pattern friendly setter functions

mod browser;
mod button;
mod flex;
mod group;
mod input;
mod menu;
mod valuator;
mod widget;

pub use browser::BrowserBuilderExt;
pub use button::ButtonBuilderExt;
pub use flex::FlexBuilderExt;
pub use group::GroupBuilderExt;
pub use input::InputBuilderExt;
pub use menu::MenuBuilderExt;
pub use valuator::ValuatorBuilderExt;
pub use widget::WidgetBuilderExt;
