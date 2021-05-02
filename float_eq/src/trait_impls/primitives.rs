#![allow(clippy::float_cmp)]

use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsTol, UlpsTol,
};

macro_rules! impl_traits {
    ($float:ident, $uint:ident) => {
        mod $float {
            #[cfg(feature = "std")]
            #[inline]
            pub(crate) fn abs(value: $float) -> $float {
                // use the intrinsic for std builds
                value.abs()
            }

            #[cfg(not(feature = "std"))]
            #[inline]
            pub(crate) fn abs(value: $float) -> $float {
                // mask away only the sign bit for no_std builds since the abs
                // method is not available
                const MASK: $uint = !(1 << ((::core::mem::size_of::<$float>() * 8) - 1));
                $float::from_bits(value.to_bits() & MASK)
            }
        }

        impl FloatEqUlpsTol for $float {
            type UlpsTol = $uint;
        }

        impl FloatEqDebugUlpsDiff for $float {
            type DebugUlpsDiff = Option<$uint>;
        }

        impl FloatEq for $float {
            type Tol = Self;

            #[inline]
            fn eq_abs(&self, other: &Self, tol: &Self::Tol) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || $float::abs(self - other).le(tol)
            }

            #[inline]
            fn eq_rmax(&self, other: &Self, tol: &Self::Tol) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || {
                    let largest = $float::abs(*self).max($float::abs(*other));
                    let tol = largest * tol;
                    $float::abs(self - other) <= tol
                }
            }

            #[inline]
            fn eq_rmin(&self, other: &Self, tol: &Self::Tol) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || {
                    let largest = $float::abs(*self).min($float::abs(*other));
                    let tol = largest * tol;
                    $float::abs(self - other) <= tol
                }
            }

            #[inline]
            fn eq_r1st(&self, other: &Self, tol: &Self::Tol) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || {
                    let tol = $float::abs(*self) * tol;
                    $float::abs(self - other) <= tol
                }
            }

            #[inline]
            fn eq_r2nd(&self, other: &Self, tol: &Self::Tol) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || {
                    let tol = $float::abs(*other) * tol;
                    $float::abs(self - other) <= tol
                }
            }

            #[inline]
            fn eq_ulps(&self, other: &Self, tol: &UlpsTol<Self::Tol>) -> bool {
                if self.is_nan() || other.is_nan() {
                    false // NaNs are never equal
                } else if self.is_sign_positive() != other.is_sign_positive() {
                    self == other // account for zero == negative zero
                } else {
                    let a = self.to_bits();
                    let b = other.to_bits();
                    let max = a.max(b);
                    let min = a.min(b);
                    (max - min).le(tol)
                }
            }
        }

        impl FloatEqAll for $float {
            type AllTol = $float;

            #[inline]
            fn eq_abs_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
                self.eq_abs(other, tol)
            }

            #[inline]
            fn eq_rmax_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
                self.eq_rmax(other, tol)
            }

            #[inline]
            fn eq_rmin_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
                self.eq_rmin(other, tol)
            }

            #[inline]
            fn eq_r1st_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
                self.eq_r1st(other, tol)
            }

            #[inline]
            fn eq_r2nd_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
                self.eq_r2nd(other, tol)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &Self, tol: &UlpsTol<Self::AllTol>) -> bool {
                self.eq_ulps(other, tol)
            }
        }

        impl AssertFloatEq for $float {
            type DebugAbsDiff = Self;
            type DebugTol = Self::Tol;

            #[inline]
            fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
                $float::abs(self - other)
            }

            #[inline]
            fn debug_ulps_diff(&self, other: &Self) -> DebugUlpsDiff<Self::DebugAbsDiff> {
                if self == other {
                    Some(0)
                } else if self.is_nan() || other.is_nan() {
                    None
                } else if self.is_sign_positive() != other.is_sign_positive() {
                    None
                } else {
                    let a = self.to_bits();
                    let b = other.to_bits();
                    let max = a.max(b);
                    let min = a.min(b);
                    Some(max - min)
                }
            }

            #[inline]
            fn debug_abs_tol(&self, _other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                *tol
            }

            #[inline]
            fn debug_rmax_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                $float::abs(*self).max($float::abs(*other)) * tol
            }

            #[inline]
            fn debug_rmin_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                $float::abs(*self).min($float::abs(*other)) * tol
            }

            #[inline]
            fn debug_r1st_tol(&self, _other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                $float::abs(*self) * tol
            }

            #[inline]
            fn debug_r2nd_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                $float::abs(*other) * tol
            }

            #[inline]
            fn debug_ulps_tol(
                &self,
                _other: &Self,
                tol: &UlpsTol<Self::Tol>,
            ) -> UlpsTol<Self::DebugTol> {
                *tol
            }
        }

        impl AssertFloatEqAll for $float {
            type AllDebugTol = Self::AllTol;

            #[inline]
            fn debug_abs_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
                self.debug_abs_tol(other, tol)
            }

            #[inline]
            fn debug_rmax_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
                self.debug_rmax_tol(other, tol)
            }

            #[inline]
            fn debug_rmin_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
                self.debug_rmin_tol(other, tol)
            }

            #[inline]
            fn debug_r1st_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
                self.debug_r1st_tol(other, tol)
            }

            #[inline]
            fn debug_r2nd_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
                self.debug_r2nd_tol(other, tol)
            }

            #[inline]
            fn debug_ulps_all_tol(
                &self,
                other: &Self,
                tol: &UlpsTol<Self::AllTol>,
            ) -> UlpsTol<Self::AllDebugTol> {
                self.debug_ulps_tol(other, tol)
            }
        }
    };
}

impl_traits!(f32, u32);
impl_traits!(f64, u64);
