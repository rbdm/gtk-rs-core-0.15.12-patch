// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use cairo;
pub use ffi;
pub use glib;
pub use pango;

#[allow(clippy::too_many_arguments)]
#[allow(unused_imports)]
mod auto;

pub use crate::auto::functions::*;
pub use crate::auto::*;
pub mod prelude;

mod font_map;
