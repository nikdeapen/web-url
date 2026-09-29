use crate::{ParseError, Query};

/// Parses the optional query from the prefix of `s`.
///
/// The query is `None` when `s` does not start with a '?'.
pub(crate) fn parse_query(s: &str) -> Result<(Option<Query<'_>>, &str), ParseError> {
    if !s.starts_with('?') {
        return Ok((None, s));
    }
    let end: usize = s
        .as_bytes()
        .iter()
        .position(|c| *c == b'#')
        .unwrap_or(s.len());
    let (query, rest) = s.split_at(end);
    Ok((Some(Query::try_from(query)?), rest))
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_query;
    use crate::{ParseError, Query};

    type TestCase<'a> = (&'a str, Result<(Option<Query<'a>>, &'a str), ParseError>);

    #[test]
    fn query() {
        let test_cases: &[TestCase] = &[
            ("", Ok((None, ""))),
            ("no&start=q", Ok((None, "no&start=q"))),
            ("?", Ok((Some(Query::new("?").unwrap()), ""))),
            (
                "?the&url=query",
                Ok((Some(Query::new("?the&url=query").unwrap()), "")),
            ),
            ("#fragment", Ok((None, "#fragment"))),
            (
                "?#fragment",
                Ok((Some(Query::new("?").unwrap()), "#fragment")),
            ),
            (
                "?the&url=query#fragment",
                Ok((Some(Query::new("?the&url=query").unwrap()), "#fragment")),
            ),
        ];
        for (s, expected) in test_cases {
            let result: Result<(Option<Query>, &str), ParseError> = parse_query(s);
            assert_eq!(result, *expected, "s={}", s);
        }
    }
}
