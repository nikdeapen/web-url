use crate::{ParseError, Path};

/// Parses the path from the prefix of `s`.
pub(crate) fn parse_path(s: &str) -> Result<(Path<'_>, &str), ParseError> {
    let end: usize = s
        .as_bytes()
        .iter()
        .position(|c| *c == b'?' || *c == b'#')
        .unwrap_or(s.len());
    let (path, rest) = s.split_at(end);
    Ok((Path::try_from(path)?, rest))
}

#[cfg(test)]
mod tests {
    use crate::ParseError;
    use crate::ParseError::InvalidPath;
    use crate::parse::parse_path;

    type TestCase<'a> = (&'a str, Result<(&'a str, &'a str), ParseError>);

    #[test]
    fn path() {
        let test_cases: &[TestCase] = &[
            ("", Err(InvalidPath)),
            ("no/starting/slash", Err(InvalidPath)),
            ("/", Ok(("/", ""))),
            ("/the/path", Ok(("/the/path", ""))),
            ("/the/path?query", Ok(("/the/path", "?query"))),
            ("/the/path#fragment", Ok(("/the/path", "#fragment"))),
        ];
        for (s, expected) in test_cases {
            let result: Result<(&str, &str), ParseError> =
                parse_path(s).map(|(path, rest)| (path.as_str(), rest));
            assert_eq!(result, *expected, "s={}", s);
        }
    }
}
