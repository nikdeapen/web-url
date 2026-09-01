use crate::Error;
use crate::Error::InvalidQuery;
use crate::Param;
use crate::PieceIterator;
use crate::parse;
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Map;

/// A web-based URL query.
///
/// # RFC 3986
/// The query string includes the '?' delimiter, which the RFC excludes from the `query` production.
/// <https://www.rfc-editor.org/rfc/rfc3986#section-3.4>
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Query<'a> {
    query: &'a str,
}

impl<'a> Query<'a> {
    //! Validation

    /// Checks if the `query` is valid.
    ///
    /// A query string can never be empty & must start with a '?'. The query itself can be empty. The valid query
    /// format is defined by [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986#section-3.4).
    #[must_use]
    pub const fn is_valid(query: &str) -> bool {
        parse::is_valid_segment(query, b'?', "")
    }
}

impl<'a> Query<'a> {
    //! Construction

    /// Creates a new query.
    pub const fn new(query: &'a str) -> Result<Self, Error> {
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
    pub const unsafe fn new_unchecked(query: &'a str) -> Self {
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
    //! Properties

    /// Gets the query string. (will contain the '?' prefix)
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.query
    }

    /// Gets the query value. (will not contain the '?' prefix)
    #[must_use]
    pub const fn value(self) -> &'a str {
        self.query.split_at(1).1
    }
}

impl<'a> Query<'a> {
    //! Params

    /// Creates a new iterator for the query parameters.
    ///
    /// A query always has at least one param. The '?' & '&' chars are separators, so the regions between them are the
    /// params & an empty region is an empty param. The query `"?"` is a single empty param & the query `"?&"` is two
    /// of them.
    ///
    /// Only a literal '&' char separates, so a `%26` escape stays within its param. Every piece of a valid query is a
    /// valid param, which is why the params are not re-validated.
    ///
    /// This is why a URL with no query has no query at all rather than an empty one: the URL `"/"` has zero params
    /// while the URL `"/?"` has one.
    ///
    /// # Example
    /// `"?a=1&b"` -> `[("a", Some("1")), ("b", None)]`
    pub fn iter_params(self) -> Map<PieceIterator<'a>, fn(&'a str) -> Param<'a>> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for Query<'a> {
    type Item = Param<'a>;
    type IntoIter = Map<PieceIterator<'a>, fn(&'a str) -> Param<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        // Every piece of a valid query is a valid param. (see [`Self::iter_params`])
        PieceIterator::new(self.query, b'&')
            .map(|piece| unsafe { Param::from_str_unchecked(piece) })
    }
}

impl<'a> PartialEq<str> for Query<'a> {
    fn eq(&self, other: &str) -> bool {
        self.query == other
    }
}

impl<'a> PartialEq<Query<'a>> for str {
    fn eq(&self, other: &Query<'a>) -> bool {
        self == other.query
    }
}

impl<'a> PartialEq<&str> for Query<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.query == *other
    }
}

impl<'a> PartialEq<Query<'a>> for &str {
    fn eq(&self, other: &Query<'a>) -> bool {
        *self == other.query
    }
}

impl<'a> PartialEq<String> for Query<'a> {
    fn eq(&self, other: &String) -> bool {
        self.query == other.as_str()
    }
}

impl<'a> PartialEq<Query<'a>> for String {
    fn eq(&self, other: &Query<'a>) -> bool {
        self.as_str() == other.query
    }
}

impl<'a> AsRef<str> for Query<'a> {
    fn as_ref(&self) -> &str {
        self.query
    }
}

impl<'a> Borrow<str> for Query<'a> {
    fn borrow(&self) -> &str {
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
        f.pad(self.query)
    }
}

#[cfg(test)]
mod tests {
    use crate::Query;

    /// The name & optional value of a query param. `(name, value)`
    type ParamParts<'a> = (&'a str, Option<&'a str>);

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", false),
            ("?", true),
            ("??", true),
            ("?azAZ09", true),
            ("?-._~!$&'()*+,;=:@/?", true),
            // A '%' char begins a percent-encoded octet & must be followed by two hex digits.
            ("?a=%20", true),
            ("?a=%26b", true),
            ("?%00", true),
            ("?%", false),
            ("?a=%2", false),
            ("?a=%zz", false),
            ("?a=%2g", false),
            // The '#' char ends the query & is not a query char.
            ("?#", false),
            ("no-question", false),
            ("? ", false),
            ("?<>", false),
            ("?[]", false),
            ("?^`{|}\\\"", false),
            ("?\u{0}", false),
            ("?\u{4f60}", false),
        ];

        for (query, expected) in test_cases {
            let result: bool = Query::is_valid(query);
            assert_eq!(result, *expected, "query={}", query);
        }
    }

    #[test]
    fn iter_params() {
        let test_cases: &[(&str, &[ParamParts])] = &[
            ("?", &[("", None)]),
            ("?&", &[("", None), ("", None)]),
            ("?a", &[("a", None)]),
            ("?a=", &[("a", Some(""))]),
            (
                "?the&query=params",
                &[("the", None), ("query", Some("params"))],
            ),
            (
                "?a=1&b=2&a=3",
                &[("a", Some("1")), ("b", Some("2")), ("a", Some("3"))],
            ),
        ];

        for (query, expected) in test_cases {
            let query: Query = Query::new(query).unwrap();
            let result: Vec<ParamParts> =
                query.iter_params().map(|p| (p.name(), p.value())).collect();
            assert_eq!(result.as_slice(), *expected, "query={}", query);
        }
    }
}
