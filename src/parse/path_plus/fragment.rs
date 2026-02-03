use crate::parse::Error;
use crate::parse::Error::InvalidFragment;
use crate::Fragment;

/// Parses the optional `fragment`.
///
/// The `fragment` must start with a `#` or be empty.
///
/// Returns `Ok(Some(fragment))`.
/// Returns `Ok(None)` if the `fragment` is empty.
/// Returns `Err(InvalidFragment)` if the fragment is invalid.
pub fn parse_fragment(fragment: &str) -> Result<Option<&str>, Error> {
    if fragment.is_empty() {
        Ok(None)
    } else if Fragment::is_valid(fragment) {
        Ok(Some(fragment))
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
            let result: Result<Option<&str>, Error> = parse_fragment(fragment);
            assert_eq!(result, *expected);
        }
    }
}
