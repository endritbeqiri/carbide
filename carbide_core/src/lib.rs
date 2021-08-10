//! # carbide
//!
//! An easy-to-use, immediate-mode, 2D GUI library featuring a range of useful widgets.
//!
//! If you are new to carbide, we recommend checking out [The Guide](./guide/index.html).

#![deny(unsafe_code)] //Todo deny when unsafe code removed from foreach
//#![feature(associated_type_bounds)]
//#![warn(missing_copy_implementations)]
//#![warn(missing_docs)]

#[macro_use]
extern crate carbide_derive;
extern crate lyon;
extern crate self as carbide_core;

pub use serde::*;
pub use serde::de::*;

pub use carbide_derive::*;
pub use draw::Scalar;

pub use crate::color::Color;
pub use crate::ui::Ui;

pub mod color;
pub mod cursor;
pub mod draw;
pub mod environment;
pub mod event;
pub mod flags;
pub mod focus;
pub mod image_map;
pub mod layout;
pub mod mesh;
pub mod prelude;
pub mod render;
pub mod state;
pub mod text;
mod ui;
pub mod utils;
pub mod widget;
pub mod window;
