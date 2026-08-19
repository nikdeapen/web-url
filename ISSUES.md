# Issues

## Parsing

- Add support for user-info. URLs like `http://user:pass@host/` currently fail with `UserInfoNotSupported`.
- Decide whether to accept trailing-dot hosts. A trailing dot is valid in DNS but `http://example.com./` fails with
  `InvalidHost`; the rule is upstream in `address`.
- Decide whether to reject dotted-numeric hosts that are not valid IPv4, like browsers do. `http://256.0.0.1/` parses
  as a domain name, so IP-literal checks disagree with what a resolver connects to.
- Decide whether to validate percent-encoding & add encode/decode helpers. The '%' char is ordinary text, so `%zz` is
  accepted & there is no encode or decode API.
- Check the URL length before it is normalized so `UrlTooLong` never allocates. `TryFrom<String>` recovers the
  normalized string rather than the original, breaking its unchanged-on-error contract.
- Add `TryFrom<&str>` for `WebUrl`. Every component type takes `TryFrom<&'a str>` but the URL takes only `FromStr` &
  `TryFrom<String>`, so every call site needs a `use std::str::FromStr` import.

## Mutations

- Fold `remove_params` & `replace_params` into one query-rebuild helper. They share the scan-filter-splice loop &
  differ only in whether a replacement param is pushed on the first match.

## Normalization

- Elide the `http` 80 & `https` 443 default ports when normalizing; other schemes keep their port. `http://host:80/`
  & `http://host/` are unequal, so equality-based dedupes & allow-lists can be bypassed.

## Performance

- Build the canonical host string once per parse & carry it in the parts. `CanonicalHost::new` reformats the IP up to
  five times for an IP-address host.

## Testing

- Add fuzz or property tests for the parse & normalize invariants.
