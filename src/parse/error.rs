use std::fmt::{Display, Formatter};

/// An error parsing a web-based URL.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ParseError {
    InvalidScheme,
    InvalidHost,
    InvalidPort,
    InvalidPath,
    InvalidQuery,
    InvalidFragment,
}

impl ParseError {
    //! Display

    /// Gets the error message.
    pub const fn message(&self) -> &'static str {
        match self {
            Self::InvalidScheme => "invalid scheme",
            Self::InvalidHost => "invalid host",
            Self::InvalidPort => "invalid port",
            Self::InvalidPath => "invalid path",
            Self::InvalidQuery => "invalid query",
            Self::InvalidFragment => "invalid fragment",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for ParseError {}
