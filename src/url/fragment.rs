use crate::{Fragment, WebUrl};

impl WebUrl {
    //! Fragment

    /// Gets the optional fragment.
    #[must_use]
    pub fn fragment(&self) -> Option<Fragment<'_>> {
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
    fn fragment_str(&self) -> &str {
        let start: usize = self.query_end as usize;
        &self.url[start..]
    }
}

impl WebUrl {
    //! Fragment Mutation

    /// Sets the `fragment`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn set_fragment<'a, F>(&mut self, fragment: F)
    where
        F: Into<Option<Fragment<'a>>>,
    {
        let fragment: Option<Fragment> = fragment.into();

        // The fragment runs from `query_end` to the end of the URL, so truncating to `query_end`
        // drops it.
        let base_len: usize = self.query_end as usize;

        // The length is checked before anything is modified so an over-long URL panics with the URL
        // intact. No offset changes here since the fragment is last, but the URL must still stay
        // short enough for its own parser to accept it.
        Self::check_len(base_len + fragment.map(|f| f.as_str().len()).unwrap_or(0));

        self.url.truncate(base_len);
        if let Some(fragment) = fragment {
            self.url.push_str(fragment.as_str())
        }
    }

    /// Sets the `fragment`.
    pub fn with_fragment<'a, F>(mut self, fragment: F) -> Self
    where
        F: Into<Option<Fragment<'a>>>,
    {
        self.set_fragment(fragment);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{Fragment, WebUrl};
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn fragment_accessor() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com/path#section")?;
        let fragment = url.fragment().unwrap();
        assert_eq!(fragment.as_str(), "#section");
        assert_eq!(fragment.value(), "section");

        let url = WebUrl::from_str("https://example.com/path")?;
        assert!(url.fragment().is_none());

        Ok(())
    }

    #[test]
    fn set_fragment() -> Result<(), Box<dyn Error>> {
        let mut url: WebUrl = WebUrl::from_str("https://example.com")?;

        url.set_fragment(Fragment::try_from("#fragment")?);
        assert_eq!(url.as_str(), "https://example.com/#fragment");

        Ok(())
    }

    #[test]
    fn set_fragment_none() -> Result<(), Box<dyn Error>> {
        let mut url = WebUrl::from_str("https://example.com/path#fragment")?;
        assert!(url.fragment().is_some());

        url.set_fragment(None);
        assert!(url.fragment().is_none());
        assert_eq!(url.as_str(), "https://example.com/path");

        Ok(())
    }

    #[test]
    fn with_fragment() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com")?.with_fragment(Fragment::try_from("#frag")?);
        assert_eq!(url.as_str(), "https://example.com/#frag");

        let url = WebUrl::from_str("https://example.com/path#old")?.with_fragment(None);
        assert_eq!(url.as_str(), "https://example.com/path");

        Ok(())
    }
}
