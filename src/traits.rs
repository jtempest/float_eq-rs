use core::fmt;

/// Algorithms to compute the difference between two IEEE floating point values.
///
/// By default this trait is implemented on `f32` and `f64`, and for arrays of
/// compatible type which have size 0 to 32 (inclusive).
///
/// ## How can I implement `FloatDiff`?
///
/// Implementing `FloatDiff` on your types is straightfoward if they're already
/// composed of compatible types. You'll need some way to represent difference in
/// [ULPs] for your type, probably following the same structure as the type itself.
/// For example, `MyComplex32` here has `re` and `im` fields and `MyComplex32UlpsDiff`
/// follows that layout. Once you have this type, implementing the methods is a case
/// of calling through to the underlying implementation of each member:
///
/// ```rust
/// # use float_eq::FloatDiff;
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// struct MyComplex32UlpsDiff {
///     re: u32,
///     im: u32,
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
/// If your type does *not* already have an underlying implementation of `FloatDiff`
/// for its members, then you'll need to take a close look at the descriptions of
/// the algorithms on a member by member basis.
///
/// ## How can I implement `FloatDiff` between two different types?
///
/// The type you can `diff` with is controlled by `FloatDiff`'s parameter. Following
/// on from our previous example, if we wanted to treat `f32` as a complex number
/// with no imaginary component:
///
/// ```rust
/// # use float_eq::FloatDiff;
/// struct MyComplex32 {
///     re: f32,
///     im: f32,
/// }
///
/// struct MyComplex32UlpsDiff {
///     re: u32,
///     im: u32,
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
    /// let a = (self.to_bits()) as u32;
    /// let b = (other.to_bits()) as u32;
    /// let max = a.max(b);
    /// let min = a.min(b);
    /// max - min
    /// ```
    ///
    /// [ULPs]: index.html#units-in-the-last-place-ulps-comparison
    fn ulps_diff(&self, other: &Rhs) -> Self::UlpsDiff;
}

/// Algorithms to compare two IEEE floating point values for equality.
pub trait FloatEq<Rhs: ?Sized = Self> {
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

/// Provides additional context for debugging when an assert fires.
///
/// This is used internally by `float_eq` assert macros, and the epsilons do not
/// necessarily match those used directly in the calculations. Implementations of
/// this trait should try to provide context information for the overall calculation
/// as an aid to the user. For example, arrays being compared may expose their
/// debug info as an array of epsilon values, whereas their `FloatEq` methods
/// perform calculations one by one to allow shortcutting.
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
    /// This should display [`Self::DiffEpsilon`] in an appropriate form to the
    /// user. For example, when implemented for an array type, it should be an
    /// array of the epsilon values so the user can see the link between the diff
    /// items to the values tested against.
    ///
    /// [`Self::DiffEpsilon`]: trait.FloatEq.html#associatedtype.DiffEpsilon
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
