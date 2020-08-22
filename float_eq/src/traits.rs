use core::fmt;

/// Per-field thresholds for [ULPs](index.html#units-in-the-last-place-ulps-comparison)
/// based comparisons.
///
/// This trait establishes a one-to-one relation between an IEEE floating point
/// type and a type whose fields are expected to be structurally identical but
/// specified in [ULPs]. It is used by ULPS-based equality checks to specify
/// per-field thresholds. The [`UlpsEpsilon`] type alias exists to simplify
/// usage, for example `UlpsEpsilon<f32>` is `u32`. Usually, this type is named
/// `FooUlps` for a given type `Foo`.
///
/// ## Derivable
#[cfg_attr(
    not(feature = "derive"),
    doc = r##"
This trait is derivable if the `"derive"` feature is enabled.

For example, add this to your Cargo.toml:

```text
[dependencies.float_eq]
version = "0.4"
features = ["derive"]
```
"##
)]
#[cfg_attr(
    feature = "derive",
    doc = r##"
This trait can be used with `#[derive]`. The easiest way to do so is to use the
[`#[derive_float_eq]`](attr.derive_float_eq.html) helper macro, see the top
level docs for [example usage](index.html#derivable).

If you wish to derive this trait by itself, you will need to use a `#[float_eq]`
attribute and provide `ulps_epsilon`, which will be used as the name of a new
type. This type will be structurally identical to the type being derived, using
the same visibility as the parent type and with identically named fields that
use the derived fields' types wrapped by `UlpsEpsilon`. The new struct derives
`Debug`, `Clone`, `Copy` and `PartialEq`. This trait may not be derived for enums
or generic structs at present.

```
# use float_eq::{FloatEqUlpsEpsilon, UlpsEpsilon};
#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon)]
#[float_eq(ulps_epsilon = "PointUlps")]
struct Point {
    x: f64,
    y: f64,
}

assert_eq!(
    PointUlps { x: 1, y: 2 },
    UlpsEpsilon::<Point> { x: 1, y: 2 }
);
```
"##
)]
///
/// ## How can I implement `FloatEqUlpsEpsilon`?
///
/// Types should provide an [`UlpsEpsilon`] representation for each of their fields:
///
/// ```
/// # use float_eq::{FloatEqUlpsEpsilon, UlpsEpsilon};
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct Point {
///     x: f64,
///     y: f64,
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct PointUlps {
///     x: UlpsEpsilon<f64>,
///     y: UlpsEpsilon<f64>,
/// }
///
/// impl FloatEqUlpsEpsilon for Point {
///     type UlpsEpsilon = PointUlps;
/// }
///
/// assert_eq!(
///     PointUlps { x: 1, y: 2 },
///     UlpsEpsilon::<Point> { x: 1, y: 2 }
/// );
/// ```
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
/// [`UlpsEpsilon`]: type.UlpsEpsilon.html
pub trait FloatEqUlpsEpsilon {
    /// A structurally identical type to `Self`, with fields recursively wrapped
    /// by `UlpsEpsilon`.
    type UlpsEpsilon: ?Sized;
}

/// Per-field thresholds for [ULPs] based comparisons.
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
pub type UlpsEpsilon<T> = <T as FloatEqUlpsEpsilon>::UlpsEpsilon;

/// Per-field results of [ULPs](index.html#units-in-the-last-place-ulps-comparison)
/// based diff calculations.
///
/// This trait establishes a one-to-one relation between an IEEE floating point
/// type and a type whose fields are expected to be structurally identical but
/// specified as the result of calculating a diff in [ULPs]. It is used by testing
/// and debugging tools to show the difference between two values on a per-field
/// basis and is built for clarity, not runtime efficiency. The [`DebugUlpsDiff`]
/// type alias exists to simplify usage, for example `DebugUlpsDiff<f32>` is
/// `Option<u32>`. Usually, this type is named `FooDebugUlpsDiff` for a given
/// type `Foo`.
///
/// ## Derivable
#[cfg_attr(
    not(feature = "derive"),
    doc = r##"
This trait is derivable if the `"derive"` feature is enabled.

For example, add this to your Cargo.toml:

```text
[dependencies.float_eq]
version = "0.4"
features = ["derive"]
```
"##
)]
#[cfg_attr(
    feature = "derive",
    doc = r##"
This trait can be used with `#[derive]`. The easiest way to do so is to use the
[`#[derive_float_eq]`](attr.derive_float_eq.html) helper macro, see the top
level docs for [example usage](index.html#derivable).

If you wish to derive this trait by itself, you will need to use a `#[float_eq]`
attribute and provide `debug_ulps_diff`, which will be used as the name of a new
type. This type will be structurally identical to the type being derived, using
the same visibility as the parent type and with identically named fields that
use the derived fields' types wrapped by `DebugUlpsDiff`. The new struct derives
`Debug`, `Clone`, `Copy` and `PartialEq`. This trait may not be derived for enums
or generic structs at present.

```
# use float_eq::{FloatEqDebugUlpsDiff, DebugUlpsDiff};
#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
#[float_eq(debug_ulps_diff = "PointDebugUlpsDiff")]
struct Point {
    x: f64,
    y: f64,
}

assert_eq!(
    PointDebugUlpsDiff { x: Some(1), y: None },
    DebugUlpsDiff::<Point> { x: Some(1), y: None }
);
```
"##
)]
///
/// ## How can I implement `FloatEqDebugUlpsDiff`?
///
/// Types should provide a [`DebugUlpsDiff`] representation for each of their fields:
///
/// ```
/// # use float_eq::{FloatEqDebugUlpsDiff, DebugUlpsDiff};
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct Point {
///     x: f64,
///     y: f64,
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct PointDebugUlpsDiff {
///     x: DebugUlpsDiff<f64>,
///     y: DebugUlpsDiff<f64>,
/// }
///
/// impl FloatEqDebugUlpsDiff for Point {
///     type DebugUlpsDiff = PointDebugUlpsDiff;
/// }
///
/// assert_eq!(
///     PointDebugUlpsDiff { x: Some(1), y: None },
///     DebugUlpsDiff::<Point> { x: Some(1), y: None }
/// );
/// ```
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
/// [`DebugUlpsDiff`]: type.DebugUlpsDiff.html
pub trait FloatEqDebugUlpsDiff {
    /// A structurally identical type to `Self`, with fields recursively wrapped
    /// by `DebugUlpsDiff`.
    type DebugUlpsDiff;
}

/// Per-field results of [ULPs] based diff calculations.
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
pub type DebugUlpsDiff<T> = <T as FloatEqDebugUlpsDiff>::DebugUlpsDiff;

/// Compare IEEE floating point values for equality using per-field thresholds.
///
/// This trait is used in the implementation of the [`float_eq!`] and [`assert_float_eq!`]
/// families of macros.
///
/// ## Derivable
#[cfg_attr(
    not(feature = "derive"),
    doc = r##"
This trait is derivable if the `"derive"` feature is enabled.

For example, add this to your Cargo.toml:

```text
[dependencies.float_eq]
version = "0.4"
features = ["derive"]
```
"##
)]
#[cfg_attr(
    feature = "derive",
    doc = r##"
This trait can be used with `#[derive]`. The easiest way to do so is to use the
[`#[derive_float_eq]`](attr.derive_float_eq.html) helper macro, see the top
level docs for [example usage](index.html#derivable).

If you wish to derive this trait by itself, you must first implement [`FloatEqUlpsEpsilon`],
which may also be derived. You will also need a `#[float_eq]` attribute and provide
`ulps_epsilon`, which should match the name of the `UlpsEpsilon` type. Two
instances are equal if all fields are equal, and not equal if any are not. This
trait may not be derived for enums or generic structs at present.

```
# use float_eq::{FloatEqUlpsEpsilon, FloatEq, UlpsEpsilon};
#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon, FloatEq)]
#[float_eq(ulps_epsilon = "PointUlps")]
struct Point {
    x: f32,
    y: f32,
}

let a = Point { x: 1.0, y: -2.0 };
let b = Point { x: 1.5, y: -3.0 };
assert!(a.eq_abs(&b, &Point { x: 0.5, y: 1.0 }));
assert!(a.ne_abs(&b, &Point { x: 0.4, y: 1.0 }));
assert!(a.ne_abs(&b, &Point { x: 0.5, y: 0.9 }));

let c = Point { x: 1.000_000_1, y: -2.000_000_5 };
assert!(a.eq_ulps(&c, &PointUlps { x: 1, y: 2 }));
assert!(a.ne_ulps(&c, &PointUlps { x: 0, y: 2 }));
assert!(a.ne_ulps(&c, &PointUlps { x: 1, y: 1 }));
```
"##
)]
///
/// ## How can I implement `FloatEq`?
///
/// You will need to implement [`FloatEqUlpsEpsilon`] for your type. Implementation
/// is then usually a matter of calling through to an underlying `FloatEq` method
/// for each field in turn. If not, you will need to take a close look at the
/// descriptions of the algorithms on a method by method basis:
///
/// ```
/// # use float_eq::{FloatEqUlpsEpsilon, FloatEq, UlpsEpsilon};
/// #[derive(Debug, Copy, Clone, PartialEq)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// #[derive(Debug, Copy, Clone, PartialEq)]
/// struct MyComplex32Ulps {
///     re: UlpsEpsilon<f32>,
///     im: UlpsEpsilon<f32>,
/// }
///
/// impl FloatEqUlpsEpsilon for MyComplex32 {
///     type UlpsEpsilon = MyComplex32Ulps;   
/// }
///
/// impl FloatEq for MyComplex32 {
///     type Epsilon = MyComplex32;
///
///     fn eq_abs(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_abs(&other.re, &max_diff.re) && self.im.eq_abs(&other.im, &max_diff.im)
///     }
///
///     fn eq_rmax(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_rmax(&other.re, &max_diff.re) && self.im.eq_rmax(&other.im, &max_diff.im)
///     }
///
///     fn eq_rmin(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_rmin(&other.re, &max_diff.re) && self.im.eq_rmin(&other.im, &max_diff.im)
///     }
///
///     fn eq_r1st(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_r1st(&other.re, &max_diff.re) && self.im.eq_r1st(&other.im, &max_diff.im)
///     }
///
///     fn eq_r2nd(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_r2nd(&other.re, &max_diff.re) && self.im.eq_r2nd(&other.im, &max_diff.im)
///     }
///
///     fn eq_ulps(&self, other: &Self, max_diff: &UlpsEpsilon<MyComplex32>) -> bool {
///         self.re.eq_ulps(&other.re, &max_diff.re) && self.im.eq_ulps(&other.im, &max_diff.im)
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 2.000_003_6, };
/// let b = MyComplex32 { re: 1.000_000_1, im: 2.0, };
///
/// assert!(a.eq_abs(&b, &MyComplex32 { re: 0.000_000_15, im: 0.000_003_6 }));
/// assert!(a.ne_abs(&b, &MyComplex32 { re: 0.000_000_05, im: 0.000_003_6 }));
/// assert!(a.ne_abs(&b, &MyComplex32 { re: 0.000_000_15, im: 0.000_003_5 }));
///
/// assert!(a.eq_rel(&b, &MyComplex32 { re: 0.000_000_15, im: 0.000_001_8 }));
/// assert!(a.ne_rel(&b, &MyComplex32 { re: 0.000_000_05, im: 0.000_001_8 }));
/// assert!(a.ne_rel(&b, &MyComplex32 { re: 0.000_000_15, im: 0.000_001_7 }));
///
/// assert!(a.eq_ulps(&b, &MyComplex32Ulps { re: 1, im: 15 }));
/// assert!(a.ne_ulps(&b, &MyComplex32Ulps { re: 0, im: 15 }));
/// assert!(a.ne_ulps(&b, &MyComplex32Ulps { re: 1, im: 14 }));
/// ```
///
/// ## Examples
///
/// ```
/// # use float_eq::FloatEq;
/// assert!(4.0_f32.eq_abs(&4.000_001_5, &0.000_001_6));
/// assert!(4.0_f32.ne_abs(&4.000_001_5, &0.000_001_4));
///
/// assert!(4.0_f32.eq_rel(&4.000_001_5, &0.000_000_4));
/// assert!(4.0_f32.ne_rel(&4.000_001_5, &0.000_000_3));
///
/// assert!(4.0_f32.eq_ulps(&4.000_001_5, &3));
/// assert!(4.0_f32.ne_ulps(&4.000_001_5, &2));
/// ```
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
/// [`FloatEqUlpsEpsilon`]: trait.FloatEqUlpsEpsilon.html
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`float_eq!`]: macro.float_eq.html
pub trait FloatEq<Rhs: ?Sized = Self> {
    /// Type of the maximum allowed difference between two values for them to be
    /// considered equal.
    type Epsilon: ?Sized + FloatEqUlpsEpsilon;

    /// Check whether `self` is equal to `other`, using an [absolute epsilon
    /// comparison].
    ///
    /// Implementations should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_abs(&self, other: &Self, max_diff: &Self) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_abs(&self, other: &Self, max_diff: &Self) -> bool {
    /// // the PartialEq check covers equality of infinities
    /// self == other || (self - other).abs().le(max_diff)
    /// # }}
    /// ```
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn eq_abs(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using an [absolute epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_abs(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    #[inline]
    fn ne_abs(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool {
        !self.eq_abs(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `self.eq_rmax(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn eq_rel(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool {
        self.eq_rmax(other, max_diff)
    }

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_rel(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_rel(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool {
        !self.eq_rel(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison], scaled to the granularity of the input with the largest
    /// magnitude.
    ///
    /// The implementation should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_rel(&self, other: &Self, max_diff: &Self) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_rel(&self, other: &Self, max_diff: &Self) -> bool {
    /// // the PartialEq check covers equality of infinities
    /// self == other || {
    ///     let largest = self.abs().max(other.abs());
    ///     let epsilon = largest * max_diff;
    ///     (self - other).abs() <= epsilon
    /// }
    /// # }}
    /// ```
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_rmax(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_rmax(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_rmax(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool {
        !self.eq_rmax(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison], scaled to the granularity of the input with the smallest
    /// magnitude.
    ///
    /// The implementation should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_rel(&self, other: &Self, max_diff: &Self) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_rel(&self, other: &Self, max_diff: &Self) -> bool {
    /// // the PartialEq check covers equality of infinities
    /// self == other || {
    ///     let smallest = self.abs().min(other.abs());
    ///     let epsilon = smallest * max_diff;
    ///     (self - other).abs() <= epsilon
    /// }
    /// # }}
    /// ```
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_rmin(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_rmin(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_rmin(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool {
        !self.eq_rmin(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison], scaled to the granularity of the first input.
    ///
    /// The implementation should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_rel(&self, other: &Self, max_diff: &Self) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_rel(&self, other: &Self, max_diff: &Self) -> bool {
    /// // the PartialEq check covers equality of infinities
    /// self == other || {
    ///     let epsilon = self.abs() * max_diff;
    ///     (self - other).abs() <= epsilon
    /// }
    /// # }}
    /// ```
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_r1st(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_r1st(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_r1st(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool {
        !self.eq_r1st(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison], scaled to the granularity of the input with the largest
    /// magnitude.
    ///
    /// The implementation should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_rel(&self, other: &Self, max_diff: &Self) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_rel(&self, other: &Self, max_diff: &Self) -> bool {
    /// // the PartialEq check covers equality of infinities
    /// self == other || {
    ///     let epsilon = other.abs() * max_diff;
    ///     (self - other).abs() <= epsilon
    /// }
    /// # }}
    /// ```
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_r2nd(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_r2nd(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_r2nd(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool {
        !self.eq_r2nd(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using an [ULPs comparison].
    ///
    /// The implementation should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_ulps(&self, other: &Self, max_diff: &u32) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_ulps(&self, other: &Self, max_diff: &u32) -> bool {
    /// if self.is_nan() || other.is_nan() {
    ///     false // NaNs are never equal
    /// }
    /// else if self.is_sign_positive() != other.is_sign_positive() {
    ///     self == other // account for zero == negative zero
    /// } else {
    ///     let a = self.to_bits();
    ///     let b = other.to_bits();
    ///     let max = a.max(b);
    ///     let min = a.min(b);
    ///     (max - min).le(max_diff)
    /// }
    /// # }}
    /// ```
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn eq_ulps(&self, other: &Rhs, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool;

    /// Check whether `self` is not equal to `other`, using an [ULPs comparison].
    ///
    /// Equal to `!self.eq_ulps(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    #[inline]
    fn ne_ulps(&self, other: &Rhs, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
        !self.eq_ulps(other, max_diff)
    }
}

/// Compare IEEE floating point values for equality using a uniform threshold.
///
/// This trait is used in the implementation of the [`float_eq!`] and [`assert_float_eq!`]
/// families of macros. Comparison via this trait may not fit every composite
/// type. For example, it likely ought not to be implemented for `(f32, f64)`,
/// which has a big difference in granularity between its fields.
///
/// ## Derivable
#[cfg_attr(
    not(feature = "derive"),
    doc = r##"
This trait is derivable if the `"derive"` feature is enabled.

For example, add this to your Cargo.toml:

```text
[dependencies.float_eq]
version = "0.4"
features = ["derive"]
```
"##
)]
#[cfg_attr(
    feature = "derive",
    doc = r##"
This trait can be used with `#[derive]`. The easiest way to do so is to use the
[`#[derive_float_eq]`](attr.derive_float_eq.html) helper macro, see the top
level docs for [example usage](index.html#derivable).

If you wish to derive this trait by itself, you will need a `#[float_eq]` attribute
specifying `all_epsilon`, which is the type to be used as [`AllEpsilon`], and is
usually `f32` or `f64`. Two instances are equal if all fields are equal, and not
equal if any are not. This trait may not be derived for enums or generic structs
at present.

```
# use float_eq::FloatEqAll;
#[derive(Debug, Clone, Copy, PartialEq, FloatEqAll)]
#[float_eq(ulps_epsilon = "PointUlps", all_epsilon = "f32")]
struct Point {
    x: f32,
    y: f32,
}

let a = Point { x: 1.0, y: -2.0 };
let b = Point { x: 1.5, y: -3.0 };
assert!(a.eq_abs_all(&b, &1.0));
assert!(a.ne_abs_all(&b, &0.9));

let c = Point { x: 1.000_000_1, y: -2.000_000_5 };
assert!(a.eq_ulps_all(&c, &2));
assert!(a.ne_ulps_all(&c, &1));
```
"##
)]
///
/// ## How can I implement `FloatEqAll`?
///
/// You will need to select an epsilon type to compare recursively with each field
/// in your type, usually `f32` or `f64`. Implementation is then usually a matter
/// of calling through to an underlying `FloatEqAll` method for each field in turn.
/// If not, you will need to take a close look at the descriptions of the algorithms
/// on a method by method basis:
///
/// ```
/// # use float_eq::{FloatEqAll, UlpsEpsilon};
/// #[derive(Debug, Copy, Clone, PartialEq)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// impl FloatEqAll for MyComplex32 {
///     type AllEpsilon = f32;
///
///     fn eq_abs_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_abs_all(&other.re, max_diff) && self.im.eq_abs_all(&other.im, max_diff)
///     }
///
///     fn eq_rmax_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_rmax_all(&other.re, max_diff) && self.im.eq_rmax_all(&other.im, max_diff)
///     }
///
///     fn eq_rmin_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_rmin_all(&other.re, max_diff) && self.im.eq_rmin_all(&other.im, max_diff)
///     }
///
///     fn eq_r1st_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_r1st_all(&other.re, max_diff) && self.im.eq_r1st_all(&other.im, max_diff)
///     }
///
///     fn eq_r2nd_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_r2nd_all(&other.re, max_diff) && self.im.eq_r2nd_all(&other.im, max_diff)
///     }
///
///     fn eq_ulps_all(&self, other: &Self, max_diff: &UlpsEpsilon<f32>) -> bool {
///         self.re.eq_ulps_all(&other.re, max_diff) && self.im.eq_ulps_all(&other.im, max_diff)
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 2.000_003_6, };
/// let b = MyComplex32 { re: 1.000_000_1, im: 2.0, };
///
/// assert!(a.eq_abs_all(&b, &0.000_003_6));
/// assert!(a.ne_abs_all(&b, &0.000_003_5));
///
/// assert!(a.eq_rmax_all(&b, &0.000_001_8));
/// assert!(a.ne_rmax_all(&b, &0.000_001_7));
///
/// assert!(a.eq_ulps_all(&b, &15));
/// assert!(a.ne_ulps_all(&b, &14));
/// ```
///
/// ## Examples
///
/// ```
/// # use float_eq::FloatEqAll;
/// let a = [1.000_000_2f32, -2.0];
/// let b = [1.0f32, -2.000_002];
///
/// assert!(a.eq_abs_all(&b, &0.000_002));
/// assert!(a.ne_abs_all(&b, &0.000_001));
///
/// assert!(a.eq_rmax_all(&b, &0.000_001));
/// assert!(a.ne_rmax_all(&b, &0.000_000_5));
///
/// assert!(a.eq_ulps_all(&b, &8));
/// assert!(a.ne_ulps_all(&b, &7));
/// ```
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
/// [`FloatEq`]: trait.FloatEq.html
/// [`AllEpsilon`]: trait.FloatEqAll.html#associatedtype.AllEpsilon
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`float_eq!`]: macro.float_eq.html
pub trait FloatEqAll<Rhs: ?Sized = Self> {
    /// Type of the maximum allowed difference between each of two values' fields
    /// for them to be considered equal.
    type AllEpsilon: ?Sized + FloatEqUlpsEpsilon;

    /// Check whether `self` is equal to `other`, using an [absolute epsilon
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_abs`].
    ///
    /// [`FloatEq::eq_abs`]: trait.FloatEq.html#tymethod.eq_abs
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn eq_abs_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using an [absolute epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_abs_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    #[inline]
    fn ne_abs_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool {
        !self.eq_abs_all(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `self.eq_rmax_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn eq_rel_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool {
        self.eq_rmax_all(other, max_diff)
    }

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_rel_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_rel_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool {
        !self.eq_rel_all(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_rmax`].
    ///
    /// [`FloatEq::eq_rmax`]: trait.FloatEq.html#tymethod.eq_rmax
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_rmax_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_rmax_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_rmax_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool {
        !self.eq_rmax_all(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_rmin`].
    ///
    /// [`FloatEq::eq_rmin`]: trait.FloatEq.html#tymethod.eq_rmin
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_rmin_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_rmin_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_rmin_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool {
        !self.eq_rmin_all(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_r1st`].
    ///
    /// [`FloatEq::eq_r1st`]: trait.FloatEq.html#tymethod.eq_r1st
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_r1st_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_r1st_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_r1st_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool {
        !self.eq_r1st_all(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_r2nd`].
    ///
    /// [`FloatEq::eq_r2nd`]: trait.FloatEq.html#tymethod.eq_r2nd
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_r2nd_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_r2nd_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_r2nd_all(&self, other: &Rhs, max_diff: &Self::AllEpsilon) -> bool {
        !self.eq_r2nd_all(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using an [ULPs comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_ulps`].
    ///
    /// [`FloatEq::eq_ulps`]: trait.FloatEq.html#tymethod.eq_ulps
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn eq_ulps_all(&self, other: &Rhs, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool;

    /// Check whether `self` is not equal to `other`, using an [ULPs comparison].
    ///
    /// Equal to `!self.eq_ulps_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    #[inline]
    fn ne_ulps_all(&self, other: &Rhs, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
        !self.eq_ulps_all(other, max_diff)
    }
}

/// Debug context for when an assert fails.
///
/// This is used internally by the [`assert_float_eq!`] family of macros.
///
/// ## Derivable
#[cfg_attr(
    not(feature = "derive"),
    doc = r##"
This trait is derivable if the `"derive"` feature is enabled.

For example, add this to your Cargo.toml:

```text
[dependencies.float_eq]
version = "0.4"
features = ["derive"]
```
"##
)]
#[cfg_attr(
    feature = "derive",
    doc = r##"
This trait can be used with `#[derive]`. The easiest way to do so is to use the
[`#[derive_float_eq]`](attr.derive_float_eq.html) helper macro, see the top
level docs for [example usage](index.html#derivable).

If you wish to derive this trait by itself, you must first implement [`FloatEqUlpsEpsilon`],
[`FloatEq`] and [`FloatEqDebugUlpsDiff`], all of which may also be derived. You
will also need a `#[float_eq]` attribute and provide `ulps_epsilon` and `ulps_debug_diff`,
which should match the name of the `UlpsEpsilon` and `DebugUlpsDiff` types. Each
field's epsilon is calculated via a recursive call to the algorithm being used.
This trait may not be derived for enums or generic structs at present.

```
# use float_eq::{
# FloatEqUlpsEpsilon, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq, UlpsEpsilon
# };
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(FloatEqUlpsEpsilon, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq)]
#[float_eq(ulps_epsilon = "PointUlps", debug_ulps_diff = "PointDebugUlpsDiff")]
struct Point {
    x: f32,
    y: f32,
}

let a = Point { x: 1.0, y: 200.0 };
let b = Point { x: 50.0, y: 1.0 };
let eps = Point { x: 0.1, y: 0.2 };
assert_eq!(a.debug_rmax_epsilon(&b, &eps), Point { x: 5.0, y: 40.0 });
```
"##
)]
///
/// ## How can I implement `AssertFloatEq`?
///
/// You must first implement [`FloatEqUlpsEpsilon`], [`FloatEq`] and [`FloatEqDebugUlpsDiff`].
/// Implementation is then usually a matter of simply calling through to an underlying
/// `AssertFloatEq`method for each field in turn. If not, you will need to take a
/// close look at the descriptions of the algorithms on a method by method basis:
///
/// ```
/// # use float_eq::{
/// # FloatEqUlpsEpsilon, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq, UlpsEpsilon, DebugUlpsDiff
/// # };
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32Ulps {
///     re: UlpsEpsilon<f32>,
///     im: UlpsEpsilon<f32>,
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32DebugUlpsDiff {
///     re: DebugUlpsDiff<f32>,
///     im: DebugUlpsDiff<f32>,
/// }
///
/// impl FloatEqUlpsEpsilon for MyComplex32 {
///     type UlpsEpsilon = MyComplex32Ulps;
/// }
///
/// impl FloatEqDebugUlpsDiff for MyComplex32 {
///     type DebugUlpsDiff = MyComplex32DebugUlpsDiff;
/// }
///
/// impl FloatEq for MyComplex32 {
///     type Epsilon = MyComplex32;
///
///     fn eq_abs(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_abs(&other.re, &max_diff.re) && self.im.eq_abs(&other.im, &max_diff.im)
///     }
///
///     fn eq_rmax(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_rmax(&other.re, &max_diff.re) && self.im.eq_rmax(&other.im, &max_diff.im)
///     }
///
///     fn eq_rmin(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_rmin(&other.re, &max_diff.re) && self.im.eq_rmin(&other.im, &max_diff.im)
///     }
///
///     fn eq_r1st(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_r1st(&other.re, &max_diff.re) && self.im.eq_r1st(&other.im, &max_diff.im)
///     }
///
///     fn eq_r2nd(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_r2nd(&other.re, &max_diff.re) && self.im.eq_r2nd(&other.im, &max_diff.im)
///     }
///
///     fn eq_ulps(&self, other: &Self, max_diff: &UlpsEpsilon<MyComplex32>) -> bool {
///         self.re.eq_ulps(&other.re, &max_diff.re) && self.im.eq_ulps(&other.im, &max_diff.im)
///     }
/// }
///
/// impl AssertFloatEq for MyComplex32 {
///     type DebugAbsDiff = Self;
///     type DebugEpsilon = Self;
///
///     fn debug_abs_diff(&self, other: &Self) -> MyComplex32 {
///         MyComplex32 {
///             re: self.re.debug_abs_diff(&other.re),
///             im: self.im.debug_abs_diff(&other.im),
///         }
///     }
///
///     fn debug_ulps_diff(&self, other: &Self) -> MyComplex32DebugUlpsDiff {
///         MyComplex32DebugUlpsDiff {
///             re: self.re.debug_ulps_diff(&other.re),
///             im: self.im.debug_ulps_diff(&other.im),
///         }
///     }
///
///     fn debug_abs_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &MyComplex32
///     ) -> MyComplex32 {
///         MyComplex32 {
///             re: self.re.debug_abs_epsilon(&other.re, &max_diff.re),
///             im: self.im.debug_abs_epsilon(&other.im, &max_diff.im),
///         }
///     }
///
///     fn debug_rmax_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &MyComplex32
///     ) -> MyComplex32 {
///         MyComplex32 {
///             re: self.re.debug_rmax_epsilon(&other.re, &max_diff.re),
///             im: self.im.debug_rmax_epsilon(&other.im, &max_diff.im),
///         }
///     }
///
///     fn debug_rmin_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &MyComplex32
///     ) -> MyComplex32 {
///         MyComplex32 {
///             re: self.re.debug_rmin_epsilon(&other.re, &max_diff.re),
///             im: self.im.debug_rmin_epsilon(&other.im, &max_diff.im),
///         }
///     }
///
///     fn debug_r1st_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &MyComplex32
///     ) -> MyComplex32 {
///         MyComplex32 {
///             re: self.re.debug_r1st_epsilon(&other.re, &max_diff.re),
///             im: self.im.debug_r1st_epsilon(&other.im, &max_diff.im),
///         }
///     }
///
///     fn debug_r2nd_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &MyComplex32
///     ) -> MyComplex32 {
///         MyComplex32 {
///             re: self.re.debug_r2nd_epsilon(&other.re, &max_diff.re),
///             im: self.im.debug_r2nd_epsilon(&other.im, &max_diff.im),
///         }
///     }
///
///     fn debug_ulps_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &MyComplex32Ulps,
///     ) -> MyComplex32Ulps {
///         MyComplex32Ulps {
///             re: self.re.debug_ulps_epsilon(&other.re, &max_diff.re),
///             im: self.im.debug_ulps_epsilon(&other.im, &max_diff.im),
///         }
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 200.0 };
/// let b = MyComplex32 { re: 50.0, im: 1.0 };
///
/// assert_eq!(
///     a.debug_abs_epsilon(&b, &MyComplex32 { re: 0.1, im: 0.2 }),
///     MyComplex32 { re: 0.1, im: 0.2 }
/// );
/// assert_eq!(
///     a.debug_rmax_epsilon(&b, &MyComplex32 { re: 0.1, im: 0.2 }),
///     MyComplex32 { re: 5.0, im: 40.0 }
/// );
/// assert_eq!(
///     a.debug_ulps_epsilon(&b, &MyComplex32Ulps { re: 4, im: 8 }),
///     MyComplex32Ulps { re: 4, im: 8 }
/// );
/// ```
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatEqUlpsEpsilon`]: trait.FloatEqUlpsEpsilon.html
/// [`FloatEqDebugUlpsDiff`]: trait.FloatEqDebugUlpsDiff.html
pub trait AssertFloatEq<Rhs: ?Sized = Self>: FloatEq<Rhs> {
    /// The absolute difference between two values, displayed to the user via
    /// `fmt::Debug` when an assert fails.
    ///
    /// This is usually the wider of `Self` and `Rhs`.
    type DebugAbsDiff: fmt::Debug + Sized + FloatEqDebugUlpsDiff;

    /// The per-field epsilon value used for comparison between two values,
    /// displayed to the user via `fmt::Debug` when an assert fails.
    ///
    /// This should match [`Self::Epsilon`].
    ///
    /// [`Self::Epsilon`]: trait.FloatEq.html#associatedtype.Epsilon
    type DebugEpsilon: fmt::Debug + FloatEqUlpsEpsilon;

    /// Always positive absolute difference between two values.
    ///
    /// Implementations should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatDiff { fn abs_diff(&self, other: &Self) -> Self; }
    /// # impl TestFloatDiff for f32 {
    /// # fn abs_diff(&self, other: &Self) -> Self {
    /// (self - other).abs()
    /// # }}
    /// ```
    fn debug_abs_diff(&self, other: &Rhs) -> Self::DebugAbsDiff;

    /// Always positive absolute difference between two values in terms of [ULPs].
    ///
    /// For primitive values, this should be a partial function that returns:
    /// - `Some(0)` if either argument is `0.0` or `-0.0`
    /// - `None` if either argument is `NaN`
    /// - `None` if the arguments have differing signs
    /// - `Some(bitwise-difference)` otherwise
    ///
    /// For composite types, this should return per-field recursively calculated
    /// results in order to present the most possible context to the user.
    ///
    /// Implementations over primitive types should be the equivalent of (using
    /// `f32` as an example):
    ///
    /// ```
    /// # trait TestFloatDiff { fn ulps_diff(&self, other: &Self) -> Option<u32>; }
    /// # impl TestFloatDiff for f32 {
    /// # fn ulps_diff(&self, other: &Self) -> Option<u32> {
    /// if self == other {
    ///     Some(0)
    /// } else if self.is_nan() || other.is_nan() {
    ///     None
    /// } else if self.is_sign_positive() != other.is_sign_positive() {
    ///     None
    /// } else {
    ///     let a = self.to_bits();
    ///     let b = other.to_bits();
    ///     let max = a.max(b);
    ///     let min = a.min(b);
    ///     Some(max - min)
    /// }
    /// # }}
    /// ```
    ///
    /// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
    fn debug_ulps_diff(&self, other: &Rhs) -> DebugUlpsDiff<Self::DebugAbsDiff>;

    /// The epsilon used by an `abs` [absolute epsilon comparison], displayed when
    /// an assert fails.
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn debug_abs_epsilon(&self, other: &Rhs, max_diff: &Self::Epsilon) -> Self::DebugEpsilon;

    /// The epsilon used by a `rel` [relative epsilon comparison], displayed when
    /// an assert fails.
    ///
    /// Equivalent to `self.debug_rmax_epsilon(self, other, max_diff)`, there is
    /// no need to reimplement this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn debug_rel_epsilon(&self, other: &Rhs, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        self.debug_rmax_epsilon(other, max_diff)
    }

    /// The epsilon used by an `rmax` [relative epsilon comparison], displayed when
    /// an assert fails.
    ///
    /// This should take into account the fact that the epsilon values are scaled
    /// based on the size of their inputs.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_rmax_epsilon(&self, other: &Rhs, max_diff: &Self::Epsilon) -> Self::DebugEpsilon;

    /// The epsilon used by an `rmin` [relative epsilon comparison], displayed when
    /// an assert fails.
    ///
    /// This should take into account the fact that the epsilon values are scaled
    /// based on the size of their inputs.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_rmin_epsilon(&self, other: &Rhs, max_diff: &Self::Epsilon) -> Self::DebugEpsilon;

    /// The epsilon used by an `r1st` [relative epsilon comparison], displayed when
    /// an assert fails.
    ///
    /// This should take into account the fact that the epsilon values are scaled
    /// based on the size of their inputs.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_r1st_epsilon(&self, other: &Rhs, max_diff: &Self::Epsilon) -> Self::DebugEpsilon;

    /// The epsilon used by an `r2nd` [relative epsilon comparison], displayed when
    /// an assert fails.
    ///
    /// This should take into account the fact that the epsilon values are scaled
    /// based on the size of their inputs.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_r2nd_epsilon(&self, other: &Rhs, max_diff: &Self::Epsilon) -> Self::DebugEpsilon;

    /// The epsilon used by an `ulps` [ULPs comparison], displayed when an assert
    /// fails.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn debug_ulps_epsilon(
        &self,
        other: &Rhs,
        max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon>
    where
        UlpsEpsilon<Self::DebugEpsilon>: Sized;
}

/// Debug context for when an assert using an `all` check fails.
///
/// This is used internally by the [`assert_float_eq!`] family of macros.
///
/// ## Derivable
#[cfg_attr(
    not(feature = "derive"),
    doc = r##"
This trait is derivable if the `"derive"` feature is enabled.

For example, add this to your Cargo.toml:

```text
[dependencies.float_eq]
version = "0.4"
features = ["derive"]
```
"##
)]
#[cfg_attr(
    feature = "derive",
    doc = r##"
This trait can be used with `#[derive]`. The easiest way to do so is to use the
[`#[derive_float_eq]`](attr.derive_float_eq.html) helper macro, see the top
level docs for [example usage](index.html#derivable).

If you wish to derive this trait by itself, you must first implement [`FloatEqUlpsEpsilon`],
[`FloatEq`], [`FloatEqAll`], [`FloatEqDebugUlpsDiff`] and [`AssertFloatEq`], all
of which may also be derived. You will also need a `#[float_eq]` attribute and
provide `ulps_epsilon`, `ulps_debug_diff`, and `all_epsilon`, which should match
the names of the `UlpsEpsilon`, `DebugUlpsDiff` and `AllEpsilon` types. Each
field's epsilon is calculated via a recursive call to the algorithm being used.
This trait may not be derived for enums or generic structs at present.

```
# use float_eq::{
#    FloatEqUlpsEpsilon, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff, AssertFloatEq, 
#    AssertFloatEqAll
# };
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(
    FloatEqUlpsEpsilon, FloatEq, FloatEqAll,
    FloatEqDebugUlpsDiff, AssertFloatEq, AssertFloatEqAll
)]
#[float_eq(
    ulps_epsilon = "PointUlps",
    debug_ulps_diff = "PointUlpsDebugUlpsDiff",
    all_epsilon = "f32",
)]
struct Point {
    x: f32,
    y: f32,
}

let a = Point { x: 1.0, y: 200.0 };
let b = Point { x: 50.0, y: 1.0 };
assert_eq!(
    a.debug_rmax_all_epsilon(&b, &0.2),
    Point { x: 10.0, y: 40.0 }
);
```
"##
)]
///
/// ## How can I implement `AssertFloatEqAll`?
///
/// You must first implement [`FloatEqUlpsEpsilon`], [`FloatEq`], [`FloatEqAll`],
/// [`FloatEqDebugUlpsDiff`] and [`AssertFloatEq`]. Implementation is then usually
/// a matter of simply calling through to an underlying `AssertFloatEqAll`method
/// for each field in turn. If not, you will need to take a close look at the
/// descriptions of the algorithms on a method by method basis:
///
/// ```
/// # use float_eq::{FloatEqUlpsEpsilon, FloatEqAll, AssertFloatEqAll, UlpsEpsilon};
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32Ulps {
///     re: UlpsEpsilon<f32>,
///     im: UlpsEpsilon<f32>,
/// }
///
/// impl FloatEqUlpsEpsilon for MyComplex32 {
///     type UlpsEpsilon = MyComplex32Ulps;   
/// }
///
/// impl FloatEqAll for MyComplex32 {
///     type AllEpsilon = f32;
///
///     fn eq_abs_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_abs_all(&other.re, max_diff) && self.im.eq_abs_all(&other.im, max_diff)
///     }
///
///     fn eq_rmax_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_rmax_all(&other.re, max_diff) && self.im.eq_rmax_all(&other.im, max_diff)
///     }
///
///     fn eq_rmin_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_rmin_all(&other.re, max_diff) && self.im.eq_rmin_all(&other.im, max_diff)
///     }
///
///     fn eq_r1st_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_r1st_all(&other.re, max_diff) && self.im.eq_r1st_all(&other.im, max_diff)
///     }
///
///     fn eq_r2nd_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_r2nd_all(&other.re, max_diff) && self.im.eq_r2nd_all(&other.im, max_diff)
///     }
///
///     fn eq_ulps_all(&self, other: &Self, max_diff: &UlpsEpsilon<f32>) -> bool {
///         self.re.eq_ulps_all(&other.re, max_diff) && self.im.eq_ulps_all(&other.im, max_diff)
///     }
/// }
///
/// impl AssertFloatEqAll for MyComplex32 {
///     type AllDebugEpsilon = Self;
///
///     fn debug_abs_all_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::AllEpsilon
///     ) -> Self::AllDebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_abs_all_epsilon(&other.re, max_diff),
///             im: self.im.debug_abs_all_epsilon(&other.im, max_diff),
///         }
///     }
///
///     fn debug_rmax_all_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::AllEpsilon
///     ) -> Self::AllDebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_rmax_all_epsilon(&other.re, max_diff),
///             im: self.im.debug_rmax_all_epsilon(&other.im, max_diff),
///         }
///     }
///
///     fn debug_rmin_all_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::AllEpsilon
///     ) -> Self::AllDebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_rmin_all_epsilon(&other.re, max_diff),
///             im: self.im.debug_rmin_all_epsilon(&other.im, max_diff),
///         }
///     }
///
///     fn debug_r1st_all_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::AllEpsilon
///     ) -> Self::AllDebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_r1st_all_epsilon(&other.re, max_diff),
///             im: self.im.debug_r1st_all_epsilon(&other.im, max_diff),
///         }
///     }
///
///     fn debug_r2nd_all_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::AllEpsilon
///     ) -> Self::AllDebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_r2nd_all_epsilon(&other.re, max_diff),
///             im: self.im.debug_r2nd_all_epsilon(&other.im, max_diff),
///         }
///     }
///
///     fn debug_ulps_all_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &UlpsEpsilon<Self::AllEpsilon>,
///     ) -> UlpsEpsilon<Self::AllDebugEpsilon> {
///         MyComplex32Ulps {
///             re: self.re.debug_ulps_all_epsilon(&other.re, max_diff),
///             im: self.im.debug_ulps_all_epsilon(&other.im, max_diff),
///         }
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 200.0 };
/// let b = MyComplex32 { re: 50.0, im: 1.0 };
///
/// assert_eq!(
///     a.debug_abs_all_epsilon(&b, &0.2),
///     MyComplex32 { re: 0.2, im: 0.2 }
/// );
/// assert_eq!(
///     a.debug_rmax_all_epsilon(&b, &0.2),
///     MyComplex32 { re: 10.0, im: 40.0 }
/// );
/// assert_eq!(
///     a.debug_ulps_all_epsilon(&b, &8),
///     MyComplex32Ulps { re: 8, im: 8 }
/// );
/// ```
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`FloatEqUlpsEpsilon`]: trait.FloatEqUlpsEpsilon.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatEqAll`]: trait.FloatEqAll.html
/// [`FloatEqDebugUlpsDiff`]: trait.FloatEqDebugUlpsDiff.html
/// [`AssertFloatEq`]: trait.AssertFloatEq.html
/// [`FloatEqAll::AllEpsilon`]: trait.FloatEqAll.html#associatedtype.AllEpsilon
pub trait AssertFloatEqAll<Rhs: ?Sized = Self>: FloatEqAll<Rhs> {
    /// Displayed to the user when an assert fails, using `fmt::Debug`.
    ///
    /// This should match the fields of the the most complex type in the comparison.
    type AllDebugEpsilon: fmt::Debug + FloatEqUlpsEpsilon;

    /// The epsilon used by an `abs_all` [absolute epsilon comparison], displayed
    /// when an assert fails.
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn debug_abs_all_epsilon(
        &self,
        other: &Rhs,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon;

    /// The epsilon used by a `rel_all` [relative epsilon comparison], displayed
    /// when an assert fails.
    ///
    /// Equivalent to `self.debug_rmax_all_epsilon(self, other, max_diff)`, there
    /// is no need to reimplement this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &Rhs,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        self.debug_rmax_all_epsilon(other, max_diff)
    }

    /// The epsilon used by an `rmax_all` [relative epsilon comparison], displayed
    /// when an assert fails.
    ///
    /// This should take into account the fact that the epsilon values are scaled
    /// based on the size of their inputs.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_rmax_all_epsilon(
        &self,
        other: &Rhs,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon;

    /// The epsilon used by an `rmin_all` [relative epsilon comparison], displayed
    /// when an assert fails.
    ///
    /// This should take into account the fact that the epsilon values are scaled
    /// based on the size of their inputs.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_rmin_all_epsilon(
        &self,
        other: &Rhs,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon;

    /// The epsilon used by an `r1st_all` [relative epsilon comparison], displayed
    /// when an assert fails.
    ///
    /// This should take into account the fact that the epsilon values are scaled
    /// based on the size of their inputs.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_r1st_all_epsilon(
        &self,
        other: &Rhs,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon;

    /// The epsilon used by an `r2nd_all` [relative epsilon comparison], displayed
    /// when an assert fails.
    ///
    /// This should take into account the fact that the epsilon values are scaled
    /// based on the size of their inputs.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_r2nd_all_epsilon(
        &self,
        other: &Rhs,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon;

    /// The epsilon used by an `ulps_all` [ULPs comparison], displayed when an assert
    /// fails.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn debug_ulps_all_epsilon(
        &self,
        other: &Rhs,
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon>
    where
        UlpsEpsilon<Self::AllDebugEpsilon>: Sized;
}
