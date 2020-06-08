use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug, FloatUlps, Ulps};
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

impl<T: FloatUlps> FloatUlps for Complex<T> {
    type Ulps = ComplexUlps<T::Ulps>;
}

/// [`ComplexUlps<T>`] type matching [`Complex32`].
///
/// [`ComplexUlps<T>`]: struct.ComplexUlps.html
/// [`Complex32`]: https://docs.rs/num-complex/0.2.4/num_complex/type.Complex32.html
pub type ComplexUlps32 = Ulps<Complex<f32>>;

/// [`ComplexUlps<T>`] type matching [`Complex64`].
///
/// [`ComplexUlps<T>`]: struct.ComplexUlps.html
/// [`Complex64`]: https://docs.rs/num-complex/0.2.4/num_complex/type.Complex64.html
pub type ComplexUlps64 = Ulps<Complex<f64>>;

impl<T> FloatDiff for Complex<T>
where
    T: FloatDiff,
{
    type Output = Complex<<T as FloatDiff>::Output>;

    #[inline]
    fn abs_diff(&self, other: &Self) -> Option<Self::Output> {
        Some(Self::Output {
            re: self.re.abs_diff(&other.re)?,
            im: self.im.abs_diff(&other.im)?,
        })
    }

    #[inline]
    fn ulps_diff(&self, other: &Self) -> Option<Ulps<Self::Output>> {
        Some(Ulps::<Self::Output> {
            re: self.re.ulps_diff(&other.re)?,
            im: self.im.ulps_diff(&other.im)?,
        })
    }
}

impl<T: FloatEq> FloatEq for Complex<T> {
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
    fn eq_ulps(&self, other: &Self, max_diff: &Ulps<Self::Epsilon>) -> bool {
        self.re.eq_ulps(&other.re, &max_diff.re) && self.im.eq_ulps(&other.im, &max_diff.im)
    }
}

impl<T: FloatEqAll> FloatEqAll for Complex<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn eq_abs_all(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_abs_all(&other.re, max_diff) && self.im.eq_abs_all(&other.im, max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_rel_all(&other.re, max_diff) && self.im.eq_rel_all(&other.im, max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Self, max_diff: &Ulps<Self::Epsilon>) -> bool {
        self.re.eq_ulps_all(&other.re, max_diff) && self.im.eq_ulps_all(&other.im, max_diff)
    }
}

impl<T> FloatEqDebug for Complex<T>
where
    T: FloatEqDebug,
{
    type DebugEpsilon = Complex<T::DebugEpsilon>;

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
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        Ulps::<Self::DebugEpsilon> {
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
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        Ulps::<Self::DebugEpsilon> {
            re: self.re.debug_ulps_all_epsilon(&other.re, max_diff),
            im: self.im.debug_ulps_all_epsilon(&other.im, max_diff),
        }
    }
}
