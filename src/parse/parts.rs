use crate::parse::path_plus::{parse_path_plus, parse_query_plus, PathPlus};
use crate::parse::pre_path::{parse_pre_path, PrePath};
use crate::parse::Error;
use std::fmt::Write;

/// The validated parts of a web-based URL.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Parts {
    pub pre_path: PrePath,
    pub path_plus: PathPlus,

    /// Set when the URL has no explicit path and a '/' must be inserted after the authority.
    pub needs_slash: bool,
}

impl Parts {
    //! Properties

    /// Checks if the port must be rewritten to normalize the URL.
    ///
    /// This is set when the parsed port was empty or had leading zeros.
    pub const fn needs_port_rewrite(self) -> bool {
        self.pre_path.port_len != self.pre_path.canonical_port_len()
    }

    /// Checks if the parsed URL string is already normalized, ignoring the letter case.
    ///
    /// The letter case is excluded since it is normalized in place & never changes the length.
    pub const fn is_normalized(self) -> bool {
        !self.needs_slash && !self.needs_port_rewrite()
    }

    /// Gets the length of the normalized URL string for a parsed URL of `len` chars.
    pub const fn normalized_len(self, len: usize) -> usize {
        (len - self.pre_path.len()) + self.pre_path.canonical_len() + (self.needs_slash as usize)
    }

    /// Gets the index the '/' must be inserted at. (only meaningful when `needs_slash` is set)
    ///
    /// This is an index into the parsed URL, so it is only valid when the port is not rewritten.
    pub const fn slash_index(self) -> usize {
        self.pre_path.len()
    }
}

/// Writes the normalized URL for the parsed URL `s` to `url`.
///
/// The `parts` must have been parsed from `s`. The letter case is **not** normalized here; that is
/// done in place once the URL string is built.
pub fn write_normalized(s: &str, parts: &Parts, url: &mut String) {
    let pre_path: &PrePath = &parts.pre_path;

    url.push_str(&s[..pre_path.host_end()]);
    if let Some(port) = pre_path.port {
        url.push(':');
        let _ = write!(url, "{}", port);
    }
    if parts.needs_slash {
        url.push('/');
    }
    url.push_str(&s[pre_path.len()..]);
}

/// Parses & validates the web-based URL `s` without allocating.
///
/// The URL is **not** required to have an explicit path. When it does not, `needs_slash` will be
/// set on the returned parts and the caller is responsible for inserting the '/' at
/// `slash_index()` when it builds the normalized URL string.
///
/// Returns `Ok(parts)`.
/// Returns `Err(_)` if any part of the URL is invalid.
pub fn parse_parts(s: &str) -> Result<Parts, Error> {
    let pre_path: PrePath = parse_pre_path(s)?;

    // The authority is terminated by a '/', '?', or '#' char, or by the end of the URL. Only the
    // '/' case has an explicit path; the others imply the path is a single '/'.
    let after_authority: &str = &s[pre_path.len()..];
    let needs_slash: bool = !after_authority.starts_with('/');
    let path_plus: PathPlus = if needs_slash {
        parse_query_plus(after_authority)?
    } else {
        parse_path_plus(after_authority)?
    };

    Ok(Parts {
        pre_path,
        path_plus,
        needs_slash,
    })
}

#[cfg(test)]
mod tests {
    use crate::parse::parts::{parse_parts, Parts};
    use crate::parse::Error;
    use crate::parse::Error::{InvalidHost, InvalidPath, InvalidQuery, InvalidScheme};

    /// The summary of the parsed parts. `(needs_slash, slash_index, path_len, query_len)`
    type Summary = (bool, usize, usize, usize);

    fn parts_of(s: &str) -> Result<Summary, Error> {
        parse_parts(s).map(|p: Parts| {
            (
                p.needs_slash,
                p.slash_index(),
                p.path_plus.path_len,
                p.path_plus.query_len,
            )
        })
    }

    #[test]
    fn fn_parse_parts() {
        let test_cases: &[(&str, Result<Summary, Error>)] = &[
            ("http://host", Ok((true, 11, 1, 0))),
            ("http://host/", Ok((false, 11, 1, 0))),
            ("http://host/p", Ok((false, 11, 2, 0))),
            ("http://host?q", Ok((true, 11, 1, 2))),
            ("http://host#f", Ok((true, 11, 1, 0))),
            ("http://host?q#f", Ok((true, 11, 1, 2))),
            ("http://host:80", Ok((true, 14, 1, 0))),
            ("http://host:80?q", Ok((true, 14, 1, 2))),
            ("http://host:80/p?q", Ok((false, 14, 2, 2))),
            ("http://[::1]?q", Ok((true, 12, 1, 2))),
            ("http://[::1]:80#f", Ok((true, 15, 1, 0))),
            ("no-scheme", Err(InvalidScheme)),
            ("http://ho!st?q", Err(InvalidHost)),
            ("http://host/p q", Err(InvalidPath)),
            ("http://host?q q", Err(InvalidQuery)),
        ];
        for (url, expected) in test_cases {
            let result: Result<Summary, Error> = parts_of(url);
            assert_eq!(result, *expected, "url={}", url);
        }
    }
}
