# api-version

[![license][license-badge]][license-url]
[![build][build-badge]][build-url]

[license-badge]: https://img.shields.io/github/license/hseeberger/api-version
[license-url]: https://github.com/hseeberger/api-version/blob/main/LICENSE
[build-badge]: https://img.shields.io/github/actions/workflow/status/hseeberger/api-version/ci.yaml
[build-url]: https://github.com/hseeberger/api-version/actions/workflows/ci.yaml

Axum middleware to rewrite a request such that a version prefix is added to the path. This is based on a set of versions and an optional `"x-api-version"` custom HTTP header: if no such header is present, the highest version is used. Yet this only applies to requests the URIs of which pass a filter; others are not rewritten.

Paths must not start with a version prefix, e.g. `"/v0"`.

## Example

```rust
use api_version::api_version;

let app = Router::new()
    .route("/", get(ok_0))
    .route("/v0/test", get(ok_0))
    .route("/v1/test", get(ok_1));

/// Create an [ApiVersionLayer] correctly initialized with non-empty and strictly monotonically
/// increasing versions in the given inclusive range as well as an [ApiVersionFilter] making all
/// requests be rewritten.
let app = api_version!(0..=1).layer(app);
```

## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
