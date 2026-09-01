use address::IPAddress;
use std::fmt::Write;

/// The canonical host string of an IP address, written to a stack buffer.
pub struct CanonicalHost {
    buffer: [u8; Self::MAX_LEN],
    len: usize,
}

impl CanonicalHost {
    //! Limits

    /// The maximum length of a canonical host string.
    ///
    /// The longest the `address` display produces is an IPv6 address with all eight groups & the '[]' brackets:
    /// `[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff]`, which is 41 chars. The embedded-IPv4 form is only used when the
    /// leading groups are zero, so it is shorter. The extra room covers that form at full width in case the display
    /// ever emits it: `[ffff:ffff:ffff:ffff:ffff:ffff:255.255.255.255]`.
    const MAX_LEN: usize = 47;
}

impl CanonicalHost {
    //! Construction

    /// Creates the canonical host string for the `ip`.
    ///
    /// An IPv6 address is bracketed, as it appears in a URL. The canonical form is the `address` display, so it is
    /// lowercase & its zero groups are elided.
    pub fn new(ip: IPAddress) -> Self {
        let mut host: Self = Self {
            buffer: [0; Self::MAX_LEN],
            len: 0,
        };

        let result: std::fmt::Result = match ip {
            IPAddress::V4(ip) => write!(host, "{}", ip),
            IPAddress::V6(ip) => write!(host, "[{}]", ip),
        };
        // The buffer fits the longest canonical host, so this cannot fail. It is asserted in every build since a
        // partial write would silently put a truncated host in the URL rather than fail.
        assert!(result.is_ok(), "the canonical host of '{}' is too long", ip);

        host
    }
}

impl CanonicalHost {
    //! Properties

    /// Gets the canonical host string.
    pub fn as_str(&self) -> &str {
        debug_assert!(std::str::from_utf8(&self.buffer[..self.len]).is_ok());

        // The buffer holds the strings written to it, so it is valid UTF-8 up to the length.
        unsafe { std::str::from_utf8_unchecked(&self.buffer[..self.len]) }
    }
}

impl Write for CanonicalHost {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let end: usize = self.len + s.len();
        if end > Self::MAX_LEN {
            Err(std::fmt::Error)
        } else {
            self.buffer[self.len..end].copy_from_slice(s.as_bytes());
            self.len = end;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::CanonicalHost;
    use address::{IPv4Address, IPv6Address};
    use std::str::FromStr;

    #[test]
    fn new() {
        let test_cases: &[(&str, &str)] = &[
            ("::1", "[::1]"),
            ("0:0:0:0:0:0:0:1", "[::1]"),
            ("2001:DB8::", "[2001:db8::]"),
            ("::FFFF:1.2.3.4", "[::ffff:1.2.3.4]"),
        ];
        for (ip, expected) in test_cases {
            let ip: IPv6Address = IPv6Address::from_str(ip).unwrap();
            let canonical: CanonicalHost = CanonicalHost::new(ip.to_ip());
            assert_eq!(canonical.as_str(), *expected, "ip={}", ip);
        }

        let canonical: CanonicalHost = CanonicalHost::new(IPv4Address::LOCALHOST.to_ip());
        assert_eq!(canonical.as_str(), "127.0.0.1");
    }

    /// The buffer is sized for the longest canonical host, so the longest ones must still fit.
    #[test]
    fn new_longest() {
        let test_cases: &[(&str, &str)] = &[
            // All eight groups is the longest the display produces; the embedded IPv4 is not used at full width.
            (
                "ffff:ffff:ffff:ffff:ffff:ffff:255.255.255.255",
                "[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff]",
            ),
            // The embedded-IPv4 form only appears when the leading groups are zero, so it is shorter.
            ("::ffff:255.255.255.255", "[::ffff:255.255.255.255]"),
        ];
        for (ip, expected) in test_cases {
            let ip: IPv6Address = IPv6Address::from_str(ip).unwrap();
            let canonical: CanonicalHost = CanonicalHost::new(ip.to_ip());
            assert_eq!(canonical.as_str(), *expected, "ip={}", ip);
            assert!(
                canonical.as_str().len() <= CanonicalHost::MAX_LEN,
                "ip={}",
                ip
            );
        }
    }
}
