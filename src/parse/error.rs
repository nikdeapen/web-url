use std::fmt::{Display, Formatter};

/// An error parsing a web-based URL.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ParseError {
    InvalidScheme,
}

impl ParseError {
    //! Display

    /// Gets the error message.
    pub const fn message(&self) -> &'static str {
        match self {
            Self::InvalidScheme => "invalid scheme",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for ParseError {}
