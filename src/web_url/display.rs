use crate::WebUrl;
use std::fmt::{Debug, Display, Formatter};

impl AsRef<str> for WebUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for WebUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.url.as_str(), f)
    }
}

impl Display for WebUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(self.url.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::WebUrl;
    use std::str::FromStr;

    #[test]
    fn display() {
        let url: WebUrl = WebUrl::from_str("https://example.com/path?query=1#frag").unwrap();
        assert_eq!(url.as_ref(), "https://example.com/path?query=1#frag");
        assert_eq!(url.to_string(), "https://example.com/path?query=1#frag");
    }
}
