use std::fmt::{Display, Formatter};

/// An error parsing a web-based URL.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ParseError {
    /// The scheme is invalid.
    InvalidScheme,

    /// The URL has user info, which is not supported.
    UserInfoNotSupported,

    /// The host is invalid.
    InvalidHost,

    /// The port is invalid.
    InvalidPort,

    /// The path is invalid.
    InvalidPath,

    /// The query is invalid.
    InvalidQuery,

    /// The query parameter is invalid.
    InvalidQueryParam,

    /// The fragment is invalid.
    InvalidFragment,

    /// The URL is too long. (must be under 4 GiB)
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
            Self::InvalidQueryParam => "invalid query parameter",
            Self::InvalidFragment => "invalid fragment",
            Self::UrlTooLong => "URL too long (>= 4 GiB)",
        };
        f.pad(s)
    }
}

impl std::error::Error for ParseError {}
