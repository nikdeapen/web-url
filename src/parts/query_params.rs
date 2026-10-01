use crate::{PieceIterator, Query, QueryParam};
use std::iter::FusedIterator;

impl<'a> Query<'a> {
    //! Params

    /// Gets the params.
    pub const fn params(self) -> QueryParams<'a> {
        QueryParams {
            pieces: PieceIterator::new(self.value(), b'&'),
        }
    }

    /// Gets the first param with the `name`.
    #[must_use]
    pub fn param(self, name: &str) -> Option<QueryParam<'a>> {
        self.params().find(|param| param.name() == name)
    }
}

impl<'a> IntoIterator for Query<'a> {
    type Item = QueryParam<'a>;
    type IntoIter = QueryParams<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.params()
    }
}

/// An iterator over the params of a query.
#[must_use]
#[derive(Clone, Debug)]
pub struct QueryParams<'a> {
    pieces: PieceIterator<'a>,
}

impl<'a> Iterator for QueryParams<'a> {
    type Item = QueryParam<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.pieces
            .next()
            .map(|param| unsafe { QueryParam::new_unchecked(param) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.pieces.size_hint()
    }
}

impl<'a> DoubleEndedIterator for QueryParams<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pieces
            .next_back()
            .map(|param| unsafe { QueryParam::new_unchecked(param) })
    }
}

impl<'a> FusedIterator for QueryParams<'a> {}

#[cfg(test)]
mod tests {
    use crate::Query;

    /// `(name, value)`
    type ParamParts<'a> = (&'a str, Option<&'a str>);

    #[test]
    fn params() {
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
            let result: Vec<ParamParts> = query.params().map(|p| (p.name(), p.value())).collect();
            assert_eq!(result.as_slice(), *expected, "query={}", query);
        }
    }

    #[test]
    fn param() {
        // The first param with the name is found. The name is compared as it appears in the query.
        let test_cases: &[(&str, &str, Option<ParamParts>)] = &[
            ("?a=1&b=2&a=3", "a", Some(("a", Some("1")))),
            ("?a=1&b=2&a=3", "b", Some(("b", Some("2")))),
            ("?flag", "flag", Some(("flag", None))),
            ("?a=1", "b", None),
            // A query that is just a '?' is still one empty param.
            ("?", "", Some(("", None))),
            ("?a%20b=1", "a b", None),
            ("?a%20b=1", "a%20b", Some(("a%20b", Some("1")))),
        ];

        for (query, name, expected) in test_cases {
            let query: Query = Query::new(query).unwrap();
            let result: Option<ParamParts> = query.param(name).map(|p| (p.name(), p.value()));
            assert_eq!(result, *expected, "query={} name={}", query, name);
        }
    }
}
