# api-version

[![license][license-badge]][license-url]
[![build][build-badge]][build-url]

[license-badge]: https://img.shields.io/github/license/hseeberger/api-version
[license-url]: https://github.com/hseeberger/api-version/blob/main/LICENSE
[build-badge]: https://img.shields.io/github/actions/workflow/status/hseeberger/api-version/ci.yaml
[build-url]: https://github.com/hseeberger/api-version/actions/workflows/ci.yaml

Axum middleware to rewrite a request such that a version prefix is added to the path. This is based on a set of API versions and an optional `"x-api-version"` custom HTTP header: if no such header is present, the highest version is used. Yet this only applies to requests the URIs of which pass a filter; others are not rewritten. Also, paths starting with a valid/existing version prefix, e.g. `"/v0"`, are not rewritten.

## Example

```rust
struct ReadyFilter;

impl ApiVersionFilter for ReadyFilter {
    type Error = Infallible;

    async fn should_rewrite(&self, uri: &Uri) -> Result<bool, Self::Error> {
        Ok(uri.path() != "/")
    }
}

let app = Router::new()
    .route("/", get(ready))
    .route("/v0/test", get(ok_0))
    .route("/v1/test", get(ok_1));

const API_VERSIONS: ApiVersions<2> = ApiVersions::new([0, 1]);

let app = ApiVersionLayer::new(API_VERSIONS, ReadyFilter).layer(app);
```

## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
