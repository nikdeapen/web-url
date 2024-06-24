use std::fmt::{Display, Formatter};

/// An error parsing a web-based URL.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    InvalidPath,
    InvalidQuery,
    InvalidFragment,
}

impl Error {
    //! Display

    /// Gets the error message.
    pub const fn message(&self) -> &'static str {
        match self {
            Self::InvalidPath => "invalid path",
            Self::InvalidQuery => "invalid query",
            Self::InvalidFragment => "invalid fragment",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for Error {}
