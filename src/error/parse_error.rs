use std::fmt::{Display, Formatter};

/// An error parsing a web-based URL.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ParseError {
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

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Self::InvalidScheme => "invalid scheme",
            Self::UserInfoNotSupported => "user info is not supported",
            Self::InvalidHost => "invalid host",
            Self::InvalidPort => "invalid port",
            Self::InvalidPath => "invalid path",
            Self::InvalidQuery => "invalid query",
            Self::InvalidParam => "invalid query parameter",
            Self::InvalidFragment => "invalid fragment",
            Self::UrlTooLong => "URL too long (>= 4 GiB)",
        };
        f.pad(s)
    }
}

impl std::error::Error for ParseError {}
