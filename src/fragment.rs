use std::fmt::{Display, Formatter};

use crate::parse::Error;
use crate::parse::Error::*;

/// A web-based URL fragment.
///
/// # Validation
/// A fragment will never be empty and will always start with a '#'.
///
/// The fragment string can contain any US-ASCII letter, number, or punctuation char.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Fragment<'a> {
    fragment: &'a str,
}

impl Default for Fragment<'static> {
    fn default() -> Self {
        Self { fragment: "#" }
    }
}

impl<'a> Fragment<'a> {
    //! Construction

    /// Creates a new fragment.
    ///
    /// # Safety
    /// The `fragment` must be valid.
    pub unsafe fn new(fragment: &'a str) -> Self {
        debug_assert!(Self::is_valid(fragment));

        Self { fragment }
    }
}

impl<'a> TryFrom<&'a str> for Fragment<'a> {
    type Error = Error;

    fn try_from(fragment: &'a str) -> Result<Self, Self::Error> {
        if Self::is_valid(fragment) {
            Ok(Self { fragment })
        } else {
            Err(InvalidFragment)
        }
    }
}

impl<'a> Fragment<'a> {
    //! Validation

    /// Checks if the char `c` is valid.
    fn is_valid_char(c: u8) -> bool {
        c.is_ascii_alphanumeric() || (c.is_ascii_punctuation())
    }

    /// Checks if the `fragment` is valid.
    pub fn is_valid(fragment: &str) -> bool {
        !fragment.is_empty()
            && fragment.as_bytes()[0] == b'#'
            && fragment.as_bytes()[1..]
                .iter()
                .all(|c| Self::is_valid_char(*c))
    }
}

impl<'a> Fragment<'a> {
    //! Display

    /// Gets the fragment. (will not contain the '#' prefix)
    pub fn fragment(&self) -> &str {
        &self.fragment[1..]
    }

    /// Gets the fragment string. (will contain the '#' prefix)
    pub const fn as_str(&self) -> &str {
        self.fragment
    }
}

impl<'a> AsRef<str> for Fragment<'a> {
    fn as_ref(&self) -> &str {
        self.fragment
    }
}

impl<'a> Display for Fragment<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fragment)
    }
}

#[cfg(test)]
mod tests {
    use crate::Fragment;

    #[test]
    fn new() {
        let fragment: Fragment = unsafe { Fragment::new("#the-fragment") };
        assert_eq!(fragment.fragment, "#the-fragment");
    }

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", false),
            ("#", true),
            ("###", true),
            ("#azAZ09", true),
            ("#!/&/=/~/", true),
            ("#?", true),
            ("#!", true),
            ("# ", false),
            ("# x", false),
        ];
        for (fragment, expected) in test_cases {
            let result: bool = Fragment::is_valid(fragment);
            assert_eq!(result, *expected, "fragment={}", fragment);
        }
    }

    #[test]
    fn display() {
        let fragment: Fragment = unsafe { Fragment::new("#the-fragment") };
        assert_eq!(fragment.as_str(), "#the-fragment");
        assert_eq!(fragment.as_ref(), "#the-fragment");
        assert_eq!(fragment.to_string(), "#the-fragment");
    }
}
