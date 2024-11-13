use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use address::{DomainRef, HostRef, IPAddress};

use crate::{Path, Query, Scheme};

/// A web-based URL.
///
/// # Format
/// All web-based URLs will be in the format:    scheme://host:port/path?query#fragment
///
/// The port, query, and fragment are all optional.
/// The path will never be empty and will always start with a '/'.
#[derive(Clone, Debug)]
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

impl Ord for WebUrl {
    fn cmp(&self, other: &Self) -> Ordering {
        self.url.cmp(&other.url)
    }
}

impl PartialOrd for WebUrl {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.url.partial_cmp(&other.url)
    }
}

impl Eq for WebUrl {}

impl PartialEq for WebUrl {
    fn eq(&self, other: &Self) -> bool {
        self.url.eq(&other.url)
    }
}

impl Hash for WebUrl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state)
    }
}

impl WebUrl {
    //! Construction

    /// Creates a new web-based URL.
    ///
    /// # Unsafe
    /// The URL string, lengths, indices, IP address, & port must all be valid.
    pub unsafe fn new_unchecked(
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
    //! Scheme

    /// Gets the scheme.
    pub fn scheme(&self) -> Scheme {
        unsafe { Scheme::new_unchecked(self.scheme_str()) }
    }

    /// Gets the scheme string. (will be a valid scheme)
    fn scheme_str(&self) -> &str {
        let end: usize = self.scheme_len as usize;
        &self.url[..end]
    }
}

impl WebUrl {
    //! Host

    /// Gets the host reference.
    pub fn host(&self) -> HostRef {
        if let Some(ip) = self.ip {
            HostRef::Address(ip)
        } else {
            HostRef::Name(unsafe { DomainRef::new_unchecked(self.host_str()) })
        }
    }

    /// Gets the host string. (will be a valid, lowercase host; may include the [] for IPv6)
    fn host_str(&self) -> &str {
        let start: usize = (self.scheme_len + 3) as usize;
        let end: usize = self.host_end as usize;
        &self.url[start..end]
    }
}

impl WebUrl {
    //! Port

    /// Gets the optional port.
    pub fn port(&self) -> Option<u16> {
        self.port
    }
}

impl WebUrl {
    //! Path

    /// Gets the path.
    pub fn path(&self) -> Path {
        unsafe { Path::new_unchecked(self.path_str()) }
    }

    /// Gets the path string. This will be a valid path starting with a '/'.
    fn path_str(&self) -> &str {
        let start: usize = self.port_end as usize;
        let end: usize = self.path_end as usize;
        &self.url[start..end]
    }
}

impl WebUrl {
    //! Query

    /// Gets the optional query.
    pub fn query(&self) -> Option<Query> {
        let query: &str = self.query_str();
        if query.is_empty() {
            None
        } else {
            Some(unsafe { Query::new_unchecked(query) })
        }
    }

    /// Gets the query string. This will be a valid query string starting with a '?' or empty.
    fn query_str(&self) -> &str {
        let start: usize = self.path_end as usize;
        let end: usize = self.query_end as usize;
        &self.url[start..end]
    }
}

impl WebUrl {
    //! Fragment

    /// Gets the optional fragment. (will not contain the #)
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

    /// Gets the fragment string. This will be a valid fragment starting with a '#' or empty.
    fn fragment_str(&self) -> &str {
        let start: usize = self.query_end as usize;
        &self.url[start..]
    }
}

impl WebUrl {
    //! Display

    /// Gets the URL as a string.
    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }
}

impl AsRef<str> for WebUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for WebUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use address::{DomainRef, IPv4Address};

    use crate::WebUrl;

    #[test]
    fn properties() {
        let url: WebUrl = WebUrl::from_str("scheme://localhost:80/path?query#fragment").unwrap();
        assert_eq!(url.scheme().as_str(), "scheme");
        assert_eq!(url.host().to_string(), DomainRef::LOCALHOST.to_string());
        assert_eq!(url.port().unwrap(), 80);
        assert_eq!(url.path().as_str(), "/path");
        assert_eq!(url.query().unwrap().as_str(), "?query");
        assert_eq!(url.fragment_with_hash().unwrap(), "#fragment");

        let url: WebUrl = WebUrl::from_str("scheme://127.0.0.1/").unwrap();
        assert_eq!(url.scheme().as_str(), "scheme");
        assert_eq!(url.host(), IPv4Address::LOCALHOST.to_host().to_ref());
        assert_eq!(url.port(), None);
        assert_eq!(url.path().as_str(), "/");
        assert_eq!(url.query(), None);
        assert_eq!(url.fragment_with_hash(), None);
    }

    #[test]
    fn display() {
        let url: WebUrl = WebUrl::from_str("scheme://127.0.0.1/").unwrap();
        assert_eq!(url.as_str(), "scheme://127.0.0.1/");
        assert_eq!(url.as_ref(), "scheme://127.0.0.1/");
        assert_eq!(url.to_string(), "scheme://127.0.0.1/");
    }
}
