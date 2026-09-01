/// Gets the length of the `path` with the dot-segments removed.
///
/// The `path` must be valid, so it starts with a '/'.
///
/// # RFC 3986
/// <https://www.rfc-editor.org/rfc/rfc3986#section-5.2.4>
pub fn canonical_path_len(path: &str) -> usize {
    // The segments are scanned in reverse so the segments a ".." removes are known without a stack:
    // a ".." raises the skip count & the next kept segment lowers it.
    let mut len: usize = 0;
    let mut skip: usize = 0;
    for segment in path[1..].rsplit('/') {
        match segment {
            "." => {}
            ".." => skip += 1,
            segment => {
                if skip != 0 {
                    skip -= 1;
                } else {
                    len += 1 + segment.len();
                }
            }
        }
    }

    len + (ends_with_dot_segment(path) as usize)
}

/// Writes the `path` to `url` with the dot-segments removed.
///
/// The `path` must be valid, so it starts with a '/'.
///
/// # RFC 3986
/// <https://www.rfc-editor.org/rfc/rfc3986#section-5.2.4>
pub fn write_canonical_path(path: &str, url: &mut String) {
    // The segments are written as they are scanned & a ".." truncates the last written segment, so
    // the written path is the segment stack. The truncation never reaches past `start`, which is
    // what keeps a leading ".." from escaping the path.
    let start: usize = url.len();
    for segment in path[1..].split('/') {
        match segment {
            "." => {}
            ".." => {
                if let Some(index) = url[start..].rfind('/') {
                    url.truncate(start + index);
                }
            }
            segment => {
                url.push('/');
                url.push_str(segment);
            }
        }
    }

    if ends_with_dot_segment(path) {
        url.push('/');
    }
}

/// Checks if the last segment of the `path` is a "." or ".." segment, which leaves the canonical
/// path ending with a '/'.
fn ends_with_dot_segment(path: &str) -> bool {
    path.ends_with("/.") || path.ends_with("/..")
}

#[cfg(test)]
mod tests {
    use crate::parse::{canonical_path_len, write_canonical_path};

    #[test]
    fn fn_write_canonical_path() {
        let test_cases: &[(&str, &str)] = &[
            // A path with no dot-segments is unchanged.
            ("/", "/"),
            ("//", "//"),
            ("/a", "/a"),
            ("/a/", "/a/"),
            ("/a/b", "/a/b"),
            // A "." segment is dropped.
            ("/.", "/"),
            ("/./", "/"),
            ("/a/.", "/a/"),
            ("/a/./b", "/a/b"),
            // A ".." segment drops itself & the segment before it.
            ("/..", "/"),
            ("/../", "/"),
            ("/a/..", "/"),
            ("/a/../", "/"),
            ("/a/b/..", "/a/"),
            ("/a/b/../c", "/a/c"),
            ("/a/b/../../c", "/c"),
            ("/a//../b", "/a/b"),
            // A ".." segment at the root has nothing to drop.
            ("/../a", "/a"),
            ("/../../a", "/a"),
            ("/a/../..", "/"),
            // Only a whole segment is a dot-segment.
            ("/..a", "/..a"),
            ("/a..", "/a.."),
            ("/.a/..", "/"),
            // The percent-encoding is not decoded, so an escaped dot-segment is ordinary text.
            ("/%2e%2e", "/%2e%2e"),
            ("/a/%2e%2e", "/a/%2e%2e"),
        ];
        for (path, expected) in test_cases {
            let mut result: String = String::new();
            write_canonical_path(path, &mut result);
            assert_eq!(result, *expected, "path={}", path);

            // The length must match what is written exactly; it sizes the URL allocation.
            assert_eq!(canonical_path_len(path), result.len(), "path={}", path);

            // The canonical path is the canonical form, so canonicalizing it again must not change
            // it.
            let mut again: String = String::new();
            write_canonical_path(result.as_str(), &mut again);
            assert_eq!(again, result, "path={}", path);
        }
    }

    /// The path is appended to the URL, so a ".." must never truncate past what was already
    /// written.
    #[test]
    fn fn_write_canonical_path_appends() {
        let mut result: String = String::from("https://host");
        write_canonical_path("/a/../../b", &mut result);
        assert_eq!(result, "https://host/b");
    }
}
