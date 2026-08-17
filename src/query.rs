use crate::parse;
use crate::Error;
use crate::Error::InvalidQuery;
use crate::Param;
use std::fmt::{Debug, Display, Formatter};
use std::iter::FusedIterator;

/// A web-based URL query.
///
/// # RFC 3986
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.4>
///
/// # Validation
/// A query string will never be empty and will always start with a '?'. The query string can contain the RFC 3986
/// query chars: the US-ASCII letters and numbers, the unreserved chars "-._~", the sub-delim chars "!$&'()*+,;=", and
/// the ":@/?" chars. The '%' char is also accepted but the percent-encoding is not validated.
///
/// # Parameters
/// A query always has at least one parameter. The '?' and '&' chars are separators, so the regions between them are the
/// parameters and an empty region is an empty parameter. The query `"?"` is a single empty parameter and the query
/// `"?&"` is two of them.
///
/// This is why a URL with no query has no query at all rather than an empty one: the URL `"/"` has zero parameters
/// while the URL `"/?"` has one.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Query<'a> {
    query: &'a str,
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

    /// Creates a new query.
    ///
    /// # Safety
    /// The `query` must be valid.
    pub unsafe fn new_unchecked(query: &'a str) -> Self {
        debug_assert!(Self::is_valid(query));

        Self { query }
    }
}

impl<'a> Default for Query<'a> {
    fn default() -> Self {
        Self { query: "?" }
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

    /// Checks if the `query` is valid.
    #[must_use]
    pub fn is_valid(query: &str) -> bool {
        parse::is_valid_segment(query, b'?', "")
    }
}

impl<'a> Query<'a> {
    //! Properties

    /// Gets the query value. (will not contain the '?' prefix)
    #[must_use]
    pub fn value(self) -> &'a str {
        &self.query[1..]
    }

    /// Gets the query string. (will contain the '?' prefix)
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.query
    }
}

impl<'a> AsRef<str> for Query<'a> {
    fn as_ref(&self) -> &str {
        self.query
    }
}

impl<'a> Debug for Query<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for Query<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.query)
    }
}

impl<'a> Query<'a> {
    //! Params

    /// Creates a new iterator for the query parameters.
    pub const fn iter_params(self) -> ParamIterator<'a> {
        ParamIterator {
            pieces: parse::PieceIterator::new(self.query, b'&'),
        }
    }
}

impl<'a> IntoIterator for Query<'a> {
    type Item = Param<'a>;
    type IntoIter = ParamIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_params()
    }
}

/// Responsible for iterating over query parameters.
#[must_use]
#[derive(Copy, Clone, Debug)]
pub struct ParamIterator<'a> {
    pieces: parse::PieceIterator<'a>,
}

impl<'a> Iterator for ParamIterator<'a> {
    type Item = Param<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let piece: &str = self.pieces.next()?;
        Some(unsafe { Param::from_str_unchecked(piece) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.pieces.size_hint()
    }
}

impl<'a> FusedIterator for ParamIterator<'a> {}

#[cfg(test)]
mod tests {
    use crate::Error::InvalidQuery;
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
        assert_eq!(query.iter_params().count(), 1);
    }

    #[test]
    fn try_from_str() {
        assert_eq!(Query::try_from("?").unwrap().as_str(), "?");
        assert_eq!(Query::try_from("?key=value").unwrap().as_str(), "?key=value");
        assert_eq!(Query::try_from(""), Err(InvalidQuery));
        assert_eq!(Query::try_from("no-question"), Err(InvalidQuery));
        assert_eq!(Query::try_from("?#"), Err(InvalidQuery));
    }

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", false),
            ("?", true),
            ("?azAZ09", true),
            ("?-._~!$&'()*+,;=:@%/?", true),
            ("?#", false),
            ("?<>", false),
            ("?[]", false),
            ("?^`{|}\\\"", false),
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
        let result: Vec<Param> = query.iter_params().collect();
        assert_eq!(result, vec![Param::new("", None).unwrap()]);

        let query: Query = Query::new("?&").unwrap();
        let result: Vec<Param> = query.iter_params().collect();
        assert_eq!(
            result,
            vec![Param::new("", None).unwrap(), Param::new("", None).unwrap()]
        );

        let query: Query = Query::new("?the&query=params").unwrap();
        let result: Vec<Param> = query.iter_params().collect();
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
            vec![Param::new("a", Some("1")).unwrap(), Param::new("b", None).unwrap()]
        );
    }
}
