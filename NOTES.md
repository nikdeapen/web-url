# Notes

# URLs

- URLs are ASCII-only by design. Other characters must be IDNA or percent encoded.
- URL parts are not string-equal by design. This is to avoid confusion with `#frag` & `frag` etc.
- An empty query is still a query: `/?` has one parameter with an empty name and a `None` value.

## Security

- `InvalidUrlError` includes the URL in its `Debug` output. This can leak credentials.
