use crate::ParseError;
use crate::ParseError::InvalidPort;
use std::str::FromStr;

/// Extracts the port from the prefix of `s`. Returns the optional port if `s` starts with a ':'
/// and the rest of `s` after the port.
pub fn extract_port(s: &str) -> Result<(Option<u16>, &str), ParseError> {
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
    use crate::ParseError::InvalidPort;
    use crate::{extract_port, ParseError};

    #[test]
    fn fn_extract_port() {
        let test_cases: &[(&str, Result<(Option<u16>, &str), ParseError>)] = &[
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
            let result: Result<(Option<u16>, &str), ParseError> = extract_port(*s);
            assert_eq!(result, *expected, "s={}", *s);
        }
    }
}
