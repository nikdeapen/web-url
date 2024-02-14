use std::str::FromStr;

use address::{Domain, IPAddress, IPv4Address, IPv6Address};

use crate::ParseError;
use crate::ParseError::InvalidHost;

/// Extracts the host string from the prefix of `s`. Returns the host string and the rest of `s`.
/// This function does not validate that the host string is a valid host.
pub fn extract_host(s: &str) -> (&str, &str) {
    let host_and_port: &str = if let Some(slash) = s.as_bytes().iter().position(|c| *c == b'/') {
        &s[..slash]
    } else {
        s
    };
    if host_and_port.is_empty() {
        ("", s)
    } else if host_and_port.as_bytes()[0] == b'['
        && host_and_port.as_bytes()[host_and_port.len() - 1] == b']'
    {
        (host_and_port, &s[host_and_port.len()..])
    } else if let Some(last_colon) = host_and_port.as_bytes().iter().rposition(|c| *c == b':') {
        let pre_colon: &str = &s[..last_colon];
        (pre_colon, &s[pre_colon.len()..])
    } else {
        (host_and_port, &s[host_and_port.len()..])
    }
}

/// Extracts the optional IP address from the `host` string. Returns the optional IP. If the host
/// is not an IP address the domain will be validated (case-insensitively).
pub fn extract_ip(host: &str) -> Result<Option<IPAddress>, ParseError> {
    if host.is_empty() {
        Err(InvalidHost)
    } else if host.as_bytes()[0] == b'[' {
        if host.as_bytes()[host.len() - 1] != b']' {
            Err(InvalidHost)
        } else {
            let host: &str = &host[1..(host.len() - 1)];
            let ip: IPv6Address = IPv6Address::from_str(host).map_err(|_| InvalidHost)?;
            Ok(Some(ip.to_ip()))
        }
    } else if let Ok(ip) = IPv4Address::from_str(host) {
        Ok(Some(ip.to_ip()))
    } else if Domain::is_valid_name_str(host, true) {
        Ok(None)
    } else {
        Err(InvalidHost)
    }
}

#[cfg(test)]
mod tests {
    use address::{IPAddress, IPv4Address, IPv6Address};

    use crate::ParseError::InvalidHost;
    use crate::{extract_host, extract_ip, ParseError};

    #[test]
    fn fn_extract_host() {
        let test_cases: &[(&str, (&str, &str))] = &[
            ("", ("", "")),
            ("host", ("host", "")),
            ("host/", ("host", "/")),
            ("host/rest", ("host", "/rest")),
            ("host:port/rest", ("host", ":port/rest")),
            ("[host:port/rest", ("[host", ":port/rest")),
            ("[host:port]/rest", ("[host:port]", "/rest")),
            ("host:", ("host", ":")),
        ];
        for (s, expected) in test_cases {
            let result: (&str, &str) = extract_host(*s);
            assert_eq!(result, *expected, "s={}", *s);
        }
    }

    #[test]
    fn fn_extract_ip() {
        let test_cases: &[(&str, Result<Option<IPAddress>, ParseError>)] = &[
            ("", Err(InvalidHost)),
            ("[::1", Err(InvalidHost)),
            ("[127.0.0.1]", Err(InvalidHost)),
            ("[::1]", Ok(Some(IPv6Address::LOCALHOST.to_ip()))),
            ("!", Err(InvalidHost)),
            ("127.0.0.1", Ok(Some(IPv4Address::LOCALHOST.to_ip()))),
            ("localhost", Ok(None)),
            ("LocalHost", Ok(None)),
            ("Local!Host", Err(InvalidHost)),
        ];
        for (host, expected) in test_cases {
            let result: Result<Option<IPAddress>, ParseError> = extract_ip(*host);
            assert_eq!(result, *expected, "host={}", *host);
        }
    }
}
