use crate::Error;
use crate::Query;

/// Parses the optional query from the prefix of `s`.
///
/// The query is `None` when `s` does not start with a '?'.
pub fn parse_query(s: &str) -> Result<(Option<Query<'_>>, &str), Error> {
    if !s.is_empty() && s.as_bytes()[0] == b'?' {
        if let Some(hash) = s.as_bytes().iter().position(|c| *c == b'#') {
            let (query, fragment) = s.split_at(hash);
            let query: Query = Query::try_from(query)?;
            Ok((Some(query), fragment))
        } else {
            let query: Query = Query::try_from(s)?;
            Ok((Some(query), ""))
        }
    } else {
        Ok((None, s))
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::Query;
    use crate::parse::parse_query;

    #[test]
    #[allow(clippy::type_complexity)]
    fn fn_parse_query() {
        let test_cases: &[(&str, Result<(Option<Query>, &str), Error>)] = &[
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
            let result: Result<(Option<Query>, &str), Error> = parse_query(s);
            assert_eq!(result, *expected, "s={}", *s);
        }
    }
}
