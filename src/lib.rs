#![doc = include_str!("../README.md")]

pub use address;

pub use error::*;
pub use parts::*;
pub use web_url::*;

mod error;
mod parts;
mod web_url;

mod parse;
