use crate::Error::*;
use std::fmt::{Display, Formatter};

/// An error parsing a web-based URL.
#[non_exhaustive]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    /// The scheme was invalid.
    InvalidScheme,

    /// The URL had user info, which is not supported.
    UserInfoNotSupported,

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
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            InvalidScheme => "invalid scheme",
            UserInfoNotSupported => "user info is not supported",
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
        f.pad(self.message())
    }
}

impl std::error::Error for Error {}
