use crate::ParseError::InvalidScheme;
use crate::{ParseError, Scheme};

/// Parses the scheme length from the prefix of `s`.
///
/// The scheme will be valid but may have uppercase chars. The returned rest starts after the `://`.
pub(crate) fn parse_scheme_len(s: &str) -> Result<(usize, &str), ParseError> {
    let colon: usize = s
        .as_bytes()
        .iter()
        .position(|c| *c == b':')
        .ok_or(InvalidScheme)?;
    let (scheme, rest) = s.split_at(colon);
    match rest.strip_prefix("://") {
        Some(rest) if Scheme::is_valid_ignore_case(scheme) => Ok((colon, rest)),
        _ => Err(InvalidScheme),
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError;
    use crate::ParseError::InvalidScheme;
    use crate::parse::parse_scheme_len;

    type TestCase<'a> = (&'a str, Result<(usize, &'a str), ParseError>);

    #[test]
    fn scheme_len() {
        let test_cases: &[TestCase] = &[
            ("", Err(InvalidScheme)),
            ("s:", Err(InvalidScheme)),
            ("s:/", Err(InvalidScheme)),
            ("s:/x", Err(InvalidScheme)),
            ("s:x/", Err(InvalidScheme)),
            ("!://", Err(InvalidScheme)),
            ("://", Err(InvalidScheme)),
            ("s://", Ok((1, ""))),
            ("s://rest", Ok((1, "rest"))),
        ];
        for (s, expected) in test_cases {
            let result: Result<(usize, &str), ParseError> = parse_scheme_len(s);
            assert_eq!(result, *expected, "s={}", s);
        }
    }
}
