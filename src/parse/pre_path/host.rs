use crate::ParseError;
use crate::ParseError::InvalidHost;
use address::{Domain, IPAddress, IPv4Address, IPv6Address};
use std::str::FromStr;

/// Splits the `authority` into the host string & the port string, which is empty or starts with
/// the ':'.
///
/// The host will **not** be validated.
pub(crate) fn split_authority(authority: &str) -> (&str, &str) {
    // A bracketed host is an IPv6 literal, whose own ':' chars are not the port separator.
    let bracketed: bool = authority.starts_with('[') && authority.ends_with(']');
    let host_len: usize = match authority.as_bytes().iter().rposition(|c| *c == b':') {
        Some(colon) if !bracketed => colon,
        _ => authority.len(),
    };
    authority.split_at(host_len)
}

/// Parses the optional IP address from the `host` string. If the host is not an IP address the
/// domain will be validated (case-insensitively).
pub(crate) fn parse_host(host: &str) -> Result<Option<IPAddress>, ParseError> {
    if let Some(ip) = host.strip_prefix('[') {
        let ip: &str = ip.strip_suffix(']').ok_or(InvalidHost)?;
        let ip: IPv6Address = IPv6Address::from_str(ip).map_err(|_| InvalidHost)?;
        Ok(Some(ip.to_ip()))
    } else if let Ok(ip) = IPv4Address::from_str(host) {
        Ok(Some(ip.to_ip()))
    } else if Domain::is_valid_name_ignore_case_str(host) {
        Ok(None)
    } else {
        Err(InvalidHost)
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError;
    use crate::ParseError::InvalidHost;
    use crate::parse::{parse_host, split_authority};
    use address::{IPAddress, IPv4Address, IPv6Address};

    #[test]
    fn authority() {
        let test_cases: &[(&str, (&str, &str))] = &[
            ("", ("", "")),
            ("host", ("host", "")),
            ("host:", ("host", ":")),
            ("host:80", ("host", ":80")),
            ("host:port", ("host", ":port")),
            // The port is split at the last ':' char.
            ("a:b:80", ("a:b", ":80")),
            // The ':' chars of a bracketed host are not port separators.
            ("[::1]", ("[::1]", "")),
            ("[::1]:", ("[::1]", ":")),
            ("[::1]:80", ("[::1]", ":80")),
            ("[host:port]", ("[host:port]", "")),
            ("[host:port", ("[host", ":port")),
            ("[host:port]80", ("[host", ":port]80")),
        ];
        for (authority, expected) in test_cases {
            let result: (&str, &str) = split_authority(authority);
            assert_eq!(result, *expected, "authority={}", authority);
        }
    }

    #[test]
    fn host() {
        let test_cases: &[(&str, Result<Option<IPAddress>, ParseError>)] = &[
            ("", Err(InvalidHost)),
            ("[::1", Err(InvalidHost)),
            ("[127.0.0.1]", Err(InvalidHost)),
            ("[::1]", Ok(Some(IPv6Address::LOCALHOST.to_ip()))),
            ("!", Err(InvalidHost)),
            ("127.0.0.1", Ok(Some(IPv4Address::LOCALHOST.to_ip()))),
            ("localhost", Ok(None)),
            // The final label of a domain name cannot be all-numeric, which requires `address` >=
            // 0.21.
            ("256.0.0.1", Err(InvalidHost)),
            ("999.1.1.1", Err(InvalidHost)),
            ("9999.com", Ok(None)),
            ("LocalHost", Ok(None)),
            ("Local!Host", Err(InvalidHost)),
            // The `xn--` ACE prefix has consecutive hyphens, which requires `address` >= 0.19.
            ("xn--bcher-kva.example", Ok(None)),
        ];
        for (host, expected) in test_cases {
            let result: Result<Option<IPAddress>, ParseError> = parse_host(host);
            assert_eq!(result, *expected, "host={}", host);
        }
    }
}
