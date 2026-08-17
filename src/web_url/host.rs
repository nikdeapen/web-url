use crate::parse;
use crate::WebUrl;
use address::{DomainRef, HostRef, IPAddress};

impl WebUrl {
    //! Host

    /// Gets the host reference.
    pub fn host(&self) -> HostRef<'_> {
        if let Some(ip) = self.ip {
            HostRef::Address(ip)
        } else {
            HostRef::Name(unsafe { DomainRef::new_unchecked(self.host_str()) })
        }
    }

    /// Gets the host string.
    ///
    /// This will be valid:
    /// - If the host is a domain it will be lowercase.
    /// - If the host is an IPv6 address it will include the '[]' brackets.
    #[must_use]
    pub fn host_str(&self) -> &str {
        let start: usize = (self.scheme_len + 3) as usize;
        let end: usize = self.host_end as usize;
        &self.url[start..end]
    }
}

impl WebUrl {
    //! Host Mutation

    /// Sets the `host`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn set_host<'a, H>(&mut self, host: H)
    where
        H: Into<HostRef<'a>>,
    {
        let host: HostRef = host.into();

        // An IPv6 host is written with the '[]' brackets & the host is lowercase, which is the
        // normalized form.
        let mut insert: String = match host {
            HostRef::Address(IPAddress::V6(ip)) => format!("[{}]", ip),
            host => host.to_string(),
        };
        insert.make_ascii_lowercase();

        let start: usize = (self.scheme_len + 3) as usize;
        let end: usize = self.host_end as usize;

        // The length is checked before anything is modified so an over-long URL panics with the URL
        // intact rather than leaving the string inconsistent with the component offsets.
        Self::check_len((self.url.len() - (end - start)) + insert.len());

        // The port, path, query, & fragment follow the host & are unchanged, so their lengths are
        // saved to rebuild the offsets that the splice shifts.
        let port_len: u32 = self.port_end - self.host_end;
        let path_len: u32 = self.path_end - self.port_end;
        let query_len: u32 = self.query_end - self.path_end;

        // The IP comes from the host string rather than the `HostRef` since a domain name that is
        // also a valid IP address is parsed as an IP address.
        self.ip = parse::parse_ip_and_validate_domain(insert.as_str()).ok().flatten();

        self.url.replace_range(start..end, insert.as_str());

        self.host_end = (start + insert.len()) as u32;
        self.port_end = self.host_end + port_len;
        self.path_end = self.port_end + path_len;
        self.query_end = self.path_end + query_len;

        debug_assert!(self.is_consistent());
    }

    /// Sets the `host`.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_host<'a, H>(mut self, host: H) -> Self
    where
        H: Into<HostRef<'a>>,
    {
        self.set_host(host);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::WebUrl;
    use address::{HostRef, IPv4Address, IPv6Address};
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn host_domain() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com")?;
        match url.host() {
            HostRef::Name(domain) => assert_eq!(domain.name(), "example.com"),
            _ => panic!("expected domain"),
        }
        Ok(())
    }

    #[test]
    fn host_domain_uppercase() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://EXAMPLE.COM")?;
        match url.host() {
            HostRef::Name(domain) => assert_eq!(domain.name(), "example.com"),
            _ => panic!("expected domain"),
        }
        Ok(())
    }

    #[test]
    fn host_domain_idn() -> Result<(), Box<dyn Error>> {
        // The `xn--` ACE prefix has consecutive hyphens, which requires `address` >= 0.19.
        let url = WebUrl::from_str("https://xn--bcher-kva.example")?;
        match url.host() {
            HostRef::Name(domain) => assert_eq!(domain.name(), "xn--bcher-kva.example"),
            _ => panic!("expected domain"),
        }
        Ok(())
    }

    #[test]
    fn host_ipv4() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://127.0.0.1")?;
        match url.host() {
            HostRef::Address(ip) => assert_eq!(ip, IPv4Address::LOCALHOST.to_ip()),
            _ => panic!("expected ip address"),
        }
        Ok(())
    }

    #[test]
    fn host_ipv6() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://[::1]")?;
        match url.host() {
            HostRef::Address(ip) => assert_eq!(ip, IPv6Address::LOCALHOST.to_ip()),
            _ => panic!("expected ip address"),
        }
        Ok(())
    }

    #[test]
    fn host_str() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://EXAMPLE.com")?;
        assert_eq!(url.host_str(), "example.com");

        let url = WebUrl::from_str("https://[::1]:80")?;
        assert_eq!(url.host_str(), "[::1]");

        Ok(())
    }
}
