use crate::{Path, WebUrl};

impl WebUrl {
    //! Path

    /// Gets the path.
    pub fn path(&self) -> Path<'_> {
        unsafe { Path::new_unchecked(self.path_str()) }
    }

    /// Gets the path string.
    ///
    /// This will be a valid path starting with a '/'.
    fn path_str(&self) -> &str {
        let start: usize = self.port_end as usize;
        let end: usize = self.path_end as usize;
        &self.url[start..end]
    }
}

impl WebUrl {
    //! Path Mutation

    /// Sets the `path`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn set_path(&mut self, path: Path) {
        let start: usize = self.port_end as usize;
        let end: usize = self.path_end as usize;

        // The length is checked before anything is modified so an over-long URL panics with the URL
        // intact rather than leaving the string inconsistent with the component offsets.
        Self::check_len((self.url.len() - (end - start)) + path.as_str().len());

        // The query & fragment follow the path & are unchanged, so the query length is saved to
        // rebuild the offsets that the splice shifts.
        let query_len: u32 = self.query_end - self.path_end;

        self.url.replace_range(start..end, path.as_str());

        self.path_end = (start + path.as_str().len()) as u32;
        self.query_end = self.path_end + query_len;

        debug_assert!(self.is_consistent());
    }

    /// Sets the `path`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_path(mut self, path: Path) -> Self {
        self.set_path(path);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::WebUrl;
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn path_explicit() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com/the/path")?;
        assert_eq!(url.path().as_str(), "/the/path");
        Ok(())
    }

    #[test]
    fn path_default() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com")?;
        assert_eq!(url.path().as_str(), "/");
        Ok(())
    }
}
