use crate::parse::Error;
use crate::parse::Error::InvalidParam;
use std::fmt::{Display, Formatter};

/// A web-based URL query parameter.
///
/// # Validation
/// Both the name and value of a query parameter may be the empty string. The value string may also
/// be absent altogether which signifies a missing '=' in the query parameter string.
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
    ///
    /// # Safety
    /// The `name` and `value` must be valid.
    pub unsafe fn new(name: &'a str, value: Option<&'a str>) -> Self {
        debug_assert!(Self::is_valid_name(name));
        debug_assert!(value.iter().all(|v| Self::is_valid_value(v)));

        Self { name, value }
    }

    /// Creates a new query parameter from the `param`.
    ///
    /// The `param` will be split on the first '=' char. If not present the value will be `None`.
    ///
    /// # Safety
    /// The `param` must be valid.
    pub unsafe fn from_str(param: &'a str) -> Self {
        if let Some(eq) = param.as_bytes().iter().position(|c| *c == b'=') {
            let (name, eq_value) = param.split_at(eq);
            Self::new(name, Some(&eq_value[1..]))
        } else {
            Self::new(param, None)
        }
    }
}

impl<'a> TryFrom<&'a str> for Param<'a> {
    type Error = Error;

    fn try_from(param: &'a str) -> Result<Self, Self::Error> {
        if let Some(eq) = param.as_bytes().iter().position(|c| *c == b'=') {
            let (name, eq_value) = param.split_at(eq);
            if Self::is_valid_name(name) && Self::is_valid_value(eq_value) {
                Ok(unsafe { Self::new(name, Some(&eq_value[1..])) })
            } else {
                Err(InvalidParam)
            }
        } else if Self::is_valid_name(param) {
            Ok(unsafe { Self::new(param, None) })
        } else {
            Err(InvalidParam)
        }
    }
}

impl<'a> Param<'a> {
    //! Validation

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
    pub const fn name(&self) -> &str {
        self.name
    }

    /// Gets the optional value.
    pub const fn value(&self) -> Option<&str> {
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
    use crate::Param;

    #[test]
    fn new() {
        let param: Param = unsafe { Param::new("name", Some("value")) };
        assert_eq!(param.name, "name");
        assert_eq!(param.value, Some("value"));
    }

    #[test]
    fn from_str() {
        let param: Param = unsafe { Param::from_str("name") };
        assert_eq!(param.name, "name");
        assert_eq!(param.value, None);

        let param: Param = unsafe { Param::from_str("name=") };
        assert_eq!(param.name, "name");
        assert_eq!(param.value, Some(""));

        let param: Param = unsafe { Param::from_str("name=value") };
        assert_eq!(param.name, "name");
        assert_eq!(param.value, Some("value"));
    }

    #[test]
    fn properties() {
        let param: Param = unsafe { Param::new("name", Some("value")) };
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), Some("value"));

        let param: Param = unsafe { Param::new("name", None) };
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), None);
    }

    #[test]
    fn display() {
        let param: Param = unsafe { Param::new("name", Some("value")) };
        assert_eq!(param.to_string(), "name=value");

        let param: Param = unsafe { Param::new("name", None) };
        assert_eq!(param.to_string(), "name");
    }
}
