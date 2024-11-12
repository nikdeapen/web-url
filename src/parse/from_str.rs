use std::str::FromStr;

use crate::parse::finalize::finalize_web_url;
use crate::parse::pre_path::{parse_pre_path, PrePath};
use crate::parse::Error;
use crate::parse::{parse_path_plus, PathPlus};
use crate::WebUrl;

impl FromStr for WebUrl {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pre_path: PrePath = parse_pre_path(s)?;
        let path_plus: &str = &s[pre_path.len()..];

        if path_plus.len() == 0 {
            let mut url: String = String::with_capacity(pre_path.len() + 1);
            url.push_str(s);
            url.push_str("/");

            let path_plus: PathPlus = parse_path_plus("/")?;
            unsafe { finalize_web_url(url, pre_path, path_plus) }
        } else {
            let url: String = s.to_string();
            let path_plus: PathPlus = parse_path_plus(path_plus)?;
            unsafe { finalize_web_url(url, pre_path, path_plus) }
        }
    }
}
