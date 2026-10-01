use crate::web_url::Part;
use crate::{Query, WebUrl};

impl WebUrl {
    //! Query

    /// Gets the optional query.
    #[must_use]
    pub const fn query(&self) -> Option<Query<'_>> {
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
    const fn query_str(&self) -> &str {
        let start: usize = self.offsets.path_end as usize;
        let end: usize = self.offsets.query_end as usize;
        self.url.as_str().split_at(end).0.split_at(start).1
    }
}

impl WebUrl {
    //! Query Mutation

    /// Sets the optional `query`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn set_query<'a, Q: Into<Option<Query<'a>>>>(&mut self, query: Q) {
        let query: Option<Query> = query.into();
        self.set_query_str(query.map(Query::as_str).unwrap_or(""));
    }

    /// Sets the optional `query`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_query<'a, Q: Into<Option<Query<'a>>>>(mut self, query: Q) -> Self {
        self.set_query(query);
        self
    }

    /// Sets the query string, which must be a valid query or be empty.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub(in crate::web_url) fn set_query_str(&mut self, query: &str) {
        self.splice(Part::Query, query);

        debug_assert!(self.is_consistent());
    }
}

#[cfg(test)]
mod tests {
    use crate::{Query, WebUrl};
    use std::str::FromStr;

    #[test]
    fn query() {
        let url: WebUrl = WebUrl::from_str("https://example.com/path?key=value").unwrap();
        let query: Query = url.query().unwrap();
        assert_eq!(query.as_str(), "?key=value");

        let url: WebUrl = WebUrl::from_str("https://example.com/path").unwrap();
        assert!(url.query().is_none());
    }

    #[test]
    fn set_query() {
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
            let mut url: WebUrl = WebUrl::from_str(input).unwrap();
            url.set_query(query.map(|query| Query::try_from(query).unwrap()));
            assert_eq!(url.as_str(), *expected, "input={}", input);
            assert_eq!(url.query().map(Query::as_str), *query, "input={}", input);
        }
    }

    #[test]
    fn with_query() {
        let url: WebUrl = WebUrl::from_str("https://example.com/p")
            .unwrap()
            .with_query(Query::try_from("?a=1").unwrap());
        assert_eq!(url.as_str(), "https://example.com/p?a=1");

        let url: WebUrl = url.with_query(None);
        assert_eq!(url.as_str(), "https://example.com/p");
    }
}
