use std::fmt::{Display, Formatter};

/// A web-based URL scheme.
///
/// # RFC 3986
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.1
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Scheme<'a> {
    scheme: &'a str,
}

impl<'a> Scheme<'a> {
    //! Construction

    /// Creates a new scheme.
    ///
    /// # Safety
    /// The `scheme` must be valid.
    pub unsafe fn new(scheme: &'a str) -> Self {
        debug_assert!(Self::is_valid(scheme, false));

        Self { scheme }
    }
}

impl<'a> Scheme<'a> {
    //! Validation

    /// Checks if the char `c` is a valid first char.
    fn is_valid_first_char(c: u8, ignore_case: bool) -> bool {
        c.is_ascii_lowercase() || (ignore_case && c.is_ascii_uppercase())
    }

    /// Checks if the char `c` is a valid scheme char.
    fn is_valid_char(c: u8, ignore_case: bool) -> bool {
        Self::is_valid_first_char(c, ignore_case)
            || c.is_ascii_digit()
            || c == b'+'
            || c == b'-'
            || c == b'.'
    }

    /// Checks if the `scheme` is valid.
    pub fn is_valid(scheme: &'a str, ignore_case: bool) -> bool {
        !scheme.is_empty()
            && Self::is_valid_first_char(scheme.as_bytes()[0], ignore_case)
            && scheme.as_bytes()[1..]
                .iter()
                .all(|c| Self::is_valid_char(*c, ignore_case))
    }
}

impl<'a> Scheme<'a> {
    //! Display

    /// Gets the scheme string.
    pub const fn as_str(&self) -> &str {
        self.scheme
    }
}

impl<'a> AsRef<str> for Scheme<'a> {
    fn as_ref(&self) -> &str {
        self.scheme
    }
}

impl<'a> Display for Scheme<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.scheme)
    }
}

#[cfg(test)]
mod tests {
    use crate::Scheme;

    #[test]
    fn new() {
        let scheme: Scheme = unsafe { Scheme::new("scheme") };
        assert_eq!(scheme.scheme, "scheme");
    }

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool, bool)] = &[
            ("", false, false),
            ("A", true, false),
            ("a", true, true),
            ("0", false, false),
            ("a~", false, false),
            ("az09+-.", true, true),
            ("azAZ09+-.", true, false),
        ];
        for (scheme, expected_ic_true, expected_ic_false) in test_cases {
            let result: bool = Scheme::is_valid(scheme, true);
            assert_eq!(result, *expected_ic_true, "scheme={}", scheme);

            let result: bool = Scheme::is_valid(scheme, false);
            assert_eq!(result, *expected_ic_false, "scheme={}", scheme);
        }
    }

    #[test]
    fn display() {
        let scheme: Scheme = unsafe { Scheme::new("scheme") };
        assert_eq!(scheme.as_str(), "scheme");
        assert_eq!(scheme.as_ref(), "scheme");
        assert_eq!(scheme.to_string(), "scheme");
    }
}
