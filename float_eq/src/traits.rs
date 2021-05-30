use core::fmt;

/// Per-field tolerances for [ULPs comparisons](https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison).
///
/// This trait establishes a one-to-one relation between an IEEE floating point
/// type and a type whose fields are expected to be structurally identical but
/// specified in [ULPs]. It is used by ULPs equality checks to specify per-field
/// tolerances.
///
/// By convention, this type is named `FooUlps` for a given type `Foo`.
///
/// The [`UlpsTol`] type alias exists to simplify usage, for example `UlpsTol<f32>`
/// is `u32`.
///
/// To implement this trait over a new type, see [How to compare custom types].
///
/// [How to compare custom types]: https://jtempest.github.io/float_eq-rs/book/how_to/compare_custom_types.html
/// [ULPs]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
pub trait FloatEqUlpsTol {
    /// A structurally identical type to `Self`, with fields recursively wrapped
    /// by `UlpsTol`.
    type UlpsTol: ?Sized;
}

/// Per-field tolerances for [ULPs comparisons].
///
/// [ULPs comparisons]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
pub type UlpsTol<T> = <T as FloatEqUlpsTol>::UlpsTol;

/// Per-field results of [ULPs](https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison)
/// based diff calculations.
///
/// This trait establishes a one-to-one relation between an IEEE floating point
/// type and a type whose fields are expected to be structurally identical but
/// specified as the result of calculating a diff in [ULPs]. This is usually
/// [`UlpsTol`] wrapped in an `Option`, since the ULPs diff between two floats is
/// a partial function that returns `None` when the signs differ. It is used by
/// testing and debugging tools to show the difference between two values on a
/// per-field basis and is built for clarity, not runtime efficiency.
///
/// By convention, this type is named `FooDebugUlpsDiff` for a given type `Foo`.
///
/// The [`DebugUlpsDiff`] type alias exists to simplify usage, for example
/// `DebugUlpsDiff<f32>` is `Option<u32>`.
///
/// To implement this trait over a new type, see [How to compare custom types].
///
/// [How to compare custom types]: https://jtempest.github.io/float_eq-rs/book/how_to/compare_custom_types.html
/// [ULPs]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
pub trait FloatEqDebugUlpsDiff {
    /// A structurally identical type to `Self`, with fields recursively wrapped
    /// by `DebugUlpsDiff`.
    type DebugUlpsDiff;
}

/// Per-field results of [ULPs] based diff calculations.
///
/// [ULPs]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
pub type DebugUlpsDiff<T> = <T as FloatEqDebugUlpsDiff>::DebugUlpsDiff;

/// Compare IEEE floating point values for equality using per-field tolerances.
///
/// This trait is used in the implementation of the [`float_eq!`] and [`assert_float_eq!`]
/// families of macros.
///
/// To implement this trait over a new type, see [How to compare custom types].
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
/// [How to compare custom types]: https://jtempest.github.io/float_eq-rs/book/how_to/compare_custom_types.html
pub trait FloatEq<Rhs: ?Sized = Self> {
    /// Type of the maximum allowed difference between two values for them to be
    /// considered equal.
    type Tol: ?Sized + FloatEqUlpsTol;

    /// Check whether `self` is equal to `other`, using an [absolute tolerance
    /// comparison].
    ///
    /// Implementations should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_abs(&self, other: &Self, tol: &Self) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_abs(&self, other: &Self, tol: &Self) -> bool {
    /// // the PartialEq check covers equality of infinities
    /// self == other || (self - other).abs().le(tol)
    /// # }}
    /// ```
    ///
    /// [absolute tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#absolute-tolerance-comparison
    fn eq_abs(&self, other: &Rhs, tol: &Self::Tol) -> bool;

    /// Check whether `self` is not equal to `other`, using an [absolute tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_abs(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [absolute tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#absolute-tolerance-comparison
    #[inline]
    fn ne_abs(&self, other: &Rhs, tol: &Self::Tol) -> bool {
        !self.eq_abs(other, tol)
    }

    /// Check whether `self` is equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `self.eq_rmax(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn eq_rel(&self, other: &Rhs, tol: &Self::Tol) -> bool {
        self.eq_rmax(other, tol)
    }

    /// Check whether `self` is not equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_rel(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn ne_rel(&self, other: &Rhs, tol: &Self::Tol) -> bool {
        !self.eq_rel(other, tol)
    }

    /// Check whether `self` is equal to `other`, using a [relative tolerance
    /// comparison], scaled to the granularity of the input with the largest
    /// magnitude.
    ///
    /// The implementation should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_rel(&self, other: &Self, tol: &Self) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_rel(&self, other: &Self, tol: &Self) -> bool {
    /// // the PartialEq check covers equality of infinities
    /// self == other || {
    ///     let largest = self.abs().max(other.abs());
    ///     let tolerance = largest * tol;
    ///     (self - other).abs() <= tolerance
    /// }
    /// # }}
    /// ```
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    fn eq_rmax(&self, other: &Rhs, tol: &Self::Tol) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_rmax(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn ne_rmax(&self, other: &Rhs, tol: &Self::Tol) -> bool {
        !self.eq_rmax(other, tol)
    }

    /// Check whether `self` is equal to `other`, using a [relative tolerance
    /// comparison], scaled to the granularity of the input with the smallest
    /// magnitude.
    ///
    /// The implementation should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_rel(&self, other: &Self, tol: &Self) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_rel(&self, other: &Self, tol: &Self) -> bool {
    /// // the PartialEq check covers equality of infinities
    /// self == other || {
    ///     let smallest = self.abs().min(other.abs());
    ///     let tolerance = smallest * tol;
    ///     (self - other).abs() <= tolerance
    /// }
    /// # }}
    /// ```
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    fn eq_rmin(&self, other: &Rhs, tol: &Self::Tol) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_rmin(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn ne_rmin(&self, other: &Rhs, tol: &Self::Tol) -> bool {
        !self.eq_rmin(other, tol)
    }

    /// Check whether `self` is equal to `other`, using a [relative tolerance
    /// comparison], scaled to the granularity of the first input.
    ///
    /// The implementation should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_rel(&self, other: &Self, tol: &Self) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_rel(&self, other: &Self, tol: &Self) -> bool {
    /// // the PartialEq check covers equality of infinities
    /// self == other || {
    ///     let tolerance = self.abs() * tol;
    ///     (self - other).abs() <= tolerance
    /// }
    /// # }}
    /// ```
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    fn eq_r1st(&self, other: &Rhs, tol: &Self::Tol) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_r1st(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn ne_r1st(&self, other: &Rhs, tol: &Self::Tol) -> bool {
        !self.eq_r1st(other, tol)
    }

    /// Check whether `self` is equal to `other`, using a [relative tolerance
    /// comparison], scaled to the granularity of the second input.
    ///
    /// The implementation should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_rel(&self, other: &Self, tol: &Self) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_rel(&self, other: &Self, tol: &Self) -> bool {
    /// // the PartialEq check covers equality of infinities
    /// self == other || {
    ///     let tolerance = other.abs() * tol;
    ///     (self - other).abs() <= tolerance
    /// }
    /// # }}
    /// ```
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    fn eq_r2nd(&self, other: &Rhs, tol: &Self::Tol) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_r2nd(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn ne_r2nd(&self, other: &Rhs, tol: &Self::Tol) -> bool {
        !self.eq_r2nd(other, tol)
    }

    /// Check whether `self` is equal to `other`, using an [ULPs comparison](https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison).
    ///
    /// The implementation should be the equivalent of:
    ///
    /// ```
    /// # trait TestFloatEq { fn eq_ulps(&self, other: &Self, tol: &u32) -> bool; }
    /// # impl TestFloatEq for f32 {
    /// # fn eq_ulps(&self, other: &Self, tol: &u32) -> bool {
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
    ///     (max - min).le(tol)
    /// }
    /// # }}
    /// ```
    ///
    /// [ULPs comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
    fn eq_ulps(&self, other: &Rhs, tol: &UlpsTol<Self::Tol>) -> bool;

    /// Check whether `self` is not equal to `other`, using an [ULPs comparison].
    ///
    /// Equal to `!self.eq_ulps(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [ULPs comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
    #[inline]
    fn ne_ulps(&self, other: &Rhs, tol: &UlpsTol<Self::Tol>) -> bool {
        !self.eq_ulps(other, tol)
    }
}

/// Compare IEEE floating point values for equality using a uniform tolerance.
///
/// This trait is used in the implementation of the [`float_eq!`] and [`assert_float_eq!`]
/// families of macros to provide the `*_all` variants of comparison algorithms
/// for homogeneous types.
///
/// To implement this trait over a new type, see [How to compare custom types].
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
/// [How to compare custom types]: https://jtempest.github.io/float_eq-rs/book/how_to/compare_custom_types.html
/// [ULPs]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
/// [`AllTol`]: trait.FloatEqAll.html#associatedtype.AllTol
pub trait FloatEqAll<Rhs: ?Sized = Self> {
    /// Type of the maximum allowed difference between each of two values' fields
    /// for them to be considered equal.
    type AllTol: ?Sized + FloatEqUlpsTol;

    /// Check whether `self` is equal to `other`, using an [absolute tolerance
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_abs`].
    ///
    /// [absolute tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#absolute-tolerance-comparison
    fn eq_abs_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool;

    /// Check whether `self` is not equal to `other`, using an [absolute tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_abs_all(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [absolute tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#absolute-tolerance-comparison
    #[inline]
    fn ne_abs_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool {
        !self.eq_abs_all(other, tol)
    }

    /// Check whether `self` is equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `self.eq_rmax_all(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn eq_rel_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool {
        self.eq_rmax_all(other, tol)
    }

    /// Check whether `self` is not equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_rel_all(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn ne_rel_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool {
        !self.eq_rel_all(other, tol)
    }

    /// Check whether `self` is equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_rmax`].
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    fn eq_rmax_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_rmax_all(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn ne_rmax_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool {
        !self.eq_rmax_all(other, tol)
    }

    /// Check whether `self` is equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_rmin`].
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    fn eq_rmin_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_rmin_all(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn ne_rmin_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool {
        !self.eq_rmin_all(other, tol)
    }

    /// Check whether `self` is equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_r1st`].
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    fn eq_r1st_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_r1st_all(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn ne_r1st_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool {
        !self.eq_r1st_all(other, tol)
    }

    /// Check whether `self` is equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_r2nd`].
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    fn eq_r2nd_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool;

    /// Check whether `self` is not equal to `other`, using a [relative tolerance
    /// comparison].
    ///
    /// Equal to `!self.eq_r2nd_all(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [relative tolerance comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#relative-tolerance-comparison
    #[inline]
    fn ne_r2nd_all(&self, other: &Rhs, tol: &Self::AllTol) -> bool {
        !self.eq_r2nd_all(other, tol)
    }

    /// Check whether `self` is equal to `other`, using an [ULPs comparison].
    ///
    /// This must use the same algorithm as [`FloatEq::eq_ulps`].
    ///
    /// [ULPs comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
    fn eq_ulps_all(&self, other: &Rhs, tol: &UlpsTol<Self::AllTol>) -> bool;

    /// Check whether `self` is not equal to `other`, using an [ULPs comparison].
    ///
    /// Equal to `!self.eq_ulps_all(other, tol)`, there is no need to reimplement
    /// this for your own types.
    ///
    /// [ULPs comparison]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
    #[inline]
    fn ne_ulps_all(&self, other: &Rhs, tol: &UlpsTol<Self::AllTol>) -> bool {
        !self.eq_ulps_all(other, tol)
    }
}

/// Debug context for when an assert fails.
///
/// This trait is used by [`assert_float_eq!`] and [`assert_float_ne!`].
///
/// To implement this trait over a new type, see [How to compare custom types].
///
/// [How to compare custom types]: https://jtempest.github.io/float_eq-rs/book/how_to/compare_custom_types.html
/// [ULPs]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
pub trait AssertFloatEq<Rhs: ?Sized = Self>: FloatEq<Rhs> {
    /// The absolute difference between two values, displayed to the user via
    /// `fmt::Debug` when an assert fails.
    ///
    /// This is usually the wider of `Self` and `Rhs`.
    type DebugAbsDiff: fmt::Debug + Sized + FloatEqDebugUlpsDiff;

    /// The per-field tolerance value used for comparison between two values,
    /// displayed to the user via `fmt::Debug` when an assert fails.
    ///
    /// This should match [`Self::Tol`].
    ///
    /// [`Self::Tol`]: trait.FloatEq.html#associatedtype.Tol
    type DebugTol: fmt::Debug + FloatEqUlpsTol;

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
    /// - `Some(0)` if both arguments are either `0.0` or `-0.0`
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
    /// [ULPs]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
    fn debug_ulps_diff(&self, other: &Rhs) -> DebugUlpsDiff<Self::DebugAbsDiff>;

    /// The tolerance used by an `abs` [comparison], displayed when an assert fails.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_abs_tol(&self, other: &Rhs, tol: &Self::Tol) -> Self::DebugTol;

    /// The tolerance used by a `rel` [comparison], displayed when an assert fails.
    ///
    /// Equivalent to `self.debug_rmax_tol(self, other, tol)`, there is
    /// no need to reimplement this for your own types.
    ///
    /// [comparison]: index.html#comparison-algorithms
    #[inline]
    fn debug_rel_tol(&self, other: &Rhs, tol: &Self::Tol) -> Self::DebugTol {
        self.debug_rmax_tol(other, tol)
    }

    /// The tolerance used by an `rmax` [comparison], displayed when an assert fails.
    ///
    /// Returns `tol` scaled by the magnitude of the larger operand.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_rmax_tol(&self, other: &Rhs, tol: &Self::Tol) -> Self::DebugTol;

    /// The tolerance used by an `rmin` [comparison], displayed when an assert fails.
    ///
    /// Returns `tol` scaled by the magnitude of the smaller operand.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_rmin_tol(&self, other: &Rhs, tol: &Self::Tol) -> Self::DebugTol;

    /// The tolerance used by an `r1st` [comparison], displayed when an assert fails.
    ///
    /// Returns `tol` scaled by the magnitude of the first operand.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_r1st_tol(&self, other: &Rhs, tol: &Self::Tol) -> Self::DebugTol;

    /// The tolerance used by an `r2nd` [comparison], displayed when an assert fails.
    ///
    /// Returns `tol` scaled by the magnitude of the second operand.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_r2nd_tol(&self, other: &Rhs, tol: &Self::Tol) -> Self::DebugTol;

    /// The tolerance used by an `ulps` [comparison], displayed when an assert fails.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_ulps_tol(&self, other: &Rhs, tol: &UlpsTol<Self::Tol>) -> UlpsTol<Self::DebugTol>
    where
        UlpsTol<Self::DebugTol>: Sized;
}

/// Debug context for when an assert using an `all` check fails.
///
/// This trait is used by [`assert_float_eq!`] and [`assert_float_ne!`].
///
/// To implement this trait over a new type, see [How to compare custom types].
///
/// [How to compare custom types]: https://jtempest.github.io/float_eq-rs/book/how_to/compare_custom_types.html
/// [ULPs]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
pub trait AssertFloatEqAll<Rhs: ?Sized = Self>: FloatEqAll<Rhs> {
    /// Displayed to the user when an assert fails, using `fmt::Debug`.
    ///
    /// This should match the fields of the the most complex type in the comparison.
    type AllDebugTol: fmt::Debug + FloatEqUlpsTol;

    /// The tolerance used by an `abs_all` [comparison], displayed when an assert fails.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_abs_all_tol(&self, other: &Rhs, tol: &Self::AllTol) -> Self::AllDebugTol;

    /// The tolerance used by a `rel_all` [comparison], displayed when an assert fails.
    ///
    /// Equivalent to `self.debug_rmax_all_tol(self, other, tol)`, there
    /// is no need to reimplement this for your own types.
    ///
    /// [comparison]: index.html#comparison-algorithms
    #[inline]
    fn debug_rel_all_tol(&self, other: &Rhs, tol: &Self::AllTol) -> Self::AllDebugTol {
        self.debug_rmax_all_tol(other, tol)
    }

    /// The tolerance used by an `rmax_all` [comparison], displayed when an assert fails.
    ///
    /// Returns `tol` scaled by the magnitude of the larger operand.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_rmax_all_tol(&self, other: &Rhs, tol: &Self::AllTol) -> Self::AllDebugTol;

    /// The tolerance used by an `rmin_all` [comparison], displayed when an assert fails.
    ///
    /// Returns `tol` scaled by the magnitude of the smaller operand.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_rmin_all_tol(&self, other: &Rhs, tol: &Self::AllTol) -> Self::AllDebugTol;

    /// The tolerance used by an `r1st_all` [comparison], displayed
    /// when an assert fails.
    ///
    /// Returns `tol` scaled by the magnitude of the first operand.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_r1st_all_tol(&self, other: &Rhs, tol: &Self::AllTol) -> Self::AllDebugTol;

    /// The tolerance used by an `r2nd_all` [comparison], displayed when an assert fails.
    ///
    /// Returns `tol` scaled by the magnitude of the second operand.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_r2nd_all_tol(&self, other: &Rhs, tol: &Self::AllTol) -> Self::AllDebugTol;

    /// The tolerance used by an `ulps_all` [comparison], displayed when an assert fails.
    ///
    /// [comparison]: index.html#comparison-algorithms
    fn debug_ulps_all_tol(
        &self,
        other: &Rhs,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol>
    where
        UlpsTol<Self::AllDebugTol>: Sized;
}
