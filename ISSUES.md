# Issues

## Parsing

- Add support for user-info. URLs like `http://user:pass@host/` currently fail with `UserInfoNotSupported`.
- Decide whether to accept trailing-dot hosts. A trailing dot is valid in DNS but `http://example.com./` fails with
  `InvalidHost`; the rule is upstream in `address`.
- Decide whether to reject dotted-numeric hosts that are not valid IPv4, like browsers do. `http://256.0.0.1/` parses
  as a domain name, so IP-literal checks disagree with what a resolver connects to.
- Add percent-encoding encode & decode helpers. The escapes are validated but there is no encode or decode API.
- Check the URL length before it is normalized so `UrlTooLong` never allocates. `TryFrom<String>` recovers the
  normalized string rather than the original, breaking its unchanged-on-error contract.
- Add `TryFrom<&str>` for `WebUrl`. Every component type takes `TryFrom<&'a str>` but the URL takes only `FromStr` &
  `TryFrom<String>`, so every call site needs a `use std::str::FromStr` import.

## Accessors

- Add `WebUrl::iter_params` yielding the query params, empty when there is no query. Callers must write
  `url.query().into_iter().flatten()`, which `remove_params` & `replace_params` already do internally.

## Mutations

- Fold `remove_params` & `replace_params` into one query-rebuild helper. They share the scan-filter-splice loop &
  differ only in whether a replacement param is pushed on the first match.

## Normalization

- Normalize the percent-encoding: uppercase the hex digits & decode the escaped unreserved chars. It must run before
  the dot-segments are removed since `%2e` decodes to '.'.
- Elide the `http` 80 & `https` 443 default ports when normalizing; other schemes keep their port. `http://host:80/`
  & `http://host/` are unequal, so equality-based dedupes & allow-lists can be bypassed.

## Performance

- Build the canonical host string once per parse & carry it in the parts. `CanonicalHost::new` reformats the IP up to
  five times for an IP-address host.
- Audit the mutation paths for wasted allocations. `replace_params` rebuilds the whole query into a new `String` even
  when no param matches, then discards it & falls back to `add_param`.

## Testing

- Add fuzz or property tests for the parse & normalize invariants.
