use crate::parse::Error;
use crate::Path;

/// Parses the path from the prefix of `s`.
///
/// Returns `Ok(path, rest_of_s)`.
/// Returns `Err(InvalidPath)` if the path is invalid.
pub fn parse_path(s: &str) -> Result<(Path<'_>, &str), Error> {
    if let Some(qh) = s.as_bytes().iter().position(|c| *c == b'?' || *c == b'#') {
        let path: Path = Path::try_from(&s[..qh])?;
        let s: &str = &s[qh..];
        Ok((path, s))
    } else {
        let path: Path = Path::try_from(s)?;
        Ok((path, ""))
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::path_plus::parse_path;
    use crate::Path;

    #[test]
    fn fn_parse_path() {
        let test_cases: &[(&str, Option<(&str, &str)>)] = &[
            ("", None),
            ("no/starting/slash", None),
            ("/", Some(("/", ""))),
            ("/the/path", Some(("/the/path", ""))),
            ("/the/path?query", Some(("/the/path", "?query"))),
            ("/the/path#fragment", Some(("/the/path", "#fragment"))),
        ];
        for (s, expected) in test_cases {
            let expected: Option<(Path, &str)> =
                expected.map(|(p, s)| (unsafe { Path::new(p) }, s));
            let result: Option<(Path, &str)> = parse_path(s).ok();
            assert_eq!(result, expected, "s={}", s);
        }
    }
}
