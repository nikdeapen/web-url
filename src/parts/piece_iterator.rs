use std::iter::FusedIterator;

/// An iterator over the separated pieces of a string.
#[must_use]
#[derive(Clone, Debug)]
pub(crate) struct PieceIterator<'a> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.rest {
            Some(rest) => (1, Some(rest.len() + 1)),
            None => (0, Some(0)),
        }
    }
}

impl<'a> DoubleEndedIterator for PieceIterator<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let rest: &'a str = self.rest?;
        let (rest, piece) = match rest.as_bytes().iter().rposition(|&b| b == self.separator) {
            Some(index) => (Some(&rest[..index]), &rest[index + 1..]),
            None => (None, rest),
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

    #[test]
    fn size_hint() {
        let test_cases: &[&str] = &["", "/", "a", "a/b", "a/b/", "//a//"];

        for s in test_cases {
            let total: usize = PieceIterator::new(s, b'/').count();
            let mut pieces: PieceIterator = PieceIterator::new(s, b'/');

            for taken in 0..=total {
                let remaining: usize = total - taken;
                let (low, high): (usize, Option<usize>) = pieces.size_hint();
                assert!(low <= remaining, "s={} taken={} low={}", s, taken, low);
                assert!(
                    high.is_some_and(|high| remaining <= high),
                    "s={} taken={} high={:?}",
                    s,
                    taken,
                    high
                );
                pieces.next();
            }
        }
    }

    #[test]
    fn rev() {
        let test_cases: &[(&str, u8, &[&str])] = &[
            ("", b'/', &[""]),
            ("/", b'/', &["", ""]),
            ("a", b'/', &["a"]),
            ("a/b", b'/', &["b", "a"]),
            ("a/b/", b'/', &["", "b", "a"]),
            ("a&b", b'&', &["b", "a"]),
        ];

        for (s, separator, expected) in test_cases {
            let result: Vec<&str> = PieceIterator::new(s, *separator).rev().collect();
            assert_eq!(result.as_slice(), *expected, "s={}", s);
        }
    }

    #[test]
    fn mixed() {
        let mut pieces: PieceIterator = PieceIterator::new("a/b/c/d/e", b'/');

        assert_eq!(pieces.next(), Some("a"));
        assert_eq!(pieces.next_back(), Some("e"));
        assert_eq!(pieces.next(), Some("b"));
        assert_eq!(pieces.next_back(), Some("d"));
        assert_eq!(pieces.next(), Some("c"));
        assert_eq!(pieces.next(), None);
        assert_eq!(pieces.next_back(), None);
    }

    #[test]
    fn fused() {
        let mut pieces: PieceIterator = PieceIterator::new("a", b'/');

        assert_eq!(pieces.next(), Some("a"));
        assert_eq!(pieces.next(), None);
        assert_eq!(pieces.next(), None);
        assert_eq!(pieces.next_back(), None);
    }
}
