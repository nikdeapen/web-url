# Issues

## Parsing

- Add support for user-info. URLs like `http://user:pass@host/` currently fail with `UserInfoNotSupported`.
- Decide whether to accept trailing-dot hosts. A trailing dot is valid in DNS but `http://example.com./` fails with
  `InvalidHost`; the rule is upstream in `address`.
- Add percent-encoding encode & decode helpers. The escapes are validated but there is no encode or decode API.
- Check the URL length before it is normalized so `UrlTooLong` never allocates. `TryFrom<String>` recovers the
  normalized string rather than the original, breaking its unchanged-on-error contract.
- Add `TryFrom<&str>` for `WebUrl`. Every component type takes `TryFrom<&'a str>` but the URL takes only `FromStr` &
  `TryFrom<String>`, so every call site needs a `use std::str::FromStr` import.

## Construction

- Group the `WebUrl` offsets into an `Offsets` struct. `new_unchecked` takes eight positional args with four
  interchangeable `u32` offsets split by the `ip` & `port`, so a transposition only fails a `debug_assert`.

## Accessors

- Add `WebUrl::iter_params` yielding the query params, empty when there is no query. Callers must write
  `url.query().into_iter().flatten()`, which `remove_params` & `replace_params` already do internally.
- Add `WebUrl::host_str` for the normalized host string. The `HostRef` from `host()` displays an IPv6 address without
  its '[]' brackets, so there is no accessor for the bracketed form.

## Mutations

- Fold `remove_params` & `replace_params` into one query-rebuild helper. They share the scan-filter-splice loop &
  differ only in whether a replacement param is pushed on the first match.
- Unify the splice mutators behind one offset-shifting helper on `Offsets`. `set_scheme`, `set_host`, `set_port`,
  `set_path`, `set_query_str`, & `add_param` each save the trailing component lengths & rebuild the offsets by hand.

## Normalization

- Normalize the percent-encoding: uppercase the hex digits & decode the escaped unreserved chars. It must run before
  the dot-segments are removed since `%2e` decodes to '.'.
- Elide the `http` 80 & `https` 443 default ports when normalizing; other schemes keep their port. `http://host:80/`
  & `http://host/` are unequal, so equality-based dedupes & allow-lists can be bypassed.

## Structure

- Generate the string comparison impls with a macro. The six `PartialEq` impls plus `AsRef`, `Borrow`, `Debug`, &
  `Display` are repeated verbatim for `Scheme`, `Path`, `Query`, `Fragment`, `Param`, & `WebUrl`, ~50 lines each.
- Decide whether `Path`, `Query`, & `Fragment` should share one generic segment type. They differ only in the prefix
  char, the excluded chars, & the error variant, but three concrete types document better than one generic alias.

## Performance

- Build the canonical host string once per parse & carry it in the parts. `CanonicalHost::new` reformats the IP up to
  five times for an IP-address host.
- Audit the mutation paths for wasted allocations. `replace_params` rebuilds the whole query into a new `String` even
  when no param matches, then discards it & falls back to `add_param`.

## Testing

- Add fuzz or property tests for the parse & normalize invariants.
- Property-test `canonical_path_len` against `write_canonical_path`. They are two different algorithms, a reverse skip
  count & a forward truncating write, whose agreement sizes the URL allocation & is checked only by a case table.
- Cover the untested public API: the `Display` & `Debug` impls for `Scheme`, `Path`, `Query`, & `Fragment`, the
  `PartialEq`, `AsRef`, & `Borrow` impls, `Query::value`, `Error::message`, `InvalidUrlError`'s `Display` & `From`,
  & the `check_len` panic paths.

## Packaging

- Set `rust-version` in `Cargo.toml`. The crate needs Rust 1.85+ for `edition = "2024"`, but an older toolchain fails
  with an edition error rather than a clear MSRV message.
