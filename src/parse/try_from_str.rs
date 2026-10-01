use crate::{ParseError, WebUrl};
use std::str::FromStr;

impl TryFrom<&str> for WebUrl {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}
