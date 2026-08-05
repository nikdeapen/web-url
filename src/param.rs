use crate::parse::Error;
use crate::parse::Error::InvalidParam;
use std::fmt::{Display, Formatter};

/// A web-based URL query parameter.
///
/// # Validation
/// Both the name and value of a query parameter may be the empty string. The value string may also
/// be absent altogether, which signifies a missing '=' in the query parameter string.
///
/// Query parameter names & values can contain any US-ASCII letters, numbers, or punctuation chars
/// excluding '&' and '#' since these chars denote the end of the parameter or query in the URL.
/// Names cannot contain the '=' char since this denotes the end of the query parameter name.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Param<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> Param<'a> {
    //! Construction

    /// Creates a new query parameter.
    pub fn new(name: &'a str, value: Option<&'a str>) -> Result<Self, Error> {
        if Self::is_valid_name(name) && value.iter().all(|v| Self::is_valid_value(v)) {
            Ok(Self { name, value })
        } else {
            Err(InvalidParam)
        }
    }

    /// Creates a new query parameter without validating it.
    ///
    /// # Safety
    /// The `name` and `value` must be valid.
    pub unsafe fn new_unchecked(name: &'a str, value: Option<&'a str>) -> Self {
        debug_assert!(Self::is_valid_name(name));
        debug_assert!(value.iter().all(|v| Self::is_valid_value(v)));

        Self { name, value }
    }

    /// Creates a new query parameter from the `param` without validating it.
    ///
    /// The `param` will be split on the first '=' char. If not present the value will be `None`.
    ///
    /// # Safety
    /// The `param` must be valid.
    pub unsafe fn from_str_unchecked(param: &'a str) -> Self {
        if let Some(eq) = param.as_bytes().iter().position(|c| *c == b'=') {
            let (name, eq_value) = param.split_at(eq);
            unsafe { Self::new_unchecked(name, Some(&eq_value[1..])) }
        } else {
            unsafe { Self::new_unchecked(param, None) }
        }
    }
}

impl<'a> TryFrom<&'a str> for Param<'a> {
    type Error = Error;

    fn try_from(param: &'a str) -> Result<Self, Self::Error> {
        if let Some(eq) = param.as_bytes().iter().position(|c| *c == b'=') {
            let (name, eq_value) = param.split_at(eq);
            Self::new(name, Some(&eq_value[1..]))
        } else {
            Self::new(param, None)
        }
    }
}

impl<'a> Param<'a> {
    //! Validation

    /// Checks if the `param` is valid.
    ///
    /// The `param` is split on the first '=' char, as in `from_str_unchecked`.
    pub fn is_valid(param: &str) -> bool {
        if let Some(eq) = param.as_bytes().iter().position(|c| *c == b'=') {
            let (name, eq_value) = param.split_at(eq);
            Self::is_valid_name(name) && Self::is_valid_value(&eq_value[1..])
        } else {
            Self::is_valid_name(param)
        }
    }

    /// Checks if the `name` is valid.
    pub fn is_valid_name(name: &str) -> bool {
        name.as_bytes().iter().all(|c| {
            c.is_ascii_alphanumeric()
                || (c.is_ascii_punctuation() && *c != b'&' && *c != b'#' && *c != b'=')
        })
    }

    /// Checks if the `value` is valid.
    pub fn is_valid_value(value: &str) -> bool {
        value.as_bytes().iter().all(|c| {
            c.is_ascii_alphanumeric() || (c.is_ascii_punctuation() && *c != b'&' && *c != b'#')
        })
    }
}

impl<'a> Param<'a> {
    //! Properties

    /// Gets the name.
    pub const fn name(self) -> &'a str {
        self.name
    }

    /// Gets the optional value.
    pub const fn value(self) -> Option<&'a str> {
        self.value
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
    use crate::parse::Error::InvalidParam;
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
        let param = Param::try_from("name").unwrap();
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), None);

        let param = Param::try_from("name=value").unwrap();
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), Some("value"));

        let param = Param::try_from("name=").unwrap();
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), Some(""));

        let param = Param::try_from("").unwrap();
        assert_eq!(param.name(), "");
        assert_eq!(param.value(), None);

        let param = Param::try_from("=value").unwrap();
        assert_eq!(param.name(), "");
        assert_eq!(param.value(), Some("value"));

        let param = Param::try_from("=").unwrap();
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
            ("na&me", false),
            ("na#me", false),
            ("name=val&ue", false),
            ("name=val#ue", false),
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
