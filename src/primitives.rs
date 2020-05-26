use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug};

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
            pub(crate) fn abs(value: $float) -> $float {
                // mask away only the sign bit for no_std builds since the abs
                // method is not available
                const MASK: $uint = !(1 << ((::core::mem::size_of::<$float>() * 8) - 1));
                $float::from_bits(value.to_bits() & MASK)
            }
        }

        impl FloatDiff for $float {
            type AbsDiff = Self;
            type UlpsDiff = $uint;

            #[inline]
            fn abs_diff(&self, other: &Self) -> Self {
                $float::abs(self - other)
            }

            #[inline]
            fn ulps_diff(&self, other: &Self) -> Self::UlpsDiff {
                let a = self.to_bits();
                let b = other.to_bits();
                let max = a.max(b);
                let min = a.min(b);
                max - min
            }
        }

        impl FloatEq for $float {
            type Epsilon = $float;
            type UlpsEpsilon = $uint;

            #[inline]
            fn eq_abs(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || self.abs_diff(other).le(max_diff)
            }

            #[inline]
            fn eq_rel(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                // the PartialEq check covers equality of infinities
                self == other || {
                    let largest = $float::abs(*self).max($float::abs(*other));
                    let epsilon = largest * max_diff;
                    self.abs_diff(other) <= epsilon
                }
            }

            #[inline]
            fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsEpsilon) -> bool {
                if self.is_nan() || other.is_nan() {
                    false // NaNs are never equal
                } else if self.is_sign_positive() != other.is_sign_positive() {
                    self == other // account for zero == negative zero
                } else {
                    self.ulps_diff(other).le(max_diff)
                }
            }
        }

        impl FloatEqAll for $float {
            type Epsilon = <$float as FloatEq>::Epsilon;
            type UlpsEpsilon = <$float as FloatEq>::UlpsEpsilon;

            #[inline]
            fn eq_abs_all(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                self.eq_abs(other, max_diff)
            }

            #[inline]
            fn eq_rel_all(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                self.eq_rel(other, max_diff)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &Self, max_diff: &Self::UlpsEpsilon) -> bool {
                self.eq_ulps(other, max_diff)
            }
        }

        impl FloatEqDebug for $float {
            type DebugEpsilon = <Self as FloatEq>::Epsilon;
            type DebugUlpsEpsilon = <Self as FloatEq>::UlpsEpsilon;

            #[inline]
            fn debug_abs_epsilon(
                &self,
                _other: &Self,
                max_diff: &<Self as FloatEq>::Epsilon,
            ) -> Self::DebugEpsilon {
                *max_diff
            }

            #[inline]
            fn debug_rel_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEq>::Epsilon,
            ) -> Self::DebugEpsilon {
                $float::abs(*self).max($float::abs(*other)) * max_diff
            }

            #[inline]
            fn debug_ulps_epsilon(
                &self,
                _other: &Self,
                max_diff: &<Self as FloatEq>::UlpsEpsilon,
            ) -> Self::DebugUlpsEpsilon {
                *max_diff
            }
        }

        impl FloatEqAllDebug for $float {
            type DebugEpsilon = <Self as FloatEqAll>::Epsilon;
            type DebugUlpsEpsilon = <Self as FloatEqAll>::UlpsEpsilon;

            #[inline]
            fn debug_abs_all_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEqAll>::Epsilon,
            ) -> Self::DebugEpsilon {
                self.debug_abs_epsilon(other, max_diff)
            }

            #[inline]
            fn debug_rel_all_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEq>::Epsilon,
            ) -> Self::DebugEpsilon {
                self.debug_rel_epsilon(other, max_diff)
            }

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEq>::UlpsEpsilon,
            ) -> Self::DebugUlpsEpsilon {
                self.debug_ulps_epsilon(other, max_diff)
            }
        }
    };
}

impl_traits!(f32, u32);
impl_traits!(f64, u64);

// tests
#[cfg(test)]
mod tests {
    use crate::FloatDiff;

    macro_rules! impl_tests {
        ($float:ident, $uint:ident, $nan_bits:ident) => {
            mod $float {
                use super::*;
                use core::$float;

                fn nan_test_values() -> [$float; 4] {
                    [
                        $float::from_bits($nan_bits.pos_min),
                        $float::from_bits($nan_bits.pos_max),
                        $float::from_bits($nan_bits.neg_min),
                        $float::from_bits($nan_bits.neg_max),
                    ]
                }

                #[test]
                fn abs_diff() {
                    let check = |a: $float, b, expected| {
                        assert!(a.abs_diff(&b) - expected <= $float::EPSILON)
                    };

                    check(1., 1., 0.);
                    check(1., 1.5, 0.5);
                    check(1., -1., 2.);

                    let nans = nan_test_values();
                    for a in &nans {
                        assert!(a.abs_diff(&1.0).is_nan());
                        assert!(1.0.abs_diff(a).is_nan());
                        for b in &nans {
                            assert!(a.abs_diff(b).is_nan());
                        }
                    }
                }

                #[test]
                fn ulps_diff() {
                    let check = |a: $float, b, expected| assert_eq!(a.ulps_diff(&b), expected);

                    let one: $float = 1.;
                    check(one, one, 0);

                    let next = $float::from_bits(one.to_bits() + 1);
                    check(one, next, 1);
                    check(next, one, 1);

                    let prev = $float::from_bits(one.to_bits() - 1);
                    check(one, prev, 1);
                    check(prev, one, 1);
                    check(next, prev, 2);
                    check(prev, next, 2);

                    //TODO: NaNs?
                }
            }
        };
    }

    // A selection of NaN values from the edges of the ranges of negative and
    // positive NaN values and their payloads. Testing every single NaN value
    // is viable in reasonable time for f32, but there are just too many f64
    // values so this is the compromise.
    #[derive(Clone, Copy)]
    struct NaNBits<T> {
        pos_min: T,
        pos_max: T,
        neg_min: T,
        neg_max: T,
    }

    const F32_NAN_BITS: NaNBits<u32> = NaNBits {
        pos_min: 0x7F_80_00_01,
        pos_max: 0x7F_FF_FF_FF,
        neg_min: 0xFF_80_00_01,
        neg_max: 0xFF_FF_FF_FF,
    };

    const F64_NAN_BITS: NaNBits<u64> = NaNBits {
        pos_min: 0x7F_F0_00_00_00_00_00_01,
        pos_max: 0x7F_FF_FF_FF_FF_FF_FF_FF,
        neg_min: 0xFF_F0_00_00_00_00_00_01,
        neg_max: 0xFF_FF_FF_FF_FF_FF_FF_FF,
    };

    impl_tests!(f32, u32, F32_NAN_BITS);
    impl_tests!(f64, u64, F64_NAN_BITS);
}
