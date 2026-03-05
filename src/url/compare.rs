use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use crate::WebUrl;

impl Ord for WebUrl {
    fn cmp(&self, other: &Self) -> Ordering {
        self.url.cmp(&other.url)
    }
}

impl PartialOrd for WebUrl {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for WebUrl {}

impl PartialEq for WebUrl {
    fn eq(&self, other: &Self) -> bool {
        self.url.eq(&other.url)
    }
}

impl Hash for WebUrl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::str::FromStr;

    use crate::WebUrl;

    fn hash_of(url: &WebUrl) -> u64 {
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn eq() {
        let a = WebUrl::from_str("https://example.com").unwrap();
        let b = WebUrl::from_str("https://example.com").unwrap();
        let c = WebUrl::from_str("https://other.com").unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn ord() {
        let a = WebUrl::from_str("https://aaa.com").unwrap();
        let b = WebUrl::from_str("https://bbb.com").unwrap();

        assert!(a < b);
        assert!(b > a);
        assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
    }

    #[test]
    fn hash() {
        let a = WebUrl::from_str("https://example.com").unwrap();
        let b = WebUrl::from_str("https://example.com").unwrap();

        assert_eq!(hash_of(&a), hash_of(&b));
    }
}
