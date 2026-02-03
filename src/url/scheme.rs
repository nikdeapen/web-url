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
