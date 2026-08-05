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
///
/// # Parameters
/// A query always has at least one parameter. The '?' and '&' chars are separators, so the regions
/// between them are the parameters and an empty region is an empty parameter. The query `"?"` is a
/// single empty parameter and the query `"?&"` is two of them.
///
/// This is why a URL with no query has no query at all rather than an empty one: the URL `"/"` has
/// zero parameters while the URL `"/?"` has one.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Query<'a> {
    query: &'a str,
}

impl Default for Query<'static> {
    fn default() -> Self {
        Self { query: "?" }
    }
}

impl<'a> Query<'a> {
    //! Construction

    /// Creates a new query.
    pub fn new(query: &'a str) -> Result<Self, Error> {
        if Self::is_valid(query) {
            Ok(Self { query })
        } else {
            Err(InvalidQuery)
        }
    }

    /// Creates a new query without validating it.
    ///
    /// # Safety
    /// The `query` must be valid.
    pub unsafe fn new_unchecked(query: &'a str) -> Self {
        debug_assert!(Self::is_valid(query));

        Self { query }
    }
}

impl<'a> TryFrom<&'a str> for Query<'a> {
    type Error = Error;

    fn try_from(query: &'a str) -> Result<Self, Self::Error> {
        Self::new(query)
    }
}

impl<'a> Query<'a> {
    //! Validation

    /// Checks if the char `c` is valid.
    fn is_valid_char(c: u8) -> bool {
        c.is_ascii_alphanumeric() || (c.is_ascii_punctuation() && c != b'#')
    }

    /// Checks if the `query` is valid.
    pub fn is_valid(query: &str) -> bool {
        !query.is_empty()
            && query.as_bytes()[0] == b'?'
            && query.as_bytes()[1..].iter().all(|c| Self::is_valid_char(*c))
    }
}

impl<'a> Query<'a> {
    //! String

    /// Gets the query string.
    pub const fn as_str(self) -> &'a str {
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
    pub const fn iter(self) -> ParamIterator<'a> {
        ParamIterator {
            remaining: self.query,
        }
    }
}

impl<'a> IntoIterator for Query<'a> {
    type Item = Param<'a>;
    type IntoIter = ParamIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Responsible for iterating over query parameters.
#[derive(Copy, Clone, Debug)]
pub struct ParamIterator<'a> {
    remaining: &'a str,
}

impl<'a> Iterator for ParamIterator<'a> {
    type Item = Param<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            None
        } else {
            // SAFETY: The query was validated, so the region between separators has only valid
            // param chars & no '&' or '#'.
            self.remaining = &self.remaining[1..];
            if let Some(amp) = self.remaining.as_bytes().iter().position(|c| *c == b'&') {
                let result: Param = unsafe { Param::from_str_unchecked(&self.remaining[..amp]) };
                self.remaining = &self.remaining[amp..];
                Some(result)
            } else {
                let result: Param = unsafe { Param::from_str_unchecked(self.remaining) };
                self.remaining = "";
                Some(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::Error::InvalidQuery;
    use crate::{Param, Query};

    #[test]
    fn new() {
        let query: Query = Query::new("?the&query=params").unwrap();
        assert_eq!(query.query, "?the&query=params");

        assert_eq!(Query::new("no-question"), Err(InvalidQuery));
    }

    #[test]
    fn default() {
        let query: Query = Query::default();
        assert_eq!(query.as_str(), "?");
        assert_eq!(query.iter().count(), 1);
    }

    #[test]
    fn try_from_str() {
        assert_eq!(Query::try_from("?").unwrap().as_str(), "?");
        assert_eq!(
            Query::try_from("?key=value").unwrap().as_str(),
            "?key=value"
        );
        assert_eq!(Query::try_from(""), Err(InvalidQuery));
        assert_eq!(Query::try_from("no-question"), Err(InvalidQuery));
        assert_eq!(Query::try_from("?#"), Err(InvalidQuery));
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
        let query: Query = Query::new("?the&query=params").unwrap();
        assert_eq!(query.as_str(), "?the&query=params");
        assert_eq!(query.as_ref(), "?the&query=params");
        assert_eq!(query.to_string(), "?the&query=params");
    }

    #[test]
    fn iter_params() {
        let query: Query = Query::new("?").unwrap();
        let result: Vec<Param> = query.iter().collect();
        assert_eq!(result, vec![Param::new("", None).unwrap()]);

        let query: Query = Query::new("?&").unwrap();
        let result: Vec<Param> = query.iter().collect();
        assert_eq!(
            result,
            vec![Param::new("", None).unwrap(), Param::new("", None).unwrap()]
        );

        let query: Query = Query::new("?the&query=params").unwrap();
        let result: Vec<Param> = query.iter().collect();
        assert_eq!(
            result,
            vec![
                Param::new("the", None).unwrap(),
                Param::new("query", Some("params")).unwrap()
            ]
        );
    }

    #[test]
    fn into_iter() {
        let query: Query = Query::new("?a=1&b").unwrap();
        let mut result: Vec<Param> = Vec::new();
        for param in query {
            result.push(param);
        }
        assert_eq!(
            result,
            vec![
                Param::new("a", Some("1")).unwrap(),
                Param::new("b", None).unwrap()
            ]
        );
    }
}
