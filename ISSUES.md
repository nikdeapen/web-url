# Issues

Known issues & planned work for the `web-url` crate.

## 1. Support user info

The RFC 3986 authority is `[ userinfo "@" ] host [ ":" port ]` but this library supports only the
host & the optional port. URLs with user info are currently rejected:

    http://user:pass@host/    ->    Err(UserInfoNotSupported)

User info should be parsed & **preserved**. It is rejected rather than scrubbed during normalization
because scrubbing would silently drop credentials, leaving the caller with an unauthenticated URL &
no indication why. Every other normalization in the crate preserves information, so a lossy one
would be the odd case out.

### Scope

- Parse & validate the user info char set. The RFC allows `unreserved / pct-encoded / sub-delims`
  and `":"`.
- Add a `UserInfo` component type matching the existing `Scheme`, `Path`, `Query`, & `Fragment`
  types, plus a `WebUrl::user_info()` accessor returning an `Option`.
- Add a `user_info_end` index to `WebUrl` & shift the host indices accordingly. Every index after
  the scheme moves, so `parse_pre_path`, `PrePath`, & `finalize_web_url` all need updating.
- Update the format in the `WebUrl` docs to `scheme://user_info@host:port/path?query#fragment`.
- Remove the `UserInfoNotSupported` error variant, or repurpose it for an invalid user info.
- Decide whether `Display` & `as_str` should redact the password. RFC 3986 section 3.2.1 deprecates
  the `user:password` form & section 7.5 warns against rendering it as clear text. Redacting in
  `Display` while keeping `as_str` exact is one option.

### Notes

- The `user:password` form is deprecated by the RFC but remains common in git remotes, proxy
  configs, database URLs, & CI configuration, which is why rejecting is preferred over scrubbing.
- The '@' char is invalid in both a domain name & an IPv6 literal, so an '@' char anywhere in the
  authority unambiguously indicates user info. This is what `check_no_user_info` relies on.
- Watch out for `http://bank.com@evil.com/`, where the host is `evil.com`. Any user-facing rendering
  of a URL with user info should make the real host obvious.

## 2. Trailing-dot hosts are rejected (upstream `address` rule)

`Domain::is_valid_name` rejects a trailing dot, so a fully-qualified host with an explicit root
label does not parse:

    http://example.com./    ->    Err(InvalidHost)

A trailing dot is valid in DNS, accepted by browsers, & syntactically fine as an RFC 3986
`reg-name` since '.' is an unreserved char. The rule is in the `address` crate. In
`address-0.19.0-rc.1`, `src/domain/validation.rs:51`: once the loop consumes `a.` the remainder is
empty & `is_valid_label` rejects an empty label, which is right for `a..b` & wrong for `a.`. The
behavior is pinned by the `("a.", false, false)` test case on line 117.

### Scope

- Decide whether to accept a trailing dot at all. It affects host comparison & normalization:
  `example.com` & `example.com.` are the same host with different spellings, which is the same
  class of problem as issue 3.
- If accepted, allow the empty final label in `Domain::is_valid_name`, release `address`, & bump
  the dependency here.
- Add trailing-dot cases to the `web-url` host tests either way.

### Notes

- This cannot be fixed inside `web-url`. `parse_ip_and_validate_domain` delegates host validation
  entirely to `Domain::is_valid_name_str`, so there is no hook to widen the rule locally.
- WHATWG strips one trailing dot before its "ends in a number" test, so this interacts with the
  numeric-host decision in issue 4.
- The consecutive-hyphen rule that rejected every `xn--` IDN host was fixed in `address-0.19.0` &
  is pinned by the `xn--bcher-kva.example` host tests here.

## 3. IPv6 hosts are only half-normalized

The letter case of an IPv6 literal is normalized but its zero groups are not, so the same address
has more than one accepted spelling:

    http://[0:0:0:0:0:0:0:1]/    ->    http://[0:0:0:0:0:0:0:1]/    but    host() == ::1
    http://[::AB]/               ->    http://[::ab]/               (the case *is* normalized)

The cause is `write_normalized` in `src/parse/parts.rs`, which copies the authority up to
`host_end()` verbatim. The host is only lowercased in place once the URL string is built, which
covers the hex digits but not the zero groups, since collapsing those changes the length.

This is the odd case out. The scheme & host are lowercased, an empty port is dropped with its ':', &
a port with leading zeros is rewritten; an IPv6 literal is the one component left as it arrived.

The consequence is not cosmetic. `PartialEq`, `Ord`, & `Hash` compare `self.url` & nothing else, so
two URLs with an identical parsed `ip` are unequal & hash differently. A `HashSet<WebUrl>` holds
both `http://[::1]/` & `http://[0:0:0:0:0:0:0:1]/`, so any dedupe, cache key, or allow-list built on
`WebUrl` equality can be bypassed by respelling the address.

### Scope

- Add `canonical_host_len()` to `PrePath` beside `canonical_port_len()`, returning the rendered
  length of `ip` when the host is an IPv6 literal (plus 2 for the brackets) & `host_len` otherwise.
- Add `needs_host_rewrite()` beside `needs_port_rewrite()` & fold it into `Parts::is_normalized`.
- Redefine `canonical_len()` as `scheme_len + 3 + canonical_host_len() + canonical_port_len()`, &
  drop the `host_end()` doc claiming it is the same in the parsed & normalized URL.
- Render the IP in `write_normalized` rather than copying the host slice, & fix the `host_end`
  computation in `finalize_web_url`, which derives it from `pre_path.host_len`.
- Fix the `TryFrom<String>` fast path in `src/parse/try_from_str.rs`. It inserts the '/' at
  `slash_index()` in place whenever the port needs no rewrite; that index is only valid while
  nothing before it changes length, so it must check the host too.

### Notes

- Only IPv6 is affected. The RFC 3986 `dec-octet` rule admits exactly one spelling per octet, so a
  parsed IPv4 literal is already canonical. `010.0.0.1` is not an `IPv4address` at all; see issue 4.
- This is fixable here. `IPv6Address`'s `Display` in `address-0.19.0-rc.1` is RFC 5952 conformant,
  verified against `0:0:0:0:0:0:0:1`, `0:0:0:0:0:0:0:0`, `2001:0db8::0001`, `2001:db8:0:0:1:0:0:1`,
  `2001:DB8:AB::CD`, `::ffff:1.2.3.4`, & `1:0:0:2:0:0:0:3`.
- The IPv4-mapped form `::ffff:1.2.3.4` is canonical under RFC 5952 section 5 & must survive the
  rewrite rather than being folded to `::ffff:102:304`.
- Rewriting the host makes the normalized URL shorter than the parsed one for the first time. The
  `normalized_len` arithmetic already handles a shrinking pre-path since the port does the same, but
  a test that shrinks the host & the port at once is worth adding.

## 4. Numeric hosts that are not valid IPv4 parse as domain names

A dotted-numeric host that fails to parse as IPv4 is accepted as a registered name rather than
rejected, so `host()` reports it as a name:

    http://256.0.0.1/    ->    Ok, host() == HostRef::Name("256.0.0.1")
    http://1.2.3.4.5/    ->    Ok, host() == HostRef::Name("1.2.3.4.5")
    http://010.0.0.1/    ->    Ok, host() == HostRef::Name("010.0.0.1")
    http://1/            ->    Ok, host() == HostRef::Name("1")

The cause is the fall-through in `parse_ip_and_validate_domain` in `src/parse/pre_path/host.rs`.
When `IPv4Address::from_str` fails, the host goes to `Domain::is_valid_name_str`, which accepts
all-digit labels, so a malformed IPv4 literal is silently reclassified as a name.

**This is not a parsing bug.** RFC 3986 section 3.2.2 defines
`host = IP-literal / IPv4address / reg-name` & says only that a host matching `IPv4address` *should*
be treated as an address rather than a name. Since `dec-octet` caps each octet at 255 & admits one
spelling, neither `256.0.0.1` nor `010.0.0.1` matches `IPv4address`, so both fall to `reg-name` &
both satisfy it. This entry exists so the divergence from browsers is a decision, not an accident.

The WHATWG URL Standard, which browsers implement, runs the IPv4 parser whenever the host ends in a
number & fails outright with no fall-back to a name. It also reads each part with a radix, so a
leading '0' is octal. Under those rules `256.0.0.1` & `1.2.3.4.5` are hard errors, `010.0.0.1` is
`8.0.0.1`, & `1` is `0.0.0.1` — three errors & two addresses where this crate reports four names.

The risk is a caller that branches on `HostRef::Name` vs `HostRef::Address` to answer "is this a
literal IP?" — an SSRF allow-list, an egress filter, a rule blocking IP literals. It gets `Name` for
`http://010.0.0.1/`, then hands the string to a resolver that reads it as `8.0.0.1`, so the check &
the connection disagree about the host.

### Scope

Only if the decision is to match browsers:

- In `parse_ip_and_validate_domain`, reject rather than fall through when `IPv4Address::from_str`
  fails & the final label is entirely ASCII digits. This is the cheap half & closes the risk.
- Decide the trailing-dot case (`http://1.2.3.4./`). WHATWG strips one trailing dot before the
  "ends in a number" test, so this interacts with the trailing-dot discussion in issue 2.
- Decide whether to implement the WHATWG radix rules so `010.0.0.1`, `0x7f.1`, & `2130706433` parse
  as addresses instead of being rejected. That is a much larger change & is not required.
- Add host tests for the four examples above.

### Notes

- The recommendation is to reject, not to implement the radix rules. Rejecting is a few lines,
  removes the check-versus-connect mismatch, & keeps the crate's "validate & slice, never transform"
  character. Accepting `010.0.0.1` as `8.0.0.1` would rewrite a host into a form the caller never
  wrote, which no other part of the crate does.
- This is a breaking change either way. URLs that parse today would return `InvalidHost`, so it
  needs a note in the `WebUrl` docs that the host rule is narrower than RFC 3986.
- IPv6 literals are unaffected. They are bracketed & always go through `IPv6Address::from_str`, so
  there is no name fall-through.

## 5. No percent-encoding support

The crate neither validates nor decodes percent-encoding. The '%' char is ASCII punctuation, so it
passes the path, query, & fragment char sets like any other char, & there is no encode or decode API:

    http://host/a%20b    ->    Ok, path() == "/a%20b"    (never decoded to "/a b")
    http://host/%zz      ->    Ok, path() == "/%zz"      (not a valid escape sequence)
    http://host/a b      ->    Err(InvalidPath)

So a caller holding a path with a space has no in-crate way to reach a URL that parses, & a caller
reading a path back has no way to recover the bytes it encodes. A malformed escape such as `%zz` or
a lone trailing '%' is accepted as ordinary text.

This is consistent with what the crate is — a validator & slicer that never transforms the URL — but
it is undocumented, & it is the first thing a caller hits with a non-ASCII or space-bearing path.

### Scope

- Document the behavior on `Path`, `Query`, `Fragment`, & `Param` so the char-set docs say plainly
  that '%' is not special & that nothing is decoded.
- Decide whether to validate escape sequences, so `%zz` & a trailing '%' become errors. This is
  cheap & catches a real class of bug, but it rejects URLs that parse today.
- Decide whether to offer encode & decode helpers at all. RFC 3986 section 2.1 defines the encoding,
  but the reserved sets in section 2.2 differ per component, so one helper pair is not enough.

### Notes

- Decoding must never happen during parsing. Decoding `%2F` in a path would produce a '/' that
  changes the segment structure, & decoding `%26` in a query would produce a '&' that changes the
  parameter split. Any decode API has to return owned data & leave the URL itself alone.
- `iter_segments` & the param iterator split on the raw bytes, so they already behave correctly with
  respect to the point above.

## 6. Query mutation is append-only

`add_param` & `with_param` are the only query mutations. Nothing removes a parameter, replaces one,
or clears the query, so `add_param` can build a query that nothing in the crate can undo:

    url.add_param(Param::try_from("token=secret")?);    // no way back to the original URL

`set_fragment` is the only complete setter; there is no `set_path`, `set_port`, or `set_host`
either. Building a URL from parts means building a string & parsing it.

### Scope

- Add `remove_param`, `set_param` (replace-or-append), & `clear_query`. Removal has to rewrite the
  separators: dropping the first parameter must consume the '&' that follows it rather than the '?'
  that precedes it, & dropping the last remaining parameter must drop the '?' as well, since a query
  is never empty.
- Add `set_path`, `set_port`, & `set_host`. Each shifts every index after it, unlike `set_fragment`,
  which is last & shifts nothing, & unlike `add_param`, which shifts only the fragment.
- Decide whether the new setters panic on `MAX_LEN` like the existing mutations or return a `Result`.

### Notes

- Removal is the case that needs the tests. An empty region between separators is a parameter, so
  `"?&&"` has three of them & removing the middle one must yield `"?&"` rather than `"?"`.
- The existing mutations stay consistent by updating `query_end` in place. `set_path` & `set_host`
  would have to update every later index, which is the same arithmetic issue 3 needs for the host
  rewrite, so doing issue 3 first would establish the pattern.
