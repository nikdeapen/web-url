use address::IPAddress;

use crate::parse::pre_path::parse_scheme_len;
use crate::parse::pre_path::{parse_host, parse_ip_and_validate_domain, parse_port};
use crate::parse::Error;

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

    /// Gets the length of the pre-path string.
    pub fn len(&self) -> usize {
        self.scheme_len + 3 + self.host_len + self.port_len
    }

    /// Checks if the pre-path string is empty.
    pub fn is_empty(&self) -> bool {
        false
    }
}

impl PrePath {
    //! Operations

    /// Makes the pre-path prefix of `s` lowercase.
    ///
    /// # Safety
    /// This requires the string up to `len` to be US-ASCII. This will be true if it was parsed
    /// with the `parse_pre_path` function.
    pub unsafe fn make_lowercase(&self, url: &mut str) {
        url[..self.len()]
            .as_bytes_mut()
            .iter_mut()
            .for_each(|c| *c = c.to_ascii_lowercase())
    }
}

/// Parses the pre-path portion of the URL.
/// The scheme & host will be validated but may be uppercase.
///
/// Returns `Ok(pre_path)`.
/// Returns `Err(_)` if any part of the pre-path is invalid.
pub fn parse_pre_path(url: &str) -> Result<PrePath, Error> {
    let (scheme_len, after_scheme) = parse_scheme_len(url)?;
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
    use address::{IPv4Address, IPv6Address};

    use crate::parse::pre_path::{parse_pre_path, PrePath};
    use crate::parse::Error;
    use crate::parse::Error::{InvalidHost, InvalidScheme};

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
        ];
        for (input, expected) in test_cases {
            let result: Result<PrePath, Error> = parse_pre_path(input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
