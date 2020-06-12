# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
Bumped up the version number since this release includes breaking API changes.

### Added
- The `FloatUlps` trait, which is a way to syntactically tie a floating point
  type to its ULPs representation. The prefered way to express this is now to
  implement `FloatUlps for T` and then use `Ulps<T>` for the ULPs type. The
  other traits have been reworked to reflect this. 
- Blanket trait impls for Option, Vec, VecDeque, LinkedList, BTreeMap and
  HashMap.

### Changed
- Blanket array impls now allow for comparison of arrays of different types.

## [0.3.1] 2020-06-03

### Added
- Blanket trait impls for comparing mutable and immutable reference types and
  the contents of Cell, RefCell, Rc, Arc and Box instances.
- More descriptive documentation for combining types of check.

## [0.3.0] - 2020-05-30
Bumped up the version number since this release includes breaking API changes.

### Added
- The `FloatEqAll` and `FloatEqAllDebug` traits, which allow for `abs_all`,
  `rel_all` and `ulps_all` to be used in the `float_eq` and `assert_float_eq`
  families of macros, which take a single epsilon value to be applied uniformly
  across all fields being compared. The behaviour of `FloatEq` has now changed
  to apply epsilon values structurally, on a per-field basis, since that is a
  more general behaviour (e.g. all tuples may sensibly be `FloatEq` but not
  `FloatEqAll`). If existing checks against arrays or `num::Complex` break, 
  switching to use the `_all` variants ought to fix them.
- Support for tuples of up to size 12 (inclusive).
- Documentation on how to interpret assert error messages.

### Changed
- ULPs checks now treat `NaN` values as not equal, to match the default partial
  equality behaviour of floats.
- Equality of infinities is now consistent, both internally and with respect to
  general IEEE floating point behaviour.
- `FloatEq` now more specifically means equality based on a structurally defined
  epsilon type. See the notes on `FloatEqAll` above.
- `FloatDiff` is much more rigorously defined, in particular `ulps_diff` now
  returns `Option<T>`, see the API documentation for details.
- `FloatEq`'s `DiffEpsilon` is now `Epsilon`, and `UlpsDiffEpsilon` is now
  `UlpsEpsilon`. This reduces visual noise around the usage and harmonizes the
  naming with `FloatEqDebug`'s associated types.
- Directed docs.rs to build documentation for all features.

## [0.2.0] - 2020-04-12
Bumped up the version number since this release includes breaking API changes.

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

[Unreleased]: https://github.com/jtempest/float_eq-rs/compare/0.3.1...HEAD
[0.3.1]: https://github.com/jtempest/float_eq-rs/releases/tag/0.3.1
[0.3.0]: https://github.com/jtempest/float_eq-rs/releases/tag/0.3.0
[0.2.0]: https://github.com/jtempest/float_eq-rs/releases/tag/0.2.0
[0.1.3]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.3
[0.1.2]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.2
[0.1.1]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.1
[0.1.0]: https://github.com/jtempest/float_eq-rs/releases/tag/0.1.0