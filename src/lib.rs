#![crate_name = "orbtk"]
#![crate_type = "lib"]
// #![deny(warnings)]
#![feature(const_fn)]

pub use dces::prelude::*;

extern crate cssparser;
#[macro_use]
extern crate lazy_static;

pub use crate::application::*;
pub use crate::backend::*;
pub use crate::enums::{*, NotFound};
pub use crate::event::*;
pub use crate::layout_object::*;
pub use crate::render_object::*;
pub use crate::properties::*;
pub use crate::systems::*;
pub use crate::theme::{Selector, Theme, DEFAULT_THEME_CSS, LIGHT_THEME_CSS};
pub use crate::widget::*;

pub mod application;
pub mod backend;
pub mod enums;
pub mod event;
pub mod layout_object;
pub mod render_object;
pub mod properties;
pub mod systems;
pub mod theme;
pub mod widget;

extern crate orbclient;
extern crate orbfont;
extern crate orbimage;
pub use orbclient::color::Color;