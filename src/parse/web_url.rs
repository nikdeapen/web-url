use crate::parse::{Parts, finalize_web_url, parse_parts, write_normalized};
use crate::{InvalidUrlError, ParseError, WebUrl};
use std::str::FromStr;

impl FromStr for WebUrl {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Parts = parse_parts(s)?;

        // The URL is validated before it is allocated, so invalid input never allocates & the
        // normalized length is known exactly.
        let url: String = if parts.is_normalized() {
            // The string is already normalized so it is copied whole rather than rebuilt.
            String::from(s)
        } else {
            let mut url: String = String::with_capacity(parts.normalized_len(s.len()));
            write_normalized(s, parts, &mut url);
            url
        };

        Ok(unsafe { finalize_web_url(url, parts) })
    }
}

impl TryFrom<String> for WebUrl {
    type Error = InvalidUrlError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let parts: Parts = match parse_parts(s.as_str()) {
            Ok(parts) => parts,
            Err(error) => return Err(InvalidUrlError::new(error, s)),
        };

        // The URL is validated before it is modified so the string is returned unchanged on error.
        let url: String = if parts.is_normalized() {
            // The string is already normalized so it is reused without allocating.
            s
        } else if !parts.needs_rewrite() {
            // Only the '/' is missing. The reservation is exact so the string grows at most once.
            let mut url: String = s;
            url.reserve_exact(1);
            url.insert(parts.slash_index(), '/');
            url
        } else {
            // The host or port changes length once normalized so the string is rebuilt with a
            // single exactly-sized allocation.
            let mut url: String = String::with_capacity(parts.normalized_len(s.len()));
            write_normalized(s.as_str(), parts, &mut url);
            url
        };

        Ok(unsafe { finalize_web_url(url, parts) })
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{
        InvalidFragment, InvalidHost, InvalidPath, InvalidPort, InvalidQuery, InvalidScheme,
        UserInfoNotSupported,
    };
    use crate::{InvalidUrlError, ParseError, WebUrl};
    use std::str::FromStr;

    type TestCase<'a> = (&'a str, Result<&'a str, ParseError>);

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            // A normalized URL parses unchanged.
            ("http://host/", Ok("http://host/")),
            ("http://host/p?q#f", Ok("http://host/p?q#f")),
            // The scheme & host are lowercased; the path, query, & fragment keep their case.
            ("HTTP://HOST/P?Q#F", Ok("http://host/P?Q#F")),
            ("HTTP://HOST", Ok("http://host/")),
            ("HTTP://HOST:0080/P", Ok("http://host:80/P")),
            // A URL parsed without a path gets the path '/'.
            ("http://host", Ok("http://host/")),
            ("http://host?q", Ok("http://host/?q")),
            ("http://host#f", Ok("http://host/#f")),
            ("http://host?q#f", Ok("http://host/?q#f")),
            ("http://host:80", Ok("http://host:80/")),
            // An empty port is dropped along with its ':' & the leading zeros are stripped.
            ("http://host:/p", Ok("http://host/p")),
            ("http://host:?q", Ok("http://host/?q")),
            ("http://host:0080/p", Ok("http://host:80/p")),
            ("http://host:0/p", Ok("http://host:0/p")),
            // An IP address host is rewritten in its canonical form.
            ("http://[0:0:0:0:0:0:0:1]/p", Ok("http://[::1]/p")),
            ("http://[::FFFF:1.2.3.4]/", Ok("http://[::ffff:1.2.3.4]/")),
            // The path dot-segments are removed.
            ("http://host/a/../b", Ok("http://host/b")),
            ("http://host/a/./b/", Ok("http://host/a/b/")),
            // The error names the most specific part that can be blamed.
            ("", Err(InvalidScheme)),
            ("no-scheme", Err(InvalidScheme)),
            ("not a url", Err(InvalidScheme)),
            ("http:/host", Err(InvalidScheme)),
            ("http://user:pass@host/", Err(UserInfoNotSupported)),
            ("http://", Err(InvalidHost)),
            ("http://ho st/", Err(InvalidHost)),
            ("http://[::1/", Err(InvalidHost)),
            ("http://host:x/", Err(InvalidPort)),
            ("http://host:bad", Err(InvalidPort)),
            ("http://host:65536/", Err(InvalidPort)),
            ("http://host/p q", Err(InvalidPath)),
            ("http://host/p?q q", Err(InvalidQuery)),
            ("http://host/p#f f", Err(InvalidFragment)),
        ];

        for (input, expected) in test_cases {
            let url: Result<WebUrl, ParseError> = WebUrl::from_str(input);
            let result: Result<&str, ParseError> =
                url.as_ref().map(WebUrl::as_str).map_err(|error| *error);
            assert_eq!(result, *expected, "from_str input={}", input);

            let url: Result<WebUrl, ParseError> = WebUrl::try_from(*input);
            let result: Result<&str, ParseError> =
                url.as_ref().map(WebUrl::as_str).map_err(|error| *error);
            assert_eq!(result, *expected, "try_from(&str) input={}", input);

            let url: Result<WebUrl, InvalidUrlError> = WebUrl::try_from(input.to_string());
            let result: Result<&str, ParseError> =
                url.as_ref().map(WebUrl::as_str).map_err(|error| {
                    assert_eq!(error.url(), *input, "recovered input={}", input);
                    error.error()
                });
            assert_eq!(result, *expected, "try_from(String) input={}", input);
        }
    }

    #[test]
    fn round_trip() {
        let canonical: &[&str] = &[
            "http://host/",
            "http://host//",
            "http://host/p?q#f",
            "http://host:8080/a/b?x=1&y=2#z",
            "http://host:0/p",
            "http://127.0.0.1/p",
            "http://[::1]:80/p",
            "s://host/?#",
        ];

        for input in canonical {
            let value: WebUrl = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
