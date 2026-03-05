use std::str::FromStr;

use crate::parse::Error;
use crate::WebUrl;

impl FromStr for WebUrl {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url: String = s.to_string();
        WebUrl::try_from(url).map_err(|(e, _)| e)
    }
}
