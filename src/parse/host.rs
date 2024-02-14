/// Extracts the host string from the prefix of `s`. Returns the host string and the rest of `s`.
/// This function does not validate that the host string is a valid host.
pub fn extract_host(s: &str) -> (&str, &str) {
    if let Some(slash) = s.as_bytes().iter().position(|c| *c == b'/') {
        s.split_at(slash)
    } else {
        (s, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::extract_host;

    #[test]
    fn fn_extract_host() {
        let test_cases: &[(&str, (&str, &str))] = &[
            ("", ("", "")),
            ("host", ("host", "")),
            ("host/", ("host", "/")),
            ("host/rest", ("host", "/rest")),
        ];
        for (s, expected) in test_cases {
            let result: (&str, &str) = extract_host(*s);
            assert_eq!(result, *expected, "s={}", *s);
        }
    }
}
