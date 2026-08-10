use std::fmt::{Debug, Display, Formatter};

use crate::WebUrl;

impl WebUrl {
    //! Display

    /// Gets the URL string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }
}

impl AsRef<str> for WebUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for WebUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for WebUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

#[cfg(test)]
mod tests {
    use crate::WebUrl;
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn display() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com/path?query=1#frag")?;
        assert_eq!(url.as_str(), "https://example.com/path?query=1#frag");
        assert_eq!(url.as_ref(), "https://example.com/path?query=1#frag");
        assert_eq!(url.to_string(), "https://example.com/path?query=1#frag");
        Ok(())
    }
}
