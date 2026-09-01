use crate::parse::port_decimal_len;

/// The canonical port string of a port, written to a stack buffer.
pub struct CanonicalPort {
    buffer: [u8; Self::MAX_LEN],
    len: usize,
}

impl CanonicalPort {
    //! Limits

    /// The maximum length of a canonical port string. (the ':' & the 5 digits of `u16::MAX`)
    const MAX_LEN: usize = 6;
}

impl CanonicalPort {
    //! Construction

    /// Creates the canonical port string for the `port`.
    ///
    /// The canonical form is the ':' prefix followed by the decimal digits with no leading zeros.
    pub fn new(port: u16) -> Self {
        let mut canonical: Self = Self {
            buffer: [b':'; Self::MAX_LEN],
            len: 1 + port_decimal_len(port),
        };

        // The digits are written from the end so the count from `port_decimal_len` places them
        // without a reversal.
        let mut rest: u16 = port;
        let mut index: usize = canonical.len;
        while index > 1 {
            index -= 1;
            canonical.buffer[index] = b'0' + (rest % 10) as u8;
            rest /= 10;
        }

        canonical
    }
}

impl CanonicalPort {
    //! Properties

    /// Gets the canonical port string.
    pub fn as_str(&self) -> &str {
        debug_assert!(std::str::from_utf8(&self.buffer[..self.len]).is_ok());

        // The buffer holds the ':' & the ASCII digits, so it is valid UTF-8 up to the length.
        unsafe { std::str::from_utf8_unchecked(&self.buffer[..self.len]) }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::CanonicalPort;

    #[test]
    fn new() {
        let test_cases: &[(u16, &str)] = &[
            (0, ":0"),
            (9, ":9"),
            (80, ":80"),
            (443, ":443"),
            (8_080, ":8080"),
            (65_535, ":65535"),
        ];
        for (port, expected) in test_cases {
            let canonical: CanonicalPort = CanonicalPort::new(*port);
            assert_eq!(canonical.as_str(), *expected, "port={}", port);
        }
    }

    #[test]
    fn new_every_port() {
        // The string sizes a URL splice, so it must match the rendered port for every port exactly.
        for port in 0..=u16::MAX {
            let canonical: CanonicalPort = CanonicalPort::new(port);
            assert_eq!(canonical.as_str(), format!(":{}", port), "port={}", port);
        }
    }
}
