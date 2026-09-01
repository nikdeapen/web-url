use crate::Error;
use std::fmt::{Display, Formatter};

/// An error parsing a web-based URL from an owned string.
///
/// The invalid URL string can be recovered, like `std::string::FromUtf8Error`.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct InvalidUrlError {
    error: Error,
    url: String,
}

impl InvalidUrlError {
    //! Construction

    /// Creates a new invalid URL error.
    pub(crate) const fn new(error: Error, url: String) -> Self {
        Self { error, url }
    }
}

impl InvalidUrlError {
    //! Properties

    /// Gets the parse error.
    #[must_use]
    pub const fn error(&self) -> Error {
        self.error
    }

    /// Gets the invalid URL string.
    #[must_use]
    pub fn url(&self) -> &str {
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

impl From<InvalidUrlError> for Error {
    fn from(error: InvalidUrlError) -> Self {
        error.error
    }
}

impl Display for InvalidUrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(self.error.message())
    }
}

/// The `source` is not the parse error: its message is this error's message, so a chain would just
/// print it twice. Use `error` for the typed parse error.
impl std::error::Error for InvalidUrlError {}
