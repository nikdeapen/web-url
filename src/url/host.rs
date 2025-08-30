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
    /// This will be a valid.
    /// - If the host is a domain it will be lowercase.
    /// - If the host is an IPv6 address it will include the '[]' brackets.
    fn host_str(&self) -> &str {
        let start: usize = (self.scheme_len + 3) as usize;
        let end: usize = self.host_end as usize;
        &self.url[start..end]
    }
}
