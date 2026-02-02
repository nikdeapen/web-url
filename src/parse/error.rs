use std::fmt::{Display, Formatter};

use crate::parse::Error::*;

/// An error parsing a web-based URL.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    /// The scheme was invalid.
    InvalidScheme,

    /// The host was invalid.
    InvalidHost,

    /// The port was invalid.
    InvalidPort,

    /// The path was invalid.
    InvalidPath,

    /// The query was invalid.
    InvalidQuery,

    /// The query parameter was invalid.
    InvalidParam,

    /// The fragment was invalid.
    InvalidFragment,

    /// The URL was too long. (must be under 4 GiB)
    UrlTooLong,
}

impl Error {
    //! Display

    /// Gets the error message.
    pub const fn message(&self) -> &'static str {
        match self {
            InvalidScheme => "invalid scheme",
            InvalidHost => "invalid host",
            InvalidPort => "invalid port",
            InvalidPath => "invalid path",
            InvalidQuery => "invalid query",
            InvalidParam => "invalid query parameter",
            InvalidFragment => "invalid fragment",
            UrlTooLong => "URL too long (>= 4 GiB)",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for Error {}
