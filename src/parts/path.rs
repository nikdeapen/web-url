use crate::Error;
use crate::Error::InvalidPath;
use crate::PieceIterator;
use crate::parse;
use std::fmt::{Debug, Display, Formatter};

/// A web-based URL path.
///
/// - The `path` string will never be empty and always start with a '/'.
/// - The `path` value (after the '/') may be empty.
///
/// # RFC 3986
/// <https://www.rfc-editor.org/rfc/rfc3986#section-3.3>
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Path<'a> {
    path: &'a str,
}

impl<'a> Path<'a> {
    //! Validation

    /// Checks if the `path` is valid.
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

    /// Gets the path value. (will not contain the '/' prefix)
    #[must_use]
    pub const fn value(self) -> &'a str {
        self.path.split_at(1).1
    }
}

impl<'a> Path<'a> {
    //! Segments

    /// Creates a new iterator for the path segments.
    pub const fn iter_segments(self) -> PieceIterator<'a> {
        PieceIterator::new(self.value(), b'/')
    }
}

impl<'a> IntoIterator for Path<'a> {
    type Item = &'a str;
    type IntoIter = PieceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_segments()
    }
}

impl<'a> Debug for Path<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.path, f)
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
