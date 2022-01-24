#![forbid(missing_docs)]
#![doc = include_str!("../readme.md")]
#![allow(clippy::needless_return)]

mod error;
mod error_3rd;

pub use self::error::{GGError, MaybeRanged, Result};

#[cfg(feature = "csscolorparser")]
pub extern crate csscolorparser;
// #[cfg(feature = "globset")]
// pub extern crate globset;
// #[cfg(feature = "num")]
// pub extern crate num;
