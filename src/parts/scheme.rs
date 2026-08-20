use crate::Error;
use crate::Error::InvalidScheme;
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};

/// A web-based URL scheme.
///
/// # RFC 3986
/// The scheme is restricted to the canonical lowercase form the RFC recommends; [`Self::is_valid_ignore_case`]
/// accepts the mixed-case forms it also allows. The scheme string does not include the ':' delimiter.
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

    /// Checks if the `scheme` is valid, optionally ignoring case.
    const fn is_valid_op_ignore_case(scheme: &str, ignore_case: bool) -> bool {
        let bytes: &[u8] = scheme.as_bytes();

        // The first char must be a letter.
        if bytes.is_empty() || !(bytes[0].is_ascii_lowercase() || (ignore_case && bytes[0].is_ascii_uppercase())) {
            return false;
        }

        // The rest may also be digits & the '+', '-', & '.' chars.
        let mut index: usize = 1;
        while index < bytes.len() {
            let c: u8 = bytes[index];
            let alpha: bool = c.is_ascii_lowercase() || (ignore_case && c.is_ascii_uppercase());
            if !alpha && !c.is_ascii_digit() && c != b'+' && c != b'-' && c != b'.' {
                return false;
            }
            index += 1;
        }
        true
    }

    /// Checks if the `scheme` is valid.
    ///
    /// A scheme can never be empty & must start with a lowercase letter, followed by any number of lowercase letters,
    /// digits, '+', '-', & '.' chars. The valid scheme format is defined by
    /// [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986#section-3.1).
    #[must_use]
    pub const fn is_valid(scheme: &str) -> bool {
        Self::is_valid_op_ignore_case(scheme, false)
    }

    /// Checks if the `scheme` is valid, ignoring case.
    ///
    /// See [`Self::is_valid`].
    #[must_use]
    pub const fn is_valid_ignore_case(scheme: &str) -> bool {
        Self::is_valid_op_ignore_case(scheme, true)
    }
}

impl<'a> Scheme<'a> {
    //! Construction

    /// Creates a new scheme.
    ///
    /// The `scheme` must be valid.
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

impl<'a> PartialEq<str> for Scheme<'a> {
    fn eq(&self, other: &str) -> bool {
        self.scheme == other
    }
}

impl<'a> PartialEq<Scheme<'a>> for str {
    fn eq(&self, other: &Scheme<'a>) -> bool {
        self == other.scheme
    }
}

impl<'a> PartialEq<&str> for Scheme<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.scheme == *other
    }
}

impl<'a> PartialEq<Scheme<'a>> for &str {
    fn eq(&self, other: &Scheme<'a>) -> bool {
        *self == other.scheme
    }
}

impl<'a> PartialEq<String> for Scheme<'a> {
    fn eq(&self, other: &String) -> bool {
        self.scheme == other.as_str()
    }
}

impl<'a> PartialEq<Scheme<'a>> for String {
    fn eq(&self, other: &Scheme<'a>) -> bool {
        self.as_str() == other.scheme
    }
}

impl<'a> AsRef<str> for Scheme<'a> {
    fn as_ref(&self) -> &str {
        self.scheme
    }
}

impl<'a> Borrow<str> for Scheme<'a> {
    fn borrow(&self) -> &str {
        self.scheme
    }
}

impl<'a> Debug for Scheme<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
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

    /// The constants bypass validation, so a typo in one would not be caught anywhere else.
    #[test]
    fn constants() {
        assert_eq!(Scheme::HTTP, "http");
        assert_eq!(Scheme::HTTPS, "https");
    }

    #[test]
    fn is_valid() {
        // The third column is `is_valid_ignore_case`, which accepts the mixed-case forms `is_valid` rejects.
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
