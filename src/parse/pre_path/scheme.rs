use crate::parse::Error;
use crate::parse::Error::InvalidScheme;
use crate::Scheme;

/// Parses the scheme length from the prefix of `s`.
///
/// The scheme will be valid but may have uppercase chars.
///
/// Returns `Ok(scheme_len, rest_of_s)`. The `rest_of_s` starts after the `://`.
/// Returns `Err(InvalidScheme)` if the scheme or `://` postfix is invalid.
pub fn parse_scheme_len(s: &str) -> Result<(usize, &str), Error> {
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
    use crate::parse::pre_path::parse_scheme_len;

    #[test]
    fn fn_get_scheme_len() {
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
            let result: Option<(usize, &str)> = parse_scheme_len(s).ok();
            assert_eq!(result, *expected);
        }
    }
}
