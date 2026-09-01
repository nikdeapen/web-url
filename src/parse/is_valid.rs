/// The set of valid chars, as a 256-bit mask indexed by the char.
///
/// The valid chars are the RFC 3986 `pchar` chars plus '/' & '?', the widest set accepted by a path, query, or
/// fragment. The '%' char is in the set since it begins a percent-encoded octet.
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

/// Checks if the char `c` is valid on its own. The chars in `exclude` are invalid.
///
/// The '%' char is valid here since it begins a percent-encoded octet; the two hex digits that must follow it are
/// checked by [`is_valid_chars`].
pub const fn is_valid_char(c: u8, exclude: &str) -> bool {
    // The mask is a table lookup rather than a scan since this runs once per char of every path, query, & fragment.
    if VALID[(c >> 6) as usize] & (1 << (c & 0b11_1111)) == 0 {
        return false;
    }

    // The `exclude` is scanned by hand since `<[u8]>::contains` is not const. It never holds more than two chars.
    let exclude: &[u8] = exclude.as_bytes();
    let mut index: usize = 0;
    while index < exclude.len() {
        if exclude[index] == c {
            return false;
        }
        index += 1;
    }
    true
}

/// Checks if the `chars` are valid. The chars in `exclude` are invalid.
///
/// A percent-encoded octet is consumed whole, so an excluded char is only excluded literally: excluding '=' rejects
/// the '=' char but not its `%3D` escape.
///
/// # RFC 3986
/// A '%' char begins a percent-encoded octet & must be followed by exactly two hex digits.
/// <https://www.rfc-editor.org/rfc/rfc3986#section-2.1>
pub const fn is_valid_chars(chars: &[u8], exclude: &str) -> bool {
    let mut index: usize = 0;
    while index < chars.len() {
        let c: u8 = chars[index];
        if !is_valid_char(c, exclude) {
            return false;
        } else if c == b'%' {
            // The octet is the '%' & the two hex digits, which are consumed together.
            if index + 2 >= chars.len()
                || !chars[index + 1].is_ascii_hexdigit()
                || !chars[index + 2].is_ascii_hexdigit()
            {
                return false;
            }
            index += 3;
        } else {
            index += 1;
        }
    }
    true
}

/// Checks if the `segment` is valid. The `segment` must start with `start` & the chars in `exclude` are invalid.
pub const fn is_valid_segment(segment: &str, start: u8, exclude: &str) -> bool {
    let bytes: &[u8] = segment.as_bytes();
    if bytes.is_empty() || bytes[0] != start {
        return false;
    }

    // The prefix is split off by hand since range indexing is not const.
    let (_, chars) = bytes.split_at(1);
    is_valid_chars(chars, exclude)
}

#[cfg(test)]
mod tests {
    use crate::parse::{is_valid_char, is_valid_chars, is_valid_segment};

    #[test]
    fn fn_is_valid_char() {
        // Every char is checked so the mask cannot drift from the RFC 3986 char set.
        for c in 0..=u8::MAX {
            let expected: bool = c.is_ascii_alphanumeric() || b"-._~!$&'()*+,;=%:@/?".contains(&c);
            assert_eq!(is_valid_char(c, ""), expected, "c={}", c);

            // An excluded char is invalid even when it is in the set.
            assert_eq!(is_valid_char(c, "?"), expected && c != b'?', "c={}", c);
            assert_eq!(
                is_valid_char(c, "&="),
                expected && c != b'&' && c != b'=',
                "c={}",
                c
            );
        }
    }

    #[test]
    fn fn_is_valid_chars() {
        let test_cases: &[(&str, bool)] = &[
            ("", true),
            ("azAZ09", true),
            ("-._~!$&'()*+,;=:@/?", true),
            (" ", false),
            ("<>", false),
            // A '%' char must be followed by exactly two hex digits.
            ("%20", true),
            ("%2e", true),
            ("%2E", true),
            ("%aF", true),
            ("%00", true),
            ("%ff", true),
            ("a%20b%21c", true),
            ("%20%21", true),
            ("%", false),
            ("%2", false),
            ("%z", false),
            ("%zz", false),
            ("%2z", false),
            ("%z2", false),
            ("%GG", false),
            ("a%", false),
            ("%20%", false),
            ("% 20", false),
            ("%%20", false),
            ("%2%20", false),
        ];
        for (chars, expected) in test_cases {
            let result: bool = is_valid_chars(chars.as_bytes(), "");
            assert_eq!(result, *expected, "chars={}", chars);
        }
    }

    /// The escape is consumed whole, so excluding a char must not reject its percent-encoded form.
    #[test]
    fn fn_is_valid_chars_exclude() {
        assert!(!is_valid_chars(b"=", "&="));
        assert!(!is_valid_chars(b"&", "&="));
        assert!(is_valid_chars(b"%3D", "&="));
        assert!(is_valid_chars(b"%26", "&="));
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
            // The prefix char is not part of the chars, so an escape cannot start with it.
            ("/%2f", b'/', "?", true),
            ("/a%", b'/', "", false),
            ("#%zz", b'#', "", false),
        ];
        for (segment, start, exclude, expected) in test_cases {
            let result: bool = is_valid_segment(segment, *start, exclude);
            assert_eq!(result, *expected, "segment={}", segment);
        }
    }
}
