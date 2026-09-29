use crate::ParseError;
use crate::ParseError::InvalidHost;
use crate::parse::is_authority_end;
use address::{Domain, IPAddress, IPv4Address, IPv6Address};
use std::str::FromStr;

/// Parses the host string from the prefix of `s`.
///
/// The host will **not** be validated.
pub(crate) fn parse_host(s: &str) -> (&str, &str) {
    let end: usize = s
        .as_bytes()
        .iter()
        .position(|c| is_authority_end(*c))
        .unwrap_or(s.len());
    let host_and_port: &str = &s[..end];
    let bracketed: bool = host_and_port.starts_with('[') && host_and_port.ends_with(']');
    let host_len: usize = match host_and_port.as_bytes().iter().rposition(|c| *c == b':') {
        Some(colon) if !bracketed => colon,
        _ => end,
    };
    s.split_at(host_len)
}

/// Parses the optional IP address from the `host` string. If the host is not an IP address the
/// domain will be validated (case-insensitively).
pub(crate) fn parse_ip_and_validate_domain(host: &str) -> Result<Option<IPAddress>, ParseError> {
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
    use crate::parse::{parse_host, parse_ip_and_validate_domain};
    use address::{IPAddress, IPv4Address, IPv6Address};

    #[test]
    fn host() {
        let test_cases: &[(&str, (&str, &str))] = &[
            ("", ("", "")),
            ("host", ("host", "")),
            ("host/", ("host", "/")),
            ("host/rest", ("host", "/rest")),
            ("host:port/rest", ("host", ":port/rest")),
            ("[host:port/rest", ("[host", ":port/rest")),
            ("[host:port]/rest", ("[host:port]", "/rest")),
            ("[host:port]", ("[host:port]", "")),
            ("[host:port]80", ("[host", ":port]80")),
            ("host:", ("host", ":")),
            ("host?query", ("host", "?query")),
            ("host#frag", ("host", "#frag")),
            ("host?", ("host", "?")),
            ("host#", ("host", "#")),
            ("host:80?query", ("host", ":80?query")),
            ("host:80#frag", ("host", ":80#frag")),
            ("[::1]?query", ("[::1]", "?query")),
            ("[::1]#frag", ("[::1]", "#frag")),
            ("[::1]:80?query", ("[::1]", ":80?query")),
            ("?query", ("", "?query")),
            ("#frag", ("", "#frag")),
        ];
        for (s, expected) in test_cases {
            let result: (&str, &str) = parse_host(s);
            assert_eq!(result, *expected, "s={}", s);
        }
    }

    #[test]
    fn ip_and_validate_domain() {
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
            let result: Result<Option<IPAddress>, ParseError> = parse_ip_and_validate_domain(host);
            assert_eq!(result, *expected, "host={}", host);
        }
    }
}
