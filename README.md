# web-url

This library aids in processing web-based URLs.

Web-based URLs are URLs in the format:

```text
scheme://host:port/path?query#fragment
```

This is a subset of [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986#section-4.3). The port,
query, & fragment are optional. The path is never empty & always starts with a '/', so a URL parsed
without one is normalized to a path of '/'.

Parsing normalizes the URL. The scheme & host are lowercased, an empty port is dropped along with
its ':', & a port with leading zeros is rewritten. The path, query, & fragment are preserved exactly.

Parsing with `TryFrom<String>` reuses the allocation when the URL is already normalized & recovers
the original string on error. URLs can be mutated in place with `add_param` & `set_fragment`.

## Example

```rust
use std::str::FromStr;
use web_url::WebUrl;

let url = WebUrl::from_str("HTTPS://Example.com:0443/path?key=value#section").unwrap();

assert_eq!(url.as_str(), "https://example.com:443/path?key=value#section");
assert_eq!(url.scheme().as_str(), "https");
assert_eq!(url.host_str(), "example.com");
assert_eq!(url.port(), Some(443));
assert_eq!(url.path().as_str(), "/path");
assert_eq!(url.query().unwrap().as_str(), "?key=value");
assert_eq!(url.fragment().unwrap().as_str(), "#section");
```

## Features & Dependencies

```toml
web-url = "0.11.0-rc.3"
```

This crate has no features.

### Dependencies

```text
address     # used for host addresses (re-exported as `web_url::address`)
```

## Known Issues

Known issues & planned work are tracked in [ISSUES.md](ISSUES.md).
