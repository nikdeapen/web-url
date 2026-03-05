use crate::{Scheme, WebUrl};

impl WebUrl {
    //! Scheme

    /// Gets the scheme.
    pub fn scheme(&self) -> Scheme<'_> {
        unsafe { Scheme::new(self.scheme_str()) }
    }

    /// Gets the scheme string.
    ///
    /// This will be a valid lowercase scheme string.
    fn scheme_str(&self) -> &str {
        let end: usize = self.scheme_len as usize;
        &self.url[..end]
    }
}

#[cfg(test)]
mod tests {
    use crate::WebUrl;
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn scheme() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com")?;
        assert_eq!(url.scheme().as_str(), "https");

        let url = WebUrl::from_str("http://example.com")?;
        assert_eq!(url.scheme().as_str(), "http");

        let url = WebUrl::from_str("HTTPS://example.com")?;
        assert_eq!(url.scheme().as_str(), "https");

        Ok(())
    }
}
