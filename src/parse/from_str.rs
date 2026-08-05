use std::str::FromStr;

use crate::parse::finalize::finalize_web_url;
use crate::parse::parts::{parse_parts, write_normalized, Parts};
use crate::parse::Error;
use crate::WebUrl;

impl FromStr for WebUrl {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Parts = parse_parts(s)?;

        // The URL is validated before it is allocated, so invalid input never allocates and the
        // normalized length is known exactly.
        let mut url: String = String::with_capacity(parts.normalized_len(s.len()));
        write_normalized(s, &parts, &mut url);

        // SAFETY: `url` is the normalized URL written from the parts, which were parsed from `s`.
        unsafe { finalize_web_url(url, parts.pre_path, parts.path_plus) }.map_err(|(error, _)| error)
    }
}
