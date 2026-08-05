use std::borrow::Borrow;
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

/// The `Borrow` contract holds since `Eq`, `Ord`, & `Hash` all delegate to the URL string. This
/// enables map & set lookups by `&str`.
impl Borrow<str> for WebUrl {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashSet;
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

    #[test]
    fn eq_normalized() {
        // URLs that differ only in their pre-normalization spelling are equal & hash the same.
        let test_cases: &[(&str, &str)] = &[
            ("HTTP://EXAMPLE.COM:0080/p", "http://example.com:80/p"),
            ("http://host", "http://host/"),
            ("http://host:/", "http://host/"),
            ("http://host:080?q", "http://host:80/?q"),
        ];
        for (a, b) in test_cases {
            let a = WebUrl::from_str(a).unwrap();
            let b = WebUrl::from_str(b).unwrap();
            assert_eq!(a, b);
            assert_eq!(hash_of(&a), hash_of(&b), "a={} b={}", a, b);
        }
    }

    #[test]
    fn borrow() {
        let mut set: HashSet<WebUrl> = HashSet::new();
        set.insert(WebUrl::from_str("https://example.com").unwrap());

        assert!(set.contains("https://example.com/"));
        assert!(!set.contains("https://other.com/"));
    }
}
