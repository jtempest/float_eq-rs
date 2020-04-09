# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Implementation of `FloatEq` and `FloatDiff` for arrays of size 0 to 32
  (inclusive) where the type allows it.

## [0.1.3] - 2020-04-07
### Added
- Added codecov.io and coveralls.io support.
- Added 'actively-developed' maintenance badge.
- Basic usage documentation.

## [0.1.2] - 2020-04-07
### Added
- Support for `no_std`.
- Added the crate to the No Standard Library crates.io category.
- Tests for `Float::abs_diff` on NaNs.
- Travis CI build.
- A Contributing section to the readme.

### Fixed
- FloatDiff tests for f64 no longer test f32 values (oops!).

## [0.1.1] - 2020-04-05
### Added
- Added this changelog.
- Reworked the READMEs to be a bit more granular and provide more appropriate 
  information.
    - The API documentation `lib.rs` has been pared down a bit and several 
      sections moved to other places.
    - `crates-io.md` contains a short project introduction, links to alternative
      crates and future plans.
    - `README.md` is generated from `crates-io.md` and `LICENSE.md`. 
- Updated Cargo.toml:
    - Included a link to the documentation.
    - Added the crate to the Development Tools (Debugging) category.

## [0.1.0] - 2020-04-05
### Added
- Initial release.

[Unreleased]: https://github.com/jtempest/float_eq-rs/compare/0.1.3...HEAD
[0.1.3]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.3
[0.1.2]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.2
[0.1.1]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.1
[0.1.0]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.0