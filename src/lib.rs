#![doc = include_str!("../README.md")]
#![allow(clippy::module_inception)]
#![warn(clippy::must_use_candidate)]

pub use address;

pub use error::*;
pub use fragment::*;
pub use invalid_url_string::*;
pub use param::*;
pub use path::*;
pub use query::*;
pub use scheme::*;
pub use web_url::*;

mod error;
mod fragment;
mod invalid_url_string;
mod param;
mod path;
mod query;
mod scheme;
mod web_url;

mod parse;
