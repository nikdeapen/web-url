use crate::parse::Error;
use crate::Query;

/// Parses the optional query from the prefix of `s`.
///
/// Returns `Ok(optional_query, rest_of_s)`.
/// Returns `Ok(None)` if `s` does not start with a `?`.
/// Returns `Err(InvalidQuery)` if the query is invalid.
#[allow(clippy::type_complexity)]
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
    use crate::parse::path_plus::parse_query;
    use crate::parse::Error;
    use crate::Query;

    #[test]
    #[allow(clippy::type_complexity)]
    fn fn_parse_query() {
        let test_cases: &[(&str, Result<(Option<Query>, &str), Error>)] = &[
            ("", Ok((None, ""))),
            ("no&start=q", Ok((None, "no&start=q"))),
            ("?", Ok((Some(unsafe { Query::new("?") }), ""))),
            (
                "?the&url=query",
                Ok((Some(unsafe { Query::new("?the&url=query") }), "")),
            ),
            ("#fragment", Ok((None, "#fragment"))),
            (
                "?#fragment",
                Ok((Some(unsafe { Query::new("?") }), "#fragment")),
            ),
            (
                "?the&url=query#fragment",
                Ok((Some(unsafe { Query::new("?the&url=query") }), "#fragment")),
            ),
        ];
        for (s, expected) in test_cases {
            let result: Result<(Option<Query>, &str), Error> = parse_query(s);
            assert_eq!(result, *expected, "s={}", *s);
        }
    }
}
