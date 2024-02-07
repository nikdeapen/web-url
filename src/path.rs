/// A web URL path. (will always start with a '/')
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

    /// Creates a new path. This constructor does not validate the path.
    pub const unsafe fn new_unchecked(path: &'a str) -> Self {
        Self { path }
    }
}

impl<'a> Path<'a> {
    //! Validation

    /// Checks if the path character is valid.
    fn is_valid_char(c: u8) -> bool {
        c.is_ascii_alphanumeric() || (c.is_ascii_punctuation() && c != b'?' && c != b'#')
    }

    /// Checks if the path is valid.
    pub fn is_valid(path: &str) -> bool {
        !path.is_empty()
            && path.as_bytes()[0] == b'/'
            && (&path[1..])
                .as_bytes()
                .iter()
                .all(|c| Self::is_valid_char(*c))
    }
}

impl<'a> Path<'a> {
    //! Properties

    /// Gets the path.
    pub const fn path(&self) -> &str {
        self.path
    }
}

impl<'a> Path<'a> {
    //! Segments

    /// Creates a new iterator for the path segments.
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
    fn new_unchecked() {
        let path: Path = unsafe { Path::new_unchecked("/the/path") };
        assert_eq!(path.path, "/the/path");
    }

    #[test]
    fn validation() {
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
            let result: bool = Path::is_valid(*path);
            assert_eq!(result, *expected, "path={}", *path);
        }
    }

    #[test]
    fn properties() {
        let path: Path = unsafe { Path::new_unchecked("/the/path") };
        assert_eq!(path.path(), "/the/path");
    }

    #[test]
    fn iter_segments() {
        let path: Path = unsafe { Path::new_unchecked("/") };
        let result: Vec<&str> = path.iter_segments().collect();
        let expected: Vec<&str> = vec![""];
        assert_eq!(result, expected);

        let path: Path = unsafe { Path::new_unchecked("/the/path") };
        let result: Vec<&str> = path.iter_segments().collect();
        let expected: Vec<&str> = vec!["the", "path"];
        assert_eq!(result, expected);

        let path: Path = unsafe { Path::new_unchecked("/the/path/") };
        let result: Vec<&str> = path.iter_segments().collect();
        let expected: Vec<&str> = vec!["the", "path", ""];
        assert_eq!(result, expected)
    }
}
