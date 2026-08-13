use crate::parse::{finalize_web_url, parse_parts, write_normalized, Parts};
use crate::InvalidUrlString;
use crate::WebUrl;

impl TryFrom<String> for WebUrl {
    type Error = InvalidUrlString;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let parts: Parts = match parse_parts(s.as_str()) {
            Ok(parts) => parts,
            Err(error) => return Err(InvalidUrlString::new(error, s)),
        };

        // The URL is validated before it is modified so the string is returned unchanged on error.
        let url: String = if parts.is_normalized() {
            // The string is already normalized so it is reused without allocating.
            s
        } else if !parts.needs_port_rewrite() {
            // Only the '/' is missing. The reservation is exact so the string grows at most once.
            let mut url: String = s;
            url.reserve_exact(1);
            url.insert(parts.slash_index(), '/');
            url
        } else {
            // The port is shorter once normalized so the string is rebuilt with a single
            // exactly-sized allocation.
            let mut url: String = String::with_capacity(parts.normalized_len(s.len()));
            write_normalized(s.as_str(), &parts, &mut url);
            url
        };

        unsafe { finalize_web_url(url, parts.pre_path, parts.path_plus) }
            .map_err(|(error, url)| InvalidUrlString::new(error, url))
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::Error::{InvalidPort, InvalidScheme};
    use crate::InvalidUrlString;
    use crate::WebUrl;

    #[test]
    fn try_from_string() {
        let test_cases: &[(&str, Result<&str, Error>)] = &[
            // Already normalized: the string is reused.
            ("http://host/p?q#f", Ok("http://host/p?q#f")),
            // Only the '/' is missing: it is inserted in place.
            ("http://host", Ok("http://host/")),
            ("http://host?q#f", Ok("http://host/?q#f")),
            // The port is rewritten: the URL is rebuilt.
            ("http://host:0080/p", Ok("http://host:80/p")),
            ("http://host:?q", Ok("http://host/?q")),
            // The letter case is normalized in every branch.
            ("HTTP://HOST", Ok("http://host/")),
            ("HTTP://HOST:0080/P", Ok("http://host:80/P")),
            // Errors return the original string unchanged.
            ("not a url", Err(InvalidScheme)),
            ("http://host:bad", Err(InvalidPort)),
        ];
        for (input, expected) in test_cases {
            let result: Result<WebUrl, InvalidUrlString> = WebUrl::try_from(String::from(*input));
            match expected {
                Ok(expected) => {
                    assert_eq!(result.unwrap().as_str(), *expected, "input={input}");
                }
                Err(expected) => {
                    let error: InvalidUrlString = result.unwrap_err();
                    assert_eq!(error.error(), *expected, "input={input}");
                    assert_eq!(error.url(), *input, "input={input}");
                    assert_eq!(error.into_url(), *input, "input={input}");
                }
            }
        }
    }
}
