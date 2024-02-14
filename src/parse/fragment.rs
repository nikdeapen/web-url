use crate::ParseError;
use crate::ParseError::InvalidFragment;

/// Extracts the optional fragment. The fragment should start with a '#'.
pub fn extract_fragment(fragment: &str) -> Result<Option<&str>, ParseError> {
    if fragment.is_empty() {
        Ok(None)
    } else if fragment.as_bytes()[0] == b'#'
        && (&fragment[1..])
            .as_bytes()
            .iter()
            .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation())
    {
        Ok(Some(&fragment[1..]))
    } else {
        Err(InvalidFragment)
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidFragment;
    use crate::{extract_fragment, ParseError};

    #[test]
    fn fn_is_valid_fragment() {
        let test_cases: &[(&str, Result<Option<&str>, ParseError>)] = &[
            ("", Ok(None)),
            ("fragment", Err(InvalidFragment)),
            ("#", Ok(Some(""))),
            ("#fragment", Ok(Some("fragment"))),
            ("#\x00", Err(InvalidFragment)),
            ("#你好", Err(InvalidFragment)),
            ("#fragment ", Err(InvalidFragment)),
        ];
        for (fragment, expected) in test_cases {
            let result: Result<Option<&str>, ParseError> = extract_fragment(*fragment);
            assert_eq!(result, *expected);
        }
    }
}
