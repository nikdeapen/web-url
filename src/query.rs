use std::fmt::{Display, Formatter};

use crate::parse::Error;
use crate::parse::Error::InvalidQuery;
use crate::Param;

/// A web-based URL query.
///
/// # Validation
/// A query will never be empty and will always start with a '?'.
///
/// The query string can contain any US-ASCII letter, number, or punctuation char excluding '#'
/// since this char denotes the end of the query in the URL.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Query<'a> {
    query: &'a str,
}

impl<'a> Query<'a> {
    //! Construction

    /// Creates a new query.
    ///
    /// # Safety
    /// The `query` must be valid.
    pub unsafe fn new(query: &'a str) -> Self {
        debug_assert!(Self::is_valid(query));

        Self { query }
    }
}

impl<'a> TryFrom<&'a str> for Query<'a> {
    type Error = Error;

    fn try_from(query: &'a str) -> Result<Self, Self::Error> {
        if Self::is_valid(query) {
            Ok(Self { query })
        } else {
            Err(InvalidQuery)
        }
    }
}

impl<'a> Query<'a> {
    //! Validation

    /// Checks if the `query` is valid.
    pub fn is_valid(query: &str) -> bool {
        !query.is_empty()
            && query.as_bytes()[0] == b'?'
            && query.as_bytes()[1..]
                .iter()
                .all(|c| c.is_ascii_alphanumeric() || (c.is_ascii_punctuation() && *c != b'#'))
    }
}

impl<'a> Query<'a> {
    //! String

    /// Gets the query string.
    pub const fn as_str(&self) -> &str {
        self.query
    }
}

impl<'a> AsRef<str> for Query<'a> {
    fn as_ref(&self) -> &str {
        self.query
    }
}

impl<'a> Display for Query<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.query)
    }
}

impl<'a> Query<'a> {
    //! Iteration

    /// Creates a new iterator for the query parameters.
    pub const fn iter(&self) -> impl Iterator<Item = Param<'a>> {
        ParamIterator {
            remaining: self.query,
        }
    }
}

/// Responsible for iterating over query parameters.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct ParamIterator<'a> {
    remaining: &'a str,
}

impl<'a> Iterator for ParamIterator<'a> {
    type Item = Param<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            None
        } else {
            self.remaining = &self.remaining[1..];
            if let Some(amp) = self.remaining.as_bytes().iter().position(|c| *c == b'&') {
                let result: Param = unsafe { Param::from_str(&self.remaining[..amp]) };
                self.remaining = &self.remaining[amp..];
                Some(result)
            } else {
                let result: Param = unsafe { Param::from_str(self.remaining) };
                self.remaining = "";
                Some(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Param, Query};

    #[test]
    fn new() {
        let query: Query = unsafe { Query::new("?the&query=params") };
        assert_eq!(query.query, "?the&query=params");
    }

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", false),
            ("?", true),
            ("?#", false),
            ("?&/=!@$%^&*()", true),
            ("?azAZ09", true),
        ];
        for (query, expected) in test_cases {
            let result: bool = Query::is_valid(query);
            assert_eq!(result, *expected, "query={}", query);
        }
    }

    #[test]
    fn display() {
        let query: Query = unsafe { Query::new("?the&query=params") };
        assert_eq!(query.as_str(), "?the&query=params");
        assert_eq!(query.as_ref(), "?the&query=params");
        assert_eq!(query.to_string(), "?the&query=params");
    }

    #[test]
    fn iter_params() {
        let query: Query = unsafe { Query::new("?") };
        let result: Vec<Param> = query.iter().collect();
        assert_eq!(result, vec![unsafe { Param::new("", None) }]);

        let query: Query = unsafe { Query::new("?&") };
        let result: Vec<Param> = query.iter().collect();
        assert_eq!(
            result,
            vec![unsafe { Param::new("", None) }, unsafe {
                Param::new("", None)
            }]
        );

        let query: Query = unsafe { Query::new("?the&query=params") };
        let result: Vec<Param> = query.iter().collect();
        assert_eq!(
            result,
            vec![unsafe { Param::new("the", None) }, unsafe {
                Param::new("query", Some("params"))
            }]
        );
    }
}
