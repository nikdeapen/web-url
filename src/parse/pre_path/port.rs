use crate::ParseError;
use crate::ParseError::InvalidPort;
use crate::parse::is_authority_end;

/// Parses the port from the prefix of `s`.
///
/// The string `s` should start with a ':' if there is a port.
///
/// # RFC 3986
/// The port is `*DIGIT` so it may be empty. An empty port means the default port for the scheme so
/// it is parsed as if there were no port at all. The ':' is still consumed, which makes an empty
/// port detectable as `Ok((None, _))` with a non-zero consumed length.
/// <https://www.rfc-editor.org/rfc/rfc3986#section-3.2.3>
pub(crate) fn parse_port(s: &str) -> Result<(Option<u16>, &str), ParseError> {
    let Some(text) = s.strip_prefix(':') else {
        return Ok((None, s));
    };
    let end: usize = text
        .as_bytes()
        .iter()
        .position(|c| is_authority_end(*c))
        .unwrap_or(text.len());
    let (digits, rest) = text.split_at(end);
    if digits.is_empty() {
        return Ok((None, rest));
    }

    let mut port: u16 = 0;
    for c in digits.bytes() {
        if !c.is_ascii_digit() {
            return Err(InvalidPort);
        }
        port = port
            .checked_mul(10)
            .and_then(|port| port.checked_add(u16::from(c - b'0')))
            .ok_or(InvalidPort)?;
    }
    Ok((Some(port), rest))
}

/// Gets the number of decimal digits in the `port`.
pub(crate) const fn port_decimal_len(port: u16) -> usize {
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
    use crate::ParseError;
    use crate::ParseError::InvalidPort;
    use crate::parse::{parse_port, port_decimal_len};

    type TestCase<'a> = (&'a str, Result<(Option<u16>, &'a str), ParseError>);

    #[test]
    fn port() {
        let test_cases: &[TestCase] = &[
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
            // The RFC port is `*DIGIT` so a sign is not allowed.
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
            let result: Result<(Option<u16>, &str), ParseError> = parse_port(s);
            assert_eq!(result, *expected, "s={}", s);
        }
    }

    #[test]
    fn decimal_len() {
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
