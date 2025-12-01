# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.4](https://github.com/hseeberger/api-version/compare/v0.3.3...v0.3.4) - 2025-12-01

### Other

- add CODEOWNERS ([#36](https://github.com/hseeberger/api-version/pull/36))
- fix README ([#35](https://github.com/hseeberger/api-version/pull/35))
- extend/improve Rustdoc ([#33](https://github.com/hseeberger/api-version/pull/33))

## [0.3.3](https://github.com/hseeberger/api-version/compare/v0.3.2...v0.3.3) - 2025-11-21

### Other

- use RELEASE_PLZ_TOKEN in Release-plz workflow
- remove on push trigger from CI workflow
- *(deps)* bump Rust to 1.91.1, bump various deps ([#29](https://github.com/hseeberger/api-version/pull/29))
- *(deps)* bump Rust to 1.90.0 ([#27](https://github.com/hseeberger/api-version/pull/27))

## [0.3.2](https://github.com/hseeberger/api-version/compare/v0.3.1...v0.3.2) - 2025-08-15

### Other

- *(deps)* bump Rust to 1.89.0 ([#25](https://github.com/hseeberger/api-version/pull/25))

## [0.3.1](https://github.com/hseeberger/api-version/compare/v0.3.0...v0.3.1) - 2025-07-16

### Other

- *(deps)* bump Rust to 1.88.0 ([#23](https://github.com/hseeberger/api-version/pull/23))
- add example ([#22](https://github.com/hseeberger/api-version/pull/22))
- add docs badge
- minor clarifications, more consistency
- docs, Rust 1.85.1, better tests

## [0.3.0](https://github.com/hseeberger/api-version/compare/v0.2.0...v0.3.0) - 2025-03-17

### Added

- allow valid version prefix ([#18](https://github.com/hseeberger/api-version/pull/18))

## [0.2.0](https://github.com/hseeberger/api-version/compare/v0.1.2...v0.2.0) - 2025-03-16

### Added

- change version range to 0u16..10_000 ([#13](https://github.com/hseeberger/api-version/pull/13))
- add validated ApiVersions ([#11](https://github.com/hseeberger/api-version/pull/11))
- remove declarative macro to create ApiVersionLayer ([#9](https://github.com/hseeberger/api-version/pull/9))

### Other

- extend doc comments ([#14](https://github.com/hseeberger/api-version/pull/14))
- use async fn in ApiVersionFilter ([#16](https://github.com/hseeberger/api-version/pull/16))

## [0.1.2](https://github.com/hseeberger/api-version/compare/v0.1.1...v0.1.2) - 2025-03-16

### Other

- use LazyLock instead of OnceLock ([#7](https://github.com/hseeberger/api-version/pull/7))

## [0.1.1](https://github.com/hseeberger/api-version/compare/v0.1.0...v0.1.1) - 2025-03-14

### Other

- use edition 2024 ([#5](https://github.com/hseeberger/api-version/pull/5))
- release v0.1.0 ([#3](https://github.com/hseeberger/api-version/pull/3))

## [0.1.0](https://github.com/hseeberger/api-version/releases/tag/v0.1.0) - 2025-03-14

### Added

- add release workflow ([#5](https://github.com/hseeberger/api-version/pull/5))
- shared cache and sequencing of jobs
- consolidating jobs and reusing toolchain

### Fixed

- release workflow
- fix issue introduced earlier
- naming
- check for secret before uploading to Codecov
- needed rustfmt not clippy for fmt-check
- explicitly installing clippy and rustfmt

### Other

- remove scndcloud ([#2](https://github.com/hseeberger/api-version/pull/2))
- make clippy happy ([#1](https://github.com/hseeberger/api-version/pull/1))
- bump deps and more
- *(deps)* bump thiserror from 1.0.62 to 1.0.63 ([#43](https://github.com/hseeberger/api-version/pull/43))
- *(deps)* bump tokio from 1.38.0 to 1.38.1 ([#42](https://github.com/hseeberger/api-version/pull/42))
- *(deps)* bump thiserror from 1.0.61 to 1.0.62 ([#41](https://github.com/hseeberger/api-version/pull/41))
- *(deps)* bump MarcoIeni/release-plz-action from 0.5.61 to 0.5.62 ([#40](https://github.com/hseeberger/api-version/pull/40))
- *(deps)* bump regex from 1.10.4 to 1.10.5 ([#39](https://github.com/hseeberger/api-version/pull/39))
- *(deps)* bump MarcoIeni/release-plz-action from 0.5.60 to 0.5.61 ([#38](https://github.com/hseeberger/api-version/pull/38))
- *(deps)* bump tokio from 1.37.0 to 1.38.0 ([#37](https://github.com/hseeberger/api-version/pull/37))
- *(deps)* bump MarcoIeni/release-plz-action from 0.5.59 to 0.5.60 ([#36](https://github.com/hseeberger/api-version/pull/36))
- *(deps)* bump MarcoIeni/release-plz-action from 0.5.58 to 0.5.59 ([#35](https://github.com/hseeberger/api-version/pull/35))
- *(deps)* bump axum-extra from 0.9.2 to 0.9.3 ([#28](https://github.com/hseeberger/api-version/pull/28))
- *(deps)* bump thiserror from 1.0.60 to 1.0.61 ([#34](https://github.com/hseeberger/api-version/pull/34))
- *(deps)* bump MarcoIeni/release-plz-action from 0.5.57 to 0.5.58 ([#33](https://github.com/hseeberger/api-version/pull/33))
- *(deps)* bump MarcoIeni/release-plz-action from 0.5.55 to 0.5.57 ([#32](https://github.com/hseeberger/api-version/pull/32))
- *(deps)* bump thiserror from 1.0.59 to 1.0.60 ([#31](https://github.com/hseeberger/api-version/pull/31))
- *(actions)* add release-plz pipeline
- *(deps)* bump thiserror from 1.0.58 to 1.0.59 ([#30](https://github.com/hseeberger/api-version/pull/30))
- *(deps)* bump tokio from 1.36.0 to 1.37.0 ([#29](https://github.com/hseeberger/api-version/pull/29))
- *(deps)* bump regex from 1.10.3 to 1.10.4 ([#27](https://github.com/hseeberger/api-version/pull/27))
- *(deps)* bump axum from 0.7.4 to 0.7.5 ([#26](https://github.com/hseeberger/api-version/pull/26))
- *(deps)* bump thiserror from 1.0.57 to 1.0.58 ([#25](https://github.com/hseeberger/api-version/pull/25))
- release version 0.1.0
- fix release.toml, update deps
- *(deps)* bump thiserror from 1.0.56 to 1.0.57 ([#24](https://github.com/hseeberger/api-version/pull/24))
- *(deps)* bump tokio from 1.35.1 to 1.36.0 ([#23](https://github.com/hseeberger/api-version/pull/23))
- output error chain
- Merge remote-tracking branch 'ci-conf/main'
- cleanup
- update GH actions
- deactivate coverage step of ci workflow
- fix release workflow
- fix doc job just invocation
- fix doc job toolchain
- fix doc job in ci workflow
- minor fixes
- ci workflow works for stable or nightly
- fix permissions for ci workflow
- make dependabot job depend on lint
- include auto-merge-dependabot-pr in ci workflow
- use stable Rust
- cleanup CI workflow
- automatically merge dependabot PRs ([#4](https://github.com/hseeberger/api-version/pull/4))
- cleanup/simplify CI workflow
- minor formatting (blank line) ([#2](https://github.com/hseeberger/api-version/pull/2))
- initial GH workflows
