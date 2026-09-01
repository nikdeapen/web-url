use crate::{Param, WebUrl};

impl WebUrl {
    //! Query Parameter Mutations

    /// Adds the query `param`.
    ///
    /// This always appends exactly one parameter & never changes the parameters that are already present. The '?'
    /// separator is used when the URL has no query yet, otherwise the '&' separator is used. Since an empty region
    /// between separators is an empty parameter, a query that is just a '?' already has one parameter & therefore still
    /// needs the '&' separator.
    ///
    /// # Example
    /// Adding the param `p=1`:
    /// - `"/"` -> `"/?p=1"`
    /// - `"/?"` -> `"/?&p=1"`
    /// - `"/?a=2"` -> `"/?a=2&p=1"`
    /// - `"/?&"` -> `"/?&&p=1"`
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn add_param(&mut self, param: Param) {
        // A URL with no query has no '?' either, so the query starts with one here. Every other case appends to a
        // query that already has at least one param.
        let separator: char = if self.path_end == self.query_end {
            '?'
        } else {
            '&'
        };
        let added: usize = Self::push_param_len(param);

        // The length is checked before anything is modified so an over-long URL panics with the URL intact rather than
        // leaving the string inconsistent with the component offsets.
        Self::check_len(self.url.len() + added);

        // The param is assembled first so it can be spliced in with a single insertion. Inserting each piece directly
        // would shift everything after the query once per piece.
        let mut insert: String = String::with_capacity(added);
        Self::push_param(&mut insert, separator, param);

        // Only the fragment follows the query, so the insertion shifts the fragment alone.
        let at: usize = self.query_end as usize;
        self.url.insert_str(at, insert.as_str());

        self.query_end = (at + insert.len()) as u32;

        debug_assert!(self.is_consistent());
    }

    /// Adds the query `param`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_param(mut self, param: Param) -> Self {
        self.add_param(param);
        self
    }

    /// Removes every query param with the `name` & gets the number of removed params.
    ///
    /// Removing every param removes the query along with its '?', since a query that is just a '?' is still one empty
    /// param.
    ///
    /// # Example
    /// Removing the params named `a`:
    /// - `"/?a=1"` -> `"/"`
    /// - `"/?a=1&b=2"` -> `"/?b=2"`
    /// - `"/?a=1&b=2&a=3"` -> `"/?b=2"`
    pub fn remove_params(&mut self, name: &str) -> usize {
        // The query is scanned before it is rebuilt so that a URL without a matching param is left untouched & never
        // allocates.
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
    /// The first param with the name keeps its position & the rest are removed. When no param has the name the `param`
    /// is appended as with `add_param`.
    ///
    /// # Example
    /// Replacing with the param `a=9`:
    /// - `"/"` -> `"/?a=9"`
    /// - `"/?a=1"` -> `"/?a=9"`
    /// - `"/?b=2&a=1"` -> `"/?b=2&a=9"`
    /// - `"/?a=1&b=2&a=3"` -> `"/?a=9&b=2"`
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn replace_params(&mut self, param: Param) -> usize {
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
    pub fn with_replaced_params(mut self, param: Param) -> Self {
        self.replace_params(param);
        self
    }

    /// Appends the `separator` & the `param` to the `out` string.
    ///
    /// This is the only place a param is spelled out, so `push_param_len` must match what it writes.
    fn push_param(out: &mut String, separator: char, param: Param) {
        out.push(separator);
        out.push_str(param.name());
        if let Some(value) = param.value() {
            out.push('=');
            out.push_str(value);
        }
    }

    /// Gets the number of bytes `push_param` appends for the `param`. (including its separator)
    fn push_param_len(param: Param) -> usize {
        1 + param.name().len() + param.value().map(|v| 1 + v.len()).unwrap_or(0)
    }

    /// Appends the `param` to the `query` string being rebuilt, with its separator.
    ///
    /// The '?' separator is used when the `query` is empty, otherwise the '&' separator is used.
    fn push_query_param(query: &mut String, param: Param) {
        Self::push_param(query, if query.is_empty() { '?' } else { '&' }, param);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Fragment, Param, WebUrl};
    use std::error::Error;
    use std::str::FromStr;

    /// Snapshots the query params as owned values.
    ///
    /// The params borrow the URL, so they must be detached to be compared across a mutation.
    fn params_of(url: &WebUrl) -> Vec<(String, Option<String>)> {
        url.query()
            .map(|q| {
                q.iter_params()
                    .map(|p| (p.name().to_string(), p.value().map(str::to_string)))
                    .collect()
            })
            .unwrap_or_default()
    }

    #[test]
    fn add_param() -> Result<(), Box<dyn Error>> {
        let mut url: WebUrl = WebUrl::from_str("https://example.com")?;
        url.set_fragment(Fragment::try_from("#fragment")?);

        url.add_param(Param::try_from("one")?);
        assert_eq!("https://example.com/?one#fragment", url.as_str());

        url.add_param(Param::try_from("two=3")?);
        assert_eq!("https://example.com/?one&two=3#fragment", url.as_str());

        Ok(())
    }

    #[test]
    fn add_param_appends_exactly_one_param() -> Result<(), Box<dyn Error>> {
        // The '?' & '&' chars are separators, so an empty region between them is an empty parameter. Adding a parameter
        // must append exactly one & leave the existing ones untouched.
        let test_cases: &[(&str, &str, usize, usize)] = &[
            ("https://host/p", "https://host/p?p=1", 0, 1),
            ("https://host/p?", "https://host/p?&p=1", 1, 2),
            ("https://host/p?a=2", "https://host/p?a=2&p=1", 1, 2),
            ("https://host/p?&", "https://host/p?&&p=1", 2, 3),
            ("https://host/p?a=2&", "https://host/p?a=2&&p=1", 2, 3),
            ("https://host/p?a=2#f", "https://host/p?a=2&p=1#f", 1, 2),
            ("https://host/p?#f", "https://host/p?&p=1#f", 1, 2),
            ("https://host/p#f", "https://host/p?p=1#f", 0, 1),
        ];
        for (input, expected, before_count, after_count) in test_cases {
            let mut url: WebUrl = WebUrl::from_str(input)?;

            let before: Vec<(String, Option<String>)> = params_of(&url);
            assert_eq!(before.len(), *before_count, "input={input}");

            url.add_param(Param::try_from("p=1")?);
            assert_eq!(url.as_str(), *expected, "input={input}");

            let after: Vec<(String, Option<String>)> = params_of(&url);
            assert_eq!(after.len(), *after_count, "input={input}");

            // The existing params are preserved in order & the new one is appended last.
            assert_eq!(&after[..before.len()], &before[..], "input={input}");
            assert_eq!(
                after[after.len() - 1],
                ("p".to_string(), Some("1".to_string())),
                "input={input}"
            );

            // The path & fragment are untouched & the URL still re-parses identically.
            let reparsed: WebUrl = WebUrl::from_str(url.as_str())?;
            assert_eq!(reparsed, url, "input={input}");
            assert_eq!(reparsed.path().as_str(), "/p", "input={input}");
            assert_eq!(
                reparsed.fragment().map(|f| f.as_str()),
                url.fragment().map(|f| f.as_str()),
                "input={input}"
            );
        }

        Ok(())
    }

    #[test]
    fn add_param_repeated() -> Result<(), Box<dyn Error>> {
        // Repeated additions must keep appending one at a time.
        let mut url: WebUrl = WebUrl::from_str("https://host/p#f")?;
        for (i, expected) in [
            "https://host/p?a=0#f",
            "https://host/p?a=0&a=1#f",
            "https://host/p?a=0&a=1&a=2#f",
        ]
        .iter()
        .enumerate()
        {
            url.add_param(Param::try_from(format!("a={i}").as_str())?);
            assert_eq!(url.as_str(), *expected);
            assert_eq!(url.query().unwrap().iter_params().count(), i + 1);
        }

        Ok(())
    }

    #[test]
    fn add_param_without_value() -> Result<(), Box<dyn Error>> {
        // A param with no value has no '=' at all, which is distinct from an empty value.
        let mut url: WebUrl = WebUrl::from_str("https://host/p")?;
        url.add_param(Param::try_from("flag")?);
        assert_eq!(url.as_str(), "https://host/p?flag");

        url.add_param(Param::try_from("empty=")?);
        assert_eq!(url.as_str(), "https://host/p?flag&empty=");

        let params: Vec<Param> = url.query().unwrap().iter_params().collect();
        assert_eq!(params[0].value(), None);
        assert_eq!(params[1].value(), Some(""));

        Ok(())
    }

    #[test]
    fn with_param() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com")?
            .with_param(Param::try_from("a=1")?)
            .with_param(Param::try_from("b=2")?);
        assert_eq!(url.as_str(), "https://example.com/?a=1&b=2");

        Ok(())
    }

    #[test]
    fn remove_params() -> Result<(), Box<dyn Error>> {
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
            let mut url: WebUrl = WebUrl::from_str(input)?;
            assert_eq!(
                url.remove_params(name),
                *removed,
                "input={input} name={name}"
            );
            assert_eq!(url.as_str(), *expected, "input={input} name={name}");
        }

        Ok(())
    }

    #[test]
    fn without_params() -> Result<(), Box<dyn Error>> {
        let url: WebUrl = WebUrl::from_str("https://host/p?a=1&b=2&a=3")?.without_params("a");
        assert_eq!(url.as_str(), "https://host/p?b=2");

        Ok(())
    }

    #[test]
    fn replace_params() -> Result<(), Box<dyn Error>> {
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
            let mut url: WebUrl = WebUrl::from_str(input)?;
            let param: Param = Param::try_from(*param)?;
            assert_eq!(
                url.replace_params(param),
                *replaced,
                "input={input} param={param}"
            );
            assert_eq!(url.as_str(), *expected, "input={input} param={param}");
        }

        Ok(())
    }

    #[test]
    fn with_replaced_params() -> Result<(), Box<dyn Error>> {
        let url: WebUrl = WebUrl::from_str("https://host/p?a=1&b=2&a=3")?
            .with_replaced_params(Param::try_from("a=9")?);
        assert_eq!(url.as_str(), "https://host/p?a=9&b=2");

        Ok(())
    }
}
