use crate::{Path, WebUrl};

impl WebUrl {
    //! Path

    /// Gets the path.
    pub fn path(&self) -> Path<'_> {
        unsafe { Path::new(self.path_str()) }
    }

    /// Gets the path string.
    ///
    /// This will be a valid path starting with a '/'.
    fn path_str(&self) -> &str {
        let start: usize = self.port_end as usize;
        let end: usize = self.path_end as usize;
        &self.url[start..end]
    }
}
