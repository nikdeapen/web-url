use crate::{Query, WebUrl};

impl WebUrl {
    //! Query

    /// Gets the optional query.
    pub fn query(&self) -> Option<Query> {
        let query: &str = self.query_str();
        if query.is_empty() {
            None
        } else {
            Some(unsafe { Query::new(query) })
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
