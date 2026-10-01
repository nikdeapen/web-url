/// A part of a web-based URL string that ends at one of the `Offsets`.
///
/// The parts are in URL order, which `Offsets::resize` relies on.
#[must_use]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum Part {
    /// The scheme.
    Scheme,

    /// The host. (including the '[]' brackets of an IPv6 address)
    Host,

    /// The port. (including the ':')
    Port,

    /// The path.
    Path,

    /// The query. (including the '?')
    Query,
}
