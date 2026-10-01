use crate::{Fragment, WebUrl};

impl WebUrl {
    //! Fragment

    /// Gets the optional fragment.
    #[must_use]
    pub const fn fragment(&self) -> Option<Fragment<'_>> {
        let fragment: &str = self.fragment_str();
        if fragment.is_empty() {
            None
        } else {
            Some(unsafe { Fragment::new_unchecked(fragment) })
        }
    }

    /// Gets the fragment string.
    ///
    /// This will be a valid fragment starting with a '#' or empty.
    const fn fragment_str(&self) -> &str {
        let start: usize = self.offsets.query_end as usize;
        self.url.as_str().split_at(start).1
    }
}

impl WebUrl {
    //! Fragment Mutation

    /// Sets the optional `fragment`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn set_fragment<'a, F: Into<Option<Fragment<'a>>>>(&mut self, fragment: F) {
        let fragment: Option<Fragment> = fragment.into();
        let base_len: usize = self.offsets.query_end as usize;
        Self::check_len(base_len + fragment.map(|f| f.as_str().len()).unwrap_or(0));
        self.url.truncate(base_len);
        if let Some(fragment) = fragment {
            self.url.push_str(fragment.as_str())
        }
        debug_assert!(self.is_consistent());
    }

    /// Sets the optional `fragment`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_fragment<'a, F: Into<Option<Fragment<'a>>>>(mut self, fragment: F) -> Self {
        self.set_fragment(fragment);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{Fragment, WebUrl};
    use std::str::FromStr;

    #[test]
    fn fragment() {
        let url: WebUrl = WebUrl::from_str("https://example.com/path#section").unwrap();
        let fragment: Fragment = url.fragment().unwrap();
        assert_eq!(fragment.as_str(), "#section");
        assert_eq!(fragment.value(), "section");

        let url: WebUrl = WebUrl::from_str("https://example.com/path").unwrap();
        assert!(url.fragment().is_none());
    }

    #[test]
    fn set_fragment() {
        let mut url: WebUrl = WebUrl::from_str("https://example.com").unwrap();
        url.set_fragment(Fragment::try_from("#fragment").unwrap());
        assert_eq!(url.as_str(), "https://example.com/#fragment");

        let mut url: WebUrl = WebUrl::from_str("https://example.com/path#fragment").unwrap();
        url.set_fragment(None);
        assert!(url.fragment().is_none());
        assert_eq!(url.as_str(), "https://example.com/path");
    }

    #[test]
    fn with_fragment() {
        let url: WebUrl = WebUrl::from_str("https://example.com")
            .unwrap()
            .with_fragment(Fragment::try_from("#frag").unwrap());
        assert_eq!(url.as_str(), "https://example.com/#frag");

        let url: WebUrl = WebUrl::from_str("https://example.com/path#old")
            .unwrap()
            .with_fragment(None);
        assert_eq!(url.as_str(), "https://example.com/path");
    }
}
