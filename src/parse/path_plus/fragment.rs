use crate::parse::Error;
use crate::parse::Error::InvalidFragment;
use crate::Fragment;

/// Checks the optional `fragment`.
///
/// The `fragment` must be a valid fragment or be empty.
///
/// Returns `Ok(())` if the `fragment` is valid or empty.
/// Returns `Err(InvalidFragment)` if the fragment is invalid.
pub fn check_fragment(fragment: &str) -> Result<(), Error> {
    if fragment.is_empty() || Fragment::is_valid(fragment) {
        Ok(())
    } else {
        Err(InvalidFragment)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::path_plus::check_fragment;
    use crate::parse::Error;
    use crate::parse::Error::InvalidFragment;

    #[test]
    fn fn_check_fragment() {
        let test_cases: &[(&str, Result<(), Error>)] = &[
            ("", Ok(())),
            ("fragment", Err(InvalidFragment)),
            ("#", Ok(())),
            ("#fragment", Ok(())),
            ("#\x00", Err(InvalidFragment)),
            ("#你好", Err(InvalidFragment)),
            ("#fragment ", Err(InvalidFragment)),
        ];
        for (fragment, expected) in test_cases {
            let result: Result<(), Error> = check_fragment(fragment);
            assert_eq!(result, *expected, "fragment={}", fragment);
        }
    }
}
