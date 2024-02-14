/// Checks if the fragment is valid. The fragment should start with a '#'.
pub fn is_valid_fragment(fragment: &str) -> bool {
    !fragment.is_empty()
        && fragment.as_bytes()[0] == b'#'
        && (&fragment[1..])
            .as_bytes()
            .iter()
            .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation())
}

#[cfg(test)]
mod tests {
    use crate::is_valid_fragment;

    #[test]
    fn fn_is_valid_fragment() {
        let test_cases: &[(&str, bool)] = &[
            ("", false),
            ("fragment", false),
            ("#", true),
            ("#fragment", true),
            ("#\x00", false),
            ("#你好", false),
            ("#fragment ", false),
        ];
        for (fragment, expected) in test_cases {
            let result: bool = is_valid_fragment(*fragment);
            assert_eq!(result, *expected, "fragment={}", *fragment);
        }
    }
}
