use crate::ParseError::UrlTooLong;
use crate::parse::{
    CanonicalPort, PathPlus, PrePath, parse_path_plus, parse_pre_path, parse_query_plus,
    write_canonical_path,
};
use crate::web_url::Offsets;
use crate::{ParseError, WebUrl};

/// The validated parts of a web-based URL.
#[must_use]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct Parts {
    /// The parsing data before the path.
    pub(crate) pre_path: PrePath,

    /// The parsing data from the path to the end.
    pub(crate) path_plus: PathPlus,

    /// Set when the URL has no explicit path & a '/' must be inserted after the authority.
    pub(crate) needs_slash: bool,

    /// Set when the host is an IP address that must be rewritten in its canonical form.
    pub(crate) needs_host_rewrite: bool,
}

impl Parts {
    //! Properties

    /// Checks if the port must be rewritten to normalize the URL.
    ///
    /// This is set when the parsed port was empty or had leading zeros.
    const fn needs_port_rewrite(self) -> bool {
        self.pre_path.port_len != self.pre_path.canonical_port_len()
    }

    /// Checks if the path must be rewritten to normalize the URL.
    ///
    /// This is set when the parsed path has dot-segments, which always shorten it.
    const fn needs_path_rewrite(self) -> bool {
        self.path_plus.path_len != self.path_plus.canonical_path_len
    }

    /// Checks if the host, port, or path must be rewritten to normalize the URL.
    ///
    /// A rewrite changes the length of the URL before the query, so it cannot be normalized in
    /// place.
    pub(crate) const fn needs_rewrite(self) -> bool {
        self.needs_host_rewrite || self.needs_port_rewrite() || self.needs_path_rewrite()
    }

    /// Checks if the parsed URL string is already normalized, ignoring the letter case.
    ///
    /// The letter case is excluded since it is normalized in place & never changes the length.
    pub(crate) const fn is_normalized(self) -> bool {
        !self.needs_slash && !self.needs_rewrite()
    }

    /// Gets the length of the normalized URL string for a parsed URL of `len` chars.
    pub(crate) const fn normalized_len(self, len: usize) -> usize {
        // The canonical path is never longer than the parsed path since it only drops dot-segments.
        let dropped: usize = self.path_plus.path_len - self.path_plus.canonical_path_len;

        ((len - self.pre_path.len()) - dropped)
            + self.pre_path.canonical_len()
            + (self.needs_slash as usize)
    }

    /// Gets the index the '/' must be inserted at. (only meaningful when `needs_slash` is set)
    ///
    /// This is an index into the parsed URL, so it is only valid when neither the host nor the port
    /// is rewritten.
    pub(crate) const fn slash_index(self) -> usize {
        self.pre_path.len()
    }

    /// Gets the offsets of the normalized URL.
    ///
    /// The normalized URL never exceeds `WebUrl::MAX_LEN` since `parse_parts` checks it, so every
    /// offset fits a `u32`.
    pub(crate) const fn offsets(self) -> Offsets {
        // The canonical lengths are used since the offsets are into the normalized URL, in which an
        // IP address host may be shorter or longer & the port & path may be shorter.
        let port_end: usize = self.pre_path.canonical_len();
        let path_end: usize = port_end + self.path_plus.canonical_path_len;
        Offsets {
            scheme_len: self.pre_path.scheme_len as u32,
            host_end: self.pre_path.canonical_host_end() as u32,
            port_end: port_end as u32,
            path_end: path_end as u32,
            query_end: (path_end + self.path_plus.query_len) as u32,
        }
    }
}

/// Writes the normalized URL for the parsed URL `s` to `url`.
///
/// The `parts` must have been parsed from `s`. The scheme & a domain host keep their letter case
/// here; they are lowercased in place once the URL string is built.
pub(crate) fn write_normalized(s: &str, parts: Parts, url: &mut String) {
    let pre_path: PrePath = parts.pre_path;

    url.push_str(&s[..pre_path.host_start()]);
    if let Some(ip) = pre_path.ip {
        url.push_str(ip.as_str());
    } else {
        url.push_str(pre_path.host_str(s));
    }
    if let Some(port) = pre_path.port {
        url.push_str(CanonicalPort::new(port).as_str());
    }

    let after_authority: &str = &s[pre_path.len()..];
    if parts.needs_slash {
        // The URL has no explicit path, so the implied '/' precedes the query & fragment.
        url.push('/');
        url.push_str(after_authority);
    } else {
        let (path, path_plus) = after_authority.split_at(parts.path_plus.path_len);
        write_canonical_path(path, url);
        url.push_str(path_plus);
    }
}

/// Parses & validates the web-based URL `s` without allocating.
///
/// The URL is **not** required to have an explicit path. When it does not, `needs_slash` will be
/// set on the returned parts & the caller is responsible for inserting the '/' at `slash_index()`
/// when it builds the normalized URL string.
pub(crate) fn parse_parts(s: &str) -> Result<Parts, ParseError> {
    let pre_path: PrePath = parse_pre_path(s)?;
    let needs_host_rewrite: bool = pre_path.needs_host_rewrite(s);

    // The authority is terminated by a '/', '?', or '#' char, or by the end of the URL. Only the
    // '/' case has an explicit path; the others imply the path is a single '/'.
    let after_authority: &str = &s[pre_path.len()..];
    let needs_slash: bool = !after_authority.starts_with('/');
    let path_plus: PathPlus = if needs_slash {
        parse_query_plus(after_authority)?
    } else {
        parse_path_plus(after_authority)?
    };

    let parts: Parts = Parts {
        pre_path,
        path_plus,
        needs_slash,
        needs_host_rewrite,
    };

    // The normalized length is checked, since that is the URL that is stored. It is checked here,
    // before anything is built, so a URL that is too long is never modified.
    if parts.normalized_len(s.len()) > WebUrl::MAX_LEN {
        return Err(UrlTooLong);
    }
    Ok(parts)
}

#[cfg(test)]
mod tests {
    use crate::ParseError;
    use crate::ParseError::{InvalidHost, InvalidPath, InvalidQuery, InvalidScheme};
    use crate::parse::{Parts, parse_parts, write_normalized};
    use crate::web_url::Offsets;

    /// The summary of the parsed parts. `(needs_slash, slash_index, path_len, query_len)`
    type Summary = (bool, usize, usize, usize);

    /// The summary of the offsets. `(scheme_len, host_end, port_end, path_end, query_end)`
    type OffsetsSummary = (u32, u32, u32, u32, u32);

    fn parts_of(s: &str) -> Result<Summary, ParseError> {
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
    fn parts() {
        let test_cases: &[(&str, Result<Summary, ParseError>)] = &[
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
            let result: Result<Summary, ParseError> = parts_of(url);
            assert_eq!(result, *expected, "url={}", url);
        }
    }

    #[test]
    fn normalized() {
        let test_cases: &[(&str, &str)] = &[
            ("http://host/p?q#f", "http://host/p?q#f"),
            ("http://host", "http://host/"),
            ("http://host?q#f", "http://host/?q#f"),
            ("http://host:0080/p", "http://host:80/p"),
            ("http://host:/p?q", "http://host/p?q"),
            (
                "http://[0:0:0:0:0:0:0:1]:0080/a/../b?q#f",
                "http://[::1]:80/b?q#f",
            ),
            // The scheme & a domain host keep their letter case; an IP host is written canonically.
            ("HTTP://HOST/P", "HTTP://HOST/P"),
            ("http://[::FFFF:1.2.3.4]/", "http://[::ffff:1.2.3.4]/"),
        ];
        for (input, expected) in test_cases {
            let parts: Parts = parse_parts(input).unwrap();

            let mut result: String = String::new();
            write_normalized(input, parts, &mut result);
            assert_eq!(result, *expected, "input={}", input);

            // The length must match what is written exactly; it sizes the URL allocation.
            assert_eq!(
                parts.normalized_len(input.len()),
                result.len(),
                "input={}",
                input
            );

            // A URL written back unchanged, ignoring the letter case, is exactly the one that needs no
            // rewrite.
            assert_eq!(
                parts.is_normalized(),
                input.eq_ignore_ascii_case(&result),
                "input={}",
                input
            );
        }
    }

    #[test]
    fn offsets() {
        // The offsets are into the normalized URL, so they follow the rewritten host, port, & path.
        let test_cases: &[(&str, OffsetsSummary)] = &[
            ("http://host/p?q#f", (4, 11, 11, 13, 15)),
            ("http://host", (4, 11, 11, 12, 12)),
            ("http://host:0080/a/../b?q", (4, 11, 14, 16, 18)),
            ("HTTP://[0:0:0:0:0:0:0:1]:80#f", (4, 12, 15, 16, 16)),
            ("s://127.0.0.1:/?", (1, 13, 13, 14, 15)),
        ];
        for (input, expected) in test_cases {
            let offsets: Offsets = parse_parts(input).unwrap().offsets();
            let result: OffsetsSummary = (
                offsets.scheme_len,
                offsets.host_end,
                offsets.port_end,
                offsets.path_end,
                offsets.query_end,
            );
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
