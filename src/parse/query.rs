use crate::{ParseError, Query};

/// Extracts the query from the prefix of `s`. Returns the query and the rest of `s`.
pub fn extract_query(s: &str) -> Result<(Query, &str), ParseError> {
    if let Some(hash) = s.as_bytes().iter().position(|c| *c == b'#') {
        let query: Query = Query::try_from(&s[..hash])?;
        Ok((query, &s[hash..]))
    } else {
        let query: Query = Query::try_from(s)?;
        Ok((query, ""))
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::query::extract_query;
    use crate::Query;

    #[test]
    fn fn_extract_query() {
        let test_cases: &[(&str, Option<(&str, &str)>)] = &[
            ("", None),
            ("no&start=q", None),
            ("?", Some(("?", ""))),
            ("?the&url=query", Some(("?the&url=query", ""))),
            ("#fragment", None),
            ("?#fragment", Some(("?", "#fragment"))),
            (
                "?the&url=query#fragment",
                Some(("?the&url=query", "#fragment")),
            ),
        ];
        for (s, expected) in test_cases {
            let expected: Option<(Query, &str)> =
                expected.map(|(q, s)| (unsafe { Query::new_unchecked(q) }, s));
            let result: Option<(Query, &str)> = extract_query(*s).ok();
            assert_eq!(result, expected, "s={}", *s);
        }
    }
}
