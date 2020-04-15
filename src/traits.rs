use core::fmt;

/// Algorithms to compute the difference between IEEE floating point values.
///
/// This trait is used by the family of [`assert_float_eq!`] macros to provide
/// debug context information when they fail, but may also be called directly.
/// Types are displayed to the user with `fmt::Debug`.
///
/// By default this trait is implemented on `f32` and `f64`, and for arrays of
/// compatible type which have size 0 to 32 (inclusive).
///
/// ## How can I implement `FloatDiff`?
///
/// Implementing `FloatDiff` on your types is straightfoward if they're already
/// composed of compatible fields. You will need some way to represent difference in
/// [ULPs] for your type, probably following the same structure as the type itself.
/// For example, `MyComplex32` here has `re` and `im` fields and `MyComplex32UlpsDiff`
/// follows that layout. Once you have this type, implementing the methods is a case
/// of calling through to the underlying implementation:
///
/// ```rust
/// # use float_eq::FloatDiff;
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// struct MyComplex32UlpsDiff {
///     re: <f32 as FloatDiff>::UlpsDiff,
///     im: <f32 as FloatDiff>::UlpsDiff,
/// }
///
/// impl FloatDiff for MyComplex32 {
///     type AbsDiff = Self;
///     type UlpsDiff = MyComplex32UlpsDiff;
///
///     fn abs_diff(&self, other: &Self) -> Self::AbsDiff {
///         MyComplex32 {
///             re: self.re.abs_diff(&other.re),
///             im: self.im.abs_diff(&other.im),
///         }
///     }
///
///     fn ulps_diff(&self, other: &Self) -> Self::UlpsDiff {
///         MyComplex32UlpsDiff {
///             re: self.re.ulps_diff(&other.re),
///             im: self.im.ulps_diff(&other.im),
///         }
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 2.0000036, };
/// let b = MyComplex32 { re: 1.0000001, im: 2.0, };
///
/// let abs_diff = a.abs_diff(&b);
/// assert_eq!(abs_diff.re, 0.00000011920929);
/// assert_eq!(abs_diff.im, 0.0000035762787);
///
/// let ulps_diff = a.ulps_diff(&b);
/// assert_eq!(ulps_diff.re, 1);
/// assert_eq!(ulps_diff.im, 15);
/// ```
///
/// If your type does *not* already have an underlying implementation of `FloatDiff`,
/// then you will need to take a close look at the descriptions of the algorithms on
/// a method by method basis.
///
/// ## How can I compare two different types?
///
/// The type you can `diff` with is controlled by `FloatDiff`'s parameter. Following
/// on from our previous example, if we wanted to treat `f32` as a complex number
/// with an imaginary component of `0.0`:
///
/// ```rust
/// # use float_eq::FloatDiff;
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// struct MyComplex32UlpsDiff {
///     re: <f32 as FloatDiff>::UlpsDiff,
///     im: <f32 as FloatDiff>::UlpsDiff,
/// }
///
/// impl FloatDiff<f32> for MyComplex32 {
///     type AbsDiff = MyComplex32;
///     type UlpsDiff = MyComplex32UlpsDiff;
///
///     fn abs_diff(&self, other: &f32) -> Self::AbsDiff {
///         MyComplex32 {
///             re: self.re.abs_diff(other),
///             im: self.im.abs_diff(&0.0),
///         }
///     }
///
///     fn ulps_diff(&self, other: &f32) -> Self::UlpsDiff {
///         MyComplex32UlpsDiff {
///             re: self.re.ulps_diff(other),
///             im: self.im.ulps_diff(&0.0),
///         }
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
///     fn ulps_diff(&self, other: &MyComplex32) -> Self::UlpsDiff {
///         other.ulps_diff(self)
///     }
/// }
///
/// let a = 1.0000001_f32;
/// let b = MyComplex32 { re: 1.0, im: 2.0000036, };
///
/// let abs_diff = a.abs_diff(&b);
/// assert_eq!(abs_diff.re, 0.00000011920929);
/// assert_eq!(abs_diff.im, 2.0000036);
///
/// let ulps_diff = a.ulps_diff(&b);
/// assert_eq!(ulps_diff.re, 1);
/// assert_eq!(ulps_diff.im, 1_073_741_839);
/// ```
///
/// ## Examples
///
/// ```rust
/// # use float_eq::FloatDiff;
/// assert_eq!(1.0_f32.abs_diff(&-1.0), 2.0);
/// assert_eq!(1.0_f64.abs_diff(&-1.0), 2.0);
///
/// assert_eq!(1.0_f32.ulps_diff(&1.0000001), 1);
/// assert_eq!(1.0_f64.ulps_diff(&1.0000000000000002), 1);
///
/// let a = [0.0_f32, 2.0, -2.0];
/// let b = [0.0_f32, -1.0, 2.0];
/// assert_eq!(a.abs_diff(&b), [0.0, 3.0, 4.0]);
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
    /// ```text
    /// (self - other).abs()
    /// ```
    fn abs_diff(&self, other: &Rhs) -> Self::AbsDiff;

    /// Always positive absolute difference between two values in terms of [ULPs]
    ///
    /// Implementations should be the equivalent of (using `f32` as an example):
    ///
    /// ```text
    /// let a = self.to_bits();
    /// let b = other.to_bits();
    /// let max = a.max(b);
    /// let min = a.min(b);
    /// max - min
    /// ```
    ///
    /// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
    fn ulps_diff(&self, other: &Rhs) -> Self::UlpsDiff;
}

/// Algorithms to compare IEEE floating point values for equality.
///
/// This trait is used in the implementation of the [`float_eq!`] and [`assert_float_eq!`]
/// families of macros. It may be called directly, but the macros usually provide
/// a friendlier interface.
///
/// ## How can I implement `FloatEq`?
///
/// Implementing `FloatEq` on your types is straightfoward if they're already
/// composed of compatible fields. You will need to choose the types of `DiffEpsilon`
/// and `UlpsDiffEpsilon` that users will specify to provide bounds on their
/// comparisons. For example, `MyComplex32` uses the same types as a single `f32`
/// comparison does, and passes them through to each individual components'
/// comparisons. This means that only a single number is needed to bound both real
/// and imaginary parts in a check:
///
/// ```
/// # use float_eq::{float_eq, float_ne, FloatEq};
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// impl FloatEq for MyComplex32 {
///     type DiffEpsilon = <f32 as FloatEq>::DiffEpsilon;
///     type UlpsDiffEpsilon = <f32 as FloatEq>::UlpsDiffEpsilon;
///
///     fn eq_abs(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
///         self.re.eq_abs(&other.re, max_diff) && self.im.eq_abs(&other.im, max_diff)
///     }
///
///     fn eq_rel(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
///         self.re.eq_rel(&other.re, max_diff) && self.im.eq_rel(&other.im, max_diff)
///     }
///
///     fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool {
///         self.re.eq_ulps(&other.re, max_diff) && self.im.eq_ulps(&other.im, max_diff)
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 2.0000036, };
/// let b = MyComplex32 { re: 1.0000001, im: 2.0, };
///
/// assert!(float_eq!(a, b, abs <= 0.0000036));
/// assert!(float_ne!(a, b, abs <= 0.0000035));
///
/// assert!(float_eq!(a, b, rel <= 0.0000018));
/// assert!(float_ne!(a, b, rel <= 0.0000017));
///
/// assert!(float_eq!(a, b, ulps <= 15));
/// assert!(float_ne!(a, b, ulps <= 14));
/// ```
///
/// If your type does *not* already have an underlying implementation of `FloatEq`,
/// then you will need to take a close look at the descriptions of the algorithms on
/// a method by method basis.
///
/// ## How can I compare two different types?
///
/// The type to be compared with is controlled by `FloatEq`'s parameter. Following
/// on from our previous example, if we wanted to treat `f32` as a complex number
/// with an imaginary component of `0.0`:
///
/// ```
/// # use float_eq::{float_eq, float_ne, FloatEq};
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// impl FloatEq<f32> for MyComplex32 {
///     type DiffEpsilon = <f32 as FloatEq>::DiffEpsilon;
///     type UlpsDiffEpsilon = <f32 as FloatEq>::UlpsDiffEpsilon;
///
///     fn eq_abs(&self, other: &f32, max_diff: &Self::DiffEpsilon) -> bool {
///         self.re.eq_abs(other, max_diff) && self.im.eq_abs(&0.0, max_diff)
///     }
///
///     fn eq_rel(&self, other: &f32, max_diff: &Self::DiffEpsilon) -> bool {
///         self.re.eq_rel(other, max_diff) && self.im.eq_rel(&0.0, max_diff)
///     }
///
///     fn eq_ulps(&self, other: &f32, max_diff: &Self::UlpsDiffEpsilon) -> bool {
///         self.re.eq_ulps(other, max_diff) && self.im.eq_ulps(&0.0, max_diff)
///     }
/// }
///
/// impl FloatEq<MyComplex32> for f32 {
///     type DiffEpsilon = <MyComplex32 as FloatEq<f32>>::DiffEpsilon;
///     type UlpsDiffEpsilon = <MyComplex32 as FloatEq<f32>>::UlpsDiffEpsilon;
///
///     fn eq_abs(&self, other: &MyComplex32, max_diff: &Self::DiffEpsilon) -> bool {
///         other.eq_abs(self, max_diff)
///     }
///
///     fn eq_rel(&self, other: &MyComplex32, max_diff: &Self::DiffEpsilon) -> bool {
///         other.eq_rel(self, max_diff)
///     }
///
///     fn eq_ulps(&self, other: &MyComplex32, max_diff: &Self::UlpsDiffEpsilon) -> bool {
///         other.eq_ulps(self, max_diff)
///     }
/// }
///
/// let a = 4.0_f32;
/// let b = MyComplex32 { re: 4.0000005, im: 0.0, };
///
/// assert!(float_eq!(a, b, abs <= 0.0000008));
/// assert!(float_ne!(a, b, abs <= 0.0000004));
///
/// assert!(float_eq!(a, b, rel <= 0.00000012));
/// assert!(float_ne!(a, b, rel <= 0.00000011));
///
/// assert!(float_eq!(a, b, ulps <= 1));
/// assert!(float_ne!(a, b, ulps <= 0));
/// ```
///
/// ## Examples
///
/// ```
/// # use float_eq::FloatEq;
/// assert!(4.0_f32.eq_abs(&4.0000015, &0.0000016));
/// assert!(4.0_f32.ne_abs(&4.0000015, &0.0000014));
///
/// assert!(4.0_f32.eq_rel(&4.0000015, &0.0000004));
/// assert!(4.0_f32.ne_rel(&4.0000015, &0.0000003));
///
/// assert!(4.0_f32.eq_ulps(&4.0000015, &3));
/// assert!(4.0_f32.ne_ulps(&4.0000015, &2));
/// ```
///
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`float_eq!`]: macro.float_eq.html
pub trait FloatEq<Rhs: ?Sized = Self> {
    /// Type of the maximum allowed difference between two values for them to be
    /// considered equal in terms of their native type.
    ///
    /// This is the type of the `max_diff` parameter passed to `abs` and `rel`
    /// checks in methods and via the [`float_eq!`] macros.
    ///
    /// [`float_eq!`]: macro.float_eq.html
    type DiffEpsilon;

    /// Type of the maximum allowed difference between two values for them to be
    /// considered equal in terms of an [ULPs comparison].
    ///
    /// This is the type of the `max_diff` parameter passed to `ulps` checks in
    /// methods and via the [`float_eq!`] macros. This is usually an unsigned
    /// integer of the same width as the float value (e.g. `f32` uses `u32`).
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    /// [`float_eq!`]: macro.float_eq.html
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
    fn eq_abs(&self, other: &Rhs, max_diff: &Self::DiffEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using an [absolute epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_abs(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn ne_abs(&self, other: &Rhs, max_diff: &Self::DiffEpsilon) -> bool {
        !self.eq_abs(other, max_diff)
    }

    /// Check whether `self` is equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// The implementation should be the equivalent of (using [`FloatDiff`]):
    ///
    /// ```text
    /// let largest = self.abs().max(other.abs());
    /// let epsilon = largest * max_diff;
    /// self.abs_diff(other) <= epsilon
    /// ```
    ///
    /// [`FloatDiff`]: trait.FloatDiff.html
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn eq_rel(&self, other: &Rhs, max_diff: &Self::DiffEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative epsilon
    /// comparison].
    ///
    /// Equal to `!self.eq_rel(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn ne_rel(&self, other: &Rhs, max_diff: &Self::DiffEpsilon) -> bool {
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
    fn eq_ulps(&self, other: &Rhs, max_diff: &Self::UlpsDiffEpsilon) -> bool;

    /// Check whether `self` is not equal to `other`, using an [ULPs comparison].
    ///
    /// Equal to `!self.eq_ulps(other, max_diff)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn ne_ulps(&self, other: &Rhs, max_diff: &Self::UlpsDiffEpsilon) -> bool {
        !self.eq_ulps(other, max_diff)
    }
}

/// Provides additional context for debugging when an assert fails.
///
/// This is used internally by the [`assert_float_eq!`] family of macros to provide
/// debug context information to the user when they fail.
///
/// ## How can I implement `FloatEqDebug`?
///
/// Implementing `FloatEqDebug` on your types is straightfoward if they're already
/// composed of compatible fields. First, you will need to implement [`FloatEq`].
/// You will then need to choose the types of `DebugEpsilon` and `DebugUlpsEpsilon`
/// to provide debug context when an assert fails. These ought to display the values
/// of [`DiffEpsilon`] and [`UlpsDiffEpsilon`] for each field of your type, following
/// the same structure so as to make it easier to see which values are being compared
/// for the checks. For example, `MyComplex32` here has `re` and `im` fields and
/// `MyComplex32UlpsDiff` follows that layout:
///
/// ```
/// # use float_eq::{FloatDiff, FloatEq, FloatEqDebug};
/// #[derive(Debug)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// #[derive(Debug)]
/// struct MyComplex32UlpsDiff {
///     re: <f32 as FloatDiff>::UlpsDiff,
///     im: <f32 as FloatDiff>::UlpsDiff,
/// }
///
/// impl FloatEq for MyComplex32 {
///     type DiffEpsilon = <f32 as FloatEq>::DiffEpsilon;
///     type UlpsDiffEpsilon = <f32 as FloatEq>::UlpsDiffEpsilon;
///
///     fn eq_abs(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
///         self.re.eq_abs(&other.re, max_diff) && self.im.eq_abs(&other.im, max_diff)
///     }
///
///     fn eq_rel(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
///         self.re.eq_rel(&other.re, max_diff) && self.im.eq_rel(&other.im, max_diff)
///     }
///
///     fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool {
///         self.re.eq_ulps(&other.re, max_diff) && self.im.eq_ulps(&other.im, max_diff)
///     }
/// }
///
/// impl FloatEqDebug for MyComplex32 {
///     type DebugEpsilon = Self;
///     type DebugUlpsEpsilon = MyComplex32UlpsDiff;
///
///     fn debug_abs_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::DiffEpsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_abs_epsilon(&other.re, max_diff),
///             im: self.im.debug_abs_epsilon(&other.im, max_diff),
///         }
///     }
///
///     fn debug_rel_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::DiffEpsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_rel_epsilon(&other.re, max_diff),
///             im: self.im.debug_rel_epsilon(&other.im, max_diff),
///         }
///     }
///
///     fn debug_ulps_epsilon(
///         &self,
///         other: &Self,
///         max_diff: &Self::UlpsDiffEpsilon,
///     ) -> Self::DebugUlpsEpsilon {
///         MyComplex32UlpsDiff {
///             re: self.re.debug_ulps_epsilon(&other.re, max_diff),
///             im: self.im.debug_ulps_epsilon(&other.im, max_diff),
///         }
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 200.0 };
/// let b = MyComplex32 { re: 50.0, im: 1.0 };
///
/// let abs_epsilon = a.debug_abs_epsilon(&b, &0.1);
/// assert_eq!(abs_epsilon.re, 0.1);
/// assert_eq!(abs_epsilon.im, 0.1);
///
/// let rel_epsilon = a.debug_rel_epsilon(&b, &0.1);
/// assert_eq!(rel_epsilon.re, 5.0);
/// assert_eq!(rel_epsilon.im, 20.0);
///
/// let ulps_epsilon = a.debug_ulps_epsilon(&b, &42);
/// assert_eq!(ulps_epsilon.re, 42);
/// assert_eq!(ulps_epsilon.im, 42);
/// ```
///
/// If your type does *not* already have an underlying implementation of `FloatEqDebug`,
/// then you will need to take a close look at the descriptions of the algorithms on
/// a method by method basis.
///
/// ## How can I compare two different types?
///
/// The type to be compared with is controlled by `FloatEqDebug`'s parameter.
/// Following on from our previous example, if we wanted to treat `f32` as a
/// complex number with an imaginary component of `0.0`:
///
/// ```
/// # use float_eq::{FloatDiff, FloatEq, FloatEqDebug};
/// #[derive(Debug)]
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// #[derive(Debug)]
/// struct MyComplex32UlpsDiff {
///     re: <f32 as FloatDiff>::UlpsDiff,
///     im: <f32 as FloatDiff>::UlpsDiff,
/// }
///
/// impl FloatEq<f32> for MyComplex32 {
///     type DiffEpsilon = <f32 as FloatEq>::DiffEpsilon;
///     type UlpsDiffEpsilon = <f32 as FloatEq>::UlpsDiffEpsilon;
///
///     fn eq_abs(&self, other: &f32, max_diff: &Self::DiffEpsilon) -> bool {
///         self.re.eq_abs(other, max_diff) && self.im.eq_abs(&0.0, max_diff)
///     }
///
///     fn eq_rel(&self, other: &f32, max_diff: &Self::DiffEpsilon) -> bool {
///         self.re.eq_rel(other, max_diff) && self.im.eq_rel(&0.0, max_diff)
///     }
///
///     fn eq_ulps(&self, other: &f32, max_diff: &Self::UlpsDiffEpsilon) -> bool {
///         self.re.eq_ulps(other, max_diff) && self.im.eq_ulps(&0.0, max_diff)
///     }
/// }
///
/// impl FloatEq<MyComplex32> for f32 {
///     type DiffEpsilon = <MyComplex32 as FloatEq<f32>>::DiffEpsilon;
///     type UlpsDiffEpsilon = <MyComplex32 as FloatEq<f32>>::UlpsDiffEpsilon;
///
///     fn eq_abs(&self, other: &MyComplex32, max_diff: &Self::DiffEpsilon) -> bool {
///         other.eq_abs(self, max_diff)
///     }
///
///     fn eq_rel(&self, other: &MyComplex32, max_diff: &Self::DiffEpsilon) -> bool {
///         other.eq_rel(self, max_diff)
///     }
///
///     fn eq_ulps(&self, other: &MyComplex32, max_diff: &Self::UlpsDiffEpsilon) -> bool {
///         other.eq_ulps(self, max_diff)
///     }
/// }
///
/// impl FloatEqDebug<f32> for MyComplex32 {
///     type DebugEpsilon = Self;
///     type DebugUlpsEpsilon = MyComplex32UlpsDiff;
///
///     fn debug_abs_epsilon(
///         &self,
///         other: &f32,
///         max_diff: &Self::DiffEpsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_abs_epsilon(other, max_diff),
///             im: self.im.debug_abs_epsilon(&0.0, max_diff),
///         }
///     }
///
///     fn debug_rel_epsilon(
///         &self,
///         other: &f32,
///         max_diff: &Self::DiffEpsilon
///     ) -> Self::DebugEpsilon {
///         MyComplex32 {
///             re: self.re.debug_rel_epsilon(other, max_diff),
///             im: self.im.debug_rel_epsilon(&0.0, max_diff),
///         }
///     }
///
///     fn debug_ulps_epsilon(
///         &self,
///         other: &f32,
///         max_diff: &Self::UlpsDiffEpsilon,
///     ) -> Self::DebugUlpsEpsilon {
///         MyComplex32UlpsDiff {
///             re: self.re.debug_ulps_epsilon(other, max_diff),
///             im: self.im.debug_ulps_epsilon(&0.0, max_diff),
///         }
///     }
/// }
///
/// impl FloatEqDebug<MyComplex32> for f32 {
///     type DebugEpsilon = <MyComplex32 as FloatEqDebug<f32>>::DebugEpsilon;
///     type DebugUlpsEpsilon = <MyComplex32 as FloatEqDebug<f32>>::DebugUlpsEpsilon;
///
///     fn debug_abs_epsilon(
///         &self,
///         other: &MyComplex32,
///         max_diff: &Self::DiffEpsilon,
///     ) -> Self::DebugEpsilon {
///         other.debug_abs_epsilon(self, max_diff)
///     }
///
///     fn debug_rel_epsilon(
///         &self,
///         other: &MyComplex32,
///         max_diff: &Self::DiffEpsilon,
///     ) -> Self::DebugEpsilon {
///         other.debug_rel_epsilon(self, max_diff)
///     }
///
///     fn debug_ulps_epsilon(
///         &self,
///         other: &MyComplex32,
///         max_diff: &Self::UlpsDiffEpsilon,
///     ) -> Self::DebugUlpsEpsilon {
///         other.debug_ulps_epsilon(self, max_diff)
///     }
/// }
///
/// let a = MyComplex32 { re: 1.0, im: 200.0 };
/// let b = 9000.0_f32;
///
/// let abs_epsilon = a.debug_abs_epsilon(&b, &0.1);
/// assert_eq!(abs_epsilon.re, 0.1);
/// assert_eq!(abs_epsilon.im, 0.1);
///
/// let rel_epsilon = a.debug_rel_epsilon(&b, &0.1);
/// assert_eq!(rel_epsilon.re, 900.0);
/// assert_eq!(rel_epsilon.im, 20.0);
///
/// let ulps_epsilon = a.debug_ulps_epsilon(&b, &42);
/// assert_eq!(ulps_epsilon.re, 42);
/// assert_eq!(ulps_epsilon.im, 42);
/// ```
///
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`DiffEpsilon`]: trait.FloatEq.html#associatedtype.DiffEpsilon
/// [`UlpsDiffEpsilon`]: trait.FloatEq.html#associatedtype.UlpsDiffEpsilon
pub trait FloatEqDebug<Rhs: ?Sized = Self>: FloatEq<Rhs> {
    /// Displayed to the user when an assert fails, using `fmt::Debug`.
    ///
    /// This should display [`Self::DiffEpsilon`] in an appropriate form to the
    /// user. For example, when implemented for an array type, it should be an
    /// array of the epsilon values so the user can see the link between the diff
    /// items to the values tested against.
    ///
    /// [`Self::DiffEpsilon`]: trait.FloatEq.html#associatedtype.DiffEpsilon
    type DebugEpsilon: fmt::Debug;

    /// Displayed to the user when an assert fails, using `fmt::Debug`.
    ///
    /// This should display [`Self::UlpsDiffEpsilon`] in an appropriate form to the
    /// user. For example, when implemented for an array type, it should be an
    /// array of the epsilon values so the user can see the link between the diff
    /// items to the values tested against.
    ///
    /// [`Self::UlpsDiffEpsilon`]: trait.FloatEq.html#associatedtype.UlpsDiffEpsilon
    type DebugUlpsEpsilon: fmt::Debug;

    /// The epsilon used by an [absolute epsilon comparison], displayed when an
    /// assert fails.
    ///
    /// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
    fn debug_abs_epsilon(&self, other: &Rhs, max_diff: &Self::DiffEpsilon) -> Self::DebugEpsilon;

    /// The epsilon used by a [relative epsilon comparison], displayed when an
    /// assert fails.
    ///
    /// [relative epsilon comparison]: index.html#relative-epsilon-comparison
    fn debug_rel_epsilon(&self, other: &Rhs, max_diff: &Self::DiffEpsilon) -> Self::DebugEpsilon;

    /// The epsilon used by an [ULPs comparison], displayed when an assert fails.
    ///
    /// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
    fn debug_ulps_epsilon(
        &self,
        other: &Rhs,
        max_diff: &Self::UlpsDiffEpsilon,
    ) -> Self::DebugUlpsEpsilon;
}
