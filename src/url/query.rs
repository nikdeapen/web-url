use crate::{Param, Query, WebUrl};

impl WebUrl {
    //! Query

    /// Gets the optional query.
    pub fn query(&self) -> Option<Query<'_>> {
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

impl WebUrl {
    //! Query Parameter Mutations

    /// Adds the query `param`.
    pub fn add_param<'a, P>(&mut self, param: P)
    where
        P: Into<Param<'a>>,
    {
        let param: Param = param.into();
        let end: usize = self.query_end as usize;

        let c: char = if self.path_end == self.query_end { '?' } else { '&' };

        let mut s = String::with_capacity(1 + param.name().len() + param.value().map(|v| 1 + v.len()).unwrap_or(0));
        s.push(c);
        s.push_str(param.name());
        if let Some(value) = param.value() {
            s.push('=');
            s.push_str(value);
        }

        self.url.insert_str(end, &s);
        self.query_end = (end + s.len()) as u32;
    }

    /// Adds the query `param`.
    pub fn with_param<'a, P>(mut self, param: P) -> Self
    where
        P: Into<Param<'a>>,
    {
        self.add_param(param);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{Fragment, Param, WebUrl};
    use std::error::Error;
    use std::str::FromStr;

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
}
