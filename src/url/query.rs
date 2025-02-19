use crate::{Param, Query, WebUrl};

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

    /// Clears the query.
    pub fn clear_query(&mut self) {
        let start: usize = self.path_end as usize;
        let end: usize = self.query_end as usize;
        self.url.drain(start..end);
        self.query_end = self.path_end;
    }

    /// Clears the query.
    pub fn with_cleared_query(mut self) -> Self {
        self.clear_query();
        self
    }
}

impl WebUrl {
    //! Params

    /// Adds the `param`.
    pub fn add_param(&mut self, param: Param) {
        // todo -- sub-optimal performance, likely not an issue especially if no fragment
        // todo -- ignores max length

        let len: usize = 1 + param.name().len() + param.value().map(|v| v.len() + 1).unwrap_or(0);
        self.url.reserve(len);

        let mut index: usize = self.path_end as usize;

        if self.query_str().is_empty() {
            self.url.insert_str(index, "?")
        } else {
            self.url.insert_str(index, "&");
        };
        index += 1;

        self.url.insert_str(index, param.name());
        index += param.name().len();

        if let Some(value) = param.value() {
            self.url.insert_str(index, "=");
            index += 1;

            self.url.insert_str(index, value);
            index += value.len();
        }

        self.query_end = index as u32;
    }

    /// Adds the `param`.
    pub fn with_param(mut self, param: Param) -> Self {
        self.add_param(param);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{Fragment, Param, Query, WebUrl};

    #[test]
    fn fragment() {
        let mut url: WebUrl = WebUrl::from_str("https://example.com/#frag").unwrap();
        let fragment: Fragment = unsafe { Fragment::new("#frag") };

        assert_eq!(url.query(), None);

        url.add_param(unsafe { Param::from_str("param") });
        assert_eq!(url.query(), Some(unsafe { Query::new("?param") }));
        assert_eq!(url.fragment(), Some(fragment));

        url.clear_query();
        assert_eq!(url.query(), None);
        assert_eq!(url.fragment(), Some(fragment));

        url.add_param(unsafe { Param::from_str("param=value") });
        assert_eq!(url.query(), Some(unsafe { Query::new("?param=value") }));
        assert_eq!(url.fragment(), Some(fragment));
    }
}
