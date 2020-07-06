use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsEpsilon, UlpsEpsilon,
};
use num_complex::Complex;

/// The absolute difference between two floating point [`Complex<T>`] instances
/// in ULPs.
///
/// The `T` in [`Complex<T>`] is constrained by `Clone` and `PartialEq`, so this
/// implements those too.
///
/// [`Complex<T>`]: https://docs.rs/num/0.3.0/num/struct.Complex.html
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

impl<T: FloatEqUlpsEpsilon> FloatEqUlpsEpsilon for Complex<T>
where
    UlpsEpsilon<T>: Sized,
{
    type UlpsEpsilon = ComplexUlps<UlpsEpsilon<T>>;
}

impl<T: FloatEqDebugUlpsDiff> FloatEqDebugUlpsDiff for Complex<T> {
    type DebugUlpsDiff = ComplexUlps<DebugUlpsDiff<T>>;
}

/// [`ComplexUlps<T>`] type matching [`Complex32`].
///
/// [`ComplexUlps<T>`]: struct.ComplexUlps.html
/// [`Complex32`]: https://docs.rs/num-complex/0.2.4/num_complex/type.Complex32.html
pub type ComplexUlps32 = UlpsEpsilon<Complex<f32>>;

/// [`ComplexUlps<T>`] type matching [`Complex64`].
///
/// [`ComplexUlps<T>`]: struct.ComplexUlps.html
/// [`Complex64`]: https://docs.rs/num-complex/0.2.4/num_complex/type.Complex64.html
pub type ComplexUlps64 = UlpsEpsilon<Complex<f64>>;

impl<T: FloatEq> FloatEq for Complex<T>
where
    T::Epsilon: Sized,
    UlpsEpsilon<T::Epsilon>: Sized,
{
    type Epsilon = Complex<T::Epsilon>;

    #[inline]
    fn eq_abs(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_abs(&other.re, &max_diff.re) && self.im.eq_abs(&other.im, &max_diff.im)
    }

    #[inline]
    fn eq_rel(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_rel(&other.re, &max_diff.re) && self.im.eq_rel(&other.im, &max_diff.im)
    }

    #[inline]
    fn eq_ulps(&self, other: &Self, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
        self.re.eq_ulps(&other.re, &max_diff.re) && self.im.eq_ulps(&other.im, &max_diff.im)
    }
}

impl<T: FloatEqAll> FloatEqAll for Complex<T> {
    type AllEpsilon = T::AllEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
        self.re.eq_abs_all(&other.re, max_diff) && self.im.eq_abs_all(&other.im, max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
        self.re.eq_rel_all(&other.re, max_diff) && self.im.eq_rel_all(&other.im, max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Self, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
        self.re.eq_ulps_all(&other.re, max_diff) && self.im.eq_ulps_all(&other.im, max_diff)
    }
}

impl<T> AssertFloatEq for Complex<T>
where
    T: AssertFloatEq,
    T::Epsilon: Sized,
    T::DebugEpsilon: Sized,
    UlpsEpsilon<T::Epsilon>: Sized,
    UlpsEpsilon<T::DebugEpsilon>: Sized,
{
    type DebugAbsDiff = Complex<T::DebugAbsDiff>;
    type DebugEpsilon = Complex<T::DebugEpsilon>;

    #[inline]
    fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
        Self::DebugAbsDiff {
            re: self.re.debug_abs_diff(&other.re),
            im: self.im.debug_abs_diff(&other.im),
        }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Self) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        DebugUlpsDiff::<Self::DebugAbsDiff> {
            re: self.re.debug_ulps_diff(&other.re),
            im: self.im.debug_ulps_diff(&other.im),
        }
    }

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
        max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon>
    where
        UlpsEpsilon<Self::DebugEpsilon>: Sized,
    {
        UlpsEpsilon::<Self::DebugEpsilon> {
            re: self.re.debug_ulps_epsilon(&other.re, &max_diff.re),
            im: self.im.debug_ulps_epsilon(&other.im, &max_diff.im),
        }
    }
}

impl<T> AssertFloatEqAll for Complex<T>
where
    T: AssertFloatEqAll,
    T::AllDebugEpsilon: Sized,
    UlpsEpsilon<T::AllDebugEpsilon>: Sized,
{
    type AllDebugEpsilon = Complex<T::AllDebugEpsilon>;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Self::AllDebugEpsilon {
            re: self.re.debug_abs_all_epsilon(&other.re, max_diff),
            im: self.im.debug_abs_all_epsilon(&other.im, max_diff),
        }
    }

    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Self::AllDebugEpsilon {
            re: self.re.debug_rel_all_epsilon(&other.re, max_diff),
            im: self.im.debug_rel_all_epsilon(&other.im, max_diff),
        }
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &Self,
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon>
    where
        UlpsEpsilon<Self::AllDebugEpsilon>: Sized,
    {
        UlpsEpsilon::<Self::AllDebugEpsilon> {
            re: self.re.debug_ulps_all_epsilon(&other.re, max_diff),
            im: self.im.debug_ulps_all_epsilon(&other.im, max_diff),
        }
    }
}
