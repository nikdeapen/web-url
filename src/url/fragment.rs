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
}
