use crate::Error;
use crate::WebUrl;
use crate::parse::{Parts, finalize_web_url, parse_parts, write_normalized};
use std::str::FromStr;

impl FromStr for WebUrl {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Parts = parse_parts(s)?;

        // The URL is validated before it is allocated, so invalid input never allocates & the normalized length is
        // known exactly.
        let mut url: String = String::with_capacity(parts.normalized_len(s.len()));
        write_normalized(s, parts, &mut url);

        unsafe { finalize_web_url(url, parts.pre_path, parts.path_plus) }
            .map_err(|(error, _)| error)
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::Error::{
        InvalidFragment, InvalidHost, InvalidPath, InvalidPort, InvalidQuery, InvalidScheme,
        UserInfoNotSupported,
    };
    use crate::WebUrl;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let test_cases: &[(&str, Result<&str, Error>)] = &[
            // A normalized URL parses unchanged.
            ("http://host/", Ok("http://host/")),
            ("http://host/p?q#f", Ok("http://host/p?q#f")),
            // The scheme & host are lowercased; the path, query, & fragment keep their case.
            ("HTTP://HOST/P?Q#F", Ok("http://host/P?Q#F")),
            // A URL parsed without a path gets the path '/'.
            ("http://host", Ok("http://host/")),
            ("http://host?q", Ok("http://host/?q")),
            ("http://host#f", Ok("http://host/#f")),
            ("http://host:80", Ok("http://host:80/")),
            // An empty port is dropped along with its ':' & the leading zeros are stripped.
            ("http://host:/p", Ok("http://host/p")),
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
            ("http:/host", Err(InvalidScheme)),
            ("http://user:pass@host/", Err(UserInfoNotSupported)),
            ("http://", Err(InvalidHost)),
            ("http://ho st/", Err(InvalidHost)),
            ("http://[::1/", Err(InvalidHost)),
            ("http://host:x/", Err(InvalidPort)),
            ("http://host:65536/", Err(InvalidPort)),
            ("http://host/p q", Err(InvalidPath)),
            ("http://host/p?q q", Err(InvalidQuery)),
            ("http://host/p#f f", Err(InvalidFragment)),
        ];
        for (input, expected) in test_cases {
            let result: Result<WebUrl, Error> = WebUrl::from_str(input);
            match expected {
                Ok(expected) => assert_eq!(result.unwrap().as_str(), *expected, "input={}", input),
                Err(expected) => assert_eq!(result.unwrap_err(), *expected, "input={}", input),
            }
        }
    }

    /// The normalized URL is the canonical form, so parsing it again must produce it byte for byte.
    #[test]
    fn from_str_round_trip() {
        let test_cases: &[&str] = &[
            "http://host/",
            "http://host//",
            "http://host/p?q#f",
            "http://host:8080/a/b?x=1&y=2#z",
            "http://host:0/p",
            "http://127.0.0.1/p",
            "http://[::1]:80/p",
            "s://host/?#",
        ];
        for url in test_cases {
            let parsed: WebUrl = WebUrl::from_str(url).unwrap();
            assert_eq!(parsed.as_str(), *url, "url={}", url);

            let again: WebUrl = WebUrl::from_str(parsed.as_str()).unwrap();
            assert_eq!(again.as_str(), *url, "url={}", url);
        }
    }
}
