use crate::Error;
use crate::Error::InvalidFragment;
use crate::parse;
use std::fmt::{Debug, Display, Formatter};

/// A web-based URL fragment.
///
/// - The `fragment` string will not be empty and will always start with a '#'.
/// - The `fragment` value (after the '#') may be empty.
///
/// # RFC 3986
/// <https://www.rfc-editor.org/rfc/rfc3986#section-3.5>
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Fragment<'a> {
    fragment: &'a str,
}

impl<'a> Fragment<'a> {
    //! Validation

    /// Checks if the `fragment` is valid.
    #[must_use]
    pub const fn is_valid(fragment: &str) -> bool {
        parse::is_valid_segment(fragment, b'#', "")
    }
}

impl<'a> Fragment<'a> {
    //! Construction

    /// Creates a new fragment.
    pub const fn new(fragment: &'a str) -> Result<Self, Error> {
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
    pub const unsafe fn new_unchecked(fragment: &'a str) -> Self {
        debug_assert!(Self::is_valid(fragment));

        Self { fragment }
    }
}

impl<'a> TryFrom<&'a str> for Fragment<'a> {
    type Error = Error;

    fn try_from(fragment: &'a str) -> Result<Self, Self::Error> {
        Self::new(fragment)
    }
}

impl<'a> Fragment<'a> {
    //! Properties

    /// Gets the fragment string. (will contain the '#' prefix)
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.fragment
    }

    /// Gets the fragment value. (will not contain the '#' prefix)
    #[must_use]
    pub const fn value(self) -> &'a str {
        self.fragment.split_at(1).1
    }
}

impl<'a> Debug for Fragment<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.fragment, f)
    }
}

impl<'a> Display for Fragment<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(self.fragment)
    }
}

#[cfg(test)]
mod tests {
    use crate::Fragment;

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", false),
            ("#", true),
            ("#azAZ09", true),
            ("#-._~!$&'()*+,;=:@/?", true),
            ("#!/&/=/~/", true),
            // A '%' char begins a percent-encoded octet & must be followed by two hex digits.
            ("#%20", true),
            ("#a%2Fb%2fc", true),
            ("#%00", true),
            ("#%aF", true),
            ("#%", false),
            ("#%2", false),
            ("#%zz", false),
            ("#%2g", false),
            ("#%%20", false),
            // The '#' char is not a fragment char, so only the prefix may be one.
            ("###", false),
            ("no-hash", false),
            ("# ", false),
            ("# x", false),
            ("#<>", false),
            ("#[]", false),
            ("#^`{|}\\\"", false),
            ("#\u{0}", false),
            ("#\u{4f60}", false),
        ];

        for (fragment, expected) in test_cases {
            let result: bool = Fragment::is_valid(fragment);
            assert_eq!(result, *expected, "fragment={}", fragment);
        }
    }
}
