use crate::parse;
use crate::Error;
use crate::Error::InvalidParam;
use std::fmt::{Debug, Display, Formatter};

/// A web-based URL query parameter.
///
/// # WHATWG URL
/// <https://url.spec.whatwg.org/#application/x-www-form-urlencoded>
///
/// The `name=value` convention comes from `application/x-www-form-urlencoded`, not RFC 3986.
///
/// # Validation
/// Both the name and value of a query parameter may be the empty string. The value string may also be absent
/// altogether, which signifies a missing '=' in the query parameter string.
///
/// Query parameter names & values can contain the RFC 3986 query chars: the US-ASCII letters and numbers, the
/// unreserved chars "-._~", the sub-delim chars "!$&'()*+,;=", and the ":@/?" chars. The '%' char is also accepted but
/// the percent-encoding is not validated. The '&' char is excluded since it denotes the end of the parameter in the
/// URL. Names cannot contain the '=' char since this denotes the end of the query parameter name.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Param<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> Param<'a> {
    //! Construction

    /// Creates a new query parameter.
    pub fn new(name: &'a str, value: Option<&'a str>) -> Result<Self, Error> {
        if Self::is_valid_parts(name, value) {
            Ok(Self { name, value })
        } else {
            Err(InvalidParam)
        }
    }

    /// Creates a new query parameter.
    ///
    /// # Safety
    /// The `name` and `value` must be valid.
    pub unsafe fn new_unchecked(name: &'a str, value: Option<&'a str>) -> Self {
        debug_assert!(Self::is_valid_parts(name, value));

        Self { name, value }
    }

    /// Splits the `param` into a name & optional value on the first '=' char.
    fn split(param: &str) -> (&str, Option<&str>) {
        match param.split_once('=') {
            Some((name, value)) => (name, Some(value)),
            None => (param, None),
        }
    }

    /// Creates a new query parameter from the `param`.
    ///
    /// The `param` will be split on the first '=' char. If not present the value will be `None`.
    ///
    /// # Safety
    /// The `param` must be valid.
    pub unsafe fn from_str_unchecked(param: &'a str) -> Self {
        let (name, value) = Self::split(param);
        unsafe { Self::new_unchecked(name, value) }
    }
}

impl<'a> TryFrom<&'a str> for Param<'a> {
    type Error = Error;

    fn try_from(param: &'a str) -> Result<Self, Self::Error> {
        let (name, value) = Self::split(param);
        Self::new(name, value)
    }
}

impl<'a> Param<'a> {
    //! Validation

    /// Checks if the char `c` is a valid name char.
    fn is_valid_name_char(c: u8) -> bool {
        parse::is_valid_char(c, "&=")
    }

    /// Checks if the char `c` is a valid value char.
    fn is_valid_value_char(c: u8) -> bool {
        parse::is_valid_char(c, "&")
    }

    /// Checks if the `name` & optional `value` are valid.
    fn is_valid_parts(name: &str, value: Option<&str>) -> bool {
        Self::is_valid_name(name) && value.iter().all(|v| Self::is_valid_value(v))
    }

    /// Checks if the `param` is valid.
    ///
    /// The `param` is split on the first '=' char, as in `from_str_unchecked`.
    #[must_use]
    pub fn is_valid(param: &str) -> bool {
        let (name, value) = Self::split(param);
        Self::is_valid_parts(name, value)
    }

    /// Checks if the `name` is valid.
    #[must_use]
    pub fn is_valid_name(name: &str) -> bool {
        name.as_bytes().iter().all(|c| Self::is_valid_name_char(*c))
    }

    /// Checks if the `value` is valid.
    #[must_use]
    pub fn is_valid_value(value: &str) -> bool {
        value.as_bytes().iter().all(|c| Self::is_valid_value_char(*c))
    }
}

impl<'a> Param<'a> {
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

impl<'a> Debug for Param<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for Param<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(value) = self.value {
            write!(f, "={}", value)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Error::InvalidParam;
    use crate::Param;

    #[test]
    fn new() {
        let param: Param = Param::new("name", Some("value")).unwrap();
        assert_eq!(param.name, "name");
        assert_eq!(param.value, Some("value"));

        assert_eq!(Param::new("na&me", None), Err(InvalidParam));
        assert_eq!(Param::new("name", Some("val#ue")), Err(InvalidParam));
    }

    #[test]
    fn from_str_unchecked() {
        let param: Param = unsafe { Param::from_str_unchecked("name") };
        assert_eq!(param.name, "name");
        assert_eq!(param.value, None);

        let param: Param = unsafe { Param::from_str_unchecked("name=") };
        assert_eq!(param.name, "name");
        assert_eq!(param.value, Some(""));

        let param: Param = unsafe { Param::from_str_unchecked("name=value") };
        assert_eq!(param.name, "name");
        assert_eq!(param.value, Some("value"));

        let param: Param = unsafe { Param::from_str_unchecked("=value") };
        assert_eq!(param.name, "");
        assert_eq!(param.value, Some("value"));

        let param: Param = unsafe { Param::from_str_unchecked("=") };
        assert_eq!(param.name, "");
        assert_eq!(param.value, Some(""));
    }

    #[test]
    fn try_from_str() {
        let param: Param = Param::try_from("name").unwrap();
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), None);

        let param: Param = Param::try_from("name=value").unwrap();
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), Some("value"));

        let param: Param = Param::try_from("name=").unwrap();
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), Some(""));

        let param: Param = Param::try_from("").unwrap();
        assert_eq!(param.name(), "");
        assert_eq!(param.value(), None);

        let param: Param = Param::try_from("=value").unwrap();
        assert_eq!(param.name(), "");
        assert_eq!(param.value(), Some("value"));

        let param: Param = Param::try_from("=").unwrap();
        assert_eq!(param.name(), "");
        assert_eq!(param.value(), Some(""));

        assert_eq!(Param::try_from("name=val&ue"), Err(InvalidParam));
        assert_eq!(Param::try_from("na&me"), Err(InvalidParam));
        assert_eq!(Param::try_from("na#me"), Err(InvalidParam));
        assert_eq!(Param::try_from("name=val#ue"), Err(InvalidParam));
    }

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", true),
            ("name", true),
            ("name=value", true),
            ("=value", true),
            ("=", true),
            ("name=val=ue", true),
            ("-._~!$'()*+,;:@%/?=-._~!$'()*+,;=:@%/?", true),
            ("na&me", false),
            ("na#me", false),
            ("name=val&ue", false),
            ("name=val#ue", false),
            ("na<me", false),
            ("name=val[ue", false),
        ];
        for (param, expected) in test_cases {
            let result: bool = Param::is_valid(param);
            assert_eq!(result, *expected, "param={}", param);
        }
    }

    #[test]
    fn properties() {
        let param: Param = Param::new("name", Some("value")).unwrap();
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), Some("value"));

        let param: Param = Param::new("name", None).unwrap();
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), None);
    }

    #[test]
    fn display() {
        let param: Param = Param::new("name", Some("value")).unwrap();
        assert_eq!(param.to_string(), "name=value");

        let param: Param = Param::new("name", None).unwrap();
        assert_eq!(param.to_string(), "name");
    }
}
