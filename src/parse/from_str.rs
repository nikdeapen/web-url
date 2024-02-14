use std::str::FromStr;

use address::IPAddress;

use crate::{
    extract_fragment, extract_host, extract_ip, extract_path, extract_port, extract_query,
    extract_scheme_len, ParseError, WebUrl,
};

/// Finalizes parsing the string.
unsafe fn finalize_string(
    url: String,
    scheme_len: usize,
    host_len: usize,
    ip: Option<IPAddress>,
    port_len: usize,
    port: Option<u16>,
    path_len: usize,
    query_len: usize,
) -> WebUrl {
    // todo!(); // make url up to path lowercase

    let scheme_len: u32 = scheme_len as u32;
    let host_end: u32 = scheme_len + 3 + (host_len as u32);
    let port_end: u32 = host_end + (port_len as u32);
    let path_end: u32 = port_end + (path_len as u32);
    let query_end: u32 = path_end + (query_len as u32);
    unsafe {
        WebUrl::new_unchecked(
            url, scheme_len, host_end, ip, port_end, port, path_end, query_end,
        )
    }
}

impl FromStr for WebUrl {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (scheme_len, after_scheme) = extract_scheme_len(s)?;
        let (host_str, after_host) = extract_host(after_scheme);
        let ip: Option<IPAddress> = extract_ip(host_str)?;
        let (port, after_port) = extract_port(after_host)?;
        let port_len: usize = after_host.len() - after_port.len();
        if after_port.is_empty() {
            let mut url: String = s.to_string();
            url.push('/');
            Ok(unsafe {
                finalize_string(url, scheme_len, host_str.len(), ip, port_len, port, 1, 0)
            })
        } else {
            let (path, after_path) = extract_path(after_port)?;
            let (query, after_query) = extract_query(after_path)?;
            extract_fragment(after_query)?;
            let query_len: usize = if let Some(q) = query {
                q.query().len()
            } else {
                0
            };
            Ok(unsafe {
                finalize_string(
                    s.to_string(),
                    scheme_len,
                    host_str.len(),
                    ip,
                    port_len,
                    port,
                    path.path().len(),
                    query_len,
                )
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::ParseError::{
        InvalidFragment, InvalidHost, InvalidPath, InvalidPort, InvalidQuery, InvalidScheme,
    };
    use crate::{ParseError, WebUrl};

    #[test]
    fn from_str() {
        let test_cases: &[(&str, Result<&str, ParseError>)] = &[
            ("", Err(InvalidScheme)),
            ("s", Err(InvalidScheme)),
            ("s:", Err(InvalidScheme)),
            ("s:/", Err(InvalidScheme)),
            ("s!://", Err(InvalidScheme)),
            ("s://", Err(InvalidHost)),
            ("s://host", Ok("s://host/")),
            ("s://host/", Ok("s://host/")),
            ("s://127.0.0.1/", Ok("s://127.0.0.1/")),
            ("s://[::1]/", Ok("s://[::1]/")),
            ("s://::1/", Err(InvalidHost)),
            ("s://invalid!/", Err(InvalidHost)),
            ("s://h:", Err(InvalidPort)),
            ("s://h:/", Err(InvalidPort)),
            ("s://h:80", Ok("s://h:80/")),
            ("s://h:80/", Ok("s://h:80/")),
            ("s://h:80/p", Ok("s://h:80/p")),
            ("s://h:80/你好", Err(InvalidPath)),
            ("s://h:80/p?q", Ok("s://h:80/p?q")),
            ("s://h:80/p?你好", Err(InvalidQuery)),
            ("s://h:80/p#f", Ok("s://h:80/p#f")),
            ("s://h:80/p#你好", Err(InvalidFragment)),
            ("s://h:80/p?q#f", Ok("s://h:80/p?q#f")),
        ];
        for (s, expected) in test_cases {
            let result: Result<WebUrl, ParseError> = WebUrl::from_str(*s);
            match (result, expected) {
                (Ok(r), Ok(e)) => {
                    assert_eq!(r.to_string(), *e, "r={} e={} s={}", r, e, *s);
                }
                (Err(r), Err(e)) => {
                    assert_eq!(r, *e, "r={} e={} s={}", r, e, *s);
                }
                (Ok(r), Err(e)) => {
                    assert!(false, "r={} e={} s={}", r, e, *s);
                }
                (Err(r), Ok(e)) => {
                    assert!(false, "r={} e={} s={}", r, *e, *s);
                }
            }
        }
    }
}
