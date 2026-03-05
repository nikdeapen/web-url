use crate::WebUrl;

impl WebUrl {
    //! Port

    /// Gets the optional port.
    pub fn port(&self) -> Option<u16> {
        self.port
    }
}

#[cfg(test)]
mod tests {
    use crate::WebUrl;
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn port_present() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com:8080")?;
        assert_eq!(url.port(), Some(8080));
        Ok(())
    }

    #[test]
    fn port_absent() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com")?;
        assert_eq!(url.port(), None);
        Ok(())
    }
}
