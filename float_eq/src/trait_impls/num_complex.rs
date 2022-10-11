use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsTol, UlpsTol,
};
use num_complex::Complex;

/// The absolute difference between two floating point [`num::Complex<T>`] instances
/// in ULPs.
///
/// The `T` in [`num::Complex<T>`] is constrained by `Clone` and `PartialEq`, so this
/// implements those too.
///
/// [`num::Complex<T>`]: https://docs.rs/num/0.3.0/num/struct.Complex.html
#[allow(clippy::derive_partial_eq_without_eq)] // Most likely this is going to use floats, and we don't want to derive Eq for those
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

impl<T: FloatEqUlpsTol> FloatEqUlpsTol for Complex<T>
where
    UlpsTol<T>: Sized,
{
    type UlpsTol = ComplexUlps<UlpsTol<T>>;
}

impl<T: FloatEqDebugUlpsDiff> FloatEqDebugUlpsDiff for Complex<T> {
    type DebugUlpsDiff = ComplexUlps<DebugUlpsDiff<T>>;
}

/// [`ComplexUlps<T>`] type matching [`num::Complex32`].
///
/// [`num::Complex32`]: https://docs.rs/num-complex/0.3/num_complex/type.Complex32.html
pub type ComplexUlps32 = UlpsTol<Complex<f32>>;

/// [`ComplexUlps<T>`] type matching [`num::Complex64`].
///
/// [`num::Complex64`]: https://docs.rs/num-complex/0.3/num_complex/type.Complex64.html
pub type ComplexUlps64 = UlpsTol<Complex<f64>>;

impl<T: FloatEq> FloatEq for Complex<T>
where
    T::Tol: Sized,
    UlpsTol<T::Tol>: Sized,
{
    type Tol = Complex<T::Tol>;

    #[inline]
    fn eq_abs(&self, other: &Self, tol: &Self::Tol) -> bool {
        self.re.eq_abs(&other.re, &tol.re) && self.im.eq_abs(&other.im, &tol.im)
    }

    #[inline]
    fn eq_rmax(&self, other: &Self, tol: &Self::Tol) -> bool {
        self.re.eq_rmax(&other.re, &tol.re) && self.im.eq_rmax(&other.im, &tol.im)
    }

    #[inline]
    fn eq_rmin(&self, other: &Self, tol: &Self::Tol) -> bool {
        self.re.eq_rmin(&other.re, &tol.re) && self.im.eq_rmin(&other.im, &tol.im)
    }

    #[inline]
    fn eq_r1st(&self, other: &Self, tol: &Self::Tol) -> bool {
        self.re.eq_r1st(&other.re, &tol.re) && self.im.eq_r1st(&other.im, &tol.im)
    }

    #[inline]
    fn eq_r2nd(&self, other: &Self, tol: &Self::Tol) -> bool {
        self.re.eq_r2nd(&other.re, &tol.re) && self.im.eq_r2nd(&other.im, &tol.im)
    }

    #[inline]
    fn eq_ulps(&self, other: &Self, tol: &UlpsTol<Self::Tol>) -> bool {
        self.re.eq_ulps(&other.re, &tol.re) && self.im.eq_ulps(&other.im, &tol.im)
    }
}

impl<T: FloatEqAll> FloatEqAll for Complex<T> {
    type AllTol = T::AllTol;

    #[inline]
    fn eq_abs_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
        self.re.eq_abs_all(&other.re, tol) && self.im.eq_abs_all(&other.im, tol)
    }

    #[inline]
    fn eq_rmax_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
        self.re.eq_rmax_all(&other.re, tol) && self.im.eq_rmax_all(&other.im, tol)
    }

    #[inline]
    fn eq_rmin_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
        self.re.eq_rmin_all(&other.re, tol) && self.im.eq_rmin_all(&other.im, tol)
    }

    #[inline]
    fn eq_r1st_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
        self.re.eq_r1st_all(&other.re, tol) && self.im.eq_r1st_all(&other.im, tol)
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
        self.re.eq_r2nd_all(&other.re, tol) && self.im.eq_r2nd_all(&other.im, tol)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Self, tol: &UlpsTol<Self::AllTol>) -> bool {
        self.re.eq_ulps_all(&other.re, tol) && self.im.eq_ulps_all(&other.im, tol)
    }
}

impl<T> AssertFloatEq for Complex<T>
where
    T: AssertFloatEq,
    T::Tol: Sized,
    T::DebugTol: Sized,
    UlpsTol<T::Tol>: Sized,
    UlpsTol<T::DebugTol>: Sized,
{
    type DebugAbsDiff = Complex<T::DebugAbsDiff>;
    type DebugTol = Complex<T::DebugTol>;

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
    fn debug_abs_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        Self::DebugTol {
            re: self.re.debug_abs_tol(&other.re, &tol.re),
            im: self.im.debug_abs_tol(&other.im, &tol.im),
        }
    }

    #[inline]
    fn debug_rmax_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        Self::DebugTol {
            re: self.re.debug_rmax_tol(&other.re, &tol.re),
            im: self.im.debug_rmax_tol(&other.im, &tol.im),
        }
    }

    #[inline]
    fn debug_rmin_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        Self::DebugTol {
            re: self.re.debug_rmin_tol(&other.re, &tol.re),
            im: self.im.debug_rmin_tol(&other.im, &tol.im),
        }
    }

    #[inline]
    fn debug_r1st_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        Self::DebugTol {
            re: self.re.debug_r1st_tol(&other.re, &tol.re),
            im: self.im.debug_r1st_tol(&other.im, &tol.im),
        }
    }

    #[inline]
    fn debug_r2nd_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        Self::DebugTol {
            re: self.re.debug_r2nd_tol(&other.re, &tol.re),
            im: self.im.debug_r2nd_tol(&other.im, &tol.im),
        }
    }

    #[inline]
    fn debug_ulps_tol(&self, other: &Self, tol: &UlpsTol<Self::Tol>) -> UlpsTol<Self::DebugTol>
    where
        UlpsTol<Self::DebugTol>: Sized,
    {
        UlpsTol::<Self::DebugTol> {
            re: self.re.debug_ulps_tol(&other.re, &tol.re),
            im: self.im.debug_ulps_tol(&other.im, &tol.im),
        }
    }
}

impl<T> AssertFloatEqAll for Complex<T>
where
    T: AssertFloatEqAll,
    T::AllDebugTol: Sized,
    UlpsTol<T::AllDebugTol>: Sized,
{
    type AllDebugTol = Complex<T::AllDebugTol>;

    #[inline]
    fn debug_abs_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Self::AllDebugTol {
            re: self.re.debug_abs_all_tol(&other.re, tol),
            im: self.im.debug_abs_all_tol(&other.im, tol),
        }
    }

    #[inline]
    fn debug_rmax_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Self::AllDebugTol {
            re: self.re.debug_rmax_all_tol(&other.re, tol),
            im: self.im.debug_rmax_all_tol(&other.im, tol),
        }
    }

    #[inline]
    fn debug_rmin_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Self::AllDebugTol {
            re: self.re.debug_rmin_all_tol(&other.re, tol),
            im: self.im.debug_rmin_all_tol(&other.im, tol),
        }
    }

    #[inline]
    fn debug_r1st_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Self::AllDebugTol {
            re: self.re.debug_r1st_all_tol(&other.re, tol),
            im: self.im.debug_r1st_all_tol(&other.im, tol),
        }
    }

    #[inline]
    fn debug_r2nd_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Self::AllDebugTol {
            re: self.re.debug_r2nd_all_tol(&other.re, tol),
            im: self.im.debug_r2nd_all_tol(&other.im, tol),
        }
    }

    #[inline]
    fn debug_ulps_all_tol(
        &self,
        other: &Self,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol>
    where
        UlpsTol<Self::AllDebugTol>: Sized,
    {
        UlpsTol::<Self::AllDebugTol> {
            re: self.re.debug_ulps_all_tol(&other.re, tol),
            im: self.im.debug_ulps_all_tol(&other.im, tol),
        }
    }
}
