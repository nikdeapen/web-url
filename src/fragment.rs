use crate::parse;
use crate::Error;
use crate::Error::InvalidFragment;
use std::fmt::{Debug, Display, Formatter};

/// A web-based URL fragment.
///
/// # RFC 3986
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.5>
///
/// # Validation
/// A fragment string will never be empty and will always start with a '#' even if the fragment itself is empty. The
/// fragment value can contain the RFC 3986 fragment chars: the US-ASCII letters and numbers, the unreserved chars
/// "-._~", the sub-delim chars "!$&'()*+,;=", and the ":@/?" chars. The '%' char is also accepted but the
/// percent-encoding is not validated. Fragments are case-sensitive.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Fragment<'a> {
    fragment: &'a str,
}

impl<'a> Fragment<'a> {
    //! Construction

    /// Creates a new fragment.
    pub fn new(fragment: &'a str) -> Result<Self, Error> {
        if Self::is_valid(fragment) {
            Ok(Self { fragment })
        } else {
            Err(InvalidFragment)
        }
    }

    /// Creates a new fragment.
    ///
    /// # Safety
    /// The `fragment` must be valid.
    pub unsafe fn new_unchecked(fragment: &'a str) -> Self {
        debug_assert!(Self::is_valid(fragment));

        Self { fragment }
    }
}

impl<'a> Default for Fragment<'a> {
    fn default() -> Self {
        Self { fragment: "#" }
    }
}

impl<'a> TryFrom<&'a str> for Fragment<'a> {
    type Error = Error;

    fn try_from(fragment: &'a str) -> Result<Self, Self::Error> {
        Self::new(fragment)
    }
}

impl<'a> Fragment<'a> {
    //! Validation

    /// Checks if the `fragment` is valid.
    #[must_use]
    pub fn is_valid(fragment: &str) -> bool {
        parse::is_valid_segment(fragment, b'#', "")
    }
}

impl<'a> Fragment<'a> {
    //! Properties

    /// Gets the fragment value. (will not contain the '#' prefix)
    #[must_use]
    pub fn value(self) -> &'a str {
        &self.fragment[1..]
    }

    /// Gets the fragment string. (will contain the '#' prefix)
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.fragment
    }
}

impl<'a> AsRef<str> for Fragment<'a> {
    fn as_ref(&self) -> &str {
        self.fragment
    }
}

impl<'a> Debug for Fragment<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for Fragment<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fragment)
    }
}

#[cfg(test)]
mod tests {
    use crate::Error::InvalidFragment;
    use crate::Fragment;

    #[test]
    fn new() {
        let fragment: Fragment = Fragment::new("#the-fragment").unwrap();
        assert_eq!(fragment.fragment, "#the-fragment");

        assert_eq!(Fragment::new("no-hash"), Err(InvalidFragment));
    }

    #[test]
    fn default() {
        let fragment: Fragment = Fragment::default();
        assert_eq!(fragment.as_str(), "#");
        assert_eq!(fragment.value(), "");
    }

    #[test]
    fn try_from_str() {
        assert_eq!(Fragment::try_from("#").unwrap().as_str(), "#");
        assert_eq!(Fragment::try_from("#section").unwrap().as_str(), "#section");
        assert_eq!(Fragment::try_from(""), Err(InvalidFragment));
        assert_eq!(Fragment::try_from("no-hash"), Err(InvalidFragment));
        assert_eq!(Fragment::try_from("# space"), Err(InvalidFragment));
    }

    #[test]
    fn value() {
        let fragment: Fragment = Fragment::new("#the-fragment").unwrap();
        assert_eq!(fragment.value(), "the-fragment");

        let fragment: Fragment = Fragment::new("#").unwrap();
        assert_eq!(fragment.value(), "");
    }

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", false),
            ("#", true),
            ("#azAZ09", true),
            ("#!/&/=/~/", true),
            ("#-._~!$&'()*+,;=:@%/?", true),
            ("###", false),
            ("# ", false),
            ("# x", false),
            ("#<>", false),
            ("#[]", false),
            ("#^`{|}\\\"", false),
        ];
        for (fragment, expected) in test_cases {
            let result: bool = Fragment::is_valid(fragment);
            assert_eq!(result, *expected, "fragment={}", fragment);
        }
    }

    #[test]
    fn display() {
        let fragment: Fragment = Fragment::new("#the-fragment").unwrap();
        assert_eq!(fragment.as_str(), "#the-fragment");
        assert_eq!(fragment.as_ref(), "#the-fragment");
        assert_eq!(fragment.to_string(), "#the-fragment");
    }
}
