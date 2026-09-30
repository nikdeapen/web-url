# Notes

## Security

- `InvalidUrlError` includes the URL in its `Debug` output. This can leak credentials.

## Safety

- The `unsafe` blocks have no safety comments by design. Each wraps an unchecked operation such as
  `new_unchecked` on input that is already known to be valid.
- The `unsafe` functions such as `new_unchecked` skip runtime validation for performance. The rest
  of the crate & the `address` crate may rely on the invariants they assume.

## URLs

- URLs are ASCII by design. Every validator rejects non-ASCII chars, so slicing a URL at a byte
  offset never splits a char.
- URLs have no equality with strings by design. URLs are normalized, so it is ambiguous whether
  `url == "https://example.com"` should hold. Callers compare `as_str()`.
- User info is rejected rather than discarded, since discarding it would silently drop credentials.

### Parts

- The parts have no equality with strings by design. The path, query, & fragment strings start
  with their '/', '?', or '#' delimiter, so it is ambiguous whether `fragment == "section"` should
  hold. Callers compare `value()` or `as_str()`.
- An empty query is still a query: `/?` has one empty param & `/` has none. Absence is modeled with
  `Option`, so `Query` & `Fragment` have no `Default`.

## Errors

- `InvalidUrlError` has no `source` by design. Its message is the parse error's message, so a
  chain would print it twice. Callers get the typed parse error from `error()`.
