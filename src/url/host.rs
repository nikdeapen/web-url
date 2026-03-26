use address::{DomainRef, HostRef};

use crate::WebUrl;

impl WebUrl {
    //! Host

    /// Gets the host reference.
    pub fn host(&self) -> HostRef<'_> {
        if let Some(ip) = self.ip {
            HostRef::Address(ip)
        } else {
            HostRef::Name(unsafe { DomainRef::new(self.host_str()) })
        }
    }

    /// Gets the host string.
    ///
    /// This will be valid:
    /// - If the host is a domain it will be lowercase.
    /// - If the host is an IPv6 address it will include the '[]' brackets.
    fn host_str(&self) -> &str {
        let start: usize = (self.scheme_len + 3) as usize;
        let end: usize = self.host_end as usize;
        &self.url[start..end]
    }
}

#[cfg(test)]
mod tests {
    use address::{HostRef, IPv4Address, IPv6Address};

    use crate::WebUrl;
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
}
