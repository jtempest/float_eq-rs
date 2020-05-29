use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug};
use num_complex::Complex;

/// The absolute difference between two floating point [`Complex<T>`] instances
/// in ULPs.
///
/// The `T` in [`Complex<T>`] is constrained by `Clone` and `PartialEq`, so this
/// implements those too.
///
/// [`Complex<T>`]: https://rust-num.github.io/num/num/struct.Complex.html
#[derive(Clone, Debug, PartialEq)]
pub struct ComplexUlps<T> {
    /// Real portion of the complex number in ULPs.
    pub re: T,
    /// Imaginary portion of the complex number in ULPs.
    pub im: T,
}

impl<T> ComplexUlps<T> {
    /// Create a new ComplexUlps
    pub fn new(re: T, im: T) -> Self {
        ComplexUlps { re, im }
    }
}

/// [`ComplexUlps<T>`] type matching [`Complex32`].
///
/// [`ComplexUlps<T>`]: struct.ComplexUlps.html
/// [`Complex32`]: https://docs.rs/num-complex/0.2.4/num_complex/type.Complex32.html
pub type ComplexUlps32 = ComplexUlps<<f32 as FloatDiff>::UlpsDiff>;

/// [`ComplexUlps<T>`] type matching [`Complex64`].
///
/// [`ComplexUlps<T>`]: struct.ComplexUlps.html
/// [`Complex64`]: https://docs.rs/num-complex/0.2.4/num_complex/type.Complex64.html
pub type ComplexUlps64 = ComplexUlps<<f64 as FloatDiff>::UlpsDiff>;

impl<T> FloatDiff for Complex<T>
where
    T: FloatDiff,
{
    type AbsDiff = Complex<T::AbsDiff>;
    type UlpsDiff = ComplexUlps<T::UlpsDiff>;

    #[inline]
    fn abs_diff(&self, other: &Self) -> Self::AbsDiff {
        Self::AbsDiff {
            re: self.re.abs_diff(&other.re),
            im: self.im.abs_diff(&other.im),
        }
    }

    #[inline]
    fn ulps_diff(&self, other: &Self) -> Self::UlpsDiff {
        Self::UlpsDiff {
            re: self.re.ulps_diff(&other.re),
            im: self.im.ulps_diff(&other.im),
        }
    }
}

impl<T: FloatEq> FloatEq for Complex<T> {
    type Epsilon = Complex<T::Epsilon>;
    type UlpsEpsilon = ComplexUlps<T::UlpsEpsilon>;

    #[inline]
    fn eq_abs(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_abs(&other.re, &max_diff.re) && self.im.eq_abs(&other.im, &max_diff.im)
    }

    #[inline]
    fn eq_rel(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_rel(&other.re, &max_diff.re) && self.im.eq_rel(&other.im, &max_diff.im)
    }

    #[inline]
    fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsEpsilon) -> bool {
        self.re.eq_ulps(&other.re, &max_diff.re) && self.im.eq_ulps(&other.im, &max_diff.im)
    }
}

impl<T: FloatEqAll> FloatEqAll for Complex<T> {
    type Epsilon = T::Epsilon;
    type UlpsEpsilon = T::UlpsEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_abs_all(&other.re, max_diff) && self.im.eq_abs_all(&other.im, max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_rel_all(&other.re, max_diff) && self.im.eq_rel_all(&other.im, max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Self, max_diff: &Self::UlpsEpsilon) -> bool {
        self.re.eq_ulps_all(&other.re, max_diff) && self.im.eq_ulps_all(&other.im, max_diff)
    }
}

impl<T> FloatEqDebug for Complex<T>
where
    T: FloatEqDebug,
{
    type DebugEpsilon = Complex<T::DebugEpsilon>;
    type DebugUlpsEpsilon = ComplexUlps<T::DebugUlpsEpsilon>;

    #[inline]
    fn debug_abs_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        Self::DebugEpsilon {
            re: self.re.debug_abs_epsilon(&other.re, &max_diff.re),
            im: self.im.debug_abs_epsilon(&other.im, &max_diff.im),
        }
    }

    #[inline]
    fn debug_rel_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        Self::DebugEpsilon {
            re: self.re.debug_rel_epsilon(&other.re, &max_diff.re),
            im: self.im.debug_rel_epsilon(&other.im, &max_diff.im),
        }
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        Self::DebugUlpsEpsilon {
            re: self.re.debug_ulps_epsilon(&other.re, &max_diff.re),
            im: self.im.debug_ulps_epsilon(&other.im, &max_diff.im),
        }
    }
}

impl<T> FloatEqAllDebug for Complex<T>
where
    T: FloatEqAllDebug,
{
    type DebugEpsilon = Complex<T::DebugEpsilon>;
    type DebugUlpsEpsilon = ComplexUlps<T::DebugUlpsEpsilon>;

    #[inline]
    fn debug_abs_all_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        Self::DebugEpsilon {
            re: self.re.debug_abs_all_epsilon(&other.re, max_diff),
            im: self.im.debug_abs_all_epsilon(&other.im, max_diff),
        }
    }

    #[inline]
    fn debug_rel_all_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        Self::DebugEpsilon {
            re: self.re.debug_rel_all_epsilon(&other.re, max_diff),
            im: self.im.debug_rel_all_epsilon(&other.im, max_diff),
        }
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        Self::DebugUlpsEpsilon {
            re: self.re.debug_ulps_all_epsilon(&other.re, max_diff),
            im: self.im.debug_ulps_all_epsilon(&other.im, max_diff),
        }
    }
}
