use crate::ParseError::InvalidParam;
use crate::{ParseError, parse};
use std::fmt::{Debug, Display, Formatter};

/// A web-based URL query parameter.
///
/// - The `name` may be empty.
/// - The `value` may be empty or absent.
///
/// # WHATWG
/// The `name=value` convention comes from `application/x-www-form-urlencoded`, not RFC 3986.
/// <https://url.spec.whatwg.org/#application/x-www-form-urlencoded>
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct QueryParam<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> QueryParam<'a> {
    //! Validation

    /// Checks if the `name` is valid.
    #[must_use]
    pub const fn is_valid_name(name: &str) -> bool {
        parse::is_valid_chars(name.as_bytes(), "&=")
    }

    /// Checks if the `value` is valid.
    #[must_use]
    pub const fn is_valid_value(value: &str) -> bool {
        parse::is_valid_chars(value.as_bytes(), "&")
    }

    /// Checks if the `param` is valid.
    #[must_use]
    pub const fn is_valid(param: &str) -> bool {
        let (name, value) = Self::split(param);
        Self::is_valid_parts(name, value)
    }

    /// Checks if the `name` & optional `value` are valid.
    const fn is_valid_parts(name: &str, value: Option<&str>) -> bool {
        Self::is_valid_name(name)
            && match value {
                Some(value) => Self::is_valid_value(value),
                None => true,
            }
    }

    /// Splits the `param` into a name & optional value on the first '=' char.
    #[inline]
    const fn split(param: &str) -> (&str, Option<&str>) {
        let bytes: &[u8] = param.as_bytes();
        let mut index: usize = 0;
        while index < bytes.len() {
            if bytes[index] == b'=' {
                let (name, value) = param.split_at(index);
                let (_, value) = value.split_at(1);
                return (name, Some(value));
            }
            index += 1;
        }
        (param, None)
    }
}

impl<'a> QueryParam<'a> {
    //! Construction

    /// Creates a new query parameter.
    pub const fn new(param: &'a str) -> Result<Self, ParseError> {
        let (name, value) = Self::split(param);
        Self::from_parts(name, value)
    }

    /// Creates a new query parameter.
    ///
    /// # Safety
    /// The `param` must be valid.
    #[inline]
    pub const unsafe fn new_unchecked(param: &'a str) -> Self {
        let (name, value) = Self::split(param);
        unsafe { Self::from_parts_unchecked(name, value) }
    }

    /// Creates a new query parameter from the `name` & optional `value`.
    pub const fn from_parts(name: &'a str, value: Option<&'a str>) -> Result<Self, ParseError> {
        if Self::is_valid_parts(name, value) {
            Ok(Self { name, value })
        } else {
            Err(InvalidParam)
        }
    }

    /// Creates a new query parameter from the `name` & optional `value`.
    ///
    /// # Safety
    /// The `name` & `value` must be valid.
    pub const unsafe fn from_parts_unchecked(name: &'a str, value: Option<&'a str>) -> Self {
        debug_assert!(Self::is_valid_parts(name, value));

        Self { name, value }
    }
}

impl<'a> TryFrom<&'a str> for QueryParam<'a> {
    type Error = ParseError;

    fn try_from(param: &'a str) -> Result<Self, Self::Error> {
        Self::new(param)
    }
}

impl<'a> QueryParam<'a> {
    //! Properties

    /// Gets the name.
    #[must_use]
    pub const fn name(self) -> &'a str {
        self.name
    }

    /// Gets the optional value.
    #[must_use]
    pub const fn value(self) -> Option<&'a str> {
        self.value
    }
}

impl<'a> Debug for QueryParam<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("\"{}\"", self))
    }
}

impl<'a> Display for QueryParam<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = self.value {
            f.pad(&format!("{}={}", self.name, value))
        } else {
            f.pad(self.name)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::QueryParam;

    /// `(name, value)`
    type ParamParts<'a> = (&'a str, Option<&'a str>);

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", true),
            ("name", true),
            ("name=value", true),
            ("name=", true),
            ("=value", true),
            ("=", true),
            // Only the first '=' char is a separator, so a value may contain more.
            ("name=val=ue", true),
            ("-._~!$'()*+,;:@/?=-._~!$'()*+,;=:@/?", true),
            // A '%' char begins a percent-encoded octet & must be followed by two hex digits.
            ("%20=%21", true),
            ("name=%41", true),
            // An excluded char is only excluded literally, percent encoded chars are valid.
            ("%26=%3D", true),
            ("%", false),
            ("a=%2", false),
            ("%zz=b", false),
            ("name=%2g", false),
            // The '&' char ends the param & the '#' char is not a query char.
            ("na&me", false),
            ("name=val&ue", false),
            ("na#me", false),
            ("name=val#ue", false),
            ("na<me", false),
            ("name=val[ue", false),
            ("na me", false),
            ("na\u{0}me", false),
            ("na\u{4f60}me", false),
            ("name=val\u{4f60}ue", false),
        ];

        for (param, expected) in test_cases {
            let result: bool = QueryParam::is_valid(param);
            assert_eq!(result, *expected, "param={}", param);
        }
    }

    #[test]
    fn new() {
        let test_cases: &[(&str, ParamParts)] = &[
            ("name", ("name", None)),
            ("name=", ("name", Some(""))),
            ("name=value", ("name", Some("value"))),
            ("name=val=ue", ("name", Some("val=ue"))),
            ("=value", ("", Some("value"))),
            ("=", ("", Some(""))),
            ("", ("", None)),
        ];

        for (input, expected) in test_cases {
            let param: QueryParam = QueryParam::new(input).unwrap();
            assert_eq!((param.name, param.value), *expected, "input={}", input);

            let param: QueryParam = unsafe { QueryParam::new_unchecked(input) };
            assert_eq!((param.name, param.value), *expected, "input={}", input);
        }
    }
}
