/// Checks if the char `c` is valid. The chars in `exclude` are invalid.
pub fn is_valid_char(c: u8, exclude: &str) -> bool {
    (c.is_ascii_alphanumeric() || c.is_ascii_punctuation()) && !exclude.as_bytes().contains(&c)
}

/// Checks if the `segment` is valid. The `segment` must start with `start` and the chars in `exclude` are invalid.
pub fn is_valid_segment(segment: &str, start: u8, exclude: &str) -> bool {
    !segment.is_empty()
        && segment.as_bytes()[0] == start
        && segment.as_bytes()[1..].iter().all(|c| is_valid_char(*c, exclude))
}
