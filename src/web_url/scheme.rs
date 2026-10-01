use crate::web_url::Part;
use crate::{Scheme, WebUrl};

impl WebUrl {
    //! Scheme

    /// Gets the scheme.
    pub const fn scheme(&self) -> Scheme<'_> {
        unsafe { Scheme::new_unchecked(self.scheme_str()) }
    }

    /// Gets the scheme string.
    ///
    /// This will be a valid lowercase scheme string.
    const fn scheme_str(&self) -> &str {
        let end: usize = self.offsets.scheme_len as usize;
        self.url.as_str().split_at(end).0
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
        self.splice(Part::Scheme, scheme.as_str());

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
    use std::str::FromStr;

    #[test]
    fn scheme() {
        let test_cases: &[(&str, &str)] = &[
            ("https://example.com", "https"),
            ("http://example.com", "http"),
            ("HTTPS://example.com", "https"),
        ];
        for (input, expected) in test_cases {
            let url: WebUrl = WebUrl::from_str(input).unwrap();
            assert_eq!(url.scheme().as_str(), *expected, "input={}", input);
        }
    }

    #[test]
    fn set_scheme() {
        // The scheme changes length, so every offset after it must shift with it.
        let test_cases: &[(&str, &str, &str)] = &[
            ("http://host/p?q#f", "https", "https://host/p?q#f"),
            ("https://host/p?q#f", "s", "s://host/p?q#f"),
            ("https://host:8080/p", "http", "http://host:8080/p"),
            ("http://[::1]/p?q#f", "https", "https://[::1]/p?q#f"),
        ];
        for (input, scheme, expected) in test_cases {
            let mut url: WebUrl = WebUrl::from_str(input).unwrap();
            url.set_scheme(Scheme::try_from(*scheme).unwrap());
            assert_eq!(url.as_str(), *expected, "input={}", input);
            assert_eq!(url.scheme().as_str(), *scheme, "input={}", input);
        }
    }

    #[test]
    fn with_scheme() {
        let url: WebUrl = WebUrl::from_str("http://example.com/p")
            .unwrap()
            .with_scheme(Scheme::HTTPS);
        assert_eq!(url.as_str(), "https://example.com/p");
    }
}
