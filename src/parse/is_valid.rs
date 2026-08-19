/// The set of valid chars, as a 256-bit mask indexed by the char.
///
/// The valid chars are the RFC 3986 `pchar` chars plus '/' & '?', the widest set accepted by a path, query, or
/// fragment. The '%' char is valid but the percent-encoding is not validated.
const VALID: [u64; 4] = {
    // unreserved: "-._~", sub-delims: "!$&'()*+,;=", pchar: "%:@", path & query: "/?"
    const CHARS: &[u8] = b"-._~!$&'()*+,;=%:@/?";

    let mut mask: [u64; 4] = [0; 4];
    let mut c: u8 = 0;
    while c < 128 {
        let mut valid: bool = c.is_ascii_alphanumeric();
        let mut index: usize = 0;
        while index < CHARS.len() {
            valid |= c == CHARS[index];
            index += 1;
        }
        if valid {
            mask[(c >> 6) as usize] |= 1 << (c & 0b11_1111);
        }
        c += 1;
    }
    mask
};

/// Checks if the char `c` is valid. The chars in `exclude` are invalid.
///
/// The valid chars are the RFC 3986 `pchar` chars plus '/' & '?', the widest set accepted by a path, query, or
/// fragment. The '%' char is valid but the percent-encoding is not validated.
pub fn is_valid_char(c: u8, exclude: &str) -> bool {
    // The mask is a table lookup rather than a scan since this runs once per char of every path, query, & fragment.
    let valid: bool = VALID[(c >> 6) as usize] & (1 << (c & 0b11_1111)) != 0;

    valid && !exclude.as_bytes().contains(&c)
}

/// Checks if the `segment` is valid. The `segment` must start with `start` & the chars in `exclude` are invalid.
pub fn is_valid_segment(segment: &str, start: u8, exclude: &str) -> bool {
    !segment.is_empty()
        && segment.as_bytes()[0] == start
        && segment.as_bytes()[1..].iter().all(|c| is_valid_char(*c, exclude))
}

#[cfg(test)]
mod tests {
    use crate::parse::{is_valid_char, is_valid_segment};

    #[test]
    fn fn_is_valid_char() {
        // Every char is checked so the mask cannot drift from the RFC 3986 char set.
        for c in 0..=u8::MAX {
            let expected: bool = c.is_ascii_alphanumeric() || b"-._~!$&'()*+,;=%:@/?".contains(&c);
            assert_eq!(is_valid_char(c, ""), expected, "c={}", c);

            // An excluded char is invalid even when it is in the set.
            assert_eq!(is_valid_char(c, "?"), expected && c != b'?', "c={}", c);
            assert_eq!(is_valid_char(c, "&="), expected && c != b'&' && c != b'=', "c={}", c);
        }
    }

    #[test]
    fn fn_is_valid_segment() {
        let test_cases: &[(&str, u8, &str, bool)] = &[
            ("", b'/', "", false),
            ("/", b'/', "", true),
            ("?", b'/', "", false),
            ("/a/b", b'/', "", true),
            ("/a?b", b'/', "?", false),
            ("?a?b", b'?', "", true),
            ("#a b", b'#', "", false),
            ("#\u{4f60}", b'#', "", false),
        ];
        for (segment, start, exclude, expected) in test_cases {
            let result: bool = is_valid_segment(segment, *start, exclude);
            assert_eq!(result, *expected, "segment={}", segment);
        }
    }
}
