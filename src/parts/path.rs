use crate::Error;
use crate::Error::InvalidPath;
use crate::PieceIterator;
use crate::parse;
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};

/// A web-based URL path.
///
/// # RFC 3986
/// The path can never be empty, unlike the RFC `path-abempty` production, since a URL with no
/// explicit path gets an implied '/' path. A path may hold dot-segments;
/// [`WebUrl::set_path`](crate::WebUrl::set_path) removes them.
/// <https://www.rfc-editor.org/rfc/rfc3986#section-3.3>
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Path<'a> {
    path: &'a str,
}

impl<'a> Path<'a> {
    //! Validation

    /// Checks if the `path` is valid.
    ///
    /// A path string can never be empty & must start with a '/'. The valid path format is defined
    /// by [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986#section-3.3).
    #[must_use]
    pub const fn is_valid(path: &str) -> bool {
        parse::is_valid_segment(path, b'/', "?")
    }
}

impl<'a> Path<'a> {
    //! Construction

    /// Creates a new path.
    pub const fn new(path: &'a str) -> Result<Self, Error> {
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
    pub const unsafe fn new_unchecked(path: &'a str) -> Self {
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
    //! Properties

    /// Gets the path string. (will contain the '/' prefix)
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.path
    }
}

impl<'a> Path<'a> {
    //! Segments

    /// Creates a new iterator for the path segments.
    ///
    /// The '/' chars are separators, so the regions between them are the segments & an empty region
    /// is an empty segment. The path `"/"` is a single empty segment & the path `"//"` is two of
    /// them. Only a literal '/' char separates, so a `%2F` escape stays within its segment.
    ///
    /// # Example
    /// `"/a/b/c/"` -> `["a", "b", "c", ""]`
    pub const fn iter_segments(self) -> PieceIterator<'a> {
        PieceIterator::new(self.path, b'/')
    }
}

impl<'a> IntoIterator for Path<'a> {
    type Item = &'a str;
    type IntoIter = PieceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_segments()
    }
}

impl<'a> PartialEq<str> for Path<'a> {
    fn eq(&self, other: &str) -> bool {
        self.path == other
    }
}

impl<'a> PartialEq<Path<'a>> for str {
    fn eq(&self, other: &Path<'a>) -> bool {
        self == other.path
    }
}

impl<'a> PartialEq<&str> for Path<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.path == *other
    }
}

impl<'a> PartialEq<Path<'a>> for &str {
    fn eq(&self, other: &Path<'a>) -> bool {
        *self == other.path
    }
}

impl<'a> PartialEq<String> for Path<'a> {
    fn eq(&self, other: &String) -> bool {
        self.path == other.as_str()
    }
}

impl<'a> PartialEq<Path<'a>> for String {
    fn eq(&self, other: &Path<'a>) -> bool {
        self.as_str() == other.path
    }
}

impl<'a> AsRef<str> for Path<'a> {
    fn as_ref(&self) -> &str {
        self.path
    }
}

impl<'a> Borrow<str> for Path<'a> {
    fn borrow(&self) -> &str {
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

#[cfg(test)]
mod tests {
    use crate::Path;

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", false),
            ("/", true),
            ("///", true),
            ("/azAZ09", true),
            ("/!/&/=/~/", true),
            ("/-._~!$&'()*+,;=:@", true),
            // A '%' char begins a percent-encoded octet & must be followed by two hex digits.
            ("/%20", true),
            ("/a%2Fb%2fc", true),
            ("/%00", true),
            ("/%", false),
            ("/%2", false),
            ("/%zz", false),
            ("/a%2gb", false),
            // The '?' char ends the path & the '#' char is not a path char.
            ("/?", false),
            ("/#", false),
            ("no-slash", false),
            ("/ ", false),
            ("/<>", false),
            ("/[]", false),
            ("/^`{|}\\\"", false),
            ("/\u{0}", false),
            ("/\u{4f60}", false),
        ];

        for (path, expected) in test_cases {
            let result: bool = Path::is_valid(path);
            assert_eq!(result, *expected, "path={}", path);
        }
    }

    #[test]
    fn iter_segments() {
        // The '/' chars are separators, so an empty region between them is an empty segment.
        let test_cases: &[(&str, &[&str])] = &[
            ("/", &[""]),
            ("//", &["", ""]),
            ("/a", &["a"]),
            ("/the/path", &["the", "path"]),
            ("/the/path/", &["the", "path", ""]),
        ];

        for (path, expected) in test_cases {
            let path: Path = Path::new(path).unwrap();
            let result: Vec<&str> = path.iter_segments().collect();
            assert_eq!(result.as_slice(), *expected, "path={}", path);
        }
    }
}
