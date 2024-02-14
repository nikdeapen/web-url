use crate::{ParseError, Path};

/// Extracts the path from the prefix of `s`. Returns the path and the rest of `s`.
pub fn extract_path(s: &str) -> Result<(Path, &str), ParseError> {
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
    use crate::{extract_path, Path};

    #[test]
    fn fn_extract_path() {
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
                expected.map(|(p, s)| (unsafe { Path::new_unchecked(p) }, s));
            let result: Option<(Path, &str)> = extract_path(*s).ok();
            assert_eq!(result, expected, "s={}", *s);
        }
    }
}
