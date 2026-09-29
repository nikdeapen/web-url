use crate::ParseError;
use crate::parse::{
    CanonicalHost, check_no_user_info, parse_host, parse_ip_and_validate_domain, parse_port,
    parse_scheme_len, port_decimal_len,
};
use address::IPAddress;

/// The parsing data for a web-based URL before the path.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub(crate) struct PrePath {
    /// The length of the scheme.
    pub(crate) scheme_len: usize,

    /// The length of the host in the parsed URL. (including the '[]' brackets)
    pub(crate) host_len: usize,

    /// The IP address when the host is an IP address.
    pub(crate) ip: Option<IPAddress>,

    /// The port. (`None` when the port is absent or empty)
    pub(crate) port: Option<u16>,

    /// The length of the port in the parsed URL. (including the ':')
    pub(crate) port_len: usize,
}

impl PrePath {
    //! Properties

    /// Gets the length of the pre-path string as it appears in the parsed URL.
    pub(crate) const fn len(self) -> usize {
        self.scheme_len + 3 + self.host_len + self.port_len
    }

    /// Gets the index of the host. (just past the "://" that follows the scheme)
    pub(crate) const fn host_start(self) -> usize {
        self.scheme_len + 3
    }

    /// Gets the index just past the host in the parsed URL. (the index of the ':' when there is a
    /// port)
    pub(crate) const fn host_end(self) -> usize {
        self.host_start() + self.host_len
    }

    /// Gets the host string in the parsed URL `s`.
    pub(crate) fn host_str(self, s: &str) -> &str {
        &s[self.host_start()..self.host_end()]
    }

    /// Gets the length of the host string in the normalized URL. (including the '[]' brackets)
    ///
    /// An IP address is written in its canonical form, which can be shorter or longer than the
    /// parsed host. A domain name is unaffected since only its letter case is normalized.
    fn canonical_host_len(self) -> usize {
        match self.ip {
            Some(ip) => CanonicalHost::new(ip).as_str().len(),
            None => self.host_len,
        }
    }

    /// Gets the index just past the host in the normalized URL.
    pub(crate) fn canonical_host_end(self) -> usize {
        self.host_start() + self.canonical_host_len()
    }

    /// Gets the length of the port string in the normalized URL. (including the ':')
    ///
    /// This is 0 when there is no port. It is shorter than `port_len` when the parsed port was
    /// empty or had leading zeros, & equal to it otherwise.
    pub(crate) const fn canonical_port_len(self) -> usize {
        match self.port {
            Some(port) => 1 + port_decimal_len(port),
            None => 0,
        }
    }

    /// Gets the length of the pre-path string in the normalized URL.
    pub(crate) fn canonical_len(self) -> usize {
        self.canonical_host_end() + self.canonical_port_len()
    }

    /// Checks if the host must be rewritten to normalize the parsed URL `s`.
    ///
    /// This is set when the host is an IP address that is not written in its canonical form. The
    /// letter case is excluded since it is normalized in place & never changes the length.
    pub(crate) fn needs_host_rewrite(self, s: &str) -> bool {
        match self.ip {
            Some(ip) => !CanonicalHost::new(ip)
                .as_str()
                .eq_ignore_ascii_case(self.host_str(s)),
            None => false,
        }
    }
}

impl PrePath {
    //! Operations

    /// Makes the scheme & host prefix of the normalized `url` lowercase.
    ///
    /// The port is excluded since it is all digits & unaffected by the letter case.
    pub(crate) fn make_lowercase(self, url: &mut str) {
        url[..self.canonical_host_end()].make_ascii_lowercase()
    }
}

/// Parses the pre-path portion of the URL. The scheme & host will be validated but may be
/// uppercase.
pub(crate) fn parse_pre_path(url: &str) -> Result<PrePath, ParseError> {
    let (scheme_len, after_scheme) = parse_scheme_len(url)?;

    // User info is checked before the host & port so that every form of it reports the same error.
    // Otherwise the '@' & ':' chars fall through to the host or port parser & the reported error
    // depends on where the colons happen to be.
    check_no_user_info(after_scheme)?;

    let (host_str, after_host) = parse_host(after_scheme);
    let ip: Option<IPAddress> = parse_ip_and_validate_domain(host_str)?;
    let (port, after_port) = parse_port(after_host)?;
    let port_len: usize = after_host.len() - after_port.len();
    let pre_path: PrePath = PrePath {
        scheme_len,
        host_len: host_str.len(),
        ip,
        port,
        port_len,
    };
    Ok(pre_path)
}

#[cfg(test)]
mod tests {
    use crate::ParseError;
    use crate::ParseError::{InvalidHost, InvalidScheme, UserInfoNotSupported};
    use crate::parse::{PrePath, parse_pre_path};
    use address::{IPv4Address, IPv6Address};

    #[test]
    fn pre_path() {
        let test_cases: &[(&str, Result<PrePath, ParseError>)] = &[
            ("scheme:/", Err(InvalidScheme)),
            ("!://", Err(InvalidScheme)),
            ("scheme://", Err(InvalidHost)),
            (
                "scheme://host",
                Ok(PrePath {
                    scheme_len: 6,
                    host_len: 4,
                    ip: None,
                    port: None,
                    port_len: 0,
                }),
            ),
            (
                "scheme://127.0.0.1",
                Ok(PrePath {
                    scheme_len: 6,
                    host_len: 9,
                    ip: Some(IPv4Address::LOCALHOST.to_ip()),
                    port: None,
                    port_len: 0,
                }),
            ),
            ("scheme://::1", Err(InvalidHost)),
            (
                "scheme://[::1]",
                Ok(PrePath {
                    scheme_len: 6,
                    host_len: 5,
                    ip: Some(IPv6Address::LOCALHOST.to_ip()),
                    port: None,
                    port_len: 0,
                }),
            ),
            (
                "scheme://[::1]:80",
                Ok(PrePath {
                    scheme_len: 6,
                    host_len: 5,
                    ip: Some(IPv6Address::LOCALHOST.to_ip()),
                    port: Some(80),
                    port_len: 3,
                }),
            ),
            (
                "scheme://[::1]:80/the/path",
                Ok(PrePath {
                    scheme_len: 6,
                    host_len: 5,
                    ip: Some(IPv6Address::LOCALHOST.to_ip()),
                    port: Some(80),
                    port_len: 3,
                }),
            ),
            (
                "scheme://host?query",
                Ok(PrePath {
                    scheme_len: 6,
                    host_len: 4,
                    ip: None,
                    port: None,
                    port_len: 0,
                }),
            ),
            (
                "scheme://host#frag",
                Ok(PrePath {
                    scheme_len: 6,
                    host_len: 4,
                    ip: None,
                    port: None,
                    port_len: 0,
                }),
            ),
            (
                "scheme://host:80?query",
                Ok(PrePath {
                    scheme_len: 6,
                    host_len: 4,
                    ip: None,
                    port: Some(80),
                    port_len: 3,
                }),
            ),
            ("scheme://?query", Err(InvalidHost)),
            ("scheme://#frag", Err(InvalidHost)),
            // Every form of user info reports the same error.
            ("scheme://user@host", Err(UserInfoNotSupported)),
            ("scheme://user:pass@host", Err(UserInfoNotSupported)),
            ("scheme://user:pass@host:80/p", Err(UserInfoNotSupported)),
            ("scheme://@host", Err(UserInfoNotSupported)),
            ("scheme://user@[::1]:80", Err(UserInfoNotSupported)),
            // An '@' char outside the authority is not user info.
            (
                "scheme://host/a@b",
                Ok(PrePath {
                    scheme_len: 6,
                    host_len: 4,
                    ip: None,
                    port: None,
                    port_len: 0,
                }),
            ),
        ];
        for (input, expected) in test_cases {
            let result: Result<PrePath, ParseError> = parse_pre_path(input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
