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
