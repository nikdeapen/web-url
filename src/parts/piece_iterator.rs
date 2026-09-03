use std::iter::FusedIterator;

/// Responsible for iterating over the separated pieces of a string.
#[must_use]
#[derive(Clone, Debug)]
pub struct PieceIterator<'a> {
    rest: Option<&'a str>,
    separator: u8,
}

impl<'a> PieceIterator<'a> {
    //! Construction

    /// Creates a new piece iterator.
    ///
    /// The `separator` must be an ASCII char.
    pub(crate) const fn new(s: &'a str, separator: u8) -> Self {
        Self {
            rest: Some(s),
            separator,
        }
    }
}

impl<'a> Iterator for PieceIterator<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let rest: &'a str = self.rest?;
        let (piece, rest) = match rest.as_bytes().iter().position(|&b| b == self.separator) {
            Some(index) => (&rest[..index], Some(&rest[index + 1..])),
            None => (rest, None),
        };
        self.rest = rest;
        Some(piece)
    }
}

impl<'a> FusedIterator for PieceIterator<'a> {}

#[cfg(test)]
mod tests {
    use crate::PieceIterator;

    #[test]
    fn iterate() {
        let test_cases: &[(&str, u8, &[&str])] = &[
            ("", b'/', &[""]),
            ("/", b'/', &["", ""]),
            ("a", b'/', &["a"]),
            ("a/b", b'/', &["a", "b"]),
            ("a/b/", b'/', &["a", "b", ""]),
            ("a&b", b'&', &["a", "b"]),
        ];

        for (s, separator, expected) in test_cases {
            let result: Vec<&str> = PieceIterator::new(s, *separator).collect();
            assert_eq!(result.as_slice(), *expected, "s={}", s);
        }
    }
}
