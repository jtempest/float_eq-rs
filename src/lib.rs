//! Explicitly bounded comparison of floating point numbers.
//!
//! Comparing floating point values for equality is *really hard*. To get it
//! right requires careful thought and iteration based on the needs of each
//! specific algorithm's inputs and error margins. This API provides a toolbox
//! of components to make your options clear and your choices explicit to
//! future maintainers.
//!
//! # Table of Contents
//!
//! - [Background](#background)
//! - [Making comparisons](#making-comparisons)
//!     - [Absolute epsilon comparison](#absolute-epsilon-comparison)
//!     - [Relative epsilon comparison](#relative-epsilon-comparison)
//!     - [Units in the Last Place (ULPs) comparison](#units-in-the-last-place-ulps-comparison)
//! - [Comparing composite types](#comparing-composite-types)
//! - [Error messages](#error-messages)
//! - [Comparing custom types](#comparing-custom-types)
//!
//! # Background
//!
//! Given how widely algorithmic requirements can vary, `float_eq` explores the
//! idea that there are no generally sensible default margins for comparisons.
//! This is in contrast to the approach taken by many existing crates, which often
//! provide default epsilon values in checks or implicitly favour particular
//! algorithms. The author's hope is that by exposing the inherent complexity in
//! a uniform way, programmers will find it easier to develop an intuition for how
//! to write effective comparisons. The trade-off is that each individual
//! comparison requires more iteration time and thought.
//!
//! And yes, this is yet another crate built on the principles described in *that*
//! Random ASCII [floating point comparison] article, which is highly recommended
//! background reading 🙂.
//!
//! # Making comparisons
//!
//! The [`float_eq!`] and [`float_ne!`] macros compare two floating point
//! expressions for equality based on the result of one or more different kinds
//! of check. Each check is invoked by name and an upper boundary, so for example
//! `rel <= 0.1`, should be read as *"a [relative epsilon comparison](#relative-epsilon-comparison)
//! with a maximum difference of less than or equal to `0.1`"*. If multiple checks
//! are provided then they are executed in order from left to right, shortcutting
//! to return early if one passes. The corresponding [`assert_float_eq!`] and
//! [`assert_float_ne!`] use the same interface:
//!
//! ```rust
//! use float_eq::{assert_float_eq, assert_float_ne, float_eq, float_ne};
//! use std::f32;
//!
//! assert!(float_eq!(1000.0_f32, 1000.0002, ulps <= 4));
//!
//! const ROUNDING_ERROR: f32 = 0.00034526698; // f32::EPSILON.sqrt()
//! assert!(float_ne!(4.0_f32, 4.1, rel <= ROUNDING_ERROR));
//!
//! const RECIP_REL_EPSILON: f32 = 0.00036621094; // 1.5 * 2_f32.powi(-12)
//! assert_float_eq!(0.1_f32.recip(), 10.0, rel <= RECIP_REL_EPSILON);
//!
//! assert_float_ne!(0.0_f32, 0.0001, abs <= 0.00005, ulps <= 4);
//! ```
//!
//! The ideal choice of comparison will vary on a case by case basis, and depends
//! on the input range and error margins of the expressions to be compared. For
//! example, a test of the result of [finite difference approximation of
//! derivatives] might use a relative epsilon check with a `max_diff` of the `sqrt`
//! of machine epsilon, whereas a test of the SSE [`_mm_rcp_ps` operation] could
//! instead opt for a maximum relative error of `1.5 * 2^(-12)` based on the
//! available documentation. Algorithm stability can play a big part in the size
//! of these margins, and it can be worth seeing if code might be rearranged to
//! reduce loss of precision if you find yourself using large bounds.
//!
//! Relative comparisons (`ulps` and `rel`) are usually a good choice for comparing
//! [normal floats] (e.g. when [`f32::is_normal`] is true). However, they become
//! far too strict for comparisons of very small numbers with zero, where the
//! relative differences are very large but the absolute difference is tiny. This
//! is where you might choose to use an absolute epsilon (`abs`) comparison instead.
//! There are also potential performance implications based on the target hardware.
//!
//! Be prepared to research, test, benchmark and iterate on your comparisons. The
//! [floating point comparison] article which informed this crate's implementation
//! is a good place to start.
//!
//! ## Absolute epsilon comparison
//!
//! A check to see how far apart two expressions are by comparing the absolute
//! difference between them to an absolute, unscaled epsilon. Equivalent to, using
//! `f32` as an example:
//!
//! ```rust
//! fn float_eq_abs(a: f32, b: f32, max_diff: f32) -> bool {
//!     (a - b).abs() <= max_diff
//! }
//! # float_eq::assert_float_eq!(4_f32, 4.0000025, abs <= 0.0000025);
//! # assert!(float_eq_abs(4_f32, 4.0000025, 0.0000025));
//! ```
//!
//! Absolute epsilon tests *do not* work well for general floating point comparison,
//! because they do not take into account that floating point values' precision
//! changes with their magnitude. Thus `max_diff` must be very specific and
//! dependent on the exact values being compared:
//!
//! ```rust
//! # use float_eq::{assert_float_eq, assert_float_ne};
//! let a = 1.0_f32;
//! let b = 1.0000001_f32; // the next representable value above 1.0
//! assert_float_eq!(a, b, abs <= 0.0000002);             // equal
//! assert_float_ne!(a * 4.0, b * 4.0, abs <= 0.0000002); // not equal
//! assert_float_eq!(a * 4.0, b * 4.0, abs <= 0.0000005); // equal
//! ```
//!
//! Whereas a relative epsilon comparison could cope with this since it scales by
//! the size of the largest input parameter:
//!
//! ```rust
//! # use float_eq::{assert_float_eq, assert_float_ne};
//! # let a: f32 = 1.0;
//! # let b: f32 = 1.0000001;
//! assert_float_eq!(a, b, rel <= 0.0000002);
//! assert_float_eq!(a * 4.0, b * 4.0, rel <= 0.0000002);
//! ```
//!
//! However, absolute epsilon comparison is often the best choice when comparing
//! values directly against zero, especially when those values have undergone
//! [catastrophic cancellation], like the subtractions below. In this case, the
//! relative comparison methods break down due to the relative ratio between values
//! being so high compared to their absolute difference:
//!
//! ```rust
//! # use float_eq::{assert_float_eq, assert_float_ne};
//! assert_float_eq!(1.0_f32 - 1.0000001, 0.0, abs <= 0.0000002); // equal
//! assert_float_ne!(1.0_f32 - 1.0000001, 0.0, rel <= 0.0000002); // not equal
//! assert_float_ne!(1.0_f32 - 1.0000001, 0.0, ulps <= 1);        // not equal
//! ```
//!
//! Absolute epsilon comparisons:
//! - Are useful for checking if a float is equal to zero, especially if it has
//!   undergone an operation that suffers from [catastrophic cancellation] or is
//!   a [denormalised value] (a subnormal, in Rust terminology).
//! - Are almost certainly not what you want to use when testing [normal floats]
//!   for equality. `rel` and `ulps` checks can be easier to parameterise and
//!   reason about.
//!
//! ## Relative epsilon comparison
//!
//! A check to see how far apart two expressions are by comparing the absolute
//! difference between them to an epsilon that is scaled to the precision of the
//! larger input. Equivalent to, using `f32` as an example:
//!
//! ```rust
//! fn float_eq_rel(a: f32, b: f32, max_diff: f32) -> bool {
//!     let largest = a.abs().max(b.abs());
//!     (a - b).abs() <= (largest * max_diff)
//! }
//! # float_eq::assert_float_eq!(4_f32, 4.0000025, rel <= 0.0000006);
//! # assert!(float_eq_rel(4_f32, 4.0000025, 0.0000006));
//! ```
//!
//! This makes it suitable for general comparison of values where the ratio between
//! those values is relatively stable (e.g. [normal floats], excluding
//! infinity):
//!
//! ```rust
//! # use float_eq::{assert_float_eq, assert_float_ne};
//! let a: f32 = 1.0;
//! let b: f32 = 1.0000001; // the next representable value above 1.0
//! assert_float_eq!(a, b, rel <= 0.0000002);
//! assert_float_eq!(a * 4.0, b * 4.0, rel <= 0.0000002);
//! ```
//!
//! However, relative epsilon comparison becomes far too strict when the numbers
//! being checked are too close to zero, since the relative ratio between the values
//! can be huge whilst the absolute difference remains tiny. In these circumstances,
//! it is usually better to make an absolute epsilon check instead, especially if
//! your algorithm contains some form of [catastrophic cancellation], like these
//! subtractions:
//!
//! ```rust
//! # use float_eq::{assert_float_eq, assert_float_ne};
//! assert_float_ne!(1.0_f32 - 1.0000001, 0.0, rel <= 0.0000002); // not equal
//! assert_float_eq!(1.0_f32 - 1.0000001, 0.0, abs <= 0.0000002); // equal
//! ```
//!
//! Relative epsilon comparisons:
//! - Are useful for checking if two [normal floats] are equal.
//! - Aren't a good choice when checking values against zero, where `abs` is often
//!   far better.
//!
//! ## Units in the Last Place (ULPs) comparison
//!
//! A check to see how far apart two expressions are by comparing the number of
//! discrete values that can be expressed between them. This works by interpreting
//! the bitwise representation of the input values as integers and comparing the
//! absolute difference between those. Equivalent to, using `f32` as an example:
//!
//! ```rust
//! fn float_eq_ulps(a: f32, b: f32, max_diff: u32) -> bool {
//!     // values are only comparable if they have the same sign
//!     if a.is_sign_positive() != b.is_sign_positive() {
//!         a == b // account for zero == negative zero
//!     } else {
//!         let a_bits = a.to_bits();
//!         let b_bits = b.to_bits();
//!         let max = a_bits.max(b_bits);
//!         let min = a_bits.min(b_bits);
//!         (max - min) <= max_diff
//!     }
//! }
//! # float_eq::assert_float_eq!(4_f32, 4.0000025, ulps <= 5);
//! # assert!(float_eq_ulps(4_f32, 4.0000025, 5));
//! ```
//!
//! Thanks to a deliberate quirk in the way the [underlying format] of IEEE floats
//! was designed, this is a good measure of how near two values are that scales with
//! their relative precision:
//!
//! ```rust
//! # use float_eq::{assert_float_eq, assert_float_ne};
//! assert_float_eq!(1.0_f32, 1.0000001, ulps <= 1);
//! assert_float_eq!(4.0_f32, 4.0000005, ulps <= 1);
//! assert_float_eq!(-1_000_000.0_f32, -1_000_000.06, ulps <= 1);
//! ```
//!
//! However, it becames far too strict when both expressions are close to zero,
//! since the relative difference between them can be very large, whilst the
//! absolute difference remains small. In these circumstances, it is usually better
//! to make an absolute epsilon check instead, especially if your algorithm contains
//! some form of [catastrophic cancellation], like these subtractions:
//!
//! ```rust
//! # use float_eq::{assert_float_eq, assert_float_ne};
//! assert_float_ne!(1.0_f32 - 1.0000001, 0.0, ulps <= 1);        // not equal
//! assert_float_eq!(1.0_f32 - 1.0000001, 0.0, abs <= 0.0000002); // equal
//! ```
//!
//! ULPs based comparisons:
//! - Are useful for checking if two [normal floats] are equal.
//! - Aren't a good choice when checking values against zero, where `abs` is often
//!   far better.
//! - Provide a way to precisely tweak `max_diff` margins, since they have a 1-to-1
//!   correlation with the underlying representation.
//! - Have slightly counterintuitive results around powers of two values, where
//!   the relative precision ratio changes due to way the floating point exponent
//!   works.
//! - Do not work at all if the two values being checked have different signs.
//! - Do not respect the behaviour of special floating point values like NaN.
//!
//! # Comparing composite types
//!
//! When comparing composite values, it can be helpful to specify thresholds
//! separately for each individual field. The `abs`, `rel` and `ulps` checks
//! expect this behaviour. Conversely, the `abs_all`, `rel_all` and `ulps_all`
//! checks accept a single epsilon that is then used to compare across all fields.
//! For example, arrays may be compared using an epsilon that covers each index
//! separately:
//!
//! ```
//! # use float_eq::assert_float_eq;
//! let a = [1.0, -2.0, 3.0];
//! let b = [-1.0, 2.0, 3.5];
//! assert_float_eq!(a, b, abs <= [2.0, 4.0, 0.5]);
//! ```
//!
//! Or with the same threshold across all values:
//!
//! ```
//! # use float_eq::assert_float_eq;
//! # let a = [1.0, -2.0, 3.0];
//! # let b = [-1.0, 2.0, 3.5];
//! assert_float_eq!(a, b, abs_all <= 4.0);
//! ```
//!
//! Similarly, if [`FloatEq`] and [`FloatEqAll`] have been implemented for a
//! struct type:
//!
//! ```
//! # use float_eq::{
//! #     assert_float_eq, FloatDiff, FloatEq, FloatEqAll, FloatEqDebug, FloatEqAllDebug
//! # };
//! #
//! # #[derive(Debug, Clone, Copy, PartialEq)]
//! # struct Complex32 { re: f32, im: f32 }
//! #
//! # #[derive(Debug, Clone, Copy, PartialEq)]
//! # struct Complex32Ulps { re: u32, im: u32 }
//! #
//! # impl FloatDiff for Complex32 {
//! #     type AbsDiff = Complex32;
//! #     type UlpsDiff = Complex32Ulps;
//! #     fn abs_diff(&self, other: &Self) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.abs_diff(&other.re),
//! #             im: self.im.abs_diff(&other.im),
//! #         }
//! #     }
//! #     fn ulps_diff(&self, other: &Self) -> Complex32Ulps {
//! #         Complex32Ulps {
//! #             re: self.re.ulps_diff(&other.re),
//! #             im: self.im.ulps_diff(&other.im),
//! #         }
//! #     }
//! # }
//! #
//! # impl FloatEq for Complex32 {
//! #     type Epsilon = Complex32;
//! #     type UlpsEpsilon = Complex32Ulps;
//! #     fn eq_abs(&self, other: &Self, max_diff: &Complex32) -> bool {
//! #         self.re.eq_abs(&other.re, &max_diff.re) && self.im.eq_abs(&other.im, &max_diff.im)
//! #     }
//! #     fn eq_rel(&self, other: &Self, max_diff: &Complex32) -> bool {
//! #         self.re.eq_rel(&other.re, &max_diff.re) && self.im.eq_rel(&other.im, &max_diff.im)
//! #     }
//! #     fn eq_ulps(&self, other: &Self, max_diff: &Complex32Ulps) -> bool {
//! #         self.re.eq_ulps(&other.re, &max_diff.re) && self.im.eq_ulps(&other.im, &max_diff.im)
//! #     }
//! # }
//! #
//! # impl FloatEqAll for Complex32 {
//! #     type Epsilon = f32;
//! #     type UlpsEpsilon = u32;
//! #     fn eq_abs_all(&self, other: &Self, max_diff: &f32) -> bool {
//! #         self.re.eq_abs_all(&other.re, &max_diff) && self.im.eq_abs_all(&other.im, &max_diff)
//! #     }
//! #     fn eq_rel_all(&self, other: &Self, max_diff: &f32) -> bool {
//! #         self.re.eq_rel_all(&other.re, &max_diff) && self.im.eq_rel_all(&other.im, &max_diff)
//! #     }
//! #     fn eq_ulps_all(&self, other: &Self, max_diff: &u32) -> bool {
//! #         self.re.eq_ulps_all(&other.re, &max_diff) && self.im.eq_ulps_all(&other.im, &max_diff)
//! #     }
//! # }
//! #
//! # impl FloatEqDebug for Complex32 {
//! #     type DebugEpsilon = Complex32;
//! #     type DebugUlpsEpsilon = Complex32Ulps;
//! #     fn debug_abs_epsilon(&self, other: &Self, max_diff: &Complex32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_abs_epsilon(&other.re, &max_diff.re),
//! #             im: self.im.debug_abs_epsilon(&other.re, &max_diff.im),
//! #         }
//! #     }
//! #     fn debug_rel_epsilon(&self, other: &Self, max_diff: &Complex32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_rel_epsilon(&other.re, &max_diff.re),
//! #             im: self.im.debug_rel_epsilon(&other.re, &max_diff.im),
//! #         }
//! #     }
//! #     fn debug_ulps_epsilon(&self, other: &Self, max_diff: &Complex32Ulps) -> Complex32Ulps {
//! #         Complex32Ulps {
//! #             re: self.re.debug_ulps_epsilon(&other.re, &max_diff.re),
//! #             im: self.im.debug_ulps_epsilon(&other.re, &max_diff.im),
//! #         }
//! #     }
//! # }
//! #
//! # impl FloatEqAllDebug for Complex32 {
//! #     type DebugEpsilon = Complex32;
//! #     type DebugUlpsEpsilon = Complex32Ulps;
//! #     fn debug_abs_all_epsilon(&self, other: &Self, max_diff: &f32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_abs_all_epsilon(&other.re, &max_diff),
//! #             im: self.im.debug_abs_all_epsilon(&other.re, &max_diff),
//! #         }
//! #     }
//! #     fn debug_rel_all_epsilon(&self, other: &Self, max_diff: &f32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_rel_all_epsilon(&other.re, &max_diff),
//! #             im: self.im.debug_rel_all_epsilon(&other.re, &max_diff),
//! #         }
//! #     }
//! #     fn debug_ulps_all_epsilon(&self, other: &Self, max_diff: &u32) -> Complex32Ulps {
//! #         Complex32Ulps {
//! #             re: self.re.debug_ulps_all_epsilon(&other.re, &max_diff),
//! #             im: self.im.debug_ulps_all_epsilon(&other.re, &max_diff),
//! #         }
//! #     }
//! # }
//! let a = Complex32 { re: 2.0, im: 4.000_002 };
//! let b = Complex32 { re: 2.000_000_5, im: 4.0 };
//!
//! assert_float_eq!(a, b, rel <= Complex32 { re: 0.000_000_25, im: 0.000_000_5 });
//! assert_float_eq!(a, b, rel_all <= 0.000_000_5);
//!
//! assert_float_eq!(a, b, ulps <= Complex32Ulps { re: 2, im: 4 });
//! assert_float_eq!(a, b, ulps_all <= 4);
//! ```
//!
//! # Error messages
//!
//! Assertion failure messages provide context information that hopefully helps
//! in determining how a check failed. The absolute difference (`abs_diff`) and
//! ULPs difference (`ulps_diff`) between the values are always provided, and
//! then the epsilon values used in the check are listed afterwards. For example,
//! this line:
//!
//! ```should_panic
//! # use float_eq::assert_float_eq;
//! assert_float_eq!(4.0f32, 4.000_008, rel <= 0.000_001);
//! ```
//!
//! Gives this error message, where the relative epsilon, `[rel] ε`, has been
//! scaled based on the size of the inputs (ε is the greek letter epsilon):
//!
//! ```text
//! thread 'test' panicked at 'assertion failed: `float_eq!(left, right, ulps <= ε)`
//!      left: `4.0`,
//!     right: `4.000008`,
//!  abs_diff: `0.000008106232`,
//! ulps_diff: `17`,
//!   [rel] ε: `0.000004000008`', assert_failure.rs:15:5
//! ```
//!
//! If two or more checks are used, then the epsilons are provided in the order
//! that the checks were made in. For example, this line:
//!
//! ```should_panic
//! # use float_eq::assert_float_eq;
//! assert_float_eq!(4.0f32, 4.000_008, abs <= 0.000_001, ulps <= 4);
//! ```
//!
//! Gives this error message:
//!
//! ```text
//! thread 'test' panicked at 'assertion failed: `float_eq!(left, right, abs <= ε, ulps <= ε)`
//!      left: `4.0`,
//!     right: `4.000008`,
//!  abs_diff: `0.000008106232`,
//! ulps_diff: `17`,
//!   [abs] ε: `0.000001`,
//!  [ulps] ε: `4`', assert_failure.rs:16:5
//! ```
//!
//! The checks performed are also indicated on the first line, as `abs <= ε,
//! rel <= ε`.
//!
//! # Comparing custom types
//!
//! Comparison of new types is supported by implementing [`FloatEq`]. If assert
//! support is required, then [`FloatDiff`] and [`FloatEqDebug`] should also be
//! implemented, as they provide important context information on failure.
//!
//! [`assert_float_eq!`]: macro.assert_float_eq.html
//! [`assert_float_ne!`]: macro.assert_float_ne.html
//! [`float_eq!`]: macro.float_eq.html
//! [`float_ne!`]: macro.float_ne.html
//! [`FloatEq`]: trait.FloatEq.html
//! [`FloatEqAll`]: trait.FloatEqAll.html
//! [`FloatDiff`]: trait.FloatDiff.html
//! [`FloatEqDebug`]: trait.FloatEqDebug.html
//!
//! [catastrophic cancellation]: https://en.wikipedia.org/wiki/Loss_of_significance
//! [denormalised value]: https://en.wikipedia.org/wiki/Denormal_number
//! [finite difference approximation of derivatives]: https://scicomp.stackexchange.com/questions/14355/choosing-epsilons
//! [floating point comparison]: https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
//! [normal floats]: https://en.wikipedia.org/wiki/Normal_number_(computing)
//! [underlying format]: https://randomascii.wordpress.com/2012/01/23/stupid-float-tricks-2/
//! [`_mm_rcp_ps` operation]: https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_rcp_ps&expand=4482
//! [`f32::is_normal`]: https://doc.rust-lang.org/std/primitive.f32.html#method.is_normal

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod macros;
pub use crate::macros::*;

mod traits;
pub use crate::traits::*;

// implementations of traits
mod arrays;
mod primitives;

#[cfg(feature = "num")]
mod num_complex;
#[cfg(feature = "num")]
pub use self::num_complex::*;
