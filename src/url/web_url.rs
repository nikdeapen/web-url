use address::IPAddress;

/// A web-based URL.
///
/// # Format
/// All web-based URLs will be in the format: `scheme://host:port/path?query#fragment`.
/// This is a sub-set of [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986#section-4.3).
///
/// The port, query, and fragment are all optional.
/// The path will never be empty and will always start with a '/'.
#[derive(Clone, Debug)]
pub struct WebUrl {
    pub(in crate::url) url: String,
    pub(in crate::url) scheme_len: u32,
    pub(in crate::url) host_end: u32,
    pub(in crate::url) ip: Option<IPAddress>,
    pub(in crate::url) port_end: u32,
    pub(in crate::url) port: Option<u16>,
    pub(in crate::url) path_end: u32,
    pub(in crate::url) query_end: u32,
}

impl WebUrl {
    //! Construction

    /// Creates a new web-based URL.
    ///
    /// # Safety
    /// The parameters must be valid.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn new<S>(
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
        Self {
            url: url.into(),
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

    /// Gets the length.
    pub fn len(&self) -> usize {
        self.url.len()
    }

    /// Checks if the url is empty.
    pub fn is_empty(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::Error::*;
    use crate::WebUrl;
    use std::str::FromStr;

    #[test]
    fn len() {
        let url = WebUrl::from_str("https://example.com/path").unwrap();
        assert_eq!(url.len(), "https://example.com/path".len());
    }

    #[test]
    fn is_empty() {
        let url = WebUrl::from_str("https://example.com").unwrap();
        assert!(!url.is_empty());
    }

    #[test]
    fn parse_full_url() {
        let url = WebUrl::from_str("https://example.com:8080/path?key=value#section").unwrap();
        assert_eq!(url.scheme().as_str(), "https");
        assert_eq!(url.port(), Some(8080));
        assert_eq!(url.path().as_str(), "/path");
        assert_eq!(url.query().unwrap().as_str(), "?key=value");
        assert_eq!(url.fragment().unwrap().as_str(), "#section");
    }

    #[test]
    fn parse_no_path_appends_slash() {
        let url = WebUrl::from_str("https://example.com").unwrap();
        assert_eq!(url.path().as_str(), "/");
        assert_eq!(url.as_str(), "https://example.com/");
    }

    #[test]
    fn parse_uppercase_lowercased() {
        let url = WebUrl::from_str("HTTPS://EXAMPLE.COM/Path").unwrap();
        assert_eq!(url.scheme().as_str(), "https");
        assert_eq!(url.as_str(), "https://example.com/Path");
    }

    #[test]
    fn parse_invalid_scheme() {
        assert_eq!(WebUrl::from_str("").unwrap_err(), InvalidScheme);
        assert_eq!(WebUrl::from_str("://host").unwrap_err(), InvalidScheme);
        assert_eq!(WebUrl::from_str("0http://host").unwrap_err(), InvalidScheme);
    }

    #[test]
    fn parse_invalid_host() {
        assert_eq!(WebUrl::from_str("http://").unwrap_err(), InvalidHost);
        assert_eq!(WebUrl::from_str("http://ho!st").unwrap_err(), InvalidHost);
    }

    #[test]
    fn parse_invalid_port() {
        assert_eq!(
            WebUrl::from_str("http://host:bad").unwrap_err(),
            InvalidPort
        );
        assert_eq!(
            WebUrl::from_str("http://host:99999").unwrap_err(),
            InvalidPort
        );
    }

    #[test]
    fn parse_invalid_path() {
        assert_eq!(
            WebUrl::from_str("http://host/path with space").unwrap_err(),
            InvalidPath
        );
    }

    #[test]
    fn parse_invalid_query() {
        assert_eq!(
            WebUrl::from_str("http://host/path?query with space").unwrap_err(),
            InvalidQuery
        );
    }

    #[test]
    fn parse_invalid_fragment() {
        assert_eq!(
            WebUrl::from_str("http://host/path# bad").unwrap_err(),
            InvalidFragment
        );
    }

    #[test]
    fn try_from_string_ok() {
        let input = String::from("https://example.com/path");
        let url = WebUrl::try_from(input).unwrap();
        assert_eq!(url.as_str(), "https://example.com/path");
    }

    #[test]
    fn try_from_string_err_returns_string() {
        let input = String::from("not a url");
        let (error, returned) = WebUrl::try_from(input).unwrap_err();
        assert_eq!(error, InvalidScheme);
        assert_eq!(returned, "not a url");
    }

    #[test]
    fn try_from_string_no_path() {
        let input = String::from("https://example.com");
        let url = WebUrl::try_from(input).unwrap();
        assert_eq!(url.as_str(), "https://example.com/");
    }
}
