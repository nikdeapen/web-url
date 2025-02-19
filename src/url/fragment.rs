use crate::{Fragment, WebUrl};

impl WebUrl {
    //! Fragment

    /// Gets the optional fragment.
    pub fn fragment(&self) -> Option<Fragment> {
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

    /// Sets the `fragment`.
    pub fn set_fragment(&mut self, fragment: Fragment) {
        // todo -- ignores max length

        self.clear_fragment();
        self.url.push_str(fragment.as_str());
    }

    /// Sets the `fragment`.
    pub fn with_fragment(mut self, fragment: Fragment) -> Self {
        self.set_fragment(fragment);
        self
    }

    /// Clears the fragment.
    pub fn clear_fragment(&mut self) {
        self.url.truncate(self.query_end as usize)
    }

    /// Clears the fragment.
    pub fn with_cleared_fragment(mut self) -> Self {
        self.clear_fragment();
        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{Fragment, WebUrl};

    #[test]
    fn fragment() {
        let mut url: WebUrl = WebUrl::from_str("https://example.com/").unwrap();
        let fragment: Fragment = unsafe { Fragment::new("#frag") };

        assert_eq!(url.fragment(), None);

        url.set_fragment(fragment);
        assert_eq!(url.fragment(), Some(fragment));

        url.clear_fragment();
        assert_eq!(url.fragment(), None);
    }
}
