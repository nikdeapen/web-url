/// A web-based URL query parameter.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Param<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> Param<'a> {
    //! Construction

    /// Creates a new query parameter.
    ///
    /// # Unsafe
    /// This constructor does not validate the name or value.
    pub unsafe fn new_unchecked(name: &'a str, value: Option<&'a str>) -> Self {
        debug_assert!(Self::is_valid_name(name));
        debug_assert!(value.iter().all(|v| Self::is_valid_value(*v)));

        Self { name, value }
    }

    /// Creates a new query parameter from the string. (the string will be split on the first `=`)
    ///
    /// # Unsafe
    /// This constructor does not validate the name or value.
    pub unsafe fn from_str_unchecked(s: &'a str) -> Self {
        if let Some(eq) = s.as_bytes().iter().position(|c| *c == b'=') {
            let (name, eq_value) = s.split_at(eq);
            Self::new_unchecked(name, Some(&eq_value[1..]))
        } else {
            Self::new_unchecked(s, None)
        }
    }
}

impl<'a> Param<'a> {
    //! Validation

    /// Checks if the name is valid.
    pub fn is_valid_name(name: &str) -> bool {
        name.as_bytes().iter().all(|c| {
            c.is_ascii_alphanumeric()
                || (c.is_ascii_punctuation() && *c != b'&' && *c != b'#' && *c != b'=')
        })
    }

    /// Checks if the value is valid.
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

#[cfg(test)]
mod tests {
    use crate::Param;

    #[test]
    fn new_unchecked() {
        let param: Param = unsafe { Param::new_unchecked("name", Some("value")) };
        assert_eq!(param.name, "name");
        assert_eq!(param.value, Some("value"));
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
    }

    #[test]
    fn properties() {
        let param: Param = unsafe { Param::new_unchecked("name", Some("value")) };
        assert_eq!(param.name(), "name");
        assert_eq!(param.value(), Some("value"));
    }
}
