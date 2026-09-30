use crate::{Path, WebUrl, parse};

impl WebUrl {
    //! Path

    /// Gets the path.
    pub const fn path(&self) -> Path<'_> {
        unsafe { Path::new_unchecked(self.path_str()) }
    }

    /// Gets the path string.
    ///
    /// This will be a valid path starting with a '/' & having no dot-segments.
    const fn path_str(&self) -> &str {
        let start: usize = self.port_end as usize;
        let end: usize = self.path_end as usize;
        self.url.as_str().split_at(end).0.split_at(start).1
    }
}

impl WebUrl {
    //! Path Mutation

    /// Sets the `path`. (the dot-segments are removed)
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`. The URL is left unmodified.
    pub fn set_path(&mut self, path: Path) {
        // The path is written with the dot-segments removed, which is the normalized form.
        let mut insert: String = String::with_capacity(parse::canonical_path_len(path.as_str()));
        parse::write_canonical_path(path.as_str(), &mut insert);

        let start: usize = self.port_end as usize;
        let end: usize = self.path_end as usize;

        Self::check_len((self.url.len() - (end - start)) + insert.len());

        let query_len: u32 = self.query_end - self.path_end;

        self.url.replace_range(start..end, insert.as_str());

        self.path_end = (start + insert.len()) as u32;
        self.query_end = self.path_end + query_len;

        debug_assert!(self.is_consistent());
    }

    /// Sets the `path`. (the dot-segments are removed)
    ///
    /// # Panics
    /// Panics if the resulting URL would exceed `WebUrl::MAX_LEN`.
    pub fn with_path(mut self, path: Path) -> Self {
        self.set_path(path);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{Path, WebUrl};
    use std::str::FromStr;

    #[test]
    fn path() {
        let url: WebUrl = WebUrl::from_str("https://example.com/the/path").unwrap();
        assert_eq!(url.path().as_str(), "/the/path");

        let url: WebUrl = WebUrl::from_str("https://example.com").unwrap();
        assert_eq!(url.path().as_str(), "/");
    }

    #[test]
    fn set_path() {
        // The path is normalized as it is set, so the dot-segments never reach the URL.
        let test_cases: &[(&str, &str, &str)] = &[
            ("http://host/old?q#f", "/new", "http://host/new?q#f"),
            ("http://host/old", "/", "http://host/"),
            ("http://host/", "/a/b", "http://host/a/b"),
            ("http://host/", "//", "http://host//"),
            ("http://host/", "/a/../b", "http://host/b"),
            ("http://host/", "/a/./b/", "http://host/a/b/"),
            ("http://host/", "/..", "http://host/"),
            ("http://host:8080/old?q", "/new", "http://host:8080/new?q"),
        ];
        for (input, path, expected) in test_cases {
            let mut url: WebUrl = WebUrl::from_str(input).unwrap();
            url.set_path(Path::try_from(*path).unwrap());
            assert_eq!(url.as_str(), *expected, "input={} path={}", input, path);
        }
    }

    #[test]
    fn with_path() {
        let url: WebUrl = WebUrl::from_str("https://example.com/old")
            .unwrap()
            .with_path(Path::try_from("/new").unwrap());
        assert_eq!(url.as_str(), "https://example.com/new");
    }
}
