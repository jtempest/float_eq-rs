#![allow(clippy::float_cmp)]

use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsEpsilon, UlpsEpsilon,
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

        impl FloatEqUlpsEpsilon for $float {
            type UlpsEpsilon = $uint;
        }

        impl FloatEqDebugUlpsDiff for $float {
            type DebugUlpsDiff = Option<$uint>;
        }

        impl FloatEq for $float {
            type Epsilon = Self;

            #[inline]
            fn eq_abs(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || $float::abs(self - other).le(max_diff)
            }

            #[inline]
            fn eq_rmax(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || {
                    let largest = $float::abs(*self).max($float::abs(*other));
                    let epsilon = largest * max_diff;
                    $float::abs(self - other) <= epsilon
                }
            }

            #[inline]
            fn eq_rmin(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || {
                    let largest = $float::abs(*self).min($float::abs(*other));
                    let epsilon = largest * max_diff;
                    $float::abs(self - other) <= epsilon
                }
            }

            #[inline]
            fn eq_r1st(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || {
                    let epsilon = $float::abs(*self) * max_diff;
                    $float::abs(self - other) <= epsilon
                }
            }

            #[inline]
            fn eq_r2nd(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || {
                    let epsilon = $float::abs(*other) * max_diff;
                    $float::abs(self - other) <= epsilon
                }
            }

            #[inline]
            fn eq_ulps(&self, other: &Self, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
                if self.is_nan() || other.is_nan() {
                    false // NaNs are never equal
                } else if self.is_sign_positive() != other.is_sign_positive() {
                    self == other // account for zero == negative zero
                } else {
                    let a = self.to_bits();
                    let b = other.to_bits();
                    let max = a.max(b);
                    let min = a.min(b);
                    (max - min).le(max_diff)
                }
            }
        }

        impl FloatEqAll for $float {
            type AllEpsilon = $float;

            #[inline]
            fn eq_abs_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
                self.eq_abs(other, max_diff)
            }

            #[inline]
            fn eq_rmax_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
                self.eq_rmax(other, max_diff)
            }

            #[inline]
            fn eq_rmin_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
                self.eq_rmin(other, max_diff)
            }

            #[inline]
            fn eq_r1st_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
                self.eq_r1st(other, max_diff)
            }

            #[inline]
            fn eq_r2nd_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
                self.eq_r2nd(other, max_diff)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &Self, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
                self.eq_ulps(other, max_diff)
            }
        }

        impl AssertFloatEq for $float {
            type DebugAbsDiff = Self;
            type DebugEpsilon = Self::Epsilon;

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
            fn debug_abs_epsilon(
                &self,
                _other: &Self,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                *max_diff
            }

            #[inline]
            fn debug_rmax_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                $float::abs(*self).max($float::abs(*other)) * max_diff
            }

            #[inline]
            fn debug_rmin_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                $float::abs(*self).min($float::abs(*other)) * max_diff
            }

            #[inline]
            fn debug_r1st_epsilon(
                &self,
                _other: &Self,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                $float::abs(*self) * max_diff
            }

            #[inline]
            fn debug_r2nd_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                $float::abs(*other) * max_diff
            }

            #[inline]
            fn debug_ulps_epsilon(
                &self,
                _other: &Self,
                max_diff: &UlpsEpsilon<Self::Epsilon>,
            ) -> UlpsEpsilon<Self::DebugEpsilon> {
                *max_diff
            }
        }

        impl AssertFloatEqAll for $float {
            type AllDebugEpsilon = Self::AllEpsilon;

            #[inline]
            fn debug_abs_all_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                self.debug_abs_epsilon(other, max_diff)
            }

            #[inline]
            fn debug_rmax_all_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                self.debug_rmax_epsilon(other, max_diff)
            }

            #[inline]
            fn debug_rmin_all_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                self.debug_rmin_epsilon(other, max_diff)
            }

            #[inline]
            fn debug_r1st_all_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                self.debug_r1st_epsilon(other, max_diff)
            }

            #[inline]
            fn debug_r2nd_all_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                self.debug_r2nd_epsilon(other, max_diff)
            }

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &Self,
                max_diff: &UlpsEpsilon<Self::AllEpsilon>,
            ) -> UlpsEpsilon<Self::AllDebugEpsilon> {
                self.debug_ulps_epsilon(other, max_diff)
            }
        }
    };
}

impl_traits!(f32, u32);
impl_traits!(f64, u64);
