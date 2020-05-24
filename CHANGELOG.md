# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- The `FloatEqAll` and `FloatEqAllDebug` traits, which allow for `abs_all`,
  `rel_all` and `ulps_all` to be used in the `float_eq` and `assert_float_eq`
  families of macros. This allows for checks that use the same epsilon threshold
  across all fields being checked. This used to be the implicit behaviour of the
  original checks, but these have been redefined to perform a structural check
  with a threshold specified per field. This means that existing implementations
  of `FloatEq` and `FloatEqDebug` ought to be updated to the new form. Despite
  being more disruptive, this default was changed since it is a more general
  case (e.g. tuples can generally be `FloatEq` but not `FloatEqAll`). Existing
  checks against arrays or `num::Complex` should work again if changed to use the
  `_all` variants.
- Added documentation on how to read assert error messages.

### Changed
- ULPs based checks now treat `NaN` values as not equal, to match the default
  partial equality behaviour of floats.
- `FloatEq` now more specifically means equality based on a structurally defined
  epsilon type. See the notes on `FloatEqAll` above.
- `FloatEq`'s `DiffEpsilon` is now `Epsilon`, and `UlpsDiffEpsilon` is now
  `UlpsEpsilon`. This reduces visual noise around the usage and harmonizes the
  naming with `FloatEqDebug`'s associated types.
- Directed docs.rs to build documentation for all features.

### Fixed
- Some of the custom_type tests were using lower values than intended for some
  checks, these have been changed, although the tests were not previously unsound.

## [0.2.0] - 2020-04-12
Bumped up the minor version number since this release includes breaking API 
changes.

### Added
- Implementation of traits for arrays of size 0 to 32 (inclusive) where the type
  allows it. Epsilon is assumed to be uniform across the array being compared.
- The 'num' feature, which when enabled provides support for comparison of
  `num::Complex` instances.
- Documentation to help with implementing the traits.

### Changed
- `FloatDiff`, `FloatEq` and `FloatEqDebug` along with the macros that use them 
  now allow for a different `Rhs` type to be specified instead of assuming it is
  always `Self`.
- The somewhat awkward `FloatEq::rel_epsilon` was removed in favour of a more 
  equitable `FloatEqDebug` trait for displaying debug information. 
- Asserts now correctly dereference parameters.
- Somewhat more streamlined assert error messages that allow for easier direct
  comparison of diffs and epsilons.
- Switched back to being 'experimental' on crates.io since the API might change
  a lot in the next few versions.

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

[Unreleased]: https://github.com/jtempest/float_eq-rs/compare/0.2.0...HEAD
[0.2.0]: https://github.com/jtempest/float_eq-rs/releases/tag/0.2.0
[0.1.3]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.3
[0.1.2]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.2
[0.1.1]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.1
[0.1.0]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.0