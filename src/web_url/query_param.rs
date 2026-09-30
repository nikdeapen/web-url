use crate::{QueryParam, WebUrl};

impl WebUrl {
    //! Query Param Mutation

    /// Adds the query `param`.
    ///
    /// A query that is just a '?' is still one empty param, so the `param` is appended after a '&'.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn add_param(&mut self, param: QueryParam) {
        let separator: char = if self.path_end == self.query_end {
            '?'
        } else {
            '&'
        };

        let added: usize = Self::push_param_len(param);
        Self::check_len(self.url.len() + added);
        let mut insert: String = String::with_capacity(added);
        Self::push_param(&mut insert, separator, param);
        let at: usize = self.query_end as usize;
        self.url.insert_str(at, insert.as_str());
        self.query_end = (at + insert.len()) as u32;

        debug_assert!(self.is_consistent());
    }

    /// Adds the query `param`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_param(mut self, param: QueryParam) -> Self {
        self.add_param(param);
        self
    }

    /// Removes every query param with the `name` & gets the number of removed params.
    ///
    /// Removing every param removes the query along with its '?'.
    pub fn remove_params(&mut self, name: &str) -> usize {
        if !self.query().into_iter().flatten().any(|p| p.name() == name) {
            return 0;
        }

        let mut removed: usize = 0;
        let mut query: String = String::with_capacity(self.query_len());
        for param in self.query().into_iter().flatten() {
            if param.name() == name {
                removed += 1;
            } else {
                Self::push_query_param(&mut query, param);
            }
        }
        self.set_query_str(query.as_str());

        removed
    }

    /// Removes every query param with the `name`.
    pub fn without_params(mut self, name: &str) -> Self {
        self.remove_params(name);
        self
    }

    /// Replaces every query param named like the `param` & gets the number of replaced params.
    ///
    /// The first param with the name keeps its position & the rest are removed. When no param has
    /// the name the `param` is appended as with `add_param`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn replace_params(&mut self, param: QueryParam) -> usize {
        let mut replaced: usize = 0;
        let mut query: String = String::with_capacity(self.query_len());
        for existing in self.query().into_iter().flatten() {
            if existing.name() == param.name() {
                replaced += 1;
                if replaced == 1 {
                    Self::push_query_param(&mut query, param);
                }
            } else {
                Self::push_query_param(&mut query, existing);
            }
        }

        if replaced == 0 {
            self.add_param(param);
        } else {
            self.set_query_str(query.as_str());
        }

        replaced
    }

    /// Replaces every query param named like the `param`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_replaced_params(mut self, param: QueryParam) -> Self {
        self.replace_params(param);
        self
    }

    /// Appends the `separator` & the `param` to the `out` string.
    ///
    /// This is the only place a param is spelled out, so `push_param_len` must match what it
    /// writes.
    fn push_param(out: &mut String, separator: char, param: QueryParam) {
        out.push(separator);
        out.push_str(param.name());
        if let Some(value) = param.value() {
            out.push('=');
            out.push_str(value);
        }
    }

    /// Gets the number of bytes `push_param` appends for the `param`. (including its separator)
    fn push_param_len(param: QueryParam) -> usize {
        1 + param.name().len() + param.value().map(|v| 1 + v.len()).unwrap_or(0)
    }

    /// Appends the `param` to the `query` string being rebuilt, with its separator.
    ///
    /// The '?' separator is used when the `query` is empty, otherwise the '&' separator is used.
    fn push_query_param(query: &mut String, param: QueryParam) {
        Self::push_param(query, if query.is_empty() { '?' } else { '&' }, param);
    }
}

#[cfg(test)]
mod tests {
    use crate::{QueryParam, WebUrl};
    use std::str::FromStr;

    #[test]
    fn add_param() {
        // Exactly one param is appended & the existing params are untouched. The '?' separator
        // starts a query & the '&' separator follows any existing param, even the empty one of a
        // query that is just a '?'.
        let test_cases: &[(&str, &str, &str)] = &[
            ("https://host/p", "a=1", "https://host/p?a=1"),
            ("https://host/p#f", "a=1", "https://host/p?a=1#f"),
            ("https://host/p?", "a=1", "https://host/p?&a=1"),
            ("https://host/p?#f", "a=1", "https://host/p?&a=1#f"),
            ("https://host/p?&", "a=1", "https://host/p?&&a=1"),
            ("https://host/p?b=2", "a=1", "https://host/p?b=2&a=1"),
            ("https://host/p?b=2&", "a=1", "https://host/p?b=2&&a=1"),
            ("https://host/p?b=2#f", "a=1", "https://host/p?b=2&a=1#f"),
            ("https://host/p?a=1", "a=2", "https://host/p?a=1&a=2"),
            // A param with no value has no '=', which is distinct from an empty value.
            ("https://host/p", "flag", "https://host/p?flag"),
            (
                "https://host/p?flag",
                "empty=",
                "https://host/p?flag&empty=",
            ),
        ];
        for (input, param, expected) in test_cases {
            let mut url: WebUrl = WebUrl::from_str(input).unwrap();
            url.add_param(QueryParam::try_from(*param).unwrap());
            assert_eq!(url.as_str(), *expected, "input={} param={}", input, param);
        }
    }

    #[test]
    fn with_param() {
        let url: WebUrl = WebUrl::from_str("https://example.com")
            .unwrap()
            .with_param(QueryParam::try_from("a=1").unwrap())
            .with_param(QueryParam::try_from("b=2").unwrap());
        assert_eq!(url.as_str(), "https://example.com/?a=1&b=2");
    }

    #[test]
    fn remove_params() {
        // Removing every param removes the query along with its '?'.
        let test_cases: &[(&str, &str, usize, &str)] = &[
            ("https://host/p?a=1", "a", 1, "https://host/p"),
            ("https://host/p?a=1&b=2", "a", 1, "https://host/p?b=2"),
            ("https://host/p?a=1&b=2&a=3", "a", 2, "https://host/p?b=2"),
            ("https://host/p?a&a=", "a", 2, "https://host/p"),
            ("https://host/p?a=1&b=2#f", "a", 1, "https://host/p?b=2#f"),
            ("https://host/p?a=1#f", "a", 1, "https://host/p#f"),
            // A URL with no matching param is left untouched.
            ("https://host/p?a=1", "b", 0, "https://host/p?a=1"),
            ("https://host/p", "a", 0, "https://host/p"),
            // A query that is just a '?' is still one empty param.
            ("https://host/p?", "", 1, "https://host/p"),
            ("https://host/p?&", "", 2, "https://host/p"),
            ("https://host/p?&a=1", "", 1, "https://host/p?a=1"),
        ];
        for (input, name, removed, expected) in test_cases {
            let mut url: WebUrl = WebUrl::from_str(input).unwrap();
            assert_eq!(
                url.remove_params(name),
                *removed,
                "input={} name={}",
                input,
                name
            );
            assert_eq!(url.as_str(), *expected, "input={} name={}", input, name);
        }
    }

    #[test]
    fn without_params() {
        let url: WebUrl = WebUrl::from_str("https://host/p?a=1&b=2&a=3")
            .unwrap()
            .without_params("a");
        assert_eq!(url.as_str(), "https://host/p?b=2");
    }

    #[test]
    fn replace_params() {
        // The first param with the name keeps its position & the rest are removed.
        let test_cases: &[(&str, &str, usize, &str)] = &[
            ("https://host/p?a=1", "a=9", 1, "https://host/p?a=9"),
            ("https://host/p?b=2&a=1", "a=9", 1, "https://host/p?b=2&a=9"),
            (
                "https://host/p?a=1&b=2&a=3",
                "a=9",
                2,
                "https://host/p?a=9&b=2",
            ),
            ("https://host/p?a=1#f", "a=9", 1, "https://host/p?a=9#f"),
            // The replacement drops the value when it has none.
            ("https://host/p?a=1", "a", 1, "https://host/p?a"),
            // With no param of the name the replacement is appended, as with `add_param`.
            ("https://host/p", "a=9", 0, "https://host/p?a=9"),
            ("https://host/p?b=2", "a=9", 0, "https://host/p?b=2&a=9"),
            ("https://host/p#f", "a=9", 0, "https://host/p?a=9#f"),
            // A query that is just a '?' is still one empty param.
            ("https://host/p?", "a=9", 0, "https://host/p?&a=9"),
            ("https://host/p?", "", 1, "https://host/p?"),
        ];
        for (input, param, replaced, expected) in test_cases {
            let mut url: WebUrl = WebUrl::from_str(input).unwrap();
            let param: QueryParam = QueryParam::try_from(*param).unwrap();
            assert_eq!(
                url.replace_params(param),
                *replaced,
                "input={} param={}",
                input,
                param
            );
            assert_eq!(url.as_str(), *expected, "input={} param={}", input, param);
        }
    }

    #[test]
    fn with_replaced_params() {
        let url: WebUrl = WebUrl::from_str("https://host/p?a=1&b=2&a=3")
            .unwrap()
            .with_replaced_params(QueryParam::try_from("a=9").unwrap());
        assert_eq!(url.as_str(), "https://host/p?a=9&b=2");
    }
}
