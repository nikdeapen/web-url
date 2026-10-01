use crate::ParseError;
use std::fmt::{Display, Formatter};

/// A [ParseError] with the associated owned URL string.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct InvalidUrlError {
    error: ParseError,
    url: String,
}

impl InvalidUrlError {
    //! Construction

    /// Creates a new [InvalidUrlError].
    pub(crate) const fn new(error: ParseError, url: String) -> Self {
        Self { error, url }
    }
}

impl InvalidUrlError {
    //! Properties

    /// Gets the parse error.
    #[must_use]
    pub const fn error(&self) -> ParseError {
        self.error
    }

    /// Gets the invalid URL string.
    #[must_use]
    pub const fn url(&self) -> &str {
        self.url.as_str()
    }
}

impl InvalidUrlError {
    //! Deconstruction

    /// Converts the error back into the invalid URL string.
    #[must_use]
    pub fn into_url(self) -> String {
        self.url
    }
}

impl From<InvalidUrlError> for ParseError {
    fn from(error: InvalidUrlError) -> Self {
        error.error
    }
}

impl Display for InvalidUrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl std::error::Error for InvalidUrlError {}
