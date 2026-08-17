use crate::WebUrl;

impl WebUrl {
    //! Port

    /// Gets the optional port.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }
}

impl WebUrl {
    //! Port Mutation

    /// Sets the optional `port`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn set_port<P>(&mut self, port: P)
    where
        P: Into<Option<u16>>,
    {
        let port: Option<u16> = port.into();

        // The port is written with its ':' prefix & without leading zeros, which is the normalized
        // form. A URL with no port has no ':' either.
        let insert: String = port.map(|port| format!(":{}", port)).unwrap_or_default();

        let start: usize = self.host_end as usize;
        let end: usize = self.port_end as usize;

        // The length is checked before anything is modified so an over-long URL panics with the URL
        // intact rather than leaving the string inconsistent with the component offsets.
        Self::check_len((self.url.len() - (end - start)) + insert.len());

        // The path, query, & fragment follow the port & are unchanged, so their lengths are saved to
        // rebuild the offsets that the splice shifts.
        let path_len: u32 = self.path_end - self.port_end;
        let query_len: u32 = self.query_end - self.path_end;

        self.url.replace_range(start..end, insert.as_str());

        self.port = port;
        self.port_end = (start + insert.len()) as u32;
        self.path_end = self.port_end + path_len;
        self.query_end = self.path_end + query_len;

        debug_assert!(self.is_consistent());
    }

    /// Sets the optional `port`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_port<P>(mut self, port: P) -> Self
    where
        P: Into<Option<u16>>,
    {
        self.set_port(port);
        self
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
