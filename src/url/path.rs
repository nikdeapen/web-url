use crate::{Path, WebUrl};

impl WebUrl {
    //! Path

    /// Gets the path.
    pub fn path(&self) -> Path<'_> {
        unsafe { Path::new(self.path_str()) }
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
