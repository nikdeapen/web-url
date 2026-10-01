#![doc = include_str!("../README.md")]

pub use address;

pub use parse::*;
pub use parts::*;
pub use web_url::*;

mod parse;
mod parts;
mod web_url;
