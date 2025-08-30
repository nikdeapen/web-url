use std::fmt::{Display, Formatter};

use crate::parse::Error;
use crate::parse::Error::*;

/// A web-based URL path.
///
/// # Validation
/// A path will never be empty and will always start with a '/'.
///
/// The path string can contain any US-ASCII letter, number, or punctuation char excluding '?', and
/// '#' since these chars denote the end of the path in the URL.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Path<'a> {
    path: &'a str,
}

impl Default for Path<'static> {
    fn default() -> Self {
        Self { path: "/" }
    }
}

impl<'a> Path<'a> {
    //! Construction

    /// Creates a new path.
    ///
    /// # Safety
    /// The `path` must be valid.
    pub unsafe fn new(path: &'a str) -> Self {
        debug_assert!(Self::is_valid(path));

        Self { path }
    }
}

impl<'a> TryFrom<&'a str> for Path<'a> {
    type Error = Error;

    fn try_from(path: &'a str) -> Result<Self, Self::Error> {
        if Self::is_valid(path) {
            Ok(Self { path })
        } else {
            Err(InvalidPath)
        }
    }
}

impl<'a> Path<'a> {
    //! Validation

    /// Checks if the char `c` is valid.
    fn is_valid_char(c: u8) -> bool {
        c.is_ascii_alphanumeric() || (c.is_ascii_punctuation() && c != b'?' && c != b'#')
    }

    /// Checks if the `path` is valid.
    pub fn is_valid(path: &str) -> bool {
        !path.is_empty()
            && path.as_bytes()[0] == b'/'
            && path.as_bytes()[1..].iter().all(|c| Self::is_valid_char(*c))
    }
}

impl<'a> Path<'a> {
    //! Display

    /// Gets the path string.
    pub const fn as_str(&self) -> &str {
        self.path
    }
}

impl<'a> AsRef<str> for Path<'a> {
    fn as_ref(&self) -> &str {
        self.path
    }
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl<'a> Path<'a> {
    //! Segments

    /// Creates a new iterator for the path segments.
    ///
    /// # Example
    /// `"/a/b/c/"` -> `["a", "b", "c", ""]`
    pub const fn iter_segments(&self) -> impl Iterator<Item = &'a str> {
        SegmentIterator {
            remaining: self.path,
        }
    }
}

struct SegmentIterator<'a> {
    remaining: &'a str,
}

impl<'a> Iterator for SegmentIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            None
        } else {
            self.remaining = &self.remaining[1..];
            if let Some(slash) = self.remaining.as_bytes().iter().position(|c| *c == b'/') {
                let segment: &str = &self.remaining[..slash];
                self.remaining = &self.remaining[slash..];
                Some(segment)
            } else {
                let segment: &str = self.remaining;
                self.remaining = "";
                Some(segment)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Path;

    #[test]
    fn new() {
        let path: Path = unsafe { Path::new("/the/path") };
        assert_eq!(path.path, "/the/path");
    }

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", false),
            ("/", true),
            ("///", true),
            ("/azAZ09", true),
            ("/!/&/=/~/", true),
            ("/?", false),
            ("/#", false),
        ];
        for (path, expected) in test_cases {
            let result: bool = Path::is_valid(path);
            assert_eq!(result, *expected, "path={}", path);
        }
    }

    #[test]
    fn display() {
        let path: Path = unsafe { Path::new("/the/path") };
        assert_eq!(path.as_str(), "/the/path");
        assert_eq!(path.as_ref(), "/the/path");
        assert_eq!(path.to_string(), "/the/path");
    }

    #[test]
    fn iter_segments() {
        let path: Path = unsafe { Path::new("/") };
        let result: Vec<&str> = path.iter_segments().collect();
        let expected: Vec<&str> = vec![""];
        assert_eq!(result, expected);

        let path: Path = unsafe { Path::new("/the/path") };
        let result: Vec<&str> = path.iter_segments().collect();
        let expected: Vec<&str> = vec!["the", "path"];
        assert_eq!(result, expected);

        let path: Path = unsafe { Path::new("/the/path/") };
        let result: Vec<&str> = path.iter_segments().collect();
        let expected: Vec<&str> = vec!["the", "path", ""];
        assert_eq!(result, expected)
    }
}
