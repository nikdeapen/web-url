use crate::parse::Error;
use crate::parse::Error::InvalidFragment;

/// Parses the optional fragment.
///
/// - `fragment` should start with a `#` or be empty.
/// -`The fragment will be validated.
/// - Returns the fragment string with the '#' prefix or `None` if `fragment` was empty.
pub fn parse_fragment(fragment: &str) -> Result<Option<&str>, Error> {
    if fragment.is_empty() {
        Ok(None)
    } else if fragment.as_bytes()[0] == b'#'
        && (&fragment[1..])
            .as_bytes()
            .iter()
            .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation())
    {
        Ok(Some(&fragment))
    } else {
        Err(InvalidFragment)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::path_plus::parse_fragment;
    use crate::parse::Error;
    use crate::parse::Error::InvalidFragment;

    #[test]
    fn fn_parse_fragment() {
        let test_cases: &[(&str, Result<Option<&str>, Error>)] = &[
            ("", Ok(None)),
            ("fragment", Err(InvalidFragment)),
            ("#", Ok(Some("#"))),
            ("#fragment", Ok(Some("#fragment"))),
            ("#\x00", Err(InvalidFragment)),
            ("#你好", Err(InvalidFragment)),
            ("#fragment ", Err(InvalidFragment)),
        ];
        for (fragment, expected) in test_cases {
            let result: Result<Option<&str>, Error> = parse_fragment(*fragment);
            assert_eq!(result, *expected);
        }
    }
}
