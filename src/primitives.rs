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
            type DiffEpsilon = $float;
            type UlpsDiffEpsilon = $uint;

            #[inline]
            fn eq_abs(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
                self.abs_diff(other).le(max_diff)
            }

            #[inline]
            fn eq_rel(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
                let largest = $float::abs(*self).max($float::abs(*other));
                let epsilon = largest * max_diff;
                self.abs_diff(other) <= epsilon
            }

            #[inline]
            fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool {
                if self.is_sign_positive() != other.is_sign_positive() {
                    self == other // account for zero == negative zero
                } else {
                    self.ulps_diff(other).le(max_diff)
                }
            }
        }

        impl FloatEqAll for $float {
            type DiffEpsilon = <$float as FloatEq>::DiffEpsilon;
            type UlpsDiffEpsilon = <$float as FloatEq>::UlpsDiffEpsilon;

            #[inline]
            fn eq_abs_all(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
                self.eq_abs(other, max_diff)
            }

            #[inline]
            fn eq_rel_all(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
                self.eq_rel(other, max_diff)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool {
                self.eq_ulps(other, max_diff)
            }
        }

        impl FloatEqDebug for $float {
            type DebugEpsilon = <Self as FloatEq>::DiffEpsilon;
            type DebugUlpsEpsilon = <Self as FloatEq>::UlpsDiffEpsilon;

            fn debug_abs_epsilon(
                &self,
                _other: &Self,
                max_diff: &<Self as FloatEq>::DiffEpsilon,
            ) -> Self::DebugEpsilon {
                *max_diff
            }

            fn debug_rel_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEq>::DiffEpsilon,
            ) -> Self::DebugEpsilon {
                $float::abs(*self).max($float::abs(*other)) * max_diff
            }

            fn debug_ulps_epsilon(
                &self,
                _other: &Self,
                max_diff: &<Self as FloatEq>::UlpsDiffEpsilon,
            ) -> Self::DebugUlpsEpsilon {
                *max_diff
            }
        }

        impl FloatEqAllDebug for $float {
            type DebugEpsilon = <Self as FloatEqAll>::DiffEpsilon;
            type DebugUlpsEpsilon = <Self as FloatEqAll>::UlpsDiffEpsilon;

            fn debug_abs_all_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEqAll>::DiffEpsilon,
            ) -> Self::DebugEpsilon {
                self.debug_abs_epsilon(other, max_diff)
            }

            fn debug_rel_all_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEq>::DiffEpsilon,
            ) -> Self::DebugEpsilon {
                self.debug_rel_epsilon(other, max_diff)
            }

            fn debug_ulps_all_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEq>::UlpsDiffEpsilon,
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
    use crate::{FloatDiff, FloatEq};
    use core::fmt;

    macro_rules! impl_tests {
        ($float:ident) => {
            mod $float {
                use super::*;
                use core::$float;

                #[test]
                fn abs_diff() {
                    let check = |a: $float, b, expected| {
                        assert!(a.abs_diff(&b) - expected <= $float::EPSILON)
                    };

                    check(1., 1., 0.);
                    check(1., 1.5, 0.5);
                    check(1., -1., 2.);

                    let nan = $float::NAN;
                    assert!(nan.abs_diff(&1.0).is_nan());
                    assert!(1.0.abs_diff(&nan).is_nan());
                    assert!(nan.abs_diff(&nan).is_nan());
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
                }

                #[test]
                fn eq_abs() {
                    let eq = <$float as FloatEq>::eq_abs;
                    let ne = <$float as FloatEq>::ne_abs;

                    let check_eq = |a, b, max_diff| {
                        check(eq, ne, a, b, max_diff, true);

                        assert!(float_eq!(a, b, abs <= max_diff));
                        assert!(float_eq!(a, b, abs_all <= max_diff));

                        assert!(!float_ne!(a, b, abs <= max_diff));
                        assert!(!float_ne!(a, b, abs_all <= max_diff));

                        assert_float_eq!(a, b, abs <= max_diff);
                        assert_float_eq!(a, b, abs_all <= max_diff);

                        // trailing comma
                        assert!(float_eq!(a, b, abs <= max_diff,));
                        assert!(!float_ne!(a, b, abs <= max_diff,));
                        assert_float_eq!(a, b, abs <= max_diff,);
                    };

                    let check_ne = |a, b, max_diff| {
                        check(eq, ne, a, b, max_diff, false);

                        assert!(!float_eq!(a, b, abs <= max_diff));
                        assert!(!float_eq!(a, b, abs_all <= max_diff));

                        assert!(float_ne!(a, b, abs <= max_diff));
                        assert!(float_ne!(a, b, abs_all <= max_diff));

                        assert_float_ne!(a, b, abs <= max_diff);
                        assert_float_ne!(a, b, abs_all <= max_diff);

                        // trailing comma
                        assert_float_ne!(a, b, abs <= max_diff,);
                    };

                    // useful in range where epsilon is relevent
                    let one: $float = 1.;
                    let eps = $float::EPSILON;
                    check_eq(one, one, 0.);
                    check_eq(one, one, eps);

                    check_ne(one, one + eps, 0.);
                    check_eq(one, one + eps, eps);
                    check_ne(one, one + (2. * eps), eps);

                    check_ne(one, one - eps, 0.);
                    check_eq(one, one - eps, eps);
                    check_ne(one, one - (2. * eps), eps);

                    // unintuitive outside of range where epsilon is relevant
                    let two: $float = 2.;
                    let next = $float::from_bits(two.to_bits() + 1);
                    check_ne(two, next, eps);
                    check_eq(two, next, next - two);

                    let a: $float = -128_000.;
                    let next = $float::from_bits(a.to_bits() + 1);
                    check_ne(a, next, eps);
                    check_eq(a, next, a - next);

                    // zero
                    let zero = 0.;
                    let neg_zero = -0.;
                    check_eq(zero, neg_zero, 0.);

                    // NaN
                    let nan = $float::NAN;
                    check_ne(one, nan, eps);
                    check_ne(nan, nan, eps);
                }

                #[test]
                fn eq_rel() {
                    let eq = <$float as FloatEq>::eq_rel;
                    let ne = <$float as FloatEq>::ne_rel;

                    let check_eq = |a, b, max_diff| {
                        check(eq, ne, a, b, max_diff, true);

                        assert!(float_eq!(a, b, rel <= max_diff));
                        assert!(float_eq!(a, b, rel_all <= max_diff));

                        assert!(!float_ne!(a, b, rel <= max_diff));
                        assert!(!float_ne!(a, b, rel_all <= max_diff));

                        assert_float_eq!(a, b, rel <= max_diff);
                        assert_float_eq!(a, b, rel_all <= max_diff);
                    };

                    let check_ne = |a, b, max_diff| {
                        check(eq, ne, a, b, max_diff, false);
                        assert!(!float_eq!(a, b, rel <= max_diff));
                        assert!(!float_eq!(a, b, rel_all <= max_diff));

                        assert!(float_ne!(a, b, rel <= max_diff));
                        assert!(float_ne!(a, b, rel_all <= max_diff));

                        assert_float_ne!(a, b, rel <= max_diff);
                        assert_float_ne!(a, b, rel_all <= max_diff);
                    };

                    // useful in range where epsilon is relevent
                    let one: $float = 1.;
                    let eps = $float::EPSILON;
                    check_eq(one, one, 0.);
                    check_eq(one, one, eps);

                    check_ne(one, one + eps, 0.);
                    check_eq(one, one + eps, eps);
                    check_ne(one, one + (2. * eps), eps);

                    check_ne(one, one - eps, 0.);
                    check_eq(one, one - eps, eps);
                    check_ne(one, one - (2. * eps), eps);

                    // intuitive outside of range where epsilon is relevant
                    let two: $float = 2.;
                    let next = $float::from_bits(two.to_bits() + 1);
                    check_eq(two, next, eps);

                    let a: $float = -128_000.;
                    let next = $float::from_bits(a.to_bits() + 1);
                    check_eq(a, next, eps);

                    // zero
                    let zero = 0.;
                    let neg_zero = -0.;
                    check_eq(zero, neg_zero, 0.);

                    // NaN
                    let nan = $float::NAN;
                    check_ne(one, nan, eps);
                    check_ne(nan, nan, eps);
                }

                #[test]
                fn eq_ulps() {
                    let eq = <$float as FloatEq>::eq_ulps;
                    let ne = <$float as FloatEq>::ne_ulps;

                    let check_eq = |a, b, max_diff| {
                        check(eq, ne, a, b, max_diff, true);
                        assert!(float_eq!(a, b, ulps <= max_diff));
                        assert!(!float_ne!(a, b, ulps <= max_diff));
                        assert_float_eq!(a, b, ulps <= max_diff);
                    };

                    let check_ne = |a, b, max_diff| {
                        check(eq, ne, a, b, max_diff, false);
                        assert!(!float_eq!(a, b, ulps <= max_diff));
                        assert!(float_ne!(a, b, ulps <= max_diff));
                        assert_float_ne!(a, b, ulps <= max_diff);
                    };

                    // useful in range where epsilon is relevent
                    let one: $float = 1.;
                    let eps = $float::EPSILON;
                    check_eq(one, one, 0);
                    check_eq(one, one, 1);

                    check_ne(one, one + eps, 0);
                    check_eq(one, one + eps, 1);
                    check_ne(one, one + (2. * eps), 1);

                    // unintuitive at the boundaries of powers of two
                    check_ne(one, one - eps, 0);
                    check_ne(one, one - eps, 1);
                    check_eq(one, one - eps, 2);
                    check_ne(one, one - (2. * eps), 1);

                    // intuitive-ish outside of range where epsilon is relevant
                    let two: $float = 2.;
                    let next = $float::from_bits(two.to_bits() + 1);
                    check_eq(two, next, 1);

                    let a: $float = -128_000.;
                    let next = $float::from_bits(a.to_bits() + 1);
                    check_eq(a, next, 1);

                    // zero
                    let zero = 0.;
                    let neg_zero = -0.;
                    check_eq(zero, neg_zero, 0);
                    check_eq(zero, neg_zero, 1);

                    // NaN
                    let nan = $float::NAN;
                    check_ne(one, nan, 1);
                    check_eq(nan, nan, 1);
                }
            }
        };
    }

    impl_tests!(f32);
    impl_tests!(f64);

    fn check<T, U, EQ, NE>(eq: EQ, ne: NE, a: T, b: T, max_diff: U, expect_equal: bool)
    where
        T: FloatEq + fmt::Display,
        U: fmt::Display,
        EQ: Fn(&T, &T, &U) -> bool,
        NE: Fn(&T, &T, &U) -> bool,
    {
        assert!(
            eq(&a, &b, &max_diff) == expect_equal,
            "a: {}, b: {}, max_diff: {}",
            a,
            b,
            max_diff
        );
        assert!(
            ne(&a, &b, &max_diff) != expect_equal,
            "a: {}, b: {}, max_diff: {}",
            a,
            b,
            max_diff
        );
    }
}
