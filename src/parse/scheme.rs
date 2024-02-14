use crate::parse::ParseError;
use crate::parse::ParseError::InvalidScheme;
use crate::Scheme;

/// Extracts the scheme length from the prefix of `s`. Returns the scheme length and the rest of
/// `s` after the scheme separator (://). The scheme will be valid but may have uppercase.
pub fn extract_scheme_len(s: &str) -> Result<(usize, &str), ParseError> {
    if let Some(colon) = s.as_bytes().iter().position(|c| *c == b':') {
        if Scheme::is_valid(&s[..colon], true) {
            let s: &str = &s[colon + 1..];
            if s.len() < 2 || s.as_bytes()[0] != b'/' || s.as_bytes()[1] != b'/' {
                Err(InvalidScheme)
            } else {
                Ok((colon, &s[2..]))
            }
        } else {
            Err(InvalidScheme)
        }
    } else {
        Err(InvalidScheme)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::extract_scheme_len;

    #[test]
    fn fn_extract_scheme_len() {
        let test_cases: &[(&str, Option<(usize, &str)>)] = &[
            ("", None),
            ("s:", None),
            ("s:/", None),
            ("s:/x", None),
            ("s:x/", None),
            ("!://", None),
            ("://", None),
            ("s://", Some((1, ""))),
            ("s://rest", Some((1, "rest"))),
        ];
        for (s, expected) in test_cases {
            let result: Option<(usize, &str)> = extract_scheme_len(*s).ok();
            assert_eq!(result, *expected);
        }
    }
}
