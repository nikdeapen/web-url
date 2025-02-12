use crate::parse::{parse_path_plus, parse_pre_path, PathPlus, PrePath};
use crate::WebUrl;

impl TryFrom<String> for WebUrl {
    type Error = crate::parse::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let pre_path: PrePath = parse_pre_path(s.as_str())?;

        if pre_path.len() == s.len() {
            let mut url: String = s;
            url.push_str("/");

            let path_plus: PathPlus = parse_path_plus("/")?;
            unsafe { crate::parse::finalize::finalize_web_url(url, pre_path, path_plus) }
        } else {
            let url: String = s;
            let path_plus: PathPlus = parse_path_plus(&url.as_str()[pre_path.len()..])?;
            unsafe { crate::parse::finalize::finalize_web_url(url, pre_path, path_plus) }
        }
    }
}
