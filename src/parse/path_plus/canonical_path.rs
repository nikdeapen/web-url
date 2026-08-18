/// Gets the length of the `path` with the dot-segments removed.
///
/// The `path` must be valid, so it starts with a '/'.
///
/// # RFC 3986
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-5.2.4>
pub fn canonical_path_len(path: &str) -> usize {
    // The segments are scanned in reverse so the segments a ".." removes are known without a stack:
    // a ".." raises the skip count and the next kept segment lowers it.
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
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-5.2.4>
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
