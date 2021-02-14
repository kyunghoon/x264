//! An x264 wrapper, so that you can safely encode H.264 video.

#![no_std]
#![warn(missing_docs)]

mod x264_bindings;
mod colorspace;
mod data;
mod encoder;
mod error;
mod image;
mod picture;
mod setup;

use x264_bindings as x264;
pub use colorspace::*;
pub use data::*;
pub use encoder::*;
pub use error::*;
pub use image::*;
pub use picture::*;
pub use setup::*;
