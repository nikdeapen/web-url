# Issues

Known issues & planned work for the `web-url` crate.

## Parsing

- Add support for user-info. URLs like `http://user:pass@host/` currently fail with `UserInfoNotSupported`.
- Decide whether to accept trailing-dot hosts. A trailing dot is valid in DNS but `http://example.com./` fails with
  `InvalidHost`; the rule is upstream in `address`.
- Decide whether to reject dotted-numeric hosts that are not valid IPv4, like browsers do. `http://256.0.0.1/` parses
  as a domain name, so IP-literal checks disagree with what a resolver connects to.
- Decide whether to validate percent-encoding & add encode/decode helpers. The '%' char is ordinary text, so `%zz` is
  accepted & there is no encode or decode API.

## Validation

- Tighten `Path` to the RFC 3986 path chars. Chars like '<', '>', '[', ']', '{', '}', '\', '^', & '"' are accepted.
- Tighten `Query` to the RFC 3986 query chars. The same extra punctuation as `Path` is accepted.
- Tighten `Param` names & values to the RFC 3986 query chars. The same extra punctuation as `Query` is accepted.
- Tighten `Fragment` to the RFC 3986 fragment chars. All punctuation is accepted, including '#' itself.

## Normalization

- Normalize the zero groups of IPv6 hosts. `[::1]` & `[0:0:0:0:0:0:0:1]` are unequal & hash differently, so
  equality-based dedupes & allow-lists can be bypassed.
- Remove dot-segments when normalizing paths. `/a/../b` & `/b` are unequal & path allow-lists can be bypassed
  since `/public/../admin` starts with `/public/`.
- Elide the `http` 80 & `https` 443 default ports when normalizing; other schemes keep their port. `http://host:80/`
  & `http://host/` are unequal, so equality-based dedupes & allow-lists can be bypassed.

## Mutations

- Add query parameter removal & replacement. `add_param` is append-only, so an added param cannot be undone.
- Add `set_path`, `set_port`, & `set_host`. Only the fragment has a full setter.

## Testing

- Add fuzz or property tests for the parse & normalize invariants.
