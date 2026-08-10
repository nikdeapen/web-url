use crate::{Param, Query, WebUrl};

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

impl WebUrl {
    //! Query Parameter Mutations

    /// Adds the query `param`.
    ///
    /// This always appends exactly one parameter and never changes the parameters that are already
    /// present. The '?' separator is used when the URL has no query yet, otherwise the '&' separator
    /// is used. Since an empty region between separators is an empty parameter, a query that is just
    /// a '?' already has one parameter and therefore still needs the '&' separator.
    ///
    /// # Example
    /// Adding the param `p=1`:
    /// - `"/"` -> `"/?p=1"`
    /// - `"/?"` -> `"/?&p=1"`
    /// - `"/?a=2"` -> `"/?a=2&p=1"`
    /// - `"/?&"` -> `"/?&&p=1"`
    ///
    /// # Panics
    /// If the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn add_param(&mut self, param: Param) {
        let separator: char = if self.path_end == self.query_end { '?' } else { '&' };
        let added: usize = separator.len_utf8() + param.name().len() + param.value().map(|v| 1 + v.len()).unwrap_or(0);

        // The length is checked before anything is modified so an over-long URL panics with the URL
        // intact rather than leaving the string inconsistent with the component offsets.
        Self::check_len(self.url.len() + added);

        // The param is assembled first so it can be spliced in with a single insertion. Inserting
        // each piece directly would shift everything after the query once per piece.
        let mut insert: String = String::with_capacity(added);
        insert.push(separator);
        insert.push_str(param.name());
        if let Some(value) = param.value() {
            insert.push('=');
            insert.push_str(value);
        }

        // Only the fragment follows the query, so the insertion shifts the fragment alone.
        let at: usize = self.query_end as usize;
        self.url.insert_str(at, insert.as_str());

        self.query_end = (at + insert.len()) as u32;
    }

    /// Adds the query `param`.
    pub fn with_param(mut self, param: Param) -> Self {
        self.add_param(param);
        self
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
    fn query_accessor() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com/path?key=value")?;
        let query = url.query().unwrap();
        assert_eq!(query.as_str(), "?key=value");

        let url = WebUrl::from_str("https://example.com/path")?;
        assert!(url.query().is_none());

        Ok(())
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
        // The '?' & '&' chars are separators, so an empty region between them is an empty parameter.
        // Adding a parameter must append exactly one & leave the existing ones untouched.
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
}
