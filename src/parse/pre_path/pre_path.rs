use crate::parse::{
    check_no_user_info, parse_host, parse_ip_and_validate_domain, parse_port, parse_scheme_len, port_decimal_len,
};
use crate::Error;
use address::IPAddress;

/// The parsing data for a web-based URL before the path.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PrePath {
    pub scheme_len: usize,
    pub host_len: usize,
    pub ip: Option<IPAddress>,
    pub port: Option<u16>,
    pub port_len: usize,
}

impl PrePath {
    //! Properties

    /// Gets the length of the pre-path string as it appears in the parsed URL.
    pub const fn len(self) -> usize {
        self.scheme_len + 3 + self.host_len + self.port_len
    }

    /// Gets the index just past the host. (the index of the ':' when there is a port)
    ///
    /// This is the same in the parsed URL & the normalized URL since only the port is rewritten.
    pub const fn host_end(self) -> usize {
        self.scheme_len + 3 + self.host_len
    }

    /// Gets the length of the port string in the normalized URL. (including the ':')
    ///
    /// This is 0 when there is no port. It is shorter than `port_len` when the parsed port was
    /// empty or had leading zeros, and equal to it otherwise.
    pub const fn canonical_port_len(self) -> usize {
        match self.port {
            Some(port) => 1 + port_decimal_len(port),
            None => 0,
        }
    }

    /// Gets the length of the pre-path string in the normalized URL.
    pub const fn canonical_len(self) -> usize {
        self.host_end() + self.canonical_port_len()
    }
}

impl PrePath {
    //! Operations

    /// Makes the scheme & host prefix of `url` lowercase.
    ///
    /// The port is excluded since it is all digits and unaffected by the letter case.
    ///
    /// # Panics
    /// Panics if `host_end` is past the end of `url` or is not a char boundary. Neither happens
    /// when `url` was parsed with the `parse_pre_path` function.
    pub fn make_lowercase(self, url: &mut str) {
        url[..self.host_end()].make_ascii_lowercase()
    }
}

/// Parses the pre-path portion of the URL.
/// The scheme & host will be validated but may be uppercase.
pub fn parse_pre_path(url: &str) -> Result<PrePath, Error> {
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
    use crate::parse::{parse_pre_path, PrePath};
    use crate::Error;
    use crate::Error::{InvalidHost, InvalidScheme, UserInfoNotSupported};
    use address::{IPv4Address, IPv6Address};

    #[test]
    fn fn_parse_pre_path() {
        let test_cases: &[(&str, Result<PrePath, Error>)] = &[
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
            let result: Result<PrePath, Error> = parse_pre_path(input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
