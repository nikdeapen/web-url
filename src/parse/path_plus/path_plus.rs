use crate::parse::path_plus::parse_fragment;
use crate::parse::path_plus::parse_path;
use crate::parse::path_plus::parse_query;
use crate::parse::Error;

/// The parsing data for a web-based URL from the path to the end.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PathPlus {
    pub path_len: usize,     // length of the path including the '/' (will be 1+)
    pub query_len: usize,    // length of query including the '?' (will be 0+)
    pub fragment_len: usize, // length of fragment including the '#' (will be 0+)
}

/// Parses the web-based URL starting at the path.
///
/// The path, query, and fragment will all be validated.
pub fn parse_path_plus(path_plus: &str) -> Result<PathPlus, Error> {
    let (path, after_path) = parse_path(path_plus)?;
    let (query, after_query) = parse_query(after_path)?;
    let fragment: Option<&str> = parse_fragment(after_query)?;

    let path_len: usize = path.as_str().len();
    let query_len: usize = query.map(|q| q.as_str().len()).unwrap_or(0);
    let fragment_len: usize = fragment.map(|f| f.len()).unwrap_or(0);

    Ok(PathPlus {
        path_len,
        query_len,
        fragment_len,
    })
}
