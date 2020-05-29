use core::fmt;

/// Algorithms to compute the difference between IEEE floating point values.
///
/// This trait is used by the family of [`assert_float_eq!`] macros to provide
/// debug context information when they fail, but may also be called directly.
/// Types are displayed to the user with `fmt::Debug`.
///
/// *Note: the definition of this trait is very much tailored to what the crate
/// requires in terms of debugging information, and may not be ideal for general
/// use.*
///
/// ## How can I implement `FloatDiff`?
///
/// You will need some way to represent difference in [ULPs] for your type, following
/// the same structure as the type itself. Implementation is then usually a matter
/// of calling through to an underlying `FloatDiff` method for each field in turn.
/// If not, you will need to take a close look at the descriptions of the algorithms
/// on a method by method basis:
///
/// ```rust
/// # use float_eq::FloatDiff;
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32Ulps {
///     re: <f32 as FloatDiff>::UlpsDiff,
///     im: <f32 as FloatDiff>::UlpsDiff,
/// }
///
/// impl FloatDiff for MyComplex32 {
///     type AbsDiff = Self;
///     type UlpsDiff = MyComplex32Ulps;
///
///     fn abs_diff(&self, other: &Self) -> Self::AbsDiff {
///         MyComplex32 {
///             re: self.re.abs_diff(&other.re),
///             im: self.im.abs_diff(&other.im),
///         }
///     }
///
///     fn ulps_diff(&self, other: &Self) -> Option<Self::UlpsDiff> {
///         Some(MyComplex32Ulps {
///             re: self.re.ulps_diff(&other.re)?,
///             im: self.im.ulps_diff(&other.im)?,
///         })
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 2.000_003_6, };
/// let b = MyComplex32 { re: 1.000_000_1, im: 2.0, };
///
/// assert_eq!(
///     a.abs_diff(&b),
///     MyComplex32 {
///         re: 0.000_000_119_209_29,
///         im: 0.000_003_576_278_7,
///     }
/// );
///
/// assert_eq!(a.ulps_diff(&b), Some(MyComplex32Ulps { re: 1, im: 15 }));
/// ```
///
/// ## How can I compare two different types?
///
/// The type you can `diff` with is controlled by `FloatDiff`'s parameter. Following
/// on from our previous example, if we wanted to treat `f32` as a complex number
/// with an imaginary component of `0.0`:
///
/// ```rust
/// # use float_eq::FloatDiff;
/// # #[derive(Debug, Clone, Copy, PartialEq)]
/// # struct MyComplex32 { re: f32, im: f32, }
/// # #[derive(Debug, Clone, Copy, PartialEq)]
/// # struct MyComplex32Ulps { re: <f32 as FloatDiff>::UlpsDiff,  im: <f32 as FloatDiff>::UlpsDiff, }
/// impl FloatDiff<f32> for MyComplex32 {
///     type AbsDiff = MyComplex32;
///     type UlpsDiff = MyComplex32Ulps;
///
///     fn abs_diff(&self, other: &f32) -> Self::AbsDiff {
///         MyComplex32 {
///             re: self.re.abs_diff(other),
///             im: self.im.abs_diff(&0.0),
///         }
///     }
///
///     fn ulps_diff(&self, other: &f32) -> Option<Self::UlpsDiff> {
///         Some(MyComplex32Ulps {
///             re: self.re.ulps_diff(other)?,
///             im: self.im.ulps_diff(&0.0)?,
///         })
///     }
/// }
///
/// impl FloatDiff<MyComplex32> for f32 {
///     type AbsDiff = <MyComplex32 as FloatDiff<f32>>::AbsDiff;
///     type UlpsDiff = <MyComplex32 as FloatDiff<f32>>::UlpsDiff;
///
///     fn abs_diff(&self, other: &MyComplex32) -> Self::AbsDiff {
///         other.abs_diff(self)
///     }
///
///     fn ulps_diff(&self, other: &MyComplex32) -> Option<Self::UlpsDiff> {
///         other.ulps_diff(self)
///     }
/// }
///
/// let a = 1.000_000_1_f32;
/// let b = MyComplex32 { re: 1.0, im: 2.000_003_6, };
///
/// assert_eq!(
///     a.abs_diff(&b),
///     MyComplex32 {
///         re: 0.000_000_119_209_29,
///         im: 2.000_003_6,
///     }
/// );
///
/// assert_eq!(a.ulps_diff(&b), Some(MyComplex32Ulps { re: 1, im: 1_073_741_839 }));
/// ```
///
/// ## Examples
///
/// ```rust
/// # use float_eq::FloatDiff;
/// assert_eq!(1.0f32.abs_diff(&-1.0), 2.0);
/// assert_eq!(1.0f64.abs_diff(&-1.0), 2.0);
///
/// assert_eq!(1.0f32.ulps_diff(&1.000_000_1), Some(1));
/// assert_eq!(1.0f64.ulps_diff(&1.000_000_000_000_000_2), Some(1));
///
/// assert_eq!(1.0f32.ulps_diff(&-1.0), None);
/// assert_eq!(1.0f64.ulps_diff(&-1.0), None);
///
/// let a = [0.0_f32, 2.0, -2.0];
/// let b = [0.0_f32, -1.0, 2.0];
/// assert_eq!(a.abs_diff(&b), [0.0, 3.0, 4.0]);
/// assert_eq!(a.ulps_diff(&b), None);
///
/// let c = [1.000_000_1f32, -2.0];
/// let d = [1.0f32, -2.000_000_5];
/// assert_eq!(c.ulps_diff(&d), Some([1, 2]));
/// ```
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
/// [`assert_float_eq!`]: macro.assert_float_eq.html
pub trait FloatDiff<Rhs: ?Sized = Self> {
    /// Type of the absolute difference between two values.
    ///
    /// This is often `Self`, unless comparing two different types. Composite types
    /// should probably use a type that follows the same structure as the inputs,
    /// to make error messages more legible.
    type AbsDiff;

    /// Type of the absolute difference between two values in terms of [ULPs].
    ///
    /// This should be an unsigned integer of the same size as the underlying
    /// floating point type, for example `f32` uses `u32`. Composite types should
    /// probably use a type that follows the same structure as the inputs, to make
    /// error messages more legible.
    ///
    /// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
    type UlpsDiff;

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
    fn abs_diff(&self, other: &Rhs) -> Self::AbsDiff;

    /// Always positive absolute difference between two values in terms of [ULPs].
    ///
    /// A partial function that returns:
    /// - `Some(0)` if either argument is `0.0` or `-0.0`
    /// - `None` if either argument is `NaN`
    /// - `None` if the arguments have differing signs
    /// - `Some(bitwise-difference)` otherwise
    ///
    /// Implementations on composite types should return `None` if any of their
    /// parts is an `ulps_diff` of `None`.
    ///
    /// Implementations should be the equivalent of (using `f32` as an example):
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
    fn ulps_diff(&self, other: &Rhs) -> Option<Self::UlpsDiff>;
}

/// Algorithms to compare IEEE floating point values for equality using per-field
/// thresholds.
///
/// This trait is used in the implementation of the [`float_eq!`] and [`assert_float_eq!`]
/// families of macros to provide `abs`, `rel` and `ulps` checks. It may be called
/// directly, but the macros usually provide a friendlier interface.
///
/// ## How can I implement `FloatEq`?
///
/// You will need some way to represent difference in [ULPs] for your type, following
/// the same structure as the type itself. Implementation is then usually a matter
/// of calling through to an underlying `FloatEq` method for each field in turn.
/// If not, you will need to take a close look at the descriptions of the algorithms
/// on a method by method basis:
///
/// ```
/// # use float_eq::{FloatEq, FloatDiff};
/// #[derive(Copy, Clone)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// #[derive(Copy, Clone)]
/// struct MyComplex32Ulps {
///     re: <f32 as FloatDiff>::UlpsDiff,
///     im: <f32 as FloatDiff>::UlpsDiff,
/// }
///
/// impl FloatEq for MyComplex32 {
///     type Epsilon = MyComplex32;
///     type UlpsEpsilon = MyComplex32Ulps;
///
///     fn eq_abs(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_abs(&other.re, &max_diff.re) && self.im.eq_abs(&other.im, &max_diff.im)
///     }
///
///     fn eq_rel(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_rel(&other.re, &max_diff.re) && self.im.eq_rel(&other.im, &max_diff.im)
///     }
///
///     fn eq_ulps(&self, other: &Self, max_diff: &MyComplex32Ulps) -> bool {
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
/// ## How can I compare two different types?
///
/// The type to be compared with is controlled by `FloatEq`'s parameter. Following
/// on from our previous example, if we wanted to treat `f32` as a complex number
/// with an imaginary component of `0.0`:
///
/// ```
/// # use float_eq::{FloatEq, FloatDiff};
/// # #[derive(Copy, Clone)]
/// # struct MyComplex32 { re: f32, im: f32 }
/// # #[derive(Copy, Clone)]
/// # struct MyComplex32Ulps { re: <f32 as FloatDiff>::UlpsDiff, im: <f32 as FloatDiff>::UlpsDiff }
/// impl FloatEq<f32> for MyComplex32 {
///     type Epsilon = MyComplex32;
///     type UlpsEpsilon = MyComplex32Ulps;
///
///     fn eq_abs(&self, other: &f32, max_diff: &MyComplex32) -> bool {
///         self.re.eq_abs(other, &max_diff.re) && self.im.eq_abs(&0.0, &max_diff.im)
///     }
///
///     fn eq_rel(&self, other: &f32, max_diff: &MyComplex32) -> bool {
///         self.re.eq_rel(other, &max_diff.re) && self.im.eq_rel(&0.0, &max_diff.im)
///     }
///
///     fn eq_ulps(&self, other: &f32, max_diff: &MyComplex32Ulps) -> bool {
///         self.re.eq_ulps(other, &max_diff.re) && self.im.eq_ulps(&0.0, &max_diff.im)
///     }
/// }
///
/// let a = MyComplex32 { re: 4.000_000_5, im: 0.0 };
/// let b = 4.0_f32;
///
/// assert!(a.eq_abs(&b, &MyComplex32 { re: 0.000_000_8, im: 0.0 }));
/// assert!(a.ne_abs(&b, &MyComplex32 { re: 0.000_000_4, im: 0.0 }));
///
/// assert!(a.eq_rel(&b, &MyComplex32 { re: 0.000_000_12, im: 0.0 }));
/// assert!(a.ne_rel(&b, &MyComplex32 { re: 0.000_000_11, im: 0.0 }));
///
/// assert!(a.eq_ulps(&b, &MyComplex32Ulps { re: 1, im: 0 }));
/// assert!(a.ne_ulps(&b, &MyComplex32Ulps { re: 0, im: 0 }));
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
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`float_eq!`]: macro.float_eq.html
pub trait FloatEq<Rhs: ?Sized = Self> {
    /// Type of the maximum allowed difference between two values for them to be
    /// considered equal in terms of their native type.
    ///
    /// This is the type of the `max_diff` parameter passed to `abs` and `rel`
    /// checks in methods and via the [`float_eq!`] macros. This should be structurally
    /// similar to the most complex type being compared, most implementations will
    /// just use `Self`.
    ///
    /// [`float_eq!`]: macro.float_eq.html
    type Epsilon;

    /// Type of the maximum allowed difference between two values for them to be
    /// considered equal in terms of an [ULPs comparison].
    ///
    /// This is the type of the `max_diff` parameter passed to `ulps` checks in
    /// methods and via the [`float_eq!`] macros. This should be structurally
    /// similar to the most complex type being compared, most implementations will
    /// define an `Ulps` type that mirrors the fields of `Self`.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    /// [`float_eq!`]: macro.float_eq.html
    type UlpsEpsilon;

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
    fn eq_rel(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool;

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
    fn eq_ulps(&self, other: &Rhs, max_diff: &Self::UlpsEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using an [ULPs comparison].
    ///
    /// Equal to `!self.eq_ulps(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    #[inline]
    fn ne_ulps(&self, other: &Rhs, max_diff: &Self::UlpsEpsilon) -> bool {
        !self.eq_ulps(other, max_diff)
    }
}

/// Algorithms to compare IEEE floating point values for equality using uniform
/// thresholds.
///
/// This trait is used in the implementation of the [`float_eq!`] and [`assert_float_eq!`]
/// families of macros to provide `abs_all`, `rel_all` and `ulps_all` checks. It
/// may be called directly, but the macros usually provide a friendlier interface.
/// Comparison via this trait may not fit every composite type. For example, it
/// likely ought not to be implemented for `(f32, f64)`, which has a big difference
/// in precision between its fields.
///
/// ## How can I implement `FloatEqAll`?
///
/// You will need some way to represent difference in [ULPs] for your type, which
/// is likely to be either `u32` or `u64`. Implementation is then usually a matter
/// of calling through to an underlying `FloatEqAll` method for each field in turn.
/// If not, you will need to take a close look at the descriptions of the algorithms
/// on a method by method basis:
///
/// ```
/// # use float_eq::{FloatEqAll, FloatDiff};
/// #[derive(Copy, Clone)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// impl FloatEqAll for MyComplex32 {
///     type Epsilon = f32;
///     type UlpsEpsilon = u32;
///
///     fn eq_abs_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_abs_all(&other.re, max_diff) && self.im.eq_abs_all(&other.im, max_diff)
///     }
///
///     fn eq_rel_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_rel_all(&other.re, max_diff) && self.im.eq_rel_all(&other.im, max_diff)
///     }
///
///     fn eq_ulps_all(&self, other: &Self, max_diff: &u32) -> bool {
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
/// assert!(a.eq_rel_all(&b, &0.000_001_8));
/// assert!(a.ne_rel_all(&b, &0.000_001_7));
///
/// assert!(a.eq_ulps_all(&b, &15));
/// assert!(a.ne_ulps_all(&b, &14));
/// ```
///
/// ## How can I compare two different types?
///
/// The type to be compared with is controlled by `FloatEqAll`'s parameter. Following
/// on from our previous example, if we wanted to treat `f32` as a complex number
/// with an imaginary component of `0.0`:
///
/// ```
/// # use float_eq::{FloatEqAll, FloatDiff};
/// # #[derive(Copy, Clone)]
/// # struct MyComplex32 { re: f32, im: f32 }
/// # #[derive(Copy, Clone)]
/// # struct MyComplex32Ulps { re: <f32 as FloatDiff>::UlpsDiff, im: <f32 as FloatDiff>::UlpsDiff }
/// impl FloatEqAll<f32> for MyComplex32 {
///     type Epsilon = f32;
///     type UlpsEpsilon = u32;
///
///     fn eq_abs_all(&self, other: &f32, max_diff: &f32) -> bool {
///         self.re.eq_abs_all(other, max_diff) && self.im.eq_abs_all(&0.0, max_diff)
///     }
///
///     fn eq_rel_all(&self, other: &f32, max_diff: &f32) -> bool {
///         self.re.eq_rel_all(other, max_diff) && self.im.eq_rel_all(&0.0, max_diff)
///     }
///
///     fn eq_ulps_all(&self, other: &f32, max_diff: &u32) -> bool {
///         self.re.eq_ulps_all(other, max_diff) && self.im.eq_ulps_all(&0.0, max_diff)
///     }
/// }
///
/// let a = MyComplex32 { re: 4.000_000_5, im: 0.0 };
/// let b = 4.0_f32;
///
/// assert!(a.eq_abs_all(&b, &0.000_000_8));
/// assert!(a.ne_abs_all(&b, &0.000_000_4));
///
/// assert!(a.eq_rel_all(&b, &0.000_000_12));
/// assert!(a.ne_rel_all(&b, &0.000_000_11));
///
/// assert!(a.eq_ulps_all(&b, &1));
/// assert!(a.ne_ulps_all(&b, &0));
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
/// assert!(a.eq_rel_all(&b, &0.000_001));
/// assert!(a.ne_rel_all(&b, &0.000_000_5));
///
/// assert!(a.eq_ulps_all(&b, &8));
/// assert!(a.ne_ulps_all(&b, &7));
/// ```
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`float_eq!`]: macro.float_eq.html
pub trait FloatEqAll<Rhs: ?Sized = Self> {
    /// Type of the maximum allowed difference between two values for them to be
    /// considered equal in terms of their native type.
    ///
    /// This is the type of the `max_diff` parameter passed to `abs_all` and `rel_all`
    /// checks in methods and via the [`float_eq!`] macros. This should be a single
    /// floating point value so is often `f32` or `f64`.
    ///
    /// [`float_eq!`]: macro.float_eq.html
    type Epsilon;

    /// Type of the maximum allowed difference between two values for them to be
    /// considered equal in terms of an [ULPs comparison].
    ///
    /// This is the type of the `max_diff` parameter passed to `ulps_all` checks in
    /// methods and via the [`float_eq!`] macros. This should be a single value
    /// so is often `u32` or `u64`.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    /// [`float_eq!`]: macro.float_eq.html
    type UlpsEpsilon;

    /// Check whether `self` is equal to `other`, using an [absolute epsilon
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_abs`].
    ///
    /// [`FloatEq::eq_abs`]: trait.FloatEq.html#tymethod.eq_abs
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn eq_abs_all(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using an [absolute epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_abs_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    #[inline]
    fn ne_abs_all(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool {
        !self.eq_abs_all(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_rel`].
    ///
    /// [`FloatEq::eq_rel`]: trait.FloatEq.html#tymethod.eq_rel
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_rel_all(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_rel_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    #[inline]
    fn ne_rel_all(&self, other: &Rhs, max_diff: &Self::Epsilon) -> bool {
        !self.eq_rel_all(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using an [ULPs comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_ulps`].
    ///
    /// [`FloatEq::eq_ulps`]: trait.FloatEq.html#tymethod.eq_ulps
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn eq_ulps_all(&self, other: &Rhs, max_diff: &Self::UlpsEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using an [ULPs comparison].
    ///
    /// Equal to `!self.eq_ulps_all(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    #[inline]
    fn ne_ulps_all(&self, other: &Rhs, max_diff: &Self::UlpsEpsilon) -> bool {
        !self.eq_ulps_all(other, max_diff)
    }
}

/// Provides additional context for debugging when an assert using [`FloatEq`] fails.
///
/// This is used internally by the [`assert_float_eq!`] family of macros to provide
/// debug context information to the user when `abs`, `rel` or `ulps` checks fail.
///
/// ## How can I implement `FloatEqDebug`?
///
/// You should first implement [`FloatEq`] and [`FloatDiff`]. Most types will have
/// implemented a custom [ULPs] type that mirrors their field structure, which can
/// then be used for `FloatEqDebug`. Implementation is then usually a matter of
/// simply calling through to an underlying `FloatEqDebug`method for each field
/// in turn. If not, you will need to take a close look at the descriptions of the
/// algorithms on a method by method basis:
///
/// ```
/// # use float_eq::{FloatDiff, FloatEq, FloatEqDebug};
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32Ulps {
///     re: <f32 as FloatDiff>::UlpsDiff,
///     im: <f32 as FloatDiff>::UlpsDiff,
/// }
///
/// impl FloatEq for MyComplex32 {
///     type Epsilon = MyComplex32;
///     type UlpsEpsilon = MyComplex32Ulps;
///
///     fn eq_abs(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_abs(&other.re, &max_diff.re) && self.im.eq_abs(&other.im, &max_diff.im)
///     }
///
///     fn eq_rel(&self, other: &Self, max_diff: &MyComplex32) -> bool {
///         self.re.eq_rel(&other.re, &max_diff.re) && self.im.eq_rel(&other.im, &max_diff.im)
///     }
///
///     fn eq_ulps(&self, other: &Self, max_diff: &MyComplex32Ulps) -> bool {
///         self.re.eq_ulps(&other.re, &max_diff.re) && self.im.eq_ulps(&other.im, &max_diff.im)
///     }
/// }
///
/// impl FloatEqDebug for MyComplex32 {
///     type DebugEpsilon = MyComplex32;
///     type DebugUlpsEpsilon = MyComplex32Ulps;
///
///     fn debug_abs_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::Epsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_abs_epsilon(&other.re, &max_diff.re),
///             im: self.im.debug_abs_epsilon(&other.im, &max_diff.im),
///         }
///     }
///
///     fn debug_rel_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::Epsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_rel_epsilon(&other.re, &max_diff.re),
///             im: self.im.debug_rel_epsilon(&other.im, &max_diff.im),
///         }
///     }
///
///     fn debug_ulps_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::UlpsEpsilon,
///     ) -> Self::DebugUlpsEpsilon {
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
///     a.debug_rel_epsilon(&b, &MyComplex32 { re: 0.1, im: 0.2 }),
///     MyComplex32 { re: 5.0, im: 40.0 }
/// );
/// assert_eq!(
///     a.debug_ulps_epsilon(&b, &MyComplex32Ulps { re: 4, im: 8 }),
///     MyComplex32Ulps { re: 4, im: 8 }
/// );
/// ```
///
/// ## How can I compare two different types?
///
/// The type to be compared with is controlled by `FloatEqDebug`'s parameter.
/// Following on from our previous example, if we wanted to treat `f32` as a
/// complex number with an imaginary component of `0.0`:
///
/// ```
/// # use float_eq::{FloatDiff, FloatEq, FloatEqDebug};
/// # #[derive(Debug, PartialEq)]
/// # struct MyComplex32 { re: f32, im: f32, }
/// # #[derive(Debug, PartialEq)]
/// #  struct MyComplex32Ulps { re: <f32 as FloatDiff>::UlpsDiff, im: <f32 as FloatDiff>::UlpsDiff }
/// impl FloatEq<f32> for MyComplex32 {
///     type Epsilon = MyComplex32;
///     type UlpsEpsilon = MyComplex32Ulps;
///
///     fn eq_abs(&self, other: &f32, max_diff: &MyComplex32) -> bool {
///         self.re.eq_abs(other, &max_diff.re) && self.im.eq_abs(&0.0, &max_diff.im)
///     }
///
///     fn eq_rel(&self, other: &f32, max_diff: &MyComplex32) -> bool {
///         self.re.eq_rel(other, &max_diff.re) && self.im.eq_rel(&0.0, &max_diff.im)
///     }
///
///     fn eq_ulps(&self, other: &f32, max_diff: &MyComplex32Ulps) -> bool {
///         self.re.eq_ulps(other, &max_diff.re) && self.im.eq_ulps(&0.0, &max_diff.im)
///     }
/// }
///
/// impl FloatEqDebug<f32> for MyComplex32 {
///     type DebugEpsilon = MyComplex32;
///     type DebugUlpsEpsilon = MyComplex32Ulps;
///
///     fn debug_abs_epsilon(
///         &self,
///         other: &f32,
///         max_diff: &Self::Epsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_abs_epsilon(other, &max_diff.re),
///             im: self.im.debug_abs_epsilon(&0.0, &max_diff.im),
///         }
///     }
///
///     fn debug_rel_epsilon(
///         &self,
///         other: &f32,
///         max_diff: &Self::Epsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_rel_epsilon(other, &max_diff.re),
///             im: self.im.debug_rel_epsilon(&0.0, &max_diff.im),
///         }
///     }
///
///     fn debug_ulps_epsilon(
///         &self,
///         other: &f32,
///         max_diff: &Self::UlpsEpsilon,
///     ) -> Self::DebugUlpsEpsilon {
///         MyComplex32Ulps {
///             re: self.re.debug_ulps_epsilon(other, &max_diff.re),
///             im: self.im.debug_ulps_epsilon(&0.0, &max_diff.im),
///         }
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 200.0 };
/// let b = 9000.0_f32;
///
/// assert_eq!(
///     a.debug_abs_epsilon(&b, &MyComplex32 { re: 0.1, im: 0.2 }),
///     MyComplex32 { re: 0.1, im: 0.2 }
/// );
/// assert_eq!(
///     a.debug_rel_epsilon(&b, &MyComplex32 { re: 0.1, im: 0.2 }),
///     MyComplex32 { re: 900.0, im: 40.0 }
/// );
/// assert_eq!(
///     a.debug_ulps_epsilon(&b, &MyComplex32Ulps { re: 4, im: 8 }),
///     MyComplex32Ulps { re: 4, im: 8 }
/// );
/// ```
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`FloatDiff`]: trait.FloatDiff.html
/// [`FloatEq`]: trait.FloatEq.html
pub trait FloatEqDebug<Rhs: ?Sized = Self>: FloatEq<Rhs> {
    /// Displayed to the user when an assert fails, using `fmt::Debug`.
    ///
    /// This should match [`Self::Epsilon`].
    ///
    /// [`Self::Epsilon`]: trait.FloatEq.html#associatedtype.Epsilon
    type DebugEpsilon: fmt::Debug;

    /// Displayed to the user when an assert fails, using `fmt::Debug`.
    ///
    /// This should match [`Self::UlpsEpsilon`].
    ///
    /// [`Self::UlpsEpsilon`]: trait.FloatEq.html#associatedtype.UlpsEpsilon
    type DebugUlpsEpsilon: fmt::Debug;

    /// The epsilon used by an `abs` [absolute epsilon comparison], displayed when
    /// an assert fails.
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn debug_abs_epsilon(&self, other: &Rhs, max_diff: &Self::Epsilon) -> Self::DebugEpsilon;

    /// The epsilon used by a `rel` [relative epsilon comparison], displayed when
    /// an assert fails.
    ///
    /// This should take into account the fact that the epsilon values are scaled
    /// based on the size of their inputs.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_rel_epsilon(&self, other: &Rhs, max_diff: &Self::Epsilon) -> Self::DebugEpsilon;

    /// The epsilon used by an `ulps` [ULPs comparison], displayed when an assert
    /// fails.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn debug_ulps_epsilon(
        &self,
        other: &Rhs,
        max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon;
}

/// Provides additional context for debugging when an assert using [`FloatEqAll`] fails.
///
/// This is used internally by the [`assert_float_eq!`] family of macros to provide
/// debug context information to the user when `abs_all`, `rel_all` or `ulps_all`
/// checks fail.
///
/// ## How can I implement `FloatEqAllDebug`?
///
/// You should first implement [`FloatEqAll`] and [`FloatDiff`]. The outputs from
/// `FloatEqAllDebug` will be the epsilon values used by the comparison widened to
/// the structural shape of the most complex type. Most types will have implemented
/// a custom [ULPs] type that mirrors their field structure. Implementation is then
/// usually a matter of simply calling through to an underlying `FloatEqDebug`method
/// for each field in turn. If not, you will need to take a close look at the
/// descriptions of the algorithms on a method by method basis:
///
/// ```
/// # use float_eq::{FloatDiff, FloatEqAll, FloatEqAllDebug};
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct MyComplex32Ulps {
///     re: <f32 as FloatDiff>::UlpsDiff,
///     im: <f32 as FloatDiff>::UlpsDiff,
/// }
///
/// impl FloatEqAll for MyComplex32 {
///     type Epsilon = f32;
///     type UlpsEpsilon = u32;
///
///     fn eq_abs_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_abs_all(&other.re, max_diff) && self.im.eq_abs_all(&other.im, max_diff)
///     }
///
///     fn eq_rel_all(&self, other: &Self, max_diff: &f32) -> bool {
///         self.re.eq_rel_all(&other.re, max_diff) && self.im.eq_rel_all(&other.im, max_diff)
///     }
///
///     fn eq_ulps_all(&self, other: &Self, max_diff: &u32) -> bool {
///         self.re.eq_ulps_all(&other.re, max_diff) && self.im.eq_ulps_all(&other.im, max_diff)
///     }
/// }
///
/// impl FloatEqAllDebug for MyComplex32 {
///     type DebugEpsilon = MyComplex32;
///     type DebugUlpsEpsilon = MyComplex32Ulps;
///
///     fn debug_abs_all_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::Epsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_abs_all_epsilon(&other.re, max_diff),
///             im: self.im.debug_abs_all_epsilon(&other.im, max_diff),
///         }
///     }
///
///     fn debug_rel_all_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::Epsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_rel_all_epsilon(&other.re, max_diff),
///             im: self.im.debug_rel_all_epsilon(&other.im, max_diff),
///         }
///     }
///
///     fn debug_ulps_all_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::UlpsEpsilon,
///     ) -> Self::DebugUlpsEpsilon {
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
///     a.debug_rel_all_epsilon(&b, &0.2),
///     MyComplex32 { re: 10.0, im: 40.0 }
/// );
/// assert_eq!(
///     a.debug_ulps_all_epsilon(&b, &8),
///     MyComplex32Ulps { re: 8, im: 8 }
/// );
/// ```
///
/// ## How can I compare two different types?
///
/// The type to be compared with is controlled by `FloatEqDebug`'s parameter.
/// Following on from our previous example, if we wanted to treat `f32` as a
/// complex number with an imaginary component of `0.0`:
///
/// ```
/// # use float_eq::{FloatDiff, FloatEqAll, FloatEqAllDebug};
/// # #[derive(Debug, PartialEq)]
/// # struct MyComplex32 { re: f32, im: f32, }
/// # #[derive(Debug, PartialEq)]
/// #  struct MyComplex32Ulps { re: <f32 as FloatDiff>::UlpsDiff, im: <f32 as FloatDiff>::UlpsDiff }
/// impl FloatEqAll<f32> for MyComplex32 {
///     type Epsilon = f32;
///     type UlpsEpsilon = u32;
///
///     fn eq_abs_all(&self, other: &f32, max_diff: &f32) -> bool {
///         self.re.eq_abs_all(other, max_diff) && self.im.eq_abs_all(&0.0, max_diff)
///     }
///
///     fn eq_rel_all(&self, other: &f32, max_diff: &f32) -> bool {
///         self.re.eq_rel_all(other, max_diff) && self.im.eq_rel_all(&0.0, max_diff)
///     }
///
///     fn eq_ulps_all(&self, other: &f32, max_diff: &u32) -> bool {
///         self.re.eq_ulps_all(other, max_diff) && self.im.eq_ulps_all(&0.0, max_diff)
///     }
/// }
///
/// impl FloatEqAllDebug<f32> for MyComplex32 {
///     type DebugEpsilon = MyComplex32;
///     type DebugUlpsEpsilon = MyComplex32Ulps;
///
///     fn debug_abs_all_epsilon(
///         &self,
///         other: &f32,
///         max_diff: &Self::Epsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_abs_all_epsilon(other, max_diff),
///             im: self.im.debug_abs_all_epsilon(&0.0, max_diff),
///         }
///     }
///
///     fn debug_rel_all_epsilon(
///         &self,
///         other: &f32,
///         max_diff: &Self::Epsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_rel_all_epsilon(other, max_diff),
///             im: self.im.debug_rel_all_epsilon(&0.0, max_diff),
///         }
///     }
///
///     fn debug_ulps_all_epsilon(
///         &self,
///         other: &f32,
///         max_diff: &Self::UlpsEpsilon,
///     ) -> Self::DebugUlpsEpsilon {
///         MyComplex32Ulps {
///             re: self.re.debug_ulps_all_epsilon(other, max_diff),
///             im: self.im.debug_ulps_all_epsilon(&0.0, max_diff),
///         }
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 200.0 };
/// let b = 9000.0_f32;
///
/// assert_eq!(
///     a.debug_abs_all_epsilon(&b, &0.2),
///     MyComplex32 { re: 0.2, im: 0.2 }
/// );
/// assert_eq!(
///     a.debug_rel_all_epsilon(&b, &0.2),
///     MyComplex32 { re: 1800.0, im: 40.0 }
/// );
/// assert_eq!(
///     a.debug_ulps_all_epsilon(&b, &8),
///     MyComplex32Ulps { re: 8, im: 8 }
/// );
/// ```
///
/// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`FloatDiff`]: trait.FloatDiff.html
/// [`FloatEqAll`]: trait.FloatEqAll.html
pub trait FloatEqAllDebug<Rhs: ?Sized = Self>: FloatEqAll<Rhs> {
    /// Displayed to the user when an assert fails, using `fmt::Debug`.
    ///
    /// This should match the fields of the the most complex type in the comparison.
    type DebugEpsilon: fmt::Debug;

    /// Displayed to the user when an assert fails, using `fmt::Debug`.
    ///
    /// This should match the fields of the the ULPs diff type for the most complex
    /// type in the comparison.
    type DebugUlpsEpsilon: fmt::Debug;

    /// The epsilon used by an `abs_all` [absolute epsilon comparison], displayed
    /// when an assert fails.
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn debug_abs_all_epsilon(&self, other: &Rhs, max_diff: &Self::Epsilon) -> Self::DebugEpsilon;

    /// The epsilon used by a `rel_all` [relative epsilon comparison], displayed
    /// when an assert fails.
    ///
    /// This should take into account the fact that the epsilon values are scaled
    /// based on the size of their inputs.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_rel_all_epsilon(&self, other: &Rhs, max_diff: &Self::Epsilon) -> Self::DebugEpsilon;

    /// The epsilon used by an `ulps_all` [ULPs comparison], displayed when an assert
    /// fails.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn debug_ulps_all_epsilon(
        &self,
        other: &Rhs,
        max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon;
}
