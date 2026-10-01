use crate::web_url::Part;
use std::ops::Range;

/// The offsets of the parts of a web-based URL string.
#[must_use]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct Offsets {
    /// The length of the scheme.
    pub(crate) scheme_len: u32,

    /// The index just past the host.
    pub(crate) host_end: u32,

    /// The index just past the port. (equal to `host_end` when there is no port)
    pub(crate) port_end: u32,

    /// The index just past the path.
    pub(crate) path_end: u32,

    /// The index just past the query. (the fragment runs from here to the end)
    pub(crate) query_end: u32,
}

impl Offsets {
    //! Properties

    /// Gets the index of the host. (just past the "://" that follows the scheme)
    pub(crate) const fn host_start(self) -> u32 {
        self.scheme_len + 3
    }

    /// Gets the range of the `part` in the URL string.
    pub(crate) const fn range(self, part: Part) -> Range<usize> {
        let (start, end): (u32, u32) = match part {
            Part::Scheme => (0, self.scheme_len),
            Part::Host => (self.host_start(), self.host_end),
            Part::Port => (self.host_end, self.port_end),
            Part::Path => (self.port_end, self.path_end),
            Part::Query => (self.path_end, self.query_end),
        };
        start as usize..end as usize
    }
}

impl Offsets {
    //! Mutation

    /// Resizes the `part` to `len` bytes, shifting the parts after it.
    pub(crate) const fn resize(&mut self, part: Part, len: usize) {
        let range: Range<usize> = self.range(part);
        let old_len: u32 = (range.end - range.start) as u32;
        let len: u32 = len as u32;

        // The end of the `part` & the ends of the parts after it move by the change in its length.
        // Each of those ends is at least the old length, so the subtraction cannot underflow. The
        // part is named rather than found by its end since an empty part shares its end. The ends
        // are shifted with branches rather than a loop, which measured faster in the mutators.
        let part: u8 = part as u8;
        if part <= Part::Scheme as u8 {
            self.scheme_len = self.scheme_len - old_len + len;
        }
        if part <= Part::Host as u8 {
            self.host_end = self.host_end - old_len + len;
        }
        if part <= Part::Port as u8 {
            self.port_end = self.port_end - old_len + len;
        }
        if part <= Part::Path as u8 {
            self.path_end = self.path_end - old_len + len;
        }
        self.query_end = self.query_end - old_len + len;
    }
}

#[cfg(test)]
mod tests {
    use crate::web_url::{Offsets, Part};
    use std::ops::Range;

    /// `(before, part, len, after)` with the offsets as
    /// `[scheme_len, host_end, port_end, path_end, query_end]`.
    type TestCase = ([u32; 5], Part, usize, [u32; 5]);

    fn offsets(ends: [u32; 5]) -> Offsets {
        Offsets {
            scheme_len: ends[0],
            host_end: ends[1],
            port_end: ends[2],
            path_end: ends[3],
            query_end: ends[4],
        }
    }

    #[test]
    fn range() {
        // `http://host:80/p?q#f`
        let url: Offsets = offsets([4, 11, 14, 16, 18]);
        let test_cases: &[(Part, Range<usize>)] = &[
            (Part::Scheme, 0..4),
            (Part::Host, 7..11),
            (Part::Port, 11..14),
            (Part::Path, 14..16),
            (Part::Query, 16..18),
        ];
        for (part, expected) in test_cases {
            assert_eq!(url.range(*part), *expected, "part={:?}", part);
        }
    }

    #[test]
    fn resize() {
        // The end of the resized part & the ends after it move; the ends before it stay.
        let test_cases: &[TestCase] = &[
            // `http://host:80/p?q#f` -> `https://host:80/p?q#f`
            ([4, 11, 14, 16, 18], Part::Scheme, 5, [5, 12, 15, 17, 19]),
            // `http://host:80/p?q#f` -> `http://example.com:80/p?q#f`
            ([4, 11, 14, 16, 18], Part::Host, 11, [4, 18, 21, 23, 25]),
            // `http://host:80/p?q#f` -> `http://host/p?q#f`
            ([4, 11, 14, 16, 18], Part::Port, 0, [4, 11, 11, 13, 15]),
            // `http://host/p` -> `http://host:8080/p` (the empty port grows, the host end stays)
            ([4, 11, 11, 13, 13], Part::Port, 5, [4, 11, 16, 18, 18]),
            // `http://host:80/p?q#f` -> `http://host:80/?q#f`
            ([4, 11, 14, 16, 18], Part::Path, 1, [4, 11, 14, 15, 17]),
            // `http://host:80/p?q#f` -> `http://host:80/p#f`
            ([4, 11, 14, 16, 18], Part::Query, 0, [4, 11, 14, 16, 16]),
        ];
        for (before, part, len, after) in test_cases {
            let mut result: Offsets = offsets(*before);
            result.resize(*part, *len);
            assert_eq!(result, offsets(*after), "part={:?}", part);
        }
    }
}
