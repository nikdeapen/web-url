use std::str::FromStr;

use crate::parse::Error;
use crate::parse::Error::InvalidPort;

/// Parses the port from the prefix of `s`.
///
/// The string `s` should start with a `:` if there is a port.
///
/// Returns `Ok(Some(port), rest_of_s)`.
/// Returns `Ok(None, rest_of_s)` if `s` does not start with a `:`.
/// Returns `Err(InvalidPort)` if the port is invalid.
#[allow(clippy::type_complexity)]
pub fn parse_port(s: &str) -> Result<(Option<u16>, &str), Error> {
    if !s.is_empty() && s.as_bytes()[0] == b':' {
        let s: &str = &s[1..];
        if let Some(slash) = s.as_bytes().iter().position(|c| *c == b'/') {
            let port: u16 = u16::from_str(&s[..slash]).map_err(|_| InvalidPort)?;
            Ok((Some(port), &s[slash..]))
        } else {
            let port: u16 = u16::from_str(s).map_err(|_| InvalidPort)?;
            Ok((Some(port), ""))
        }
    } else {
        Ok((None, s))
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::pre_path::parse_port;
    use crate::parse::Error;
    use crate::parse::Error::InvalidPort;

    #[test]
    #[allow(clippy::type_complexity)]
    fn fn_extract_port() {
        let test_cases: &[(&str, Result<(Option<u16>, &str), Error>)] = &[
            ("", Ok((None, ""))),
            ("anything", Ok((None, "anything"))),
            (":invalid", Err(InvalidPort)),
            (":invalid/", Err(InvalidPort)),
            (":80", Ok((Some(80), ""))),
            (":80/", Ok((Some(80), "/"))),
            (":80?", Err(InvalidPort)),
            (":80#", Err(InvalidPort)),
            (":80 ", Err(InvalidPort)),
        ];
        for (s, expected) in test_cases {
            let result: Result<(Option<u16>, &str), Error> = parse_port(s);
            assert_eq!(result, *expected, "s={}", s);
        }
    }
}
