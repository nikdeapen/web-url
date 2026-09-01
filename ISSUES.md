# Issues

## Security

- Elide the `http` 80 & `https` 443 default ports; `http://host:80/` & `http://host/` are unequal.

## Parsing

- Add support for user-info; `http://user:pass@host/` fails with `UserInfoNotSupported`.
- Decide whether to accept trailing-dot hosts; the rule is upstream in `address`.
- Add percent-encoding encode & decode helpers.
- Check the URL length before it is normalized so `UrlTooLong` never allocates.
- Make `TryFrom<String>` recover the original string on error, not the normalized one.
- Add `TryFrom<&str>` for `WebUrl`; only `FromStr` & `TryFrom<String>` exist.

## Construction

- Group the `WebUrl` offsets into an `Offsets` struct; `new_unchecked` takes eight positional args.

## Accessors

- Add `WebUrl::iter_params` yielding the query params, empty when there is no query.
- Add `WebUrl::host_str`; the `HostRef` from `host()` drops the '[]' brackets of an IPv6 host.

## Mutations

- Fold `remove_params` & `replace_params` into one query-rebuild helper.
- Unify the splice mutators behind one offset-shifting helper on `Offsets`.

## Normalization

- Normalize the percent-encoding; it must run before the dot-segments since `%2e` decodes to '.'.

## Structure

- Generate the string comparison impls with a macro; they are repeated verbatim for six types.
- Decide whether `Path`, `Query`, & `Fragment` should share one generic segment type.

## Performance

- Build the canonical host string once per parse & carry it in the parts.
- Skip the `replace_params` query rebuild when no param matches.
- Append the added param in place when the URL has no fragment.

## Testing

- Add fuzz or property tests for the parse & normalize invariants.
- Property-test `canonical_path_len` against `write_canonical_path`; they are different algorithms.
- Cover the untested `Display`, `Debug`, `PartialEq`, `AsRef`, & `Borrow` impls & `Query::value`.
