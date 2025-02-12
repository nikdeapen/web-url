use crate::WebUrl;

impl WebUrl {
    //! Fragment

    /// Gets the optional fragment. (will not contain the #)
    pub fn fragment(&self) -> Option<&str> {
        self.fragment_with_hash().map(|f| &f[1..])
    }

    /// Gets the optional fragment with the '#' prefix.
    pub fn fragment_with_hash(&self) -> Option<&str> {
        let fragment: &str = self.fragment_str();
        if fragment.is_empty() {
            None
        } else {
            Some(fragment)
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
