use crate::Error;
use crate::Error::InvalidScheme;
use std::fmt::{Debug, Display, Formatter};

/// A web-based URL scheme.
///
/// - The `scheme` cannot be empty.
/// - The `scheme` will be lowercase.
///
/// # RFC 3986
/// <https://www.rfc-editor.org/rfc/rfc3986#section-3.1>
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Scheme<'a> {
    scheme: &'a str,
}

impl Scheme<'static> {
    //! Constants

    /// The `http` scheme.
    pub const HTTP: Self = Self { scheme: "http" };

    /// The `https` scheme.
    pub const HTTPS: Self = Self { scheme: "https" };
}

impl<'a> Scheme<'a> {
    //! Validation

    /// Checks if the `scheme` is valid.
    #[must_use]
    pub const fn is_valid(scheme: &str) -> bool {
        Self::is_valid_optionally_ignore_case(scheme, false)
    }

    /// Checks if the `scheme` is valid, ignoring case.
    #[must_use]
    pub const fn is_valid_ignore_case(scheme: &str) -> bool {
        Self::is_valid_optionally_ignore_case(scheme, true)
    }

    /// Checks if the `scheme` is valid, optionally ignoring case.
    const fn is_valid_optionally_ignore_case(scheme: &str, ignore_case: bool) -> bool {
        let bytes: &[u8] = scheme.as_bytes();
        if bytes.is_empty() || !Self::is_letter(bytes[0], ignore_case) {
            return false;
        }
        let mut index: usize = 1;
        while index < bytes.len() {
            if !Self::is_valid_char(bytes[index], ignore_case) {
                return false;
            }
            index += 1;
        }
        true
    }

    /// Checks if the char `c` is a letter, optionally ignoring case.
    const fn is_letter(c: u8, ignore_case: bool) -> bool {
        c.is_ascii_lowercase() || (ignore_case && c.is_ascii_uppercase())
    }

    /// Checks if the char `c` is valid after the first char, optionally ignoring case.
    const fn is_valid_char(c: u8, ignore_case: bool) -> bool {
        Self::is_letter(c, ignore_case) || c.is_ascii_digit() || matches!(c, b'+' | b'-' | b'.')
    }
}

impl<'a> Scheme<'a> {
    //! Construction

    /// Creates a new scheme.
    pub const fn new(scheme: &'a str) -> Result<Self, Error> {
        if Self::is_valid(scheme) {
            Ok(Self { scheme })
        } else {
            Err(InvalidScheme)
        }
    }

    /// Creates a new scheme.
    ///
    /// # Safety
    /// The `scheme` must be valid.
    pub const unsafe fn new_unchecked(scheme: &'a str) -> Self {
        debug_assert!(Self::is_valid(scheme));

        Self { scheme }
    }
}

impl<'a> TryFrom<&'a str> for Scheme<'a> {
    type Error = Error;

    fn try_from(scheme: &'a str) -> Result<Self, Self::Error> {
        Self::new(scheme)
    }
}

impl<'a> Scheme<'a> {
    //! Properties

    /// Gets the scheme string.
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.scheme
    }
}

impl<'a> Debug for Scheme<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.scheme, f)
    }
}

impl<'a> Display for Scheme<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(self.scheme)
    }
}

#[cfg(test)]
mod tests {
    use crate::Scheme;

    #[test]
    fn is_valid() {
        // (scheme, expected, expected_ignore_case)
        let test_cases: &[(&str, bool, bool)] = &[
            ("", false, false),
            ("a", true, true),
            ("A", false, true),
            ("0", false, false),
            ("+", false, false),
            ("http", true, true),
            ("HTTP", false, true),
            ("Http", false, true),
            ("az09+-.", true, true),
            ("azAZ09+-.", false, true),
            // The '~', '_', ' ', '%', & non-ASCII chars are not scheme chars.
            ("a~", false, false),
            ("a_b", false, false),
            ("a b", false, false),
            ("a%20", false, false),
            ("a\u{4f60}", false, false),
            // The first char must be a letter, so the chars the rest accepts cannot lead.
            ("0abc", false, false),
            (".abc", false, false),
            ("-abc", false, false),
            // The ':' char ends the scheme & is not a scheme char.
            ("http:", false, false),
            ("http://example.com", false, false),
        ];

        for (scheme, expected, expected_ignore_case) in test_cases {
            let result: bool = Scheme::is_valid(scheme);
            assert_eq!(result, *expected, "scheme={}", scheme);

            let result: bool = Scheme::is_valid_ignore_case(scheme);
            assert_eq!(result, *expected_ignore_case, "scheme={}", scheme);
        }
    }
}
