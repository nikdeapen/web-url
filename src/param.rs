/// A web URL query parameter.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Param<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> Param<'a> {
    //! Construction

    /// Creates a new query parameter. This constructor does not validate the name or value.
    pub const unsafe fn new_unchecked(name: &'a str, value: Option<&'a str>) -> Self {
        Self { name, value }
    }

    /// Creates a new query parameter from the string.
    /// This constructor does not validate the name or value.
    pub unsafe fn from_str_unchecked(s: &'a str) -> Self {
        if let Some(eq) = s.as_bytes().iter().position(|c| *c == b'=') {
            Self::new_unchecked(&s[..eq], Some(&s[eq + 1..]))
        } else {
            Self::new_unchecked(s, None)
        }
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
