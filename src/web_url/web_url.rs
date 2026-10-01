use crate::parse::{CanonicalHost, Parts, PrePath, parse_parts};
use crate::web_url::{Offsets, Part};
use address::IPAddress;
use std::ops::Range;

/// A web-based URL.
///
/// # Format
/// All web-based URLs will be in the format: `scheme://host:port/path?query#fragment`. This is a
/// subset of [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986#section-3).
///
/// - The `port`, `query`, & `fragment` are all optional.
/// - The `path` will not be empty & will always start with a '/'.
#[must_use]
#[derive(Clone)]
pub struct WebUrl {
    pub(in crate::web_url) url: String,
    pub(in crate::web_url) offsets: Offsets,
    pub(in crate::web_url) ip: Option<IPAddress>,
    pub(in crate::web_url) port: Option<u16>,
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
    /// - The `url` parses & is already normalized. The scheme & host are lowercase, an IP address
    ///   host is in its canonical form, the path is present, starts with a '/', & has no
    ///   dot-segments, an empty port is absent along with its ':', & the port has no leading zeros.
    /// - The `offsets` are non-decreasing & the last is within the `url`:
    ///   `scheme_len <= host_end <= port_end <= path_end <= query_end <= url.len()`.
    /// - The `ip` is the parsed host when the host is an IP address & `None` when it is a domain.
    /// - The `port` is the parsed port & matches the `[host_end..port_end]` text.
    pub(crate) unsafe fn new_unchecked(
        url: String,
        offsets: Offsets,
        ip: Option<IPAddress>,
        port: Option<u16>,
    ) -> Self {
        let url: Self = Self {
            url,
            offsets,
            ip,
            port,
        };

        debug_assert!(
            url.is_consistent(),
            "web-url: WebUrl::new_unchecked was called with a URL that is not normalized or with \
             offsets that do not describe it. The URL was {:?} with offsets={:?} ip={:?} \
             port={:?}.",
            url.url,
            offsets,
            ip,
            port
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
        // The parser is the oracle for the URL string. A normalized URL must parse, which includes
        // fitting `Self::MAX_LEN`, must need no further normalization, & must already be lowercase
        // through the host.
        let parts: Parts = match parse_parts(self.url.as_str()) {
            Ok(parts) => parts,
            Err(_) => return false,
        };
        if !parts.is_normalized() {
            return false;
        }
        let pre_path: &PrePath = &parts.pre_path;
        if self.url[..pre_path.host_end()]
            .bytes()
            .any(|c| c.is_ascii_uppercase())
        {
            return false;
        }

        // Every offset & value must match what the parser found. The offsets need no bounds checks
        // since they are only compared here, never used to slice.
        self.offsets == parts.offsets()
            && self.ip == pre_path.ip.as_ref().map(CanonicalHost::ip)
            && self.port == pre_path.port
    }
}

impl WebUrl {
    //! Splicing

    /// Replaces the `part` of the URL string with the `insert`, shifting the parts after it.
    ///
    /// The `insert` must keep the URL normalized.
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub(in crate::web_url) fn splice(&mut self, part: Part, insert: &str) {
        let range: Range<usize> = self.offsets.range(part);
        Self::check_len((self.url.len() - range.len()) + insert.len());
        self.url.replace_range(range, insert);
        self.offsets.resize(part, insert.len());
    }
}

impl WebUrl {
    //! Properties

    /// Gets the URL string.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        self.url.as_str()
    }

    /// Gets the length.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.url.len()
    }

    /// Checks if the URL is empty. (always false)
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl WebUrl {
    //! Deconstruction

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
    use std::str::FromStr;

    #[test]
    fn properties() {
        let url: WebUrl = WebUrl::from_str("https://example.com/p?q#f").unwrap();
        assert_eq!(url.as_str(), "https://example.com/p?q#f");
        assert_eq!(url.len(), "https://example.com/p?q#f".len());
        assert!(!url.is_empty());
    }

    #[test]
    fn deconstruction() {
        let url: WebUrl = WebUrl::from_str("https://example.com/p?q#f").unwrap();
        let url: String = url.into_string();
        assert_eq!(url, "https://example.com/p?q#f");

        // The string is already normalized, so the round trip reuses it.
        let url: WebUrl = WebUrl::try_from(url).unwrap();
        assert_eq!(String::from(url), "https://example.com/p?q#f");
    }
}
