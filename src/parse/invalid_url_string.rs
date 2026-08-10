use crate::parse::Error;
use std::fmt::{Display, Formatter};

/// An error parsing a web-based URL from an owned string.
///
/// The invalid URL string can be recovered, like `std::string::FromUtf8Error`.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct InvalidUrlString {
    error: Error,
    url: String,
}

impl InvalidUrlString {
    //! Construction

    /// Creates a new invalid URL string error.
    pub(crate) const fn new(error: Error, url: String) -> Self {
        Self { error, url }
    }
}

impl InvalidUrlString {
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

impl InvalidUrlString {
    //! Deconstruction

    /// Converts the error back into the invalid URL string.
    #[must_use]
    pub fn into_url(self) -> String {
        self.url
    }
}

impl From<InvalidUrlString> for Error {
    fn from(error: InvalidUrlString) -> Self {
        error.error
    }
}

impl Display for InvalidUrlString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(self.error.message())
    }
}

impl std::error::Error for InvalidUrlString {}
