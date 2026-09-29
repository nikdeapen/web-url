use crate::{Path, PieceIterator};
use std::iter::FusedIterator;

impl<'a> Path<'a> {
    //! Segments

    /// Gets the segments.
    pub const fn segments(self) -> PathSegments<'a> {
        PathSegments {
            pieces: PieceIterator::new(self.value(), b'/'),
        }
    }
}

impl<'a> IntoIterator for Path<'a> {
    type Item = &'a str;
    type IntoIter = PathSegments<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.segments()
    }
}

/// An iterator over the segments of a path.
#[must_use]
#[derive(Clone, Debug)]
pub struct PathSegments<'a> {
    pieces: PieceIterator<'a>,
}

impl<'a> Iterator for PathSegments<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.pieces.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.pieces.size_hint()
    }
}

impl<'a> DoubleEndedIterator for PathSegments<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pieces.next_back()
    }
}

impl<'a> FusedIterator for PathSegments<'a> {}

#[cfg(test)]
mod tests {
    use crate::Path;

    #[test]
    fn segments() {
        let test_cases: &[(&str, &[&str])] = &[
            ("/", &[""]),
            ("//", &["", ""]),
            ("/a", &["a"]),
            ("/the/path", &["the", "path"]),
            ("/the/path/", &["the", "path", ""]),
        ];

        for (path, expected) in test_cases {
            let path: Path = Path::new(path).unwrap();
            let result: Vec<&str> = path.segments().collect();
            assert_eq!(result.as_slice(), *expected, "path={}", path);
        }
    }
}
