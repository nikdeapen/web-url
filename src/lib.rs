#![doc = include_str!("../README.md")]
#![allow(clippy::module_inception)]
#![warn(clippy::must_use_candidate)]

pub use address;

pub use error::*;
pub use parts::*;
pub use web_url::*;

mod error;
mod parts;
mod web_url;

mod parse;
