use address::{DomainRef, HostRef, IPAddress};

use crate::{Path, Query, Scheme};

/// A web-based URL.
pub struct WebUrl {
    url: String,
    scheme_len: u32,
    host_end: u32,
    ip: Option<IPAddress>,
    port_end: u32,
    port: Option<u16>,
    path_end: u32,
    query_end: u32,
}

impl WebUrl {
    //! Construction

    /// Creates a new web-based URL.
    pub const unsafe fn new_unchecked(
        url: String,
        scheme_len: u32,
        host_end: u32,
        ip: Option<IPAddress>,
        port_end: u32,
        port: Option<u16>,
        path_end: u32,
        query_end: u32,
    ) -> Self {
        Self {
            url,
            scheme_len,
            host_end,
            ip,
            port_end,
            port,
            path_end,
            query_end,
        }
    }
}

impl WebUrl {
    //! Properties

    /// Gets the scheme.
    pub fn scheme(&self) -> Scheme {
        unsafe { Scheme::new_unchecked(self.scheme_str()) }
    }

    /// Gets the host reference.
    pub fn host(&self) -> HostRef {
        if let Some(ip) = self.ip {
            HostRef::Address(ip)
        } else {
            HostRef::Name(unsafe { DomainRef::new_unchecked(self.host_str()) })
        }
    }

    /// Gets the optional port.
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// Gets the path.
    pub fn path(&self) -> Path {
        unsafe { Path::new_unchecked(self.path_str()) }
    }

    /// Gets the optional query.
    pub fn query(&self) -> Option<Query> {
        let query: &str = self.query_str();
        if query.is_empty() {
            None
        } else {
            Some(unsafe { Query::new_unchecked(query) })
        }
    }

    /// Gets the optional fragment. (will not contain the '#')
    pub fn fragment(&self) -> Option<&str> {
        self.fragment_with_hash().map(|f| &f[1..])
    }

    /// Gets the optional fragment with the '#' prefix.
    pub fn fragment_with_hash(&self) -> Option<&str> {
        let fragment: &str = self.fragment_str();
        if fragment.is_empty() {
            None
        } else {
            Some(fragment)
        }
    }
}

impl WebUrl {
    //! Internal strings

    /// Gets the scheme string. (will be a valid scheme)
    fn scheme_str(&self) -> &str {
        let end: usize = self.scheme_len as usize;
        &self.url[..end]
    }

    /// Gets the host string. (will be a valid, lowercase host; may include the [] for IPv6)
    fn host_str(&self) -> &str {
        let start: usize = (self.scheme_len + 3) as usize;
        let end: usize = self.host_end as usize;
        &self.url[start..end]
    }

    /// Gets the path string. (will be a valid path starting with a '/')
    fn path_str(&self) -> &str {
        let start: usize = self.port_end as usize;
        let end: usize = self.path_end as usize;
        &self.url[start..end]
    }

    /// Gets the query string. (will be a valid query string starting with a '?' or empty)
    fn query_str(&self) -> &str {
        let start: usize = self.path_end as usize;
        let end: usize = self.query_end as usize;
        &self.url[start..end]
    }

    /// Gets the fragment string. (will be a valid fragment string starting with a '#' or empty)
    fn fragment_str(&self) -> &str {
        let start: usize = self.query_end as usize;
        &self.url[start..]
    }
}
