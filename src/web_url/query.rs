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

    /// Gets the length of the query string. (including the '?' prefix)
    pub(in crate::web_url) fn query_len(&self) -> usize {
        (self.query_end - self.path_end) as usize
    }
}

impl WebUrl {
    //! Query Mutation

    /// Sets the optional `query`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn set_query<'a, Q>(&mut self, query: Q)
    where
        Q: Into<Option<Query<'a>>>,
    {
        // The query is preserved exactly, so a query string is already the normalized form. A URL with no query has no
        // '?' either.
        let query: Option<Query> = query.into();
        self.set_query_str(query.map(Query::as_str).unwrap_or(""));
    }

    /// Sets the optional `query`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_query<'a, Q>(mut self, query: Q) -> Self
    where
        Q: Into<Option<Query<'a>>>,
    {
        self.set_query(query);
        self
    }

    /// Sets the query string, which must be a valid query or be empty.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub(in crate::web_url) fn set_query_str(&mut self, query: &str) {
        let start: usize = self.path_end as usize;
        let end: usize = self.query_end as usize;

        // The length is checked before anything is modified so an over-long URL panics with the URL intact rather than
        // leaving the string inconsistent with the component offsets.
        Self::check_len((self.url.len() - self.query_len()) + query.len());

        // Only the fragment follows the query, so the splice shifts the fragment alone.
        self.url.replace_range(start..end, query);

        self.query_end = (start + query.len()) as u32;

        debug_assert!(self.is_consistent());
    }
}

#[cfg(test)]
mod tests {
    use crate::{Query, WebUrl};
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

    #[test]
    fn set_query() -> Result<(), Box<dyn Error>> {
        // The query is preserved exactly & a URL with no query has no '?' either.
        let test_cases: &[(&str, Option<&str>, &str)] = &[
            ("http://host/p", Some("?a=1"), "http://host/p?a=1"),
            ("http://host/p?a=1", Some("?b=2"), "http://host/p?b=2"),
            ("http://host/p?a=1", None, "http://host/p"),
            ("http://host/p", None, "http://host/p"),
            (
                "http://host/p#f",
                Some("?a=1&b=2"),
                "http://host/p?a=1&b=2#f",
            ),
            ("http://host/p?a=1#f", Some("?"), "http://host/p?#f"),
            ("http://host/p?a=1#f", None, "http://host/p#f"),
        ];
        for (input, query, expected) in test_cases {
            let mut url: WebUrl = WebUrl::from_str(input)?;
            url.set_query(query.map(Query::try_from).transpose()?);
            assert_eq!(url.as_str(), *expected, "input={}", input);
            assert_eq!(url.query().map(Query::as_str), *query, "input={}", input);
        }

        Ok(())
    }

    #[test]
    fn with_query() -> Result<(), Box<dyn Error>> {
        let url: WebUrl =
            WebUrl::from_str("https://example.com/p")?.with_query(Query::try_from("?a=1")?);
        assert_eq!(url.as_str(), "https://example.com/p?a=1");

        let url: WebUrl = url.with_query(None);
        assert_eq!(url.as_str(), "https://example.com/p");

        Ok(())
    }
}
