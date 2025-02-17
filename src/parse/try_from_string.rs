use crate::parse::{parse_path_plus, parse_pre_path, PathPlus, PrePath};
use crate::WebUrl;

impl TryFrom<String> for WebUrl {
    type Error = (crate::parse::Error, String);

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let pre_path: PrePath = match parse_pre_path(s.as_str()) {
            Ok(pre_path) => pre_path,
            Err(error) => return Err((error, s)),
        };

        if pre_path.len() == s.len() {
            let mut url: String = s;
            url.push_str("/");

            let path_plus: PathPlus = match parse_path_plus("/") {
                Ok(path_plus) => path_plus,
                Err(error) => return Err((error, url)),
            };
            unsafe { crate::parse::finalize::finalize_web_url(url, pre_path, path_plus) }
        } else {
            let url: String = s;
            let path_plus: PathPlus = match parse_path_plus(&url.as_str()[pre_path.len()..]) {
                Ok(path_plus) => path_plus,
                Err(error) => return Err((error, url)),
            };
            unsafe { crate::parse::finalize::finalize_web_url(url, pre_path, path_plus) }
        }
    }
}
