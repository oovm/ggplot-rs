#![forbid(missing_docs)]
#![doc = include_str!("../readme.md")]
#![allow(clippy::needless_return)]

mod error;
mod error_3rd;

pub use self::error::{MaybeRanged, Result, GGError};
//
// #[cfg(feature = "git2")]
// pub extern crate git2;
// #[cfg(feature = "globset")]
// pub extern crate globset;
// #[cfg(feature = "num")]
// pub extern crate num;
