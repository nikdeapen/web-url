use crate::{Query, WebUrl};

impl WebUrl {
    //! Query

    /// Gets the optional query.
    #[must_use]
    pub fn query(&self) -> Option<Query<'_>> {
        let query: &str = self.query_str();
        if query.is_empty() {
            None
        } else {
            Some(unsafe { Query::new_unchecked(query) })
        }
    }

    /// Gets the query string.
    ///
    /// This will be a valid query string starting with a '?' or it will be empty.
    fn query_str(&self) -> &str {
        let start: usize = self.path_end as usize;
        let end: usize = self.query_end as usize;
        &self.url[start..end]
    }
}

#[cfg(test)]
mod tests {
    use crate::WebUrl;
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn query_accessor() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com/path?key=value")?;
        let query = url.query().unwrap();
        assert_eq!(query.as_str(), "?key=value");

        let url = WebUrl::from_str("https://example.com/path")?;
        assert!(url.query().is_none());

        Ok(())
    }
}
