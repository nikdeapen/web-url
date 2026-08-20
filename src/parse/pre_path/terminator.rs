/// Checks if the char `c` terminates the authority. (the host & the optional port)
///
/// # RFC 3986
/// The authority is terminated by the next '/', '?', or '#' char, or by the end of the URL.
/// <https://www.rfc-editor.org/rfc/rfc3986#section-3.2>
pub fn is_authority_end(c: u8) -> bool {
    c == b'/' || c == b'?' || c == b'#'
}

#[cfg(test)]
mod tests {
    use crate::parse::is_authority_end;

    #[test]
    fn fn_is_authority_end() {
        let test_cases: &[(u8, bool)] = &[
            (b'/', true),
            (b'?', true),
            (b'#', true),
            (b':', false),
            (b'a', false),
            (b'0', false),
            (b' ', false),
            (b'[', false),
            (b']', false),
        ];
        for (c, expected) in test_cases {
            let result: bool = is_authority_end(*c);
            assert_eq!(result, *expected, "c={}", *c as char);
        }
    }
}
