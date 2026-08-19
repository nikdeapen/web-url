use std::iter::FusedIterator;

/// Responsible for iterating over the separated pieces of a string.
///
/// Every piece consumes its leading separator, so the string must be empty or start with a separator-like lead char.
/// The lead char is consumed blindly, so it need not match the `separator`, as in `"?a&b"` -> `["a", "b"]`. An empty
/// region between separators is an empty piece & the empty string has no pieces.
#[must_use]
#[derive(Copy, Clone, Debug)]
pub(crate) struct PieceIterator<'a> {
    remaining: &'a str,
    separator: u8,
}

impl<'a> PieceIterator<'a> {
    //! Construction

    /// Creates a new piece iterator.
    pub(crate) const fn new(s: &'a str, separator: u8) -> Self {
        Self {
            remaining: s,
            separator,
        }
    }
}

impl<'a> Iterator for PieceIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            None
        } else {
            self.remaining = &self.remaining[1..];
            if let Some(index) = self.remaining.as_bytes().iter().position(|c| *c == self.separator) {
                let piece: &str = &self.remaining[..index];
                self.remaining = &self.remaining[index..];
                Some(piece)
            } else {
                let piece: &str = self.remaining;
                self.remaining = "";
                Some(piece)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.remaining.is_empty() {
            (0, Some(0))
        } else {
            // There is at least the final piece & at most one piece per byte, since every piece consumes its leading
            // separator.
            (1, Some(self.remaining.len()))
        }
    }
}

impl<'a> FusedIterator for PieceIterator<'a> {}

#[cfg(test)]
mod tests {
    use crate::parse::PieceIterator;

    #[test]
    fn iterate() {
        let test_cases: &[(&str, u8, &[&str])] = &[
            ("", b'/', &[]),
            ("/", b'/', &[""]),
            ("//", b'/', &["", ""]),
            ("/a", b'/', &["a"]),
            ("/a/b", b'/', &["a", "b"]),
            ("/a/b/", b'/', &["a", "b", ""]),
            // The lead char is consumed blindly, so it need not match the separator.
            ("?", b'&', &[""]),
            ("?&", b'&', &["", ""]),
            ("?a&b", b'&', &["a", "b"]),
        ];
        for (s, separator, expected) in test_cases {
            let result: Vec<&str> = PieceIterator::new(s, *separator).collect();
            assert_eq!(result.as_slice(), *expected, "s={}", s);

            // The hint must bound the count; it sizes the collections built from the iterator.
            let (min, max) = PieceIterator::new(s, *separator).size_hint();
            assert!(min <= result.len(), "s={}", s);
            assert!(max.unwrap() >= result.len(), "s={}", s);
        }
    }
}
