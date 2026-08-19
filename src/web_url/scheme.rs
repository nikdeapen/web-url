use crate::{Scheme, WebUrl};

impl WebUrl {
    //! Scheme

    /// Gets the scheme.
    pub fn scheme(&self) -> Scheme<'_> {
        unsafe { Scheme::new_unchecked(self.scheme_str()) }
    }

    /// Gets the scheme string.
    ///
    /// This will be a valid lowercase scheme string.
    fn scheme_str(&self) -> &str {
        let end: usize = self.scheme_len as usize;
        &self.url[..end]
    }
}

impl WebUrl {
    //! Scheme Mutation

    /// Sets the `scheme`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn set_scheme(&mut self, scheme: Scheme) {
        // A scheme is always lowercase, which is the normalized form.
        let insert: &str = scheme.as_str();

        let end: usize = self.scheme_len as usize;

        // The length is checked before anything is modified so an over-long URL panics with the URL intact rather than
        // leaving the string inconsistent with the component offsets.
        Self::check_len((self.url.len() - end) + insert.len());

        // The host, port, path, query, & fragment follow the scheme & are unchanged, so their lengths are saved to
        // rebuild the offsets that the splice shifts.
        let host_len: u32 = self.host_end - self.scheme_len - 3;
        let port_len: u32 = self.port_end - self.host_end;
        let path_len: u32 = self.path_end - self.port_end;
        let query_len: u32 = self.query_end - self.path_end;

        self.url.replace_range(..end, insert);

        self.scheme_len = insert.len() as u32;
        self.host_end = self.scheme_len + 3 + host_len;
        self.port_end = self.host_end + port_len;
        self.path_end = self.port_end + path_len;
        self.query_end = self.path_end + query_len;

        debug_assert!(self.is_consistent());
    }

    /// Sets the `scheme`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_scheme(mut self, scheme: Scheme) -> Self {
        self.set_scheme(scheme);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{Scheme, WebUrl};
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

    #[test]
    fn set_scheme() -> Result<(), Box<dyn Error>> {
        // The scheme changes length, so every offset after it must shift with it.
        let test_cases: &[(&str, &str, &str)] = &[
            ("http://host/p?q#f", "https", "https://host/p?q#f"),
            ("https://host/p?q#f", "s", "s://host/p?q#f"),
            ("https://host:8080/p", "http", "http://host:8080/p"),
            ("http://[::1]/p?q#f", "https", "https://[::1]/p?q#f"),
        ];
        for (input, scheme, expected) in test_cases {
            let mut url: WebUrl = WebUrl::from_str(input)?;
            url.set_scheme(Scheme::try_from(*scheme)?);
            assert_eq!(url.as_str(), *expected, "input={}", input);
            assert_eq!(url.scheme().as_str(), *scheme, "input={}", input);
        }

        Ok(())
    }

    #[test]
    fn with_scheme() -> Result<(), Box<dyn Error>> {
        let url: WebUrl = WebUrl::from_str("http://example.com/p")?.with_scheme(Scheme::HTTPS);
        assert_eq!(url.as_str(), "https://example.com/p");

        Ok(())
    }
}
