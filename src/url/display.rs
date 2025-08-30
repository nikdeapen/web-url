use std::fmt::{Display, Formatter};

use crate::WebUrl;

impl WebUrl {
    //! Display

    /// Gets the URL string.
    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }
}

impl AsRef<str> for WebUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for WebUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}
