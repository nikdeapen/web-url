use crate::WebUrl;
use crate::parse;

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
        let canonical: parse::CanonicalPort;
        let insert: &str = match port {
            Some(port) => {
                canonical = parse::CanonicalPort::new(port);
                canonical.as_str()
            }
            None => "",
        };

        let start: usize = self.host_end as usize;
        let end: usize = self.port_end as usize;

        // The length is checked before anything is modified so an over-long URL panics with the URL
        // intact rather than leaving the string inconsistent with the component offsets.
        Self::check_len((self.url.len() - (end - start)) + insert.len());

        // The path, query, & fragment follow the port & are unchanged, so their lengths are saved
        // to rebuild the offsets that the splice shifts.
        let path_len: u32 = self.path_end - self.port_end;
        let query_len: u32 = self.query_end - self.path_end;

        self.url.replace_range(start..end, insert);

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

    #[test]
    fn set_port() -> Result<(), Box<dyn Error>> {
        // The port changes length, so the path, query, & fragment offsets must shift with it.
        let test_cases: &[(&str, Option<u16>, &str)] = &[
            ("http://host/p?q#f", Some(8080), "http://host:8080/p?q#f"),
            ("http://host:80/p?q#f", Some(443), "http://host:443/p?q#f"),
            ("http://host:80/p?q#f", None, "http://host/p?q#f"),
            ("http://host/p", None, "http://host/p"),
            ("http://host/p", Some(0), "http://host:0/p"),
            ("http://host:1/p", Some(65535), "http://host:65535/p"),
            ("http://[::1]/p", Some(80), "http://[::1]:80/p"),
        ];
        for (input, port, expected) in test_cases {
            let mut url: WebUrl = WebUrl::from_str(input)?;
            url.set_port(*port);
            assert_eq!(url.as_str(), *expected, "input={}", input);
            assert_eq!(url.port(), *port, "input={}", input);
        }

        Ok(())
    }

    #[test]
    fn with_port() -> Result<(), Box<dyn Error>> {
        let url: WebUrl = WebUrl::from_str("https://example.com/p")?.with_port(8080);
        assert_eq!(url.as_str(), "https://example.com:8080/p");

        let url: WebUrl = url.with_port(None);
        assert_eq!(url.as_str(), "https://example.com/p");

        Ok(())
    }
}
