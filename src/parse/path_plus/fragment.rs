use crate::ParseError::InvalidFragment;
use crate::{Fragment, ParseError};

/// Checks the optional `fragment`.
///
/// The `fragment` must be a valid fragment or be empty.
pub(crate) fn check_fragment(fragment: &str) -> Result<(), ParseError> {
    if fragment.is_empty() || Fragment::is_valid(fragment) {
        Ok(())
    } else {
        Err(InvalidFragment)
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError;
    use crate::ParseError::InvalidFragment;
    use crate::parse::check_fragment;

    #[test]
    fn fragment() {
        let test_cases: &[(&str, Result<(), ParseError>)] = &[
            ("", Ok(())),
            ("fragment", Err(InvalidFragment)),
            ("#", Ok(())),
            ("#fragment", Ok(())),
            ("#\x00", Err(InvalidFragment)),
            ("#你好", Err(InvalidFragment)),
            ("#fragment ", Err(InvalidFragment)),
        ];
        for (fragment, expected) in test_cases {
            let result: Result<(), ParseError> = check_fragment(fragment);
            assert_eq!(result, *expected, "fragment={}", fragment);
        }
    }
}
