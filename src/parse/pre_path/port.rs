use crate::ParseError;
use crate::ParseError::InvalidPort;

/// Parses the optional port from the `port` string, which is empty or starts with the ':'.
///
/// # RFC 3986
/// The port is `*DIGIT` so it may be empty. An empty port means the default port for the scheme so
/// it is parsed as if there were no port at all.
/// <https://www.rfc-editor.org/rfc/rfc3986#section-3.2.3>
pub(crate) fn parse_port(port: &str) -> Result<Option<u16>, ParseError> {
    let Some(digits) = port.strip_prefix(':') else {
        return Ok(None);
    };
    if digits.is_empty() {
        return Ok(None);
    }

    let mut value: u16 = 0;
    for c in digits.bytes() {
        if !c.is_ascii_digit() {
            return Err(InvalidPort);
        }
        value = value
            .checked_mul(10)
            .and_then(|value| value.checked_add(u16::from(c - b'0')))
            .ok_or(InvalidPort)?;
    }
    Ok(Some(value))
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
    use crate::parse::{self, parse_port};

    type TestCase<'a> = (&'a str, Result<Option<u16>, ParseError>);

    #[test]
    fn port() {
        let test_cases: &[TestCase] = &[
            ("", Ok(None)),
            (":invalid", Err(InvalidPort)),
            (":80", Ok(Some(80))),
            (":80 ", Err(InvalidPort)),
            // An empty port is valid & means the default port.
            (":", Ok(None)),
            // The RFC port is `*DIGIT` so a sign is not allowed.
            (":+80", Err(InvalidPort)),
            (":-80", Err(InvalidPort)),
            (":+0", Err(InvalidPort)),
            // Leading zeros are valid & are stripped when the URL is normalized.
            (":0080", Ok(Some(80))),
            (":0", Ok(Some(0))),
            (":0000", Ok(Some(0))),
            (":00065535", Ok(Some(65535))),
            // The port must still fit in a u16.
            (":65535", Ok(Some(65535))),
            (":65536", Err(InvalidPort)),
            (":99999", Err(InvalidPort)),
        ];
        for (port, expected) in test_cases {
            let result: Result<Option<u16>, ParseError> = parse_port(port);
            assert_eq!(result, *expected, "port={}", port);
        }
    }

    #[test]
    fn port_decimal_len() {
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
            let result: usize = parse::port_decimal_len(*port);
            assert_eq!(result, *expected, "port={}", port);

            // The length must match the rendered port exactly; it sizes the URL allocation.
            assert_eq!(result, port.to_string().len(), "port={}", port);
        }
    }
}
