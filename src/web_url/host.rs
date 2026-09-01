use crate::WebUrl;
use crate::parse;
use address::{DomainRef, HostRef, IPAddress};

impl WebUrl {
    //! Host

    /// Gets the host reference.
    pub fn host(&self) -> HostRef<'_> {
        if let Some(ip) = self.ip {
            HostRef::IPAddress(ip)
        } else {
            HostRef::Domain(unsafe { DomainRef::new_unchecked(self.host_str()) })
        }
    }

    /// Gets the host string.
    ///
    /// This will be valid:
    /// - If the host is a domain it will be lowercase.
    /// - If the host is an IP address it will be in its canonical form.
    /// - If the host is an IPv6 address it will include the '[]' brackets.
    fn host_str(&self) -> &str {
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

        // An IP address is written in its canonical form & a domain name is already lowercase, so both arms are the
        // normalized form as they stand. A domain name cannot also be an IP address since its final label cannot be
        // all-numeric, so the variant alone determines the IP.
        let canonical: parse::CanonicalHost;
        let (insert, ip): (&str, Option<IPAddress>) = match host {
            HostRef::Domain(domain) => (domain.name(), None),
            HostRef::IPAddress(ip) => {
                canonical = parse::CanonicalHost::new(ip);
                (canonical.as_str(), Some(ip))
            }
        };

        let start: usize = (self.scheme_len + 3) as usize;
        let end: usize = self.host_end as usize;

        // The length is checked before anything is modified so an over-long URL panics with the URL intact rather than
        // leaving the string inconsistent with the component offsets.
        Self::check_len((self.url.len() - (end - start)) + insert.len());

        // The port, path, query, & fragment follow the host & are unchanged, so their lengths are saved to rebuild the
        // offsets that the splice shifts.
        let port_len: u32 = self.port_end - self.host_end;
        let path_len: u32 = self.path_end - self.port_end;
        let query_len: u32 = self.query_end - self.path_end;

        // The IP is assigned after the length check so an over-long URL leaves the URL & its IP unmodified.
        self.ip = ip;

        self.url.replace_range(start..end, insert);

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
    use address::{DomainRef, HostRef, IPv4Address, IPv6Address};
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn host_domain() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://example.com")?;
        match url.host() {
            HostRef::Domain(domain) => assert_eq!(domain.name(), "example.com"),
            _ => panic!("expected domain"),
        }
        Ok(())
    }

    #[test]
    fn host_domain_uppercase() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://EXAMPLE.COM")?;
        match url.host() {
            HostRef::Domain(domain) => assert_eq!(domain.name(), "example.com"),
            _ => panic!("expected domain"),
        }
        Ok(())
    }

    #[test]
    fn host_domain_idn() -> Result<(), Box<dyn Error>> {
        // The `xn--` ACE prefix has consecutive hyphens, which requires `address` >= 0.19.
        let url = WebUrl::from_str("https://xn--bcher-kva.example")?;
        match url.host() {
            HostRef::Domain(domain) => assert_eq!(domain.name(), "xn--bcher-kva.example"),
            _ => panic!("expected domain"),
        }
        Ok(())
    }

    #[test]
    fn host_ipv4() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://127.0.0.1")?;
        match url.host() {
            HostRef::IPAddress(ip) => assert_eq!(ip, IPv4Address::LOCALHOST.to_ip()),
            _ => panic!("expected ip address"),
        }
        Ok(())
    }

    #[test]
    fn host_ipv6() -> Result<(), Box<dyn Error>> {
        let url = WebUrl::from_str("https://[::1]")?;
        match url.host() {
            HostRef::IPAddress(ip) => assert_eq!(ip, IPv6Address::LOCALHOST.to_ip()),
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

    #[test]
    fn set_host() -> Result<(), Box<dyn Error>> {
        // The host changes length, so the port, path, query, & fragment offsets must shift with it.
        let mut url: WebUrl = WebUrl::from_str("http://host:8080/p?q#f")?;

        url.set_host(DomainRef::EXAMPLE);
        assert_eq!(url.as_str(), "http://example.com:8080/p?q#f");
        assert_eq!(url.host(), HostRef::Domain(DomainRef::EXAMPLE));

        // An IPv6 host is bracketed & written in its canonical form.
        url.set_host(IPv6Address::LOCALHOST);
        assert_eq!(url.as_str(), "http://[::1]:8080/p?q#f");
        assert_eq!(url.host(), HostRef::IPAddress(IPv6Address::LOCALHOST.to_ip()));

        url.set_host(IPv4Address::LOCALHOST);
        assert_eq!(url.as_str(), "http://127.0.0.1:8080/p?q#f");
        assert_eq!(url.host(), HostRef::IPAddress(IPv4Address::LOCALHOST.to_ip()));

        Ok(())
    }

    #[test]
    fn with_host() -> Result<(), Box<dyn Error>> {
        let url: WebUrl = WebUrl::from_str("http://host/p")?.with_host(DomainRef::EXAMPLE);
        assert_eq!(url.as_str(), "http://example.com/p");

        Ok(())
    }
}
