use crate::parse::{Parts, PrePath, parse_parts};
use address::IPAddress;

/// A web-based URL.
///
/// # Format
/// All web-based URLs will be in the format: `scheme://host:port/path?query#fragment`. This is a subset of
/// [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986#section-3).
///
/// - The `port`, `query`, & `fragment` are all optional.
/// - The `path` will never be empty & will always start with a '/'.
#[must_use]
#[derive(Clone)]
pub struct WebUrl {
    pub(in crate::web_url) url: String,
    pub(in crate::web_url) scheme_len: u32,
    pub(in crate::web_url) host_end: u32,
    pub(in crate::web_url) ip: Option<IPAddress>,
    pub(in crate::web_url) port_end: u32,
    pub(in crate::web_url) port: Option<u16>,
    pub(in crate::web_url) path_end: u32,
    pub(in crate::web_url) query_end: u32,
}

impl WebUrl {
    //! Limits

    /// The maximum length of a URL string.
    pub const MAX_LEN: usize = u32::MAX as usize;

    /// Ensures the URL `len` is valid.
    ///
    /// # Panics
    /// Panics if the `len` is greater than `Self::MAX_LEN`.
    pub(in crate::web_url) fn check_len(len: usize) {
        assert!(
            len <= Self::MAX_LEN,
            "a web-url with '{}' bytes is longer than the max of '{}' bytes",
            len,
            Self::MAX_LEN
        );
    }
}

impl WebUrl {
    //! Construction

    /// Creates a new web-based URL.
    ///
    /// # Safety
    /// The `url` must be a normalized web-based URL & every other parameter must describe it:
    /// - The `url` parses & is already normalized. The scheme & host are lowercase, an IP address host is in its
    ///   canonical form, the path is present, starts with a '/', & has no dot-segments, an empty port is absent along
    ///   with its ':', & the port has no leading zeros.
    /// - The offsets are non-decreasing & the last is within the `url`:
    ///   `scheme_len <= host_end <= port_end <= path_end <= query_end <= url.len()`.
    /// - The `ip` is the parsed host when the host is an IP address & `None` when it is a domain.
    /// - The `port` is the parsed port & matches the `[host_end..port_end]` text.
    ///
    /// Breaking the contract does not cause undefined behavior in this crate; the component accessors return the wrong
    /// slice or panic. It is still `unsafe` because `host()` hands the host slice to `DomainRef::new_unchecked`, whose
    /// own contract requires a valid domain name.
    ///
    /// The contract is `debug_assert`ed by re-parsing the `url`, which is the work this function exists to skip, so
    /// the check is absent from release builds. Use `FromStr` or `TryFrom<String>` to build a URL that is validated in
    /// release builds too.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn new_unchecked<S>(
        url: S,
        scheme_len: u32,
        host_end: u32,
        ip: Option<IPAddress>,
        port_end: u32,
        port: Option<u16>,
        path_end: u32,
        query_end: u32,
    ) -> Self
    where
        S: Into<String>,
    {
        let url: Self = Self {
            url: url.into(),
            scheme_len,
            host_end,
            ip,
            port_end,
            port,
            path_end,
            query_end,
        };

        debug_assert!(
            url.is_consistent(),
            "web-url: WebUrl::new_unchecked was called with a URL that is not normalized or with \
             offsets that do not describe it. The URL was {:?} with scheme_len={} host_end={} \
             ip={:?} port_end={} port={:?} path_end={} query_end={}.",
            url.url,
            scheme_len,
            host_end,
            ip,
            port_end,
            port,
            path_end,
            query_end
        );

        url
    }
}

impl WebUrl {
    //! Consistency

    /// Checks that the URL string is normalized & that every offset describes it.
    ///
    /// This is the `new_unchecked` contract. It re-parses the URL, so it is only used in
    /// `debug_assert`s; parsing is exactly the work `new_unchecked` exists to skip.
    pub(in crate::web_url) fn is_consistent(&self) -> bool {
        // The offsets are checked before anything is sliced so that a bad offset returns false rather than panicking
        // inside the check itself.
        if self.url.len() > Self::MAX_LEN {
            return false;
        }
        let scheme_len: usize = self.scheme_len as usize;
        let host_end: usize = self.host_end as usize;
        let port_end: usize = self.port_end as usize;
        let path_end: usize = self.path_end as usize;
        let query_end: usize = self.query_end as usize;
        if scheme_len > host_end
            || host_end > port_end
            || port_end > path_end
            || path_end > query_end
            || query_end > self.url.len()
        {
            return false;
        }

        // The parser is the oracle for the URL string. A normalized URL must parse, must need no further normalization,
        // & must already be lowercase through the host.
        let parts: Parts = match parse_parts(self.url.as_str()) {
            Ok(parts) => parts,
            Err(_) => return false,
        };
        if !parts.is_normalized() {
            return false;
        }
        let pre_path: &PrePath = &parts.pre_path;
        if self.url[..pre_path.host_end()].bytes().any(|c| c.is_ascii_uppercase()) {
            return false;
        }

        // Every offset must match what the parser found.
        scheme_len == pre_path.scheme_len
            && host_end == pre_path.canonical_host_end()
            && self.ip == pre_path.ip
            && port_end == host_end + pre_path.canonical_port_len()
            && self.port == pre_path.port
            && path_end == port_end + parts.path_plus.canonical_path_len
            && query_end == path_end + parts.path_plus.query_len
    }
}

impl WebUrl {
    //! Properties

    /// Gets the length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.url.len()
    }

    /// Checks if the URL is empty. (always false)
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Converts the URL into its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.url
    }
}

impl From<WebUrl> for String {
    fn from(url: WebUrl) -> Self {
        url.into_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::WebUrl;
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn properties() -> Result<(), Box<dyn Error>> {
        let url: WebUrl = WebUrl::from_str("https://example.com/p?q#f")?;
        assert_eq!(url.len(), "https://example.com/p?q#f".len());
        assert!(!url.is_empty());

        Ok(())
    }

    #[test]
    fn deconstruction() -> Result<(), Box<dyn Error>> {
        let url: WebUrl = WebUrl::from_str("https://example.com/p?q#f")?;
        let url: String = url.into_string();
        assert_eq!(url, "https://example.com/p?q#f");

        // The string is already normalized, so the round trip reuses it.
        let url: WebUrl = WebUrl::try_from(url)?;
        assert_eq!(String::from(url), "https://example.com/p?q#f");

        Ok(())
    }
}
