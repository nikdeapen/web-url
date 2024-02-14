use crate::{ParseError, Query};

/// Extracts the query from the prefix of `s`. Returns the query and the rest of `s`.
pub fn extract_query(s: &str) -> Result<(Option<Query>, &str), ParseError> {
    if !s.is_empty() && s.as_bytes()[0] == b'?' {
        if let Some(hash) = s.as_bytes().iter().position(|c| *c == b'#') {
            let query: Query = Query::try_from(&s[..hash])?;
            Ok((Some(query), &s[hash..]))
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
    use crate::parse::query::extract_query;
    use crate::{ParseError, Query};

    #[test]
    fn fn_extract_query() {
        let test_cases: &[(&str, Result<(Option<Query>, &str), ParseError>)] = &[
            ("", Ok((None, ""))),
            ("no&start=q", Ok((None, "no&start=q"))),
            ("?", Ok((Some(unsafe { Query::new_unchecked("?") }), ""))),
            (
                "?the&url=query",
                Ok((Some(unsafe { Query::new_unchecked("?the&url=query") }), "")),
            ),
            ("#fragment", Ok((None, "#fragment"))),
            (
                "?#fragment",
                Ok((Some(unsafe { Query::new_unchecked("?") }), "#fragment")),
            ),
            (
                "?the&url=query#fragment",
                Ok((
                    Some(unsafe { Query::new_unchecked("?the&url=query") }),
                    "#fragment",
                )),
            ),
        ];
        for (s, expected) in test_cases {
            let result: Result<(Option<Query>, &str), ParseError> = extract_query(*s);
            assert_eq!(result, *expected, "s={}", *s);
        }
    }
}
