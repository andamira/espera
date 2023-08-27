# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added
- new features: `safest`, `unsafe`, `unsafest`, `nightly_docs`.
- add `devela` dependency.
- add `check.sh` script.

### Changed
- bump MSRV to `1.72.0`.
- rename `no-std` feature to `no_std`.
- deprecate `no-std` feature.
- require `unsafe` instead of `not(safe)`.

### Fixed
- update dependencies: `time`, `ahash`, `libc`, `libm`.
- refactor manifest.
- update aliases.
- updat docs.
- update CI.

## [0.2.0] - 2023-04-21

### Added
- absorb `repite` crate:
  - add types: `Looper`, `LoopStatus`, `Rate`, `RateStats`, `EsperaError`, `EsperaResult`.
- add dependencies: `ahash`, `arraydeque`, `log`, `sixbit`.
- impl `From<UnixTime32> UnixTime`.
- impl `TryFrom<UnixTime> for UnixTime32`.
- impl `From` and `TryFrom` for integer primitives.
- add module `all`, `unix` module public.

### Removed
- remove for now feature `wasm` & dependency `instant`.

### Changed
- bump MSRV to `1.63.0`.
- rename `Error`, `Result` to `EsperaError`, `EsperaResult`, respectively.

### Fixed
- manually impl `Debug` for `UnixTime` & `UnixTime32`.
- misc. updates.

## [0.1.0] - 2023-03-29

### Added
- add optional dependency `libm`.

## Removed
- remove `log` dependency.

### Changed
- make `timecode_*` functions not depend on `std`.
- rename `UnixTime64` to `UnixTime`.

## [0.0.5] - 2023-03-29

### Changes
- many updates and changes to `Month`, `Weekday` and `UnixTime*` types.

## [0.0.4] - 2023-03-27

### Added
- new types: `UnixTime32`, `UnixTime64`, `Month`, `Weekday`.
- new features: `alloc`, `no-std`.

### Changed
- make `Sleeper` dependant on `std`.

### Fixed
- misc. project updates.
- fix CI.

## [0.0.3] - 2023-02-07

### Added
- add `nightly` feature.

### Fixes
- fix `docs.rs` build.

## [0.0.2] - 2023-02-06

### Fixes
- improve documentation.
- show needed features in docs.rs.

## [0.0.1] - 2023-02-06

### Added
- `Sleeper` struct, `sleep4` macro, `timecode_f64` and `timecode_ns_u64` functions.


[unreleased]: https://github.com/andamira/espera/compare/v0.2.0...HEAD
[0.1.0]: https://github.com/andamira/espera/releases/tag/v0.1.0
[0.0.5]: https://github.com/andamira/espera/releases/tag/v0.0.5
[0.0.4]: https://github.com/andamira/espera/releases/tag/v0.0.4
[0.0.3]: https://github.com/andamira/espera/releases/tag/v0.0.3
[0.0.2]: https://github.com/andamira/espera/releases/tag/v0.0.2
[0.0.1]: https://github.com/andamira/espera/releases/tag/v0.0.1

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
