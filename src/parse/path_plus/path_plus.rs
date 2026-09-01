use crate::Error;
use crate::parse::{canonical_path_len, check_fragment, parse_path, parse_query};

/// The parsing data for a web-based URL from the path to the end.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PathPlus {
    pub path_len: usize, // length of the path including the '/' (will be 1+)
    pub canonical_path_len: usize, // length of the path without the dot-segments (will be 1+)
    pub query_len: usize, // length of query including the '?' (will be 0+)
}

/// Parses the `path_plus`. (the path, the optional query, & the optional fragment)
///
/// The path, query, & fragment will be validated.
pub fn parse_path_plus(path_plus: &str) -> Result<PathPlus, Error> {
    let (path, after_path) = parse_path(path_plus)?;
    let (query, after_query) = parse_query(after_path)?;
    check_fragment(after_query)?;

    let path: &str = path.as_str();
    let query_len: usize = query.map(|q| q.as_str().len()).unwrap_or(0);

    Ok(PathPlus {
        path_len: path.len(),
        canonical_path_len: canonical_path_len(path),
        query_len,
    })
}

/// Parses the `query_plus`. (the optional query & the optional fragment)
///
/// This is used when the URL has no explicit path. The path is implied to be a single '/' which is
/// reflected in the returned `path_len` even though it is not present in `query_plus`.
///
/// The query & fragment will be validated.
pub fn parse_query_plus(query_plus: &str) -> Result<PathPlus, Error> {
    let (query, after_query) = parse_query(query_plus)?;
    check_fragment(after_query)?;

    let query_len: usize = query.map(|q| q.as_str().len()).unwrap_or(0);

    // The implied path is a single '/' which is already canonical.
    Ok(PathPlus {
        path_len: 1,
        canonical_path_len: 1,
        query_len,
    })
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::Error::{InvalidFragment, InvalidPath, InvalidQuery};
    use crate::parse::{PathPlus, parse_path_plus, parse_query_plus};

    /// Creates the expected path-plus. (for paths with no dot-segments)
    fn path_plus(path_len: usize, query_len: usize) -> PathPlus {
        PathPlus {
            path_len,
            canonical_path_len: path_len,
            query_len,
        }
    }

    #[test]
    fn fn_parse_path_plus() {
        let test_cases: &[(&str, Result<PathPlus, Error>)] = &[
            ("/", Ok(path_plus(1, 0))),
            ("/p", Ok(path_plus(2, 0))),
            ("/p?a=1", Ok(path_plus(2, 4))),
            ("/p#frag", Ok(path_plus(2, 0))),
            ("/p?a=1#frag", Ok(path_plus(2, 4))),
            ("", Err(InvalidPath)),
            ("no-slash", Err(InvalidPath)),
            ("/p q", Err(InvalidPath)),
            ("/p?a b", Err(InvalidQuery)),
            ("/p#a b", Err(InvalidFragment)),
        ];
        for (input, expected) in test_cases {
            let result: Result<PathPlus, Error> = parse_path_plus(input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn fn_parse_query_plus() {
        let test_cases: &[(&str, Result<PathPlus, Error>)] = &[
            ("", Ok(path_plus(1, 0))),
            ("?", Ok(path_plus(1, 1))),
            ("?a=1", Ok(path_plus(1, 4))),
            ("#", Ok(path_plus(1, 0))),
            ("#frag", Ok(path_plus(1, 0))),
            ("?a=1#frag", Ok(path_plus(1, 4))),
            ("?a b", Err(InvalidQuery)),
            ("#a b", Err(InvalidFragment)),
            ("not-a-query", Err(InvalidFragment)),
        ];
        for (input, expected) in test_cases {
            let result: Result<PathPlus, Error> = parse_query_plus(input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
