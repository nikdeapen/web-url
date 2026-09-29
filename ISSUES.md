# Issues

## Performance

- Build the canonical host string once per parse & carry it in the parts.
- Skip the `replace_params` query rebuild when no param matches.
- Append the added param in place when the URL has no fragment.
- Splice the path in `set_path` directly when it has no dot-segments.

## Parsing

- Check the URL length before it is normalized so `UrlTooLong` never allocates or alters the string.

## Design

- Group the `WebUrl` offsets into an `Offsets` struct; `new_unchecked` takes eight positional args.
- Fold `remove_params` & `replace_params` into one query-rebuild helper.
- Unify the splice mutators behind one offset-shifting helper on `Offsets`.
- Decide whether `Path`, `Query`, & `Fragment` should share one generic segment type.

## Future Work

- Add support for user-info.
- Elide ports for known protocols like http.
- Normalize trailing dots in domains: `example.com.`.
- Normalize the percent-encoding; it must run before the dot-segments since `%2e` decodes to '.'.
- Add percent-encoding encode & decode helpers.
- Add `TryFrom<&str>` for `WebUrl`; only `FromStr` & `TryFrom<String>` exist.
- Add `WebUrl::params` yielding the query params, empty when there is no query.
- Add `WebUrl::host_str`; the `HostRef` from `host()` drops the '[]' brackets of an IPv6 host.
- Add a `parse(&[u8])` entry point like the `address` types; URLs are ASCII, so no UTF-8 pass.
- Add an optional `serde` feature like the `address` crate has.
- Add RFC 3986 reference resolution for joining a relative reference onto a base URL.
- Add fuzz or property tests for the parse & normalize invariants.
- Property-test `canonical_path_len` against `write_canonical_path`; they are different algorithms.
