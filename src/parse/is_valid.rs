/// Checks if the char `c` is valid. The chars in `exclude` are invalid.
///
/// The valid chars are the RFC 3986 `pchar` chars plus '/' & '?', the widest set accepted by a path, query, or
/// fragment. The '%' char is valid but the percent-encoding is not validated.
pub fn is_valid_char(c: u8, exclude: &str) -> bool {
    // unreserved: "-._~", sub-delims: "!$&'()*+,;=", pchar: "%:@", path & query: "/?"
    let valid: bool = c.is_ascii_alphanumeric() || b"-._~!$&'()*+,;=%:@/?".contains(&c);

    valid && !exclude.as_bytes().contains(&c)
}

/// Checks if the `segment` is valid. The `segment` must start with `start` and the chars in `exclude` are invalid.
pub fn is_valid_segment(segment: &str, start: u8, exclude: &str) -> bool {
    !segment.is_empty()
        && segment.as_bytes()[0] == start
        && segment.as_bytes()[1..].iter().all(|c| is_valid_char(*c, exclude))
}
