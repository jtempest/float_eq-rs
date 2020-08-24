//! Compare IEEE floating point values for equality.
//!
//! Comparing floating point values for equality is notoriously difficult,
//! getting it right requires careful reasoning and iteration. This API provides
//! a variety of comparison algorithms and debugging tools to help make the
//! process more intuitive and your choices explicit and clear to future
//! maintainers.
//!
//! - [Background](#background)
//!     - [Floating point values](#floating-point-values)
//! - [Making comparisons](#making-comparisons)
//!     - [Absolute epsilon comparison](#absolute-epsilon-comparison)
//!     - [Relative epsilon comparison](#relative-epsilon-comparison)
//!     - [Units in the Last Place (ULPs) comparison](#units-in-the-last-place-ulps-comparison)
//! - [Which check(s) should I use?](#which-checks-should-i-use)
//! - [Comparing composite types](#comparing-composite-types)
//! - [Error messages](#error-messages)
//! - [Comparing custom types](#comparing-custom-types)
//!     - [Derivable](#derivable)
//!
//! # Background
//!
//! Given how widely algorithmic requirements can vary, `float_eq` explores the
//! idea that there are no generally sensible default margins for comparisons.
//! This is in contrast to the approach taken by many other crates, which often
//! provide default epsilon values in checks or implicitly favour particular
//! algorithms. The author's hope is that by exposing the inherent complexity
//! in a uniform way, programmers will find it easier to develop an intuition
//! for effective use of floats.
//!
//! This work builds on the definitions in Knuth's The Art Of Computer Programming,
//! (Vol. 2, Seminumerical Algorithms, Third Edition, section 4.2.2), and *that*
//! Random ASCII article on [floating point comparison].
//!
//! ## Floating point values
//!
//! Before diving into the comparison API, let's have quick review of the properties
//! of IEEE floating point numbers, using `f32` as a concrete example. *[Normal]*
//! floats are the most common, and due to their underlying representation are
//! useful to consider in terms of ranges bounded by powers of two:
//!
//! - `1.0` to `1.999_999_9` contains `8_388_608` representable values, each a
//! distance of `f32::EPSILON` apart.
//! - `2.0` to `3.999_999_7` also contains `8_388_608` representable
//! values, each `2.0 * f32::EPSILON` apart.
//! - `0.5` to `0.999_999_94` similarly contains `8_388_608` representable
//! values, each `0.5 * f32::EPSILON` apart.
//!
//! So, ganularity scales with magnitude - doubling a normal number doubles the
//! distance from adjacent representable values, and halving it halves that distance.
//! This allows floats to represent a *much* wider range of values than integers
//! with the same number of bits, with many values tightly packed together near
//! zero and a huge absolute distance between representable values at the top end.
//! It is worth noting that the constant `f32::EPSILON` is only directly applicable
//! to very small range just above `1.0` and must be scaled to be relevant for
//! other ranges, this will come up again shortly.
//!
//! Since the result of each step in a calculation will be rounded to a representable
//! value, the order of operations on floats has a large impact on the error margin
//! of a given algorithm's output. For example, performing a subtraction then
//! multiplying the result by a large number will have a much smaller relative
//! error than multiplying both values before the subtraction, because there is
//! less opportunity for rounding errors to accumulate and compound on one another.
//! This also means that the granularity of the output will vary with the magnitude
//! of the specific values it received as inputs.
//!
//! A common issue in equality comparison is when comparing the result of subtracting
//! two numbers from one another against zero. Due to the relative differences in
//! granularity between the inputs and the result, this will vastly amplify any
//! existing *relative* error margins in a calculation. It is worth noting that
//! the error in *absolute* terms is preserved. This is known as *[catastrophic
//! cancellation]*.
//!
//! *[Subnormal]* (or *denormal*) values are those in the range from `0.0` to
//! `f32::MIN_POSITIVE` (`1.17549435e-38`), exclusive. These, oddly, behave more
//! intuitively like integers since they have a constant difference between each
//! value. The top of the normal range is `f32::MAX`, above which lies `f32::INFINITY`,
//! which acts differently than the normals under operations like subtraction.
//! There are of course corresponding negative normals, subnormals and infinity.
//! Additionally, since a sign bit is used and not two's complement, `-0.0` is a
//! different representation from `0.0` and has a different bit pattern, though
//! comparison considers the two as equal.
//!
//! Finally, there is *NaN* (Not a Number), which is used when some error has
//! occured during a calculation, for example dividing by zero. This is actually
//! a range of values, of which `f32::NAN` is only one. Different bit patterns
//! represent different kinds of errors, although this is irrelevant to equality
//! since NaN values are not equal to anything, including themselves.
//!
//! # Making comparisons
//!
//! The [`float_eq!`] and [`float_ne!`] macros compare two floating point
//! expressions for equality based on the result of one or more different kinds
//! of check. A check is invoked by name and an upper boundary, so for example
//! `abs <= 0.1`, should be read as *"an absolute epsilon check with a maximum
//! difference of less than or equal to 0.1"*. This example makes use of the
//! [relative epsilon comparison](#relative-epsilon-comparison) `rmax`:
//!
//! ```
//! # use float_eq::{assert_float_eq, assert_float_ne, float_eq, float_ne};
//! const RECIP_REL_EPSILON: f32 = 0.000_366_210_94; // 1.5 * 2f32.powi(-12)
//! assert!(float_eq!(0.1f32.recip(), 10.0, rmax <= RECIP_REL_EPSILON));
//! ```
//!
//! Similarly for [`assert_float_eq!`] and [`assert_float_ne!`], which may
//! optionally use a custom panic message:
//!
//! ```
//! # use float_eq::assert_float_eq;
//! let a: f32 = 4.0;
//! let b: f32 = 4.000_002_5;
//! assert_float_eq!(a - b, 0.0, abs <= 0.000_01);
//! assert_float_eq!(a - b, 0.0, abs <= 0.000_01, "Checking that {} == {}", a, b);
//! ```
//!
//! Checks may be used alone or in combination. If more than one check is
//! specified then they are performed in order from left to right. If any check
//! is true, then the two values are considered equal and the process is
//! shortcut. For example, this expression:
//!
//! ```
//! # use float_eq::float_eq;
//! # let a = 1.0f32;
//! # let b = 1.0f32;
//! float_eq!(a, b, abs <= 0.000_01, ulps <= 4)
//! # ;
//! ```
//!
//! Is equivalent to:
//!
//! ```
//! # use float_eq::float_eq;
//! # let a = 1.0f32;
//! # let b = 1.0f32;
//! float_eq!(a, b, abs <= 0.000_01) || float_eq!(a, b, ulps <= 4)
//! # ;
//! ```
//!
//! ## Absolute epsilon comparison
//!
//! A check to see how far apart two expressions are by comparing the absolute
//! difference between them to an absolute threshold. Equivalent to, using `f32`
//! as an example:
//!
//! ```rust
//! fn float_eq_abs(a: f32, b: f32, max_diff: f32) -> bool {
//!     // the PartialEq check covers equality of infinities
//!     a == b || (a - b).abs() <= max_diff
//! }
//! # float_eq::assert_float_eq!(4f32, 4.000_002_5, abs <= 0.000_002_5);
//! # assert!(float_eq_abs(4f32, 4.000_002_5, 0.000_002_5));
//! ```
//!
//! Absolute epsilon tests *do not* work well for general floating point comparison,
//! because they do not take into account that normal values' granularity changes
//! with their magnitude. Thus any given choice of `max_diff` is likely to work
//! for one specific power of two range and poorly outside of it.
//!
//! However, absolute epsilon comparison is often the best choice when comparing
//! against zero, since most values that fall into this category are likely to
//! have undergone catastrophic cancellation and thus have a very high relative
//! error, making it difficult to select appropriate thresholds for relative
//! epsilon checks. They can also be useful for testing against infinities, as
//! relative checks fail in a similar fashion.
//!
//! ## Relative epsilon comparison
//!
//! A check to see how far apart two expressions are by comparing the absolute
//! difference between them to an epsilon that is scaled to the granularity of
//! one of the inputs. Equivalent to, using `f32` as an example:
//!
//! ```rust
//! # fn func(a: f32, b: f32) -> f32 { a.max(b) }
//! fn float_eq_relative(a: f32, b: f32, max_diff: f32) -> bool {
//!     // the PartialEq check covers equality of infinities
//!     a == b || {
//!         let chosen = func(a.abs(), b.abs());
//!         (a - b).abs() <= (chosen * max_diff)
//!     }
//! }
//! # float_eq::assert_float_eq!(4.0f32, 4.000_002_5, rmax <= 0.000_000_6);
//! # assert!(float_eq_relative(4.0f32, 4.000_002_5, 0.000_000_6));
//! ```
//!
//! Where `func` is one of:
//! - `rmax`: the larger magnitude (aka `rel` for legacy reasons)
//! - `rmin`: the smaller magnitude
//! - `r1st`: the first input
//! - `r2nd`: the second input
//!
//! The first of these check types (`rmax`) is a good general algorithm to use
//! for comparing normal floats in the absence of a reason to use one of the others
//! and is the one most often provided by other libraries. The `r1st` and `r2nd`
//! options may be useful in unit tests for comparing against a specifically computed
//! expected value. Note that a relative epsilon check does not implicitly include
//! an absolute check, so if you wish to use both you must specify both.
//!
//! Relative epsilon checks are a good general choice for comparing normal floats
//! since they take into account the relative granularity of the inputs, however
//! they are a bad choice for comparing against zero or infinity, since the relative
//! error at those extremes often makes it hard or impossible to select a sensible
//! threshold.
//!
//! Choice of epsilon is best made by considering the range of normal values
//! beginning with `1.0`, since then a threshold of `n * f32::EPSILON` will test
//! for equality within a relative error margin of  `n` representable values
//! regardless of the specific inputs. Be aware that this reasoning becomes a
//! little shakey around the edges of the power of two ranges due to the granularity
//! changing. If you're having trouble with these cases, an ULPs comparison may
//! be more useful.
//!
//! ## Units in the Last Place (ULPs) comparison
//!
//! A check to see how far apart two expressions are by comparing the number of
//! representable values between them. This works by interpreting the bitwise
//! representation of the input values as integers and comparing the absolute
//! difference between those. Equivalent to, using `f32` as an example:
//!
//! ```rust
//! fn float_eq_ulps(a: f32, b: f32, max_diff: u32) -> bool {
//!     if a.is_nan() || b.is_nan() {
//!         false // NaNs are never equal
//!     } else if a.is_sign_positive() != b.is_sign_positive() {
//!         a == b // values of different signs are only equal if both are zero.
//!     } else {
//!         let a_bits = a.to_bits();
//!         let b_bits = b.to_bits();
//!         let max = a_bits.max(b_bits);
//!         let min = a_bits.min(b_bits);
//!         (max - min) <= max_diff
//!     }
//! }
//! # float_eq::assert_float_eq!(4f32, 4.000_002_5, ulps <= 5);
//! # assert!(float_eq_ulps(4f32, 4.000_002_5, 5));
//! ```
//!
//! Thanks to a deliberate quirk in the way the [underlying format] of IEEE floats
//! was designed, this is a measure of how near two values are that scales with
//! their relative granularity. Note that `max_diff` is an unsigned integer, so
//! for example `ulps <= 4` means *"check that a and b are equal to within a
//! distance of four or less representable values"*.
//!
//! ULPs comparisons are very similar to relative epsilon checks, and as such are
//! useful for testing equality of normal floats but less so for comparisons with
//! zero or infinity. Additionally, because floats use their most significant bit
//! to indicate their sign, ULPs comparisons are not valid for comparing values
//! with different signs. They can be easier to parameterize than relative epsilon
//! checks once you get used to them, since ULPs are closer to the raw hardware
//! representation and don't suffer from the same problems around powers of two
//! values.
//!
//! ## Which check(s) should I use?
//!
//! This really does depend a lot on your specific algorithm's workings and the
//! magnitude of your inputs and their error margins. A test of the result of
//! finite difference [approximation of derivatives] might use a relative epsilon
//! check with a threshold of the square root of machine epsilon, whereas a test
//! of the SSE [`_mm_rcp_ps` operation] could instead opt for a maximum relative
//! error of `1.5 * 2^(-12)` based on the available documentation. Be prepared
//! to research, test, benchmark and iterate on your comparisons to get the best
//! results. Having said that, there are some rules of thumb you can apply.
//!
//! If you are comparing two non-zero normal numbers, try using `ulps` (only if
//! the two expressions are the same sign), `rmax`, or some other relative
//! epsilon check:
//!
//! ```
//! # use float_eq::assert_float_eq;
//! let a: f32 = 4.0;
//! let b: f32 = 3.999_999_2;
//!
//! assert_float_eq!(a, b, ulps <= 4);
//! assert_float_eq!(a, b, rmax <= 2.0 * f32::EPSILON);
//! assert_float_eq!(a, b, r2nd <= 4.0 * f32::EPSILON);
//! ```
//!
//! If you are comparing against zero or infinity, especially if you know the
//! value was computed from the subtraction of two larger magnitude values, try
//! an `abs` check:
//!
//! ```
//! # use float_eq::assert_float_eq;
//! let a: f32 = 4.0;
//! let b: f32 = 3.999_999_2;
//!
//! assert_float_eq!(a - b, 0.0, abs <= 0.000_001);
//! ```
//!
//! If your values may be zero or normals, you should try combining an `abs`
//! check with a relative check of some kind:
//!
//! ```
//! # use float_eq::assert_float_eq;
//! let a: f32 = 4.0;
//! let b: f32 = 3.999_999_2;
//!
//! assert_float_eq!(a, b, abs <= 0.000_001, rmax <= 4.0 * f32::EPSILON);
//! assert_float_eq!(a - b, 0.0, abs <= 0.000_001, rmax <= 4.0 * f32::EPSILON);
//!
//! assert_float_eq!(a, b, abs <= 0.000_001, ulps <= 4);
//! assert_float_eq!(a - b, 0.0, abs <= 0.000_001, ulps <= 4);
//! ```
//!
//! # Comparing composite types
//!
//! When comparing composite values using the standard check types (`abs`, `rmax`,
//! `ulps`, etc), epsilon is an instance specifying per-field threshold values.
//! If a type's fields are all of the same type, then you may make use of the
//! `_all` variants (e.g. `abs_all`, `rmax_all`, `ulps_all`) to use the same
//! epsilon value across all fields. For example, arrays may be compared using
//! an epsilon that covers each index separately:
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
//! Similarly, if the relevant traits have been implemented for a struct type:
//!
//! ```
//! # use float_eq::{
//! #     assert_float_eq, FloatEqUlpsEpsilon, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
//! #     AssertFloatEq, AssertFloatEqAll, UlpsEpsilon, DebugUlpsDiff
//! # };
//! #
//! # #[derive(Debug, Clone, Copy, PartialEq)]
//! # struct Complex32 { re: f32, im: f32 }
//! #
//! # #[derive(Debug, Clone, Copy, PartialEq)]
//! # struct Complex32Ulps { re: u32, im: u32 }
//! #
//! # #[derive(Debug, Clone, Copy, PartialEq)]
//! # struct Complex32DebugUlpsDiff { re: DebugUlpsDiff<f32>, im: DebugUlpsDiff<f32> }
//! #
//! # impl FloatEqUlpsEpsilon for Complex32 { type UlpsEpsilon = Complex32Ulps; }
//! # impl FloatEqDebugUlpsDiff for Complex32 { type DebugUlpsDiff = Complex32DebugUlpsDiff; }
//! #
//! # impl FloatEq for Complex32 {
//! #     type Epsilon = Complex32;
//! #     fn eq_abs(&self, other: &Self, max_diff: &Complex32) -> bool {
//! #         self.re.eq_abs(&other.re, &max_diff.re) && self.im.eq_abs(&other.im, &max_diff.im)
//! #     }
//! #     fn eq_rmax(&self, other: &Self, max_diff: &Complex32) -> bool {
//! #         self.re.eq_rmax(&other.re, &max_diff.re) && self.im.eq_rmax(&other.im, &max_diff.im)
//! #     }
//! #     fn eq_rmin(&self, other: &Self, max_diff: &Complex32) -> bool {
//! #         self.re.eq_rmin(&other.re, &max_diff.re) && self.im.eq_rmin(&other.im, &max_diff.im)
//! #     }
//! #     fn eq_r1st(&self, other: &Self, max_diff: &Complex32) -> bool {
//! #         self.re.eq_r1st(&other.re, &max_diff.re) && self.im.eq_r1st(&other.im, &max_diff.im)
//! #     }
//! #     fn eq_r2nd(&self, other: &Self, max_diff: &Complex32) -> bool {
//! #         self.re.eq_r2nd(&other.re, &max_diff.re) && self.im.eq_r2nd(&other.im, &max_diff.im)
//! #     }
//! #     fn eq_ulps(&self, other: &Self, max_diff: &Complex32Ulps) -> bool {
//! #         self.re.eq_ulps(&other.re, &max_diff.re) && self.im.eq_ulps(&other.im, &max_diff.im)
//! #     }
//! # }
//! #
//! # impl FloatEqAll for Complex32 {
//! #     type AllEpsilon = f32;
//! #     fn eq_abs_all(&self, other: &Self, max_diff: &f32) -> bool {
//! #         self.re.eq_abs_all(&other.re, &max_diff) && self.im.eq_abs_all(&other.im, &max_diff)
//! #     }
//! #     fn eq_rmax_all(&self, other: &Self, max_diff: &f32) -> bool {
//! #         self.re.eq_rmax_all(&other.re, &max_diff) && self.im.eq_rmax_all(&other.im, &max_diff)
//! #     }
//! #     fn eq_rmin_all(&self, other: &Self, max_diff: &f32) -> bool {
//! #         self.re.eq_rmin_all(&other.re, &max_diff) && self.im.eq_rmin_all(&other.im, &max_diff)
//! #     }
//! #     fn eq_r1st_all(&self, other: &Self, max_diff: &f32) -> bool {
//! #         self.re.eq_r1st_all(&other.re, &max_diff) && self.im.eq_r1st_all(&other.im, &max_diff)
//! #     }
//! #     fn eq_r2nd_all(&self, other: &Self, max_diff: &f32) -> bool {
//! #         self.re.eq_r2nd_all(&other.re, &max_diff) && self.im.eq_r2nd_all(&other.im, &max_diff)
//! #     }
//! #     fn eq_ulps_all(&self, other: &Self, max_diff: &u32) -> bool {
//! #         self.re.eq_ulps_all(&other.re, &max_diff) && self.im.eq_ulps_all(&other.im, &max_diff)
//! #     }
//! # }
//! #
//! # impl AssertFloatEq for Complex32 {
//! #     type DebugAbsDiff = Complex32;
//! #     type DebugEpsilon = Complex32;
//! #     fn debug_abs_diff(&self, other: &Complex32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_abs_diff(&other.re),
//! #             im: self.im.debug_abs_diff(&other.im),
//! #         }
//! #     }
//! #     fn debug_ulps_diff(&self, other: &Complex32) -> Complex32DebugUlpsDiff {
//! #         Complex32DebugUlpsDiff {
//! #             re: self.re.debug_ulps_diff(&other.re),
//! #             im: self.im.debug_ulps_diff(&other.im),
//! #         }
//! #     }
//! #     fn debug_abs_epsilon(&self, other: &Self, max_diff: &Complex32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_abs_epsilon(&other.re, &max_diff.re),
//! #             im: self.im.debug_abs_epsilon(&other.re, &max_diff.im),
//! #         }
//! #     }
//! #     fn debug_rmax_epsilon(&self, other: &Self, max_diff: &Complex32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_rmax_epsilon(&other.re, &max_diff.re),
//! #             im: self.im.debug_rmax_epsilon(&other.re, &max_diff.im),
//! #         }
//! #     }
//! #     fn debug_rmin_epsilon(&self, other: &Self, max_diff: &Complex32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_rmin_epsilon(&other.re, &max_diff.re),
//! #             im: self.im.debug_rmin_epsilon(&other.re, &max_diff.im),
//! #         }
//! #     }
//! #     fn debug_r1st_epsilon(&self, other: &Self, max_diff: &Complex32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_r1st_epsilon(&other.re, &max_diff.re),
//! #             im: self.im.debug_r1st_epsilon(&other.re, &max_diff.im),
//! #         }
//! #     }
//! #     fn debug_r2nd_epsilon(&self, other: &Self, max_diff: &Complex32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_r2nd_epsilon(&other.re, &max_diff.re),
//! #             im: self.im.debug_r2nd_epsilon(&other.re, &max_diff.im),
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
//! # impl AssertFloatEqAll for Complex32 {
//! #     type AllDebugEpsilon = Complex32;
//! #     fn debug_abs_all_epsilon(&self, other: &Self, max_diff: &f32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_abs_all_epsilon(&other.re, &max_diff),
//! #             im: self.im.debug_abs_all_epsilon(&other.re, &max_diff),
//! #         }
//! #     }
//! #     fn debug_rmax_all_epsilon(&self, other: &Self, max_diff: &f32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_rmax_all_epsilon(&other.re, &max_diff),
//! #             im: self.im.debug_rmax_all_epsilon(&other.re, &max_diff),
//! #         }
//! #     }
//! #     fn debug_rmin_all_epsilon(&self, other: &Self, max_diff: &f32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_rmin_all_epsilon(&other.re, &max_diff),
//! #             im: self.im.debug_rmin_all_epsilon(&other.re, &max_diff),
//! #         }
//! #     }
//! #     fn debug_r1st_all_epsilon(&self, other: &Self, max_diff: &f32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_r1st_all_epsilon(&other.re, &max_diff),
//! #             im: self.im.debug_r1st_all_epsilon(&other.re, &max_diff),
//! #         }
//! #     }
//! #     fn debug_r2nd_all_epsilon(&self, other: &Self, max_diff: &f32) -> Complex32 {
//! #         Complex32 {
//! #             re: self.re.debug_r2nd_all_epsilon(&other.re, &max_diff),
//! #             im: self.im.debug_r2nd_all_epsilon(&other.re, &max_diff),
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
//! assert_float_eq!(a, b, rmax <= Complex32 { re: 0.000_000_25, im: 0.000_000_5 });
//! assert_float_eq!(a, b, rmax_all <= 0.000_000_5);
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
//! assert_float_eq!(4.0f32, 4.000_008, rmax <= 0.000_001);
//! ```
//!
//! Panics with this error message, where the relative epsilon, `[rel] ε`, has
//! been scaled based on the size of the inputs (ε is the greek letter epsilon):
//!
//! ```text
//! thread 'main' panicked at 'assertion failed: `float_eq!(left, right, rmax <= ε)`
//!         left: `4.0`,
//!        right: `4.000008`,
//!     abs_diff: `0.000008106232`,
//!    ulps_diff: `Some(17)`,
//!     [rmax] ε: `0.000004000008`', assert_failure.rs:15:5
//! ```
//!
//! # Comparing custom types
//!
//! Comparison of new types using `float_eq!` is supported by implementing
//! [`FloatEqUlpsEpsilon`], [`FloatEq`] and optionally [`FloatEqAll`]. Support
//! for `assert_float_eq!` may be enabled by also implementing [`FloatEqDebugUlpsDiff`]
//! and [`AssertFloatEq`]/[`AssertFloatEqAll`].
//!
//! ## Derivable
#![cfg_attr(
    not(feature = "derive"),
    doc = r##"
These traits are all derivable if the `"derive"` feature is enabled.

For example, add this to your Cargo.toml:

```text
[dependencies.float_eq]
version = "0.5"
features = ["derive"]
```
"##
)]
#![cfg_attr(
    feature = "derive",
    doc = r##"
The easiest way to implement these traits is with the [`#[derive_float_eq]`](attr.derive_float_eq.html)
helper macro. The `ulps_epsilon` and `debug_ulps_diff` parameters are required.
They are used to name two new types that match the structure of the type being
derived from. The first is used to provide ULPs epsilon values per field, and
the second is used to provide debug information for the differerence between
values in ULPs.

The `all_epsilon` parameter is optional. If provided, it will additionally
implement the traits required to use the `_all` variants of checks, using the
given epsilon type (usually `f32` or `f64`). 

At present, only non-generic structs and tuple structs may be derived:
```
# use float_eq::{assert_float_eq, derive_float_eq};
#[derive_float_eq(
    ulps_epsilon = "PointUlps", 
    debug_ulps_diff = "PointDebugUlpsDiff",
    all_epsilon = "f64"
)]
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

let a = Point { x: 1.0, y: -2.0 };
let b = Point { x: 1.1, y: -2.2 };
assert_float_eq!(a, b, abs <= Point { x: 0.15, y: 0.25 });
assert_float_eq!(a, b, abs_all <= 0.25);

let c = Point { x: 1.000_000_000_000_000_9, y: -2.000_000_000_000_001_3 };
let eps = f64::EPSILON;
assert_float_eq!(a, c, rmax <= Point { x: 4.0 * eps, y: 5.0 * eps });
assert_float_eq!(a, c, rmax_all <= 5.0 * eps);
assert_float_eq!(a, c, ulps <= PointUlps { x: 4, y: 3 });
assert_float_eq!(a, c, ulps_all <= 4);
```
"##
)]
//!
//! [`assert_float_eq!`]: macro.assert_float_eq.html
//! [`assert_float_ne!`]: macro.assert_float_ne.html
//! [`float_eq!`]: macro.float_eq.html
//! [`float_ne!`]: macro.float_ne.html
//! [`FloatEqUlpsEpsilon`]: trait.FloatEqUlpsEpsilon.html
//! [`FloatEq`]: trait.FloatEq.html
//! [`FloatEqAll`]: trait.FloatEqAll.html
//! [`FloatEqDebugUlpsDiff`]: trait.FloatEqDebugUlpsDiff.html
//! [`AssertFloatEq`]: trait.AssertFloatEq.html
//! [`AssertFloatEqAll`]: trait.AssertFloatEqAll.html
//!
//! [catastrophic cancellation]: https://en.wikipedia.org/wiki/Loss_of_significance
//! [Subnormal]: https://en.wikipedia.org/wiki/Denormal_number
//! [floating point comparison]: https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
//! [Normal]: https://en.wikipedia.org/wiki/Normal_number_(computing)
//! [underlying format]: https://randomascii.wordpress.com/2012/01/23/stupid-float-tricks-2/
//! [approximation of derivatives]: https://scicomp.stackexchange.com/questions/14355/choosing-epsilons
//! [`_mm_rcp_ps` operation]: https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_rcp_ps&expand=4482

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
