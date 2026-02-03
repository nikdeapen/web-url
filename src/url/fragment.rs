use crate::{Fragment, WebUrl};

impl WebUrl {
    //! Fragment

    /// Gets the optional fragment.
    pub fn fragment(&self) -> Option<Fragment<'_>> {
        let fragment: &str = self.fragment_str();
        if fragment.is_empty() {
            None
        } else {
            Some(unsafe { Fragment::new(fragment) })
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
    pub fn set_fragment<'a, F>(&mut self, fragment: F)
    where
        F: Into<Option<Fragment<'a>>>,
    {
        self.url
            .truncate(self.url.len() - self.fragment_str().len());
        if let Some(fragment) = fragment.into() {
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
    fn set_fragment() -> Result<(), Box<dyn Error>> {
        let mut url: WebUrl = WebUrl::from_str("https://example.com")?;

        url.set_fragment(Fragment::try_from("#fragment")?);
        assert_eq!(url.as_str(), "https://example.com/#fragment");

        Ok(())
    }
}
