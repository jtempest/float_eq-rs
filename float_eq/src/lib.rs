//! Compare IEEE floating point primitives, structs and collections for equality.
//!
//! This is API reference documentation. For introductory material, guides and
//! discussion see [the float_eq book].
//!
//! # Basic usage
//!
//! This crate provides boolean comparisons via [`float_eq!`] and [`float_ne!`]:
//!
//! ```
//! use float_eq::float_eq;
//!
//! # let y_pos = 0.000_1;
//! if (float_eq!(y_pos, 0.0, abs <= 0.000_1)) {
//!    //...
//! }
//! ```
//!
//! And asserts via [`assert_float_eq!`] and [`assert_float_ne!`]:
//!
//! ```
//! use float_eq::assert_float_eq;
//!
//! const TOL: f32 = 0.000_366_210_94;
//! assert_float_eq!(0.1f32.recip(), 10.0, r2nd <= TOL);
//! ```
//!
//! Each of which invokes a specific comparison algorithm with an explictly
//! provided margin. In these examples:
//!
//! - `abs <= 0.000_1` is an absolute tolerance comparison with a margin of `0.000_1`.
//! - `r2nd <= TOL` is a relative tolerance comparison with a margin of `TOL`,
//!   scaled to the precision of the second operand.
//!
//! # Comparison algorithms
//!
//! These are always of the form `CHECK <= margin`, where `CHECK` is one of:
//!
//! - `abs`: an [absolute tolerance comparison].
//! - `rmax`: a [relative tolerance comparison], scaled to the precision of the larger operand/field.
//! - `rmin`: a [relative tolerance comparison], scaled to the precision of the smaller operand/field.
//! - `r1st`: a [relative tolerance comparison], scaled to the precision of the first operand/field.
//! - `r2nd`: a [relative tolerance comparison], scaled to the precision of the second operand/field.
//! - `ulps`: an [ULPs comparison].
//!
//! When comparing homogeneous composite types that implement [`FloatEqAll`],
//! variants that use a uniform `margin` value across all fields are also available:
//!
//! - `abs_all`: an [absolute tolerance comparison].
//! - `rmax_all`: a [relative tolerance comparison], scaled to the precision of the larger field.
//! - `rmin_all`: a [relative tolerance comparison], scaled to the precision of the smaller field.
//! - `r1st_all`: a [relative tolerance comparison], scaled to the precision of the first field.
//! - `r2nd_all`: a [relative tolerance comparison], scaled to the precision of the second field.
//! - `ulps_all`: an [ULPs comparison].
//!
//! *Note: `rel` and `rel_all` are legacy aliases for `rmax` and `rmax_all`, but
//! using the more precise algorithm names is recommended.*
//!
//! # Combining checks
//!
//! If multiple checks are specified in either a boolean comparison or an assert,
//! they are applied left to right and will shortcut on success. For example:
//!
//! ```
//! float_eq!(a, b, abs <= abs_tol, ulps <= ulps_tol)
//! ```
//!
//! Is equivalent to:
//!
//! ```
//! float_eq!(a, b, abs <= abs_tol) || float_eq!(a, b, ulps <= ulps_tol)
//! ```
//!
//! # Extending float_eq over custom types
//!
//! See [How to compare custom types].
//!
//! [How to compare custom types]: https://jtempest.github.io/float_eq-rs/book/how_to/compare_custom_types.html
//! [the float_eq book]: https://jtempest.github.io/float_eq-rs/book/index.html
//! [absolute tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#absolute-tolerance-comparison
//! [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
//! [ULPs comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod macros;
pub use crate::macros::*;

mod traits;
pub use crate::traits::*;

mod trait_impls;
pub use crate::trait_impls::*;

#[cfg(feature = "float_eq_derive")]
pub use float_eq_derive::*;
