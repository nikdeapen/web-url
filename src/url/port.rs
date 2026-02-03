use crate::WebUrl;

impl WebUrl {
    //! Port

    /// Gets the optional port.
    pub fn port(&self) -> Option<u16> {
        self.port
    }
}
