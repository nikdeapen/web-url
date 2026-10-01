# Future Work

## URLs

- Add support for user-info.

## Normalization

- Elide ports for known protocols like http.
- Normalize trailing dots in domain parsing: `example.com.`.
- Normalize the percent-encoding; it must run before the dot-segments since `%2e` decodes to '.'.

## Mutations

- Add RFC 3986 reference resolution for joining a relative reference onto a base URL.

## Serialization

- Add a `parse(&[u8])` entry point like the `address` types; URLs are ASCII, so no UTF-8 pass.
- Add an optional `serde` feature like the `address` crate has.

## Testing

- Add fuzz or property tests for the parse & normalize invariants.
- Property-test `canonical_path_len` against `write_canonical_path`; they are different algorithms.
