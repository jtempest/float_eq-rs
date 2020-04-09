//! Explicit and deliberate comparison of IEEE floating point numbers.
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
//! let a = 1.0;
//! let b = 1.0000001; // the next representable value above 1.0
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
//! # let a = 1.0;
//! # let b = 1.0000001;
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
//! let a = 1.0;
//! let b = 1.0000001; // the next representable value above 1.0
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
//!         let a_bits = a.to_bits() as u32;
//!         let b_bits = b.to_bits() as u32;
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
//! was designed, this is good measure of how near two values are that scales with
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
//! # Comparing custom types
//!
//! Comparison of new types is supported by implementing both [`FloatEq`] and
//! [`FloatDiff`]:
//! - [`FloatEq`] does most of the work in calculating comparisons.
//! - [`FloatDiff`] is used by the assert macros to provide intermediate context
//!   for calculations in the case of failure, although it could also be used to
//!   directly calculate differences if you wish.
//!
//! [`assert_float_eq!`]: macro.assert_float_eq.html
//! [`assert_float_ne!`]: macro.assert_float_ne.html
//! [`float_eq!`]: macro.float_eq.html
//! [`float_ne!`]: macro.float_ne.html
//! [`FloatEq`]: trait.FloatEq.html
//! [`FloatDiff`]: trait.FloatDiff.html
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

use core::mem::MaybeUninit;

/// Algorithms to compute the difference between two IEEE floating point values.
///
/// This trait is implemented for `f32` and `f64` values:
///
/// ```rust
/// use float_eq::FloatDiff;
///
/// assert_eq!(1.0_f32.abs_diff(&-1.0), 2.0);
/// assert_eq!(1.0_f64.abs_diff(&-1.0), 2.0);
///
/// assert_eq!(1.0_f32.ulps_diff(&1.0000001), 1);
/// assert_eq!(1.0_f64.ulps_diff(&1.0000000000000002), 1);
/// ```
///
/// And on arrays of size 0 to 32 (inclusive) if the element type allows it:
///
/// ```rust
/// # use float_eq::FloatDiff;
/// let a = [0.0_f32, 2.0, -2.0];
/// let b = [0.0_f32, -1.0, 2.0];
/// assert_eq!(a.abs_diff(&b), [0.0, 3.0, 4.0]);
/// ```
///
pub trait FloatDiff {
    /// Type of the absolute difference between two values.
    ///
    /// This is almost always `Self`.
    type AbsDiff;

    /// Type of the absolute difference between two values in terms of [ULPs].
    ///
    /// This should be an unsigned integer of the same size as the underlying
    /// floating point type, for example `f32` uses `u32`.
    ///
    /// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
    type UlpsDiff;

    /// Always positive absolute difference between two values.
    ///
    /// Implementations should be the equivalent of:
    ///
    /// ```text
    /// (self - other).abs()
    /// ```
    fn abs_diff(&self, other: &Self) -> Self::AbsDiff;

    /// Always positive absolute difference between two values in terms of [ULPs]
    ///
    /// Implementations should be the equivalent of (using `f32` as an example):
    ///
    /// ```text
    /// let a = (self.to_bits()) as u32;
    /// let b = (other.to_bits()) as u32;
    /// let max = a.max(b);
    /// let min = a.min(b);
    /// max - min
    /// ```
    ///
    /// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
    fn ulps_diff(&self, other: &Self) -> Self::UlpsDiff;
}

/// Algorithms to compare two IEEE floating point values for equality.
pub trait FloatEq {
    /// Type of the maximum allowed difference between two values for them to be
    /// considered equal in terms of their native type.
    ///
    /// This is usually `Self` but in some cases might differ, for example when
    /// comparing two SIMD values you might provide a single value that is then
    /// broadcast across each lane.
    type DiffEpsilon;

    /// Type of the maximum allowed difference between two values for them to be
    /// considered equal in terms of an [ULPs comparison].
    ///
    /// This is usually an unsigned integer of the same width as `Self` (e.g.
    /// `f32` uses `u32`), but in some cases might differ, for example when
    /// comparing two SIMD values you might provide a single value that is then
    /// broadcast across each lane.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    type UlpsDiffEpsilon;

    /// Check whether `self` is equal to `other`, using an [absolute epsilon
    /// comparison].
    ///
    /// Implementations should be the equivalent of (using [`FloatDiff`]):
    ///
    /// ```text
    /// self.abs_diff(other) <= *max_diff
    /// ```
    ///
    /// [`FloatDiff`]: trait.FloatDiff.html
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn eq_abs(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using an [absolute epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_abs(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn ne_abs(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
        !self.eq_abs(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// The implementation should be the equivalent of (using [`FloatDiff`]):
    ///
    /// ```text
    /// self.abs_diff(other) <= self.rel_epsilon(other, max_diff)
    /// ```
    ///
    /// [`FloatDiff`]: trait.FloatDiff.html
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_rel(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_rel(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn ne_rel(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
        !self.eq_rel(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using an [ULPs comparison].
    ///
    /// The implementation should be the equivalent of (using [`FloatDiff`]):
    ///
    /// ```text
    /// if self.is_sign_positive() != other.is_sign_positive() {
    ///     self == other // account for zero == negative zero
    /// } else {
    ///     self.ulps_diff(other) <= *max_diff
    /// }
    /// ```
    ///
    /// [`FloatDiff`]: trait.FloatDiff.html
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using an [ULPs comparison].
    ///
    /// Equal to `!self.eq_ulps(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn ne_ulps(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool {
        !self.eq_ulps(other, max_diff)
    }

    /// Calculates the epsilon value used by an `eq_rel` comparison relative to
    /// the larger of the two values being compared. This is used to provide
    /// additional context when an assert fails.
    ///
    /// The implementation should be the equivalent of (using [`FloatDiff`]):
    ///
    /// ```text
    /// let largest = self.abs().max(other.abs())
    /// largest * max_diff
    /// ```
    ///
    /// [`FloatDiff`]: trait.FloatDiff.html
    fn rel_epsilon(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> Self::DiffEpsilon;
}

/// Checks whether two floating point expressions are equal to each other (using [`FloatEq`]).
///
/// Comparisons are applied in order from left to right, shortcutting to return
/// early if a positive result is found.
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// # Examples
/// ```
/// # use float_eq::float_eq;
/// # use std::f32;
/// let a: f32 = 4.;
/// let b: f32 = 4.0000025;
///
/// assert!( float_eq!(a, 3.9999998, rel <= f32::EPSILON) );
/// assert!( float_eq!(a - b, 0., abs <= 0.00001, rel <= f32::EPSILON) );
/// assert!( float_eq!(a - b, 0., abs <= 0.00001, ulps <= 10) );
/// ```
///
/// [`FloatEq`]: trait.FloatEq.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
#[macro_export]
macro_rules! float_eq {
    ($a:expr, $b:expr, $($eq:ident <= $max_diff:expr),+) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                false $(|| $crate::FloatEqCmp::$eq(a_val, &b_val, &$max_diff))+
            }
        }
    });
    ($a:expr, $b:expr, $($eq:ident <= $max_diff:expr),+,) => ({
        $crate::float_eq!($a, $b $(, $eq <= $max_diff)+)
    })
}

/// Checks whether two floating point expressions are not equal to each other (using [`FloatEq`]).
///
/// Comparisons are applied in order from left to right, shortcutting to return
/// early if a positive result is found.
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// # Examples
/// ```
/// # use float_eq::float_ne;
/// # use std::f32;
/// let a: f32 = 4.;
/// let b: f32 = 4.1;
///
/// assert!( float_ne!(a, 3.9999990, rel <= f32::EPSILON) );
/// assert!( float_ne!(a - b, 0., abs <= 0.00001, rel <= f32::EPSILON) );
/// assert!( float_ne!(a - b, 0., abs <= 0.00001, ulps <= 10) );
/// ```
///
/// [`FloatEq`]: trait.FloatEq.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
#[macro_export]
macro_rules! float_ne {
    ($a:expr, $b:expr, $($eq:ident <= $max_diff:expr),+) => ({
        !$crate::float_eq!($a, $b $(, $eq <= $max_diff)+)
    });
    ($a:expr, $b:expr, $($eq:ident <= $max_diff:expr),+,) => ({
        !$crate::float_eq!($a, $b $(, $eq <= $max_diff)+)
    });
}

/// Asserts that two floating point expressions are equal to each other (using [`float_eq!`]).
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with additional information from the comparison operations
/// (using [`FloatEq`] and [`FloatDiff`]).
///
/// Like [`assert!`], this macro has a second form, where a custom panic message can
/// be provided.
///
/// # Examples
/// ```
/// # use float_eq::assert_float_eq;
/// # use std::f32;
/// let a: f32 = 4.;
/// let b: f32 = 4.0000025;
///
/// assert_float_eq!(a, 3.9999998, rel <= f32::EPSILON);
/// assert_float_eq!(a - b, 0., abs <= 0.00001, rel <= f32::EPSILON);
/// assert_float_eq!(a - b, 0., abs <= 0.00001, ulps <= 10);
///
/// assert_float_eq!(a - b, 0., abs <= 0.00001, ulps <= 10, "Checking that {} == {}", a, b);
/// ```
///
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [`float_eq!`]: macro.float_eq.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatDiff`]: trait.FloatDiff.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
#[macro_export]
macro_rules! assert_float_eq {
    // the order of these rules matters a *lot* for the format string functionality
    // to work, otherwise we end up consuming the general case too early.
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $eq3:ident <= $max_diff_3:expr) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2, &$max_diff_3) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val, max_diff_3_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val,
                    $eq3 <= *max_diff_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), " <= ε, ", stringify!($eq3), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpDiffName::$eq2(),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpDiffName::$eq3(),
                        $crate::FloatCmpOpEpsilon::$eq3(&*left_val, &*right_val, &*max_diff_3_val)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpDiffName::$eq2(),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr) => ({
        match (&$left, &$right, &$max_diff_1) {
            (left_val, right_val, max_diff_1_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $($eq:ident <= $max_diff:expr,)+) => ({
        $crate::assert_float_eq!($left, $right $(, $eq <= $max_diff)+)
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $eq3:ident <= $max_diff_3:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2, &$max_diff_3) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val, max_diff_3_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val,
                    $eq3 <= *max_diff_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), " <= ε, ", stringify!($eq3), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpDiffName::$eq2(),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpDiffName::$eq3(),
                        $crate::FloatCmpOpEpsilon::$eq3(&*left_val, &*right_val, &*max_diff_3_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpDiffName::$eq2(),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1) {
            (left_val, right_val, max_diff_1_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
}

/// Asserts that two floating point expressions are not equal to each other (using [`float_ne!`]).
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with additional information from the comparison operations
/// (using [`FloatEq`] and [`FloatDiff`]).
///
/// Like [`assert!`], this macro has a second form, where a custom panic message can
/// be provided.
///
/// # Examples
/// ```
/// # use float_eq::assert_float_ne;
/// # use std::f32;
/// let a: f32 = 4.;
/// let b: f32 = 4.1;
///
/// assert_float_ne!(a, 3.9999990, rel <= f32::EPSILON);
/// assert_float_ne!(a - b, 0., abs <= 0.00001, rel <= f32::EPSILON);
/// assert_float_ne!(a - b, 0., abs <= 0.00001, ulps <= 10);
///
/// assert_float_ne!(a - b, 0., abs <= 0.00001, ulps <= 10, "Checking that {} != {}", a, b);
/// ```
///
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [`float_ne!`]: macro.float_ne.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatDiff`]: trait.FloatDiff.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
#[macro_export]
macro_rules! assert_float_ne {
    // the order of these rules matters a *lot* for the format string functionality
    // to work, otherwise we end up consuming the general case too early.
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $eq3:ident <= $max_diff_3:expr) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2, &$max_diff_3) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val, max_diff_3_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val,
                    $eq3 <= *max_diff_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), " <= ε, ", stringify!($eq3), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpDiffName::$eq2(),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpDiffName::$eq3(),
                        $crate::FloatCmpOpEpsilon::$eq3(&*left_val, &*right_val, &*max_diff_3_val)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpDiffName::$eq2(),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr) => ({
        match (&$left, &$right, &$max_diff_1) {
            (left_val, right_val, max_diff_1_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $($eq:ident <= $max_diff:expr,)+) => ({
        $crate::assert_float_ne!($left, $right $(, $eq <= $max_diff)+)
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $eq3:ident <= $max_diff_3:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2, &$max_diff_3) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val, max_diff_3_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val,
                    $eq3 <= *max_diff_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), " <= ε, ", stringify!($eq3), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpDiffName::$eq2(),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpDiffName::$eq3(),
                        $crate::FloatCmpOpEpsilon::$eq3(&*left_val, &*right_val, &*max_diff_3_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`,
{:>9}: {:<9} <= `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpDiffName::$eq2(),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1) {
            (left_val, right_val, max_diff_1_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), r#" <= ε)`
     left: `{:?}`,
    right: `{:?}`,
 abs_diff: `{:?}`,
ulps_diff: `{:?}`,
{:>9}: {:<9} <= `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpDiffName::$eq1(),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
}

/// Asserts that two floating point expressions are equal to each other (using [`float_eq!`]).
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with additional information from the comparison operations
/// (using [`FloatEq`] and [`FloatDiff`]).
///
/// Like [`assert!`], this macro has a second form, where a custom panic message can
/// be provided.
///
/// Unlike [`assert_float_eq!`], `debug_assert_float_eq!` statements are only enabled in
/// non optimized builds by default. See [`debug_assert_eq!`] for more details.
///
/// # Examples
/// ```
/// # use float_eq::debug_assert_float_eq;
/// # use std::f32;
/// let a: f32 = 4.;
/// let b: f32 = 4.0000025;
///
/// debug_assert_float_eq!(a, 3.9999998, rel <= f32::EPSILON);
/// debug_assert_float_eq!(a - b, 0., abs <= 0.00001, rel <= f32::EPSILON);
/// debug_assert_float_eq!(a - b, 0., abs <= 0.00001, ulps <= 10);
///
/// debug_assert_float_eq!(a - b, 0., abs <= 0.00001, ulps <= 10, "Checking that {} == {}", a, b);
/// ```
///
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [`debug_assert_eq!`]: https://doc.rust-lang.org/std/macro.debug_assert_eq.html
/// [`float_eq!`]: macro.float_eq.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatDiff`]: trait.FloatDiff.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
#[macro_export]
macro_rules! debug_assert_float_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_float_eq!($($arg)*); })
}

/// Asserts that two floating point expressions are not equal to each other (using [`float_ne!`]).
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with additional information from the comparison operations
/// (using [`FloatEq`] and [`FloatDiff`]).
///
/// Like [`assert!`], this macro has a second form, where a custom panic message can
/// be provided.
///
/// Unlike [`assert_float_ne!`], `debug_assert_float_ne!` statements are only enabled in
/// non optimized builds by default. See [`debug_assert_ne!`] for more details.
///
/// # Examples
/// ```
/// # use float_eq::debug_assert_float_ne;
/// # use std::f32;
/// let a: f32 = 4.;
/// let b: f32 = 4.1;
///
/// debug_assert_float_ne!(a, 3.9999990, rel <= f32::EPSILON);
/// debug_assert_float_ne!(a - b, 0., abs <= 0.00001, rel <= f32::EPSILON);
/// debug_assert_float_ne!(a - b, 0., abs <= 0.00001, ulps <= 10);
///
/// debug_assert_float_ne!(a - b, 0., abs <= 0.00001, ulps <= 10, "Checking that {} == {}", a, b);
/// ```
///
/// [`assert_float_ne!`]: macro.assert_float_ne.html
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [`debug_assert_ne!`]: https://doc.rust-lang.org/std/macro.debug_assert_ne.html
/// [`float_ne!`]: macro.float_ne.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatDiff`]: trait.FloatDiff.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
#[macro_export]
macro_rules! debug_assert_float_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_float_ne!($($arg)*); })
}

#[doc(hidden)]
pub struct FloatEqCmp;

#[doc(hidden)]
impl FloatEqCmp {
    #[inline]
    pub fn abs<T: FloatEq>(a: &T, b: &T, max_diff: &T::DiffEpsilon) -> bool {
        FloatEq::eq_abs(a, b, max_diff)
    }

    #[inline]
    pub fn rel<T: FloatEq>(a: &T, b: &T, max_diff: &T::DiffEpsilon) -> bool {
        FloatEq::eq_rel(a, b, max_diff)
    }

    #[inline]
    pub fn ulps<T: FloatEq>(a: &T, b: &T, max_diff: &T::UlpsDiffEpsilon) -> bool {
        FloatEq::eq_ulps(a, b, max_diff)
    }
}

#[doc(hidden)]
pub struct FloatCmpDiffName;

#[doc(hidden)]
impl FloatCmpDiffName {
    #[inline]
    pub const fn abs() -> &'static str {
        "abs_diff"
    }

    #[inline]
    pub const fn rel() -> &'static str {
        "abs_diff"
    }

    #[inline]
    pub const fn ulps() -> &'static str {
        "ulps_diff"
    }
}

#[doc(hidden)]
pub struct FloatCmpOpEpsilon;

#[doc(hidden)]
impl FloatCmpOpEpsilon {
    #[inline]
    pub fn abs<'a, T: FloatEq>(_: &T, _: &T, max_diff: &'a T::DiffEpsilon) -> &'a T::DiffEpsilon {
        max_diff
    }

    #[inline]
    pub fn rel<T: FloatEq>(a: &T, b: &T, max_diff: &T::DiffEpsilon) -> T::DiffEpsilon {
        FloatEq::rel_epsilon(a, b, max_diff)
    }

    #[inline]
    pub fn ulps<'a, T: FloatEq>(
        _: &T,
        _: &T,
        max_diff: &'a T::UlpsDiffEpsilon,
    ) -> &'a T::UlpsDiffEpsilon {
        max_diff
    }
}

macro_rules! impl_traits {
    ($float:ident, $uint:ident) => {
        mod $float {
            #[cfg(feature = "std")]
            #[inline]
            pub(crate) fn abs(value: $float) -> $float {
                // use the intrinsic for std builds
                value.abs()
            }

            #[cfg(not(feature = "std"))]
            pub(crate) fn abs(value: $float) -> $float {
                // mask away only the sign bit for no_std builds since the abs
                // method is not available
                const MASK: $uint = !(1 << ((core::mem::size_of::<$float>() * 8) - 1));
                $float::from_bits(value.to_bits() & MASK)
            }
        }

        impl FloatDiff for $float {
            type AbsDiff = $float;
            type UlpsDiff = $uint;

            #[inline]
            fn abs_diff(&self, other: &Self) -> Self::AbsDiff {
                $float::abs(self - other)
            }

            #[inline]
            fn ulps_diff(&self, other: &Self) -> Self::UlpsDiff {
                let a = (self.to_bits()) as $uint;
                let b = (other.to_bits()) as $uint;
                let max = a.max(b);
                let min = a.min(b);
                max - min
            }
        }

        impl FloatEq for $float {
            type DiffEpsilon = $float;
            type UlpsDiffEpsilon = $uint;

            #[inline]
            fn eq_abs(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
                self.abs_diff(other).le(max_diff)
            }

            #[inline]
            fn rel_epsilon(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> Self::DiffEpsilon {
                $float::abs(*self).max($float::abs(*other)) * max_diff
            }

            #[inline]
            fn eq_rel(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
                self.abs_diff(other) <= self.rel_epsilon(other, max_diff)
            }

            #[inline]
            fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool {
                if self.is_sign_positive() != other.is_sign_positive() {
                    self == other // account for zero == negative zero
                } else {
                    self.ulps_diff(other).le(max_diff)
                }
            }
        }
    };
}

impl_traits!(f32, u32);
impl_traits!(f64, u64);

// arrays
//TODO: Should this be publically available for users to conditionally implement
// support if they need it?
macro_rules! impl_float_eq_traits_for_array {
    ($n:literal) => {
        #[doc(hidden)]
        impl<T: FloatDiff> FloatDiff for [T; $n] {
            type AbsDiff = [T::AbsDiff; $n];
            type UlpsDiff = [T::UlpsDiff; $n];

            #[inline]
            fn abs_diff(&self, other: &Self) -> Self::AbsDiff {
                let mut result: Self::AbsDiff = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].abs_diff(&other[i])
                }
                result
            }

            #[inline]
            fn ulps_diff(&self, other: &Self) -> Self::UlpsDiff {
                let mut result: Self::UlpsDiff = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].ulps_diff(&other[i])
                }
                result
            }
        }

        #[doc(hidden)]
        impl<T: FloatEq> FloatEq for [T; $n] {
            type DiffEpsilon = T::DiffEpsilon;
            type UlpsDiffEpsilon = T::UlpsDiffEpsilon;

            #[inline]
            fn eq_abs(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_abs(&other[i], max_diff) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_rel(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_rel(&other[i], max_diff) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_ulps(&other[i], max_diff) {
                        return false;
                    }
                }
                true
            }

            //TODO: Should this be debug_rel_epsilon? It isn't used here and
            // probably ought to be changed to reflect that fact.
            #[inline]
            fn rel_epsilon(
                &self,
                _other: &Self,
                _max_diff: &Self::DiffEpsilon,
            ) -> Self::DiffEpsilon {
                unimplemented!()
            }
        }
    };
}

// 0 to 32 as per primitive array traits
//TODO: Use const generics once they're stable
/// This is also implemented on other arrays up to size 32 (inclusive).
impl<T: FloatDiff> FloatDiff for [T; 0] {
    type AbsDiff = [T::AbsDiff; 0];
    type UlpsDiff = [T::UlpsDiff; 0];

    #[inline]
    fn abs_diff(&self, _other: &Self) -> Self::AbsDiff {
        []
    }

    #[inline]
    fn ulps_diff(&self, _other: &Self) -> Self::UlpsDiff {
        []
    }
}

impl<T: FloatEq> FloatEq for [T; 0] {
    type DiffEpsilon = T::DiffEpsilon;
    type UlpsDiffEpsilon = T::UlpsDiffEpsilon;

    #[inline]
    fn eq_abs(&self, _other: &Self, _max_diff: &Self::DiffEpsilon) -> bool {
        true
    }

    #[inline]
    fn eq_rel(&self, _other: &Self, _max_diff: &Self::DiffEpsilon) -> bool {
        true
    }

    #[inline]
    fn eq_ulps(&self, _other: &Self, _max_diff: &Self::UlpsDiffEpsilon) -> bool {
        true
    }

    //TODO: Should this be debug_rel_epsilon? It isn't used here and
    // probably ought to be changed to reflect that fact.
    #[inline]
    fn rel_epsilon(&self, _other: &Self, _max_diff: &Self::DiffEpsilon) -> Self::DiffEpsilon {
        unimplemented!()
    }
}

impl_float_eq_traits_for_array!(1);
impl_float_eq_traits_for_array!(2);
impl_float_eq_traits_for_array!(3);
impl_float_eq_traits_for_array!(4);
impl_float_eq_traits_for_array!(5);
impl_float_eq_traits_for_array!(6);
impl_float_eq_traits_for_array!(7);
impl_float_eq_traits_for_array!(8);
impl_float_eq_traits_for_array!(9);
impl_float_eq_traits_for_array!(10);
impl_float_eq_traits_for_array!(11);
impl_float_eq_traits_for_array!(12);
impl_float_eq_traits_for_array!(13);
impl_float_eq_traits_for_array!(14);
impl_float_eq_traits_for_array!(15);
impl_float_eq_traits_for_array!(16);
impl_float_eq_traits_for_array!(17);
impl_float_eq_traits_for_array!(18);
impl_float_eq_traits_for_array!(19);
impl_float_eq_traits_for_array!(20);
impl_float_eq_traits_for_array!(21);
impl_float_eq_traits_for_array!(22);
impl_float_eq_traits_for_array!(23);
impl_float_eq_traits_for_array!(24);
impl_float_eq_traits_for_array!(25);
impl_float_eq_traits_for_array!(26);
impl_float_eq_traits_for_array!(27);
impl_float_eq_traits_for_array!(28);
impl_float_eq_traits_for_array!(29);
impl_float_eq_traits_for_array!(30);
impl_float_eq_traits_for_array!(31);
impl_float_eq_traits_for_array!(32);
