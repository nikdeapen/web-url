# web-url

[![Crates.io](https://img.shields.io/crates/v/web-url.svg)](https://crates.io/crates/web-url)
[![Docs.rs](https://docs.rs/web-url/badge.svg)](https://docs.rs/web-url)
[![License](https://img.shields.io/crates/l/web-url.svg)](https://github.com/nikdeapen/web-url/blob/master/LICENSE)

This library aids in processing web-based URLs — URLs in the format `scheme://host:port/path?query#fragment` — with
strict validation, normalizing parsers, & borrowed component types.

## Usage

```toml
web-url = "0.10.0"
```

## Example

```rust
use std::str::FromStr;
use web_url::{Param, WebUrl};

// Parsing normalizes the URL: the scheme & host are lowercased & the port is rewritten.
let url = WebUrl::from_str("HTTPS://Example.com:0443/path?key=value#section").unwrap();
assert_eq!(url.as_str(), "https://example.com:443/path?key=value#section");
assert_eq!(url.scheme().as_str(), "https");
assert_eq!(url.host_str(), "example.com");
assert_eq!(url.port(), Some(443));
assert_eq!(url.path().as_str(), "/path");
assert_eq!(url.query().unwrap().as_str(), "?key=value");
assert_eq!(url.fragment().unwrap().as_str(), "#section");

// URLs can be mutated in place; a URL parsed without a path gets the path '/'.
let url = WebUrl::from_str("https://example.com").unwrap().with_param(Param::try_from("page=2").unwrap());
assert_eq!(url.as_str(), "https://example.com/?page=2");
```

## Features

This crate has no features. The only dependency is the [`address`](https://crates.io/crates/address) crate.

## The URL Format

```text
scheme://host:port/path?query#fragment
```

The format is a subset of the [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986#section-3) URI syntax: the
authority is required, user info is rejected, & the path is never empty. The port, query, & fragment are optional.

The accepted chars are a superset of RFC 3986: the path, query, & fragment accept any US-ASCII letter, number, or
punctuation char outside their delimiters, so URLs this crate accepts may be rejected by strict RFC parsers.

## Normalization

Parsed URLs are always normalized: the scheme & host are lowercased, an empty port is dropped along with its ':', a
port with leading zeros is rewritten, & a URL parsed without a path gets the path '/'. The path, query, & fragment
are preserved exactly.

Parsing with `TryFrom<String>` reuses the allocation when the URL is already normalized & recovers the original
string on error.

## Component Types

The component types are borrowed, validated views. The `WebUrl` accessors return them borrowing from the URL string
& they can also be created from their own strings.

- `Scheme`: A lowercase URL scheme, with `HTTP` & `HTTPS` constants.
- `Path`: A URL path starting with '/', with an iterator for its segments.
- `Query`: A URL query starting with '?', with an iterator for its `Param`s.
- `Param`: A query parameter with a name & an optional value.
- `Fragment`: A URL fragment starting with '#'.

The host is an `address::HostRef`, either a domain name or an IP address. The `address` crate is re-exported as
`web_url::address`.

## Mutations

URLs can be mutated in place: `add_param` appends a query parameter & `set_fragment` sets or removes the fragment.
The `with_param` & `with_fragment` variants chain on owned URLs.
