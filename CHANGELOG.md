# Changelog

## [0.4.0](https://github.com/hseeberger/api-version/compare/v0.3.8...v0.4.0) - 2026-10-16

### Features

- Add XApiVersion::version accessor ([#180](https://github.com/hseeberger/api-version/pull/180))

### Bug Fixes

- Keep the full query when rewriting the path ([#186](https://github.com/hseeberger/api-version/pull/186))
- Implement XApiVersion::encode ([#181](https://github.com/hseeberger/api-version/pull/181))
- [**breaking**] Reject leading zeros in x-api-version header ([#179](https://github.com/hseeberger/api-version/pull/179))
- Reject empty base path with correct panic message ([#177](https://github.com/hseeberger/api-version/pull/177))
- [**breaking**] Reject invalid x-api-version header with 400 ([#176](https://github.com/hseeberger/api-version/pull/176))
- Recognize version prefix without trailing path ([#175](https://github.com/hseeberger/api-version/pull/175))
- Call the inner service made ready by poll_ready ([#173](https://github.com/hseeberger/api-version/pull/173))

### Documentation

- Turn on warnings for missing docs ([#120](https://github.com/hseeberger/api-version/pull/120))

### Performance

- Avoid allocations in version prefix check and parsing ([#185](https://github.com/hseeberger/api-version/pull/185))

### Miscellaneous Tasks

- Clean up style, docs and pedantic lints ([#183](https://github.com/hseeberger/api-version/pull/183))

## [0.3.8](https://github.com/hseeberger/api-version/compare/v0.3.7...v0.3.8) - 2026-07-16

### Miscellaneous Tasks

- Standardize project scaffolding and package metadata ([#100](https://github.com/hseeberger/api-version/pull/100))

## [0.3.7](https://github.com/hseeberger/api-version/compare/v0.3.6...v0.3.7) - 2026-07-01

### Other

- *(deps)* bump dtolnay/rust-toolchain
- *(deps)* bump MarcoIeni/release-plz-action in the ci-patches group
- align workflows
- *(deps)* bump the all group across 1 directory with 4 updates ([#94](https://github.com/hseeberger/api-version/pull/94))
- *(deps)* bump regex from 1.12.3 to 1.12.4 in the all group ([#92](https://github.com/hseeberger/api-version/pull/92))
- *(deps)* bump the all group with 2 updates ([#91](https://github.com/hseeberger/api-version/pull/91))
- *(deps)* bump taiki-e/install-action in the all group ([#90](https://github.com/hseeberger/api-version/pull/90))
- bump rustfmt edition to 2024 ([#88](https://github.com/hseeberger/api-version/pull/88))

## [0.3.6](https://github.com/hseeberger/api-version/compare/v0.3.5...v0.3.6) - 2026-05-29

### Miscellaneous Tasks

- *(build)* Apply common hseeberger project shape ([#76](https://github.com/hseeberger/api-version/pull/76))
- *(ci)* Add cooldown to dependabot

### Other

- Bump bytes to 1.11.1 to fix CVE-2026-25541 ([#79](https://github.com/hseeberger/api-version/pull/79))

## [0.3.5](https://github.com/hseeberger/api-version/compare/v0.3.4...v0.3.5) - 2026-03-25

### Other

- *(deps)* bump regex from 1.12.2 to 1.12.3 ([#51](https://github.com/hseeberger/api-version/pull/51))
- *(deps)* bump actions/checkout from 5.0.1 to 6.0.2 ([#50](https://github.com/hseeberger/api-version/pull/50))
- *(deps)* bump tracing from 0.1.43 to 0.1.44 ([#49](https://github.com/hseeberger/api-version/pull/49))
- *(deps)* bump taiki-e/install-action from 2.62.53 to 2.69.9 ([#48](https://github.com/hseeberger/api-version/pull/48))
- *(deps)* bump axum-extra from 0.12.2 to 0.12.5 ([#47](https://github.com/hseeberger/api-version/pull/47))
- *(deps)* bump tower from 0.5.2 to 0.5.3 ([#46](https://github.com/hseeberger/api-version/pull/46))
- *(deps)* bump axum from 0.8.7 to 0.8.8 ([#45](https://github.com/hseeberger/api-version/pull/45))
- *(deps)* bump MarcoIeni/release-plz-action from 0.5.118 to 0.5.128 ([#44](https://github.com/hseeberger/api-version/pull/44))
- *(deps)* bump dtolnay/rust-toolchain ([#43](https://github.com/hseeberger/api-version/pull/43))
- *(deps)* bump Swatinem/rust-cache from 2.8.1 to 2.9.1 ([#42](https://github.com/hseeberger/api-version/pull/42))

## [0.3.4](https://github.com/hseeberger/api-version/compare/v0.3.3...v0.3.4) - 2026-01-07

### Bug Fixes

- Use full base path for rewrite decision ([#41](https://github.com/hseeberger/api-version/pull/41))

### Documentation

- Fix README ([#35](https://github.com/hseeberger/api-version/pull/35))
- Extend/improve Rustdoc ([#33](https://github.com/hseeberger/api-version/pull/33))

## [0.3.3](https://github.com/hseeberger/api-version/compare/v0.3.2...v0.3.3) - 2025-11-21

### Miscellaneous Tasks

- Bump Rust to 1.90.0 ([#27](https://github.com/hseeberger/api-version/pull/27))

## [0.3.2](https://github.com/hseeberger/api-version/compare/v0.3.1...v0.3.2) - 2025-08-15

### Miscellaneous Tasks

- Bump Rust to 1.89.0 ([#25](https://github.com/hseeberger/api-version/pull/25))

## [0.3.1](https://github.com/hseeberger/api-version/compare/v0.3.0...v0.3.1) - 2025-07-16

### Documentation

- Add docs badge
- Minor clarifications, more consistency

### Miscellaneous Tasks

- Bump Rust to 1.88.0 ([#23](https://github.com/hseeberger/api-version/pull/23))
- Add example ([#22](https://github.com/hseeberger/api-version/pull/22))
- Docs, Rust 1.85.1, better tests

## [0.3.0](https://github.com/hseeberger/api-version/compare/v0.2.0...v0.3.0) - 2025-03-17

### Features

- Allow valid version prefix ([#18](https://github.com/hseeberger/api-version/pull/18))

## [0.2.0](https://github.com/hseeberger/api-version/compare/v0.1.2...v0.2.0) - 2025-03-16

### Features

- Change version range to 0u16..10_000 ([#13](https://github.com/hseeberger/api-version/pull/13))
- Add validated ApiVersions ([#11](https://github.com/hseeberger/api-version/pull/11))
- Remove declarative macro to create ApiVersionLayer ([#9](https://github.com/hseeberger/api-version/pull/9))

### Refactor

- Use async fn in ApiVersionFilter ([#16](https://github.com/hseeberger/api-version/pull/16))

### Documentation

- Extend doc comments ([#14](https://github.com/hseeberger/api-version/pull/14))

## [0.1.2](https://github.com/hseeberger/api-version/compare/v0.1.1...v0.1.2) - 2025-03-16

### Refactor

- Use LazyLock instead of OnceLock ([#7](https://github.com/hseeberger/api-version/pull/7))

## [0.1.1](https://github.com/hseeberger/api-version/compare/v0.1.0...v0.1.1) - 2025-03-14

### Miscellaneous Tasks

- Use edition 2024 ([#5](https://github.com/hseeberger/api-version/pull/5))
- Remove release.toml

## [0.1.0](https://github.com/hseeberger/api-version/releases/tag/v0.1.0) - 2025-03-14

### Features

- Add release workflow ([#5](https://github.com/hseeberger/api-version/pull/5))
- No longer pass through root path ([#17](https://github.com/hseeberger/api-version/pull/17))
- Simplify imports ([#16](https://github.com/hseeberger/api-version/pull/16))
- Reexport array-marco ([#15](https://github.com/hseeberger/api-version/pull/15))
- Shared cache and sequencing of jobs ([#1](https://github.com/hseeberger/api-version/pull/1))
- Consolidating jobs and reusing toolchain
- Filter returns result ([#5](https://github.com/hseeberger/api-version/pull/5))
- Better api_version macro ([#4](https://github.com/hseeberger/api-version/pull/4))
- Filter ([#3](https://github.com/hseeberger/api-version/pull/3))
- Rewrite based on Tower service ([#1](https://github.com/hseeberger/api-version/pull/1))

### Bug Fixes

- Release workflow
- Fix issue introduced earlier
- Naming
- Check for secret before uploading to Codecov
- Needed rustfmt not clippy for fmt-check
- Explicitly installing clippy and rustfmt
- Make usable

### Documentation

- Typo fixes
- Sync README
- Minor fixes

### Miscellaneous Tasks

- Remove scndcloud ([#2](https://github.com/hseeberger/api-version/pull/2))
- Make clippy happy ([#1](https://github.com/hseeberger/api-version/pull/1))
- Bump deps and more
- Bump thiserror from 1.0.62 to 1.0.63 ([#43](https://github.com/hseeberger/api-version/pull/43))
- Bump tokio from 1.38.0 to 1.38.1 ([#42](https://github.com/hseeberger/api-version/pull/42))
- Bump thiserror from 1.0.61 to 1.0.62 ([#41](https://github.com/hseeberger/api-version/pull/41))
- Bump MarcoIeni/release-plz-action from 0.5.61 to 0.5.62 ([#40](https://github.com/hseeberger/api-version/pull/40))
- Bump regex from 1.10.4 to 1.10.5 ([#39](https://github.com/hseeberger/api-version/pull/39))
- Bump MarcoIeni/release-plz-action from 0.5.60 to 0.5.61 ([#38](https://github.com/hseeberger/api-version/pull/38))
- Bump tokio from 1.37.0 to 1.38.0 ([#37](https://github.com/hseeberger/api-version/pull/37))
- Bump MarcoIeni/release-plz-action from 0.5.59 to 0.5.60 ([#36](https://github.com/hseeberger/api-version/pull/36))
- Bump MarcoIeni/release-plz-action from 0.5.58 to 0.5.59 ([#35](https://github.com/hseeberger/api-version/pull/35))
- Bump axum-extra from 0.9.2 to 0.9.3 ([#28](https://github.com/hseeberger/api-version/pull/28))
- Bump thiserror from 1.0.60 to 1.0.61 ([#34](https://github.com/hseeberger/api-version/pull/34))
- Bump MarcoIeni/release-plz-action from 0.5.57 to 0.5.58 ([#33](https://github.com/hseeberger/api-version/pull/33))
- Bump MarcoIeni/release-plz-action from 0.5.55 to 0.5.57 ([#32](https://github.com/hseeberger/api-version/pull/32))
- Bump thiserror from 1.0.59 to 1.0.60 ([#31](https://github.com/hseeberger/api-version/pull/31))
- *(actions)* Add release-plz pipeline
- Bump thiserror from 1.0.58 to 1.0.59 ([#30](https://github.com/hseeberger/api-version/pull/30))
- Bump tokio from 1.36.0 to 1.37.0 ([#29](https://github.com/hseeberger/api-version/pull/29))
- Bump regex from 1.10.3 to 1.10.4 ([#27](https://github.com/hseeberger/api-version/pull/27))
- Bump axum from 0.7.4 to 0.7.5 ([#26](https://github.com/hseeberger/api-version/pull/26))
- Bump thiserror from 1.0.57 to 1.0.58 ([#25](https://github.com/hseeberger/api-version/pull/25))
- Bump thiserror from 1.0.56 to 1.0.57 ([#24](https://github.com/hseeberger/api-version/pull/24))
- Bump tokio from 1.35.1 to 1.36.0 ([#23](https://github.com/hseeberger/api-version/pull/23))
- Output error chain
- Cleanup
- Bump codecov/codecov-action from 3 to 4 ([#22](https://github.com/hseeberger/api-version/pull/22))
- Bump regex from 1.10.2 to 1.10.3 ([#21](https://github.com/hseeberger/api-version/pull/21))
- Bump axum from 0.7.3 to 0.7.4 ([#19](https://github.com/hseeberger/api-version/pull/19))
- Bump axum-extra from 0.9.1 to 0.9.2 ([#20](https://github.com/hseeberger/api-version/pull/20))
- Bump thiserror from 1.0.53 to 1.0.56 ([#18](https://github.com/hseeberger/api-version/pull/18))
- Bump axum from 0.7.2 to 0.7.3 ([#13](https://github.com/hseeberger/api-version/pull/13))
- Bump thiserror from 1.0.52 to 1.0.53 ([#14](https://github.com/hseeberger/api-version/pull/14))
- Bump axum-extra from 0.9.0 to 0.9.1 ([#12](https://github.com/hseeberger/api-version/pull/12))
- Cleanup CI workflow
- Cleanup/simplify CI workflow
- Minor formatting (blank line) ([#2](https://github.com/hseeberger/api-version/pull/2))
- Bump thiserror from 1.0.51 to 1.0.52 ([#9](https://github.com/hseeberger/api-version/pull/9))
- Bump futures from 0.3.29 to 0.3.30 ([#8](https://github.com/hseeberger/api-version/pull/8))
- Bump tokio from 1.35.0 to 1.35.1 ([#7](https://github.com/hseeberger/api-version/pull/7))
- Bump thiserror from 1.0.50 to 1.0.51 ([#6](https://github.com/hseeberger/api-version/pull/6))
- Better naming ([#2](https://github.com/hseeberger/api-version/pull/2))
- Add license, Cargo metadata, etc.

### Other

- Merge remote-tracking branch 'ci-conf/main'
- Merge remote-tracking branch 'ci-conf/main'
- Merge remote-tracking branch 'ci-conf/main'
- Merge remote-tracking branch 'ci-conf/main'
- Merge remote-tracking branch 'ci-conf/main'
- Merge remote-tracking branch 'ci-conf/main'
- Merge remote-tracking branch 'ci-conf/main'
- Merge remote-tracking branch 'ci-conf/main'
- Merge remote-tracking branch 'ci-conf/main'
- Initial GH workflows
- First stab
