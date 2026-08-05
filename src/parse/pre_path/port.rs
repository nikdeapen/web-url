use std::str::FromStr;

use crate::parse::pre_path::is_authority_end;
use crate::parse::Error;
use crate::parse::Error::InvalidPort;

/// Parses the port from the prefix of `s`.
///
/// The string `s` should start with a `:` if there is a port.
///
/// # RFC 3986
/// The port is `*DIGIT` so it may be empty. An empty port means the default port for the scheme so
/// it is parsed as if there were no port at all. The ':' is still consumed, which makes an empty
/// port detectable as `Ok(None, _)` with a non-zero consumed length.
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.3>
///
/// Returns `Ok(Some(port), rest_of_s)`.
/// Returns `Ok(None, rest_of_s)` if `s` does not start with a `:` or the port is empty.
/// Returns `Err(InvalidPort)` if the port is invalid.
#[allow(clippy::type_complexity)]
pub fn parse_port(s: &str) -> Result<(Option<u16>, &str), Error> {
    if !s.is_empty() && s.as_bytes()[0] == b':' {
        let s: &str = &s[1..];
        let end: usize = s
            .as_bytes()
            .iter()
            .position(|c| is_authority_end(*c))
            .unwrap_or(s.len());
        let (digits, rest) = s.split_at(end);

        // The `u16::from_str` fn accepts a leading '+' char which the RFC does not allow, so the
        // digits are validated explicitly before the port is parsed.
        if !digits.as_bytes().iter().all(|c| c.is_ascii_digit()) {
            Err(InvalidPort)
        } else if digits.is_empty() {
            Ok((None, rest))
        } else {
            let port: u16 = u16::from_str(digits).map_err(|_| InvalidPort)?;
            Ok((Some(port), rest))
        }
    } else {
        Ok((None, s))
    }
}

/// Gets the number of decimal digits in the `port`.
pub const fn port_decimal_len(port: u16) -> usize {
    if port < 10 {
        1
    } else if port < 100 {
        2
    } else if port < 1_000 {
        3
    } else if port < 10_000 {
        4
    } else {
        5
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::pre_path::{parse_port, port_decimal_len};
    use crate::parse::Error;
    use crate::parse::Error::InvalidPort;

    #[test]
    #[allow(clippy::type_complexity)]
    fn fn_parse_port() {
        let test_cases: &[(&str, Result<(Option<u16>, &str), Error>)] = &[
            ("", Ok((None, ""))),
            ("anything", Ok((None, "anything"))),
            (":invalid", Err(InvalidPort)),
            (":invalid/", Err(InvalidPort)),
            (":80", Ok((Some(80), ""))),
            (":80/", Ok((Some(80), "/"))),
            (":80/p", Ok((Some(80), "/p"))),
            (":80?", Ok((Some(80), "?"))),
            (":80?q", Ok((Some(80), "?q"))),
            (":80#", Ok((Some(80), "#"))),
            (":80#f", Ok((Some(80), "#f"))),
            (":80 ", Err(InvalidPort)),
            // An empty port is valid & means the default port. The ':' is still consumed.
            (":", Ok((None, ""))),
            (":/", Ok((None, "/"))),
            (":?q", Ok((None, "?q"))),
            (":#f", Ok((None, "#f"))),
            // The RFC port is `*DIGIT` so a sign is not allowed even though `u16::from_str` takes
            // a leading '+' char.
            (":+80", Err(InvalidPort)),
            (":+80/", Err(InvalidPort)),
            (":-80", Err(InvalidPort)),
            (":+0", Err(InvalidPort)),
            // Leading zeros are valid & are stripped when the URL is normalized.
            (":0080", Ok((Some(80), ""))),
            (":0080/p", Ok((Some(80), "/p"))),
            (":0", Ok((Some(0), ""))),
            (":0000", Ok((Some(0), ""))),
            (":00065535", Ok((Some(65535), ""))),
            // The port must still fit in a u16.
            (":65535", Ok((Some(65535), ""))),
            (":65536", Err(InvalidPort)),
            (":99999", Err(InvalidPort)),
        ];
        for (s, expected) in test_cases {
            let result: Result<(Option<u16>, &str), Error> = parse_port(s);
            assert_eq!(result, *expected, "s={}", s);
        }
    }

    #[test]
    fn fn_port_decimal_len() {
        let test_cases: &[(u16, usize)] = &[
            (0, 1),
            (9, 1),
            (10, 2),
            (99, 2),
            (100, 3),
            (999, 3),
            (1_000, 4),
            (9_999, 4),
            (10_000, 5),
            (65_535, 5),
        ];
        for (port, expected) in test_cases {
            let result: usize = port_decimal_len(*port);
            assert_eq!(result, *expected, "port={}", port);

            // The length must match the rendered port exactly; it sizes the URL allocation.
            assert_eq!(result, port.to_string().len(), "port={}", port);
        }
    }
}
