# Issues

## Parsing

- Add support for user-info. URLs like `http://user:pass@host/` currently fail with `UserInfoNotSupported`.
- Decide whether to accept trailing-dot hosts. A trailing dot is valid in DNS but `http://example.com./` fails with
  `InvalidHost`; the rule is upstream in `address`.
- Decide whether to reject dotted-numeric hosts that are not valid IPv4, like browsers do. `http://256.0.0.1/` parses
  as a domain name, so IP-literal checks disagree with what a resolver connects to.
- Decide whether to validate percent-encoding & add encode/decode helpers. The '%' char is ordinary text, so `%zz` is
  accepted & there is no encode or decode API.

## Normalization

- Normalize the zero groups of IPv6 hosts. `[::1]` & `[0:0:0:0:0:0:0:1]` are unequal & hash differently, so
  equality-based dedupes & allow-lists can be bypassed.
- Remove dot-segments when normalizing paths. `/a/../b` & `/b` are unequal & path allow-lists can be bypassed
  since `/public/../admin` starts with `/public/`.
- Elide the `http` 80 & `https` 443 default ports when normalizing; other schemes keep their port. `http://host:80/`
  & `http://host/` are unequal, so equality-based dedupes & allow-lists can be bypassed.

## Testing

- Add fuzz or property tests for the parse & normalize invariants.
