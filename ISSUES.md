# Issues

## Features

- Add support for user-info.

## Normalization

- Elide ports for known protocols like http.
- Normalize trailing dots in domains: `example.com.`.
- Normalize the percent-encoding; it must run before the dot-segments since `%2e` decodes to '.'.

## Utilities

- Add percent-encoding encode & decode helpers.
- Add `TryFrom<&str>` for `WebUrl`; only `FromStr` & `TryFrom<String>` exist.
- Add `WebUrl::iter_params` yielding the query params, empty when there is no query.
- Add `WebUrl::host_str`; the `HostRef` from `host()` drops the '[]' brackets of an IPv6 host.

## Performance

- Check the URL length before it is normalized so `UrlTooLong` never allocates.
- Build the canonical host string once per parse & carry it in the parts.
- Skip the `replace_params` query rebuild when no param matches.
- Append the added param in place when the URL has no fragment.

## Parsing

- Make `TryFrom<String>` recover the original string on error, not the normalized one.

## Design

- Group the `WebUrl` offsets into an `Offsets` struct; `new_unchecked` takes eight positional args.
- Fold `remove_params` & `replace_params` into one query-rebuild helper.
- Unify the splice mutators behind one offset-shifting helper on `Offsets`.
- Decide whether `Path`, `Query`, & `Fragment` should share one generic segment type.
- Name the `Query::iter_params` iterator; it exposes `Map<PieceIterator, fn>` as a public type.

## Testing

- Add fuzz or property tests for the parse & normalize invariants.
- Property-test `canonical_path_len` against `write_canonical_path`; they are different algorithms.
- Cover the untested `Display` & `Debug` impls & the `value` accessors.
