use crate::Error;
use crate::Error::InvalidPath;
use crate::parse;
use std::fmt::{Debug, Display, Formatter};
use std::iter::FusedIterator;

/// A web-based URL path.
///
/// # RFC 3986
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
///
/// # Validation
/// A path string will never be empty & will always start with a '/'. The path string can contain the RFC 3986 path
/// chars: the US-ASCII letters & numbers, the unreserved chars "-._~", the sub-delim chars "!$&'()*+,;=", & the ":@/"
/// chars. The '%' char is also accepted but the percent-encoding is not validated.
///
/// # Segments
/// The '/' chars are separators, so the regions between them are the segments & an empty region is an empty segment.
/// The path `"/"` is a single empty segment & the path `"//"` is two of them.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Path<'a> {
    path: &'a str,
}

impl<'a> Path<'a> {
    //! Construction

    /// Creates a new path.
    pub fn new(path: &'a str) -> Result<Self, Error> {
        if Self::is_valid(path) {
            Ok(Self { path })
        } else {
            Err(InvalidPath)
        }
    }

    /// Creates a new path.
    ///
    /// # Safety
    /// The `path` must be valid.
    pub unsafe fn new_unchecked(path: &'a str) -> Self {
        debug_assert!(Self::is_valid(path));

        Self { path }
    }
}

impl<'a> Default for Path<'a> {
    fn default() -> Self {
        Self { path: "/" }
    }
}

impl<'a> TryFrom<&'a str> for Path<'a> {
    type Error = Error;

    fn try_from(path: &'a str) -> Result<Self, Self::Error> {
        Self::new(path)
    }
}

impl<'a> Path<'a> {
    //! Validation

    /// Checks if the `path` is valid.
    #[must_use]
    pub fn is_valid(path: &str) -> bool {
        parse::is_valid_segment(path, b'/', "?")
    }
}

impl<'a> Path<'a> {
    //! Properties

    /// Gets the path string. (will contain the '/' prefix)
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.path
    }
}

impl<'a> PartialEq<&str> for Path<'a> {
    /// The comparison is exact; the `other` string is not validated or normalized.
    fn eq(&self, other: &&str) -> bool {
        self.path == *other
    }
}

impl<'a> PartialEq<Path<'a>> for &str {
    /// The comparison is exact; the `self` string is not validated or normalized.
    fn eq(&self, other: &Path<'a>) -> bool {
        *self == other.path
    }
}

impl<'a> AsRef<str> for Path<'a> {
    fn as_ref(&self) -> &str {
        self.path
    }
}

impl<'a> Debug for Path<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(self.path)
    }
}

impl<'a> Path<'a> {
    //! Segments

    /// Creates a new iterator for the path segments.
    ///
    /// # Example
    /// `"/a/b/c/"` -> `["a", "b", "c", ""]`
    pub const fn iter_segments(self) -> SegmentIterator<'a> {
        SegmentIterator {
            pieces: parse::PieceIterator::new(self.path, b'/'),
        }
    }
}

impl<'a> IntoIterator for Path<'a> {
    type Item = &'a str;
    type IntoIter = SegmentIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_segments()
    }
}

/// Responsible for iterating over path segments.
#[must_use]
#[derive(Copy, Clone, Debug)]
pub struct SegmentIterator<'a> {
    pieces: parse::PieceIterator<'a>,
}

impl<'a> Iterator for SegmentIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.pieces.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.pieces.size_hint()
    }
}

impl<'a> FusedIterator for SegmentIterator<'a> {}

#[cfg(test)]
mod tests {
    use crate::Error::InvalidPath;
    use crate::Path;

    #[test]
    fn new() {
        let path: Path = Path::new("/the/path").unwrap();
        assert_eq!(path.path, "/the/path");

        assert_eq!(Path::new("no-slash"), Err(InvalidPath));
    }

    #[test]
    fn default() {
        let path: Path = Path::default();
        assert_eq!(path.as_str(), "/");
    }

    #[test]
    fn try_from_str() {
        assert_eq!(Path::try_from("/").unwrap().as_str(), "/");
        assert_eq!(Path::try_from("/the/path").unwrap().as_str(), "/the/path");
        assert_eq!(Path::try_from(""), Err(InvalidPath));
        assert_eq!(Path::try_from("no-slash"), Err(InvalidPath));
        assert_eq!(Path::try_from("/?"), Err(InvalidPath));
        assert_eq!(Path::try_from("/#"), Err(InvalidPath));
    }

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", false),
            ("/", true),
            ("///", true),
            ("/azAZ09", true),
            ("/!/&/=/~/", true),
            ("/-._~!$&'()*+,;=:@%", true),
            ("/?", false),
            ("/#", false),
            ("/<>", false),
            ("/[]", false),
            ("/^`{|}\\\"", false),
        ];
        for (path, expected) in test_cases {
            let result: bool = Path::is_valid(path);
            assert_eq!(result, *expected, "path={}", path);
        }
    }

    #[test]
    fn display() {
        let path: Path = Path::new("/the/path").unwrap();
        assert_eq!(path.as_str(), "/the/path");
        assert_eq!(path.as_ref(), "/the/path");
        assert_eq!(path.to_string(), "/the/path");
    }

    #[test]
    fn iter_segments() {
        let path: Path = Path::new("/").unwrap();
        let result: Vec<&str> = path.iter_segments().collect();
        let expected: Vec<&str> = vec![""];
        assert_eq!(result, expected);

        let path: Path = Path::new("/the/path").unwrap();
        let result: Vec<&str> = path.iter_segments().collect();
        let expected: Vec<&str> = vec!["the", "path"];
        assert_eq!(result, expected);

        let path: Path = Path::new("/the/path/").unwrap();
        let result: Vec<&str> = path.iter_segments().collect();
        let expected: Vec<&str> = vec!["the", "path", ""];
        assert_eq!(result, expected)
    }

    #[test]
    fn into_iter() {
        let path: Path = Path::new("/the/path").unwrap();
        let mut result: Vec<&str> = Vec::new();
        for segment in path {
            result.push(segment);
        }
        assert_eq!(result, vec!["the", "path"]);
    }
}
