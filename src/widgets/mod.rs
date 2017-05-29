use orbclient::Renderer;
use std::any::Any;
use std::cell::Cell;

use event::Event;
use rect::Rect;

pub use self::button::Button;
pub use self::grid::Grid;
pub use self::image::Image;
pub use self::label::Label;
pub use self::menu::{Menu, Action, Separator};
pub use self::progress_bar::ProgressBar;
pub use self::text_box::TextBox;
pub use self::list::{List, Entry};

mod button;
mod grid;
mod image;
mod label;
mod menu;
mod progress_bar;
mod text_box;
mod list;

pub trait Widget: Any {
    fn rect(&self) -> &Cell<Rect>;
    fn draw(&self, renderer: &mut Renderer, focused: bool);
    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool;
}
