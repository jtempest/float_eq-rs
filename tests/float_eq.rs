#![allow(clippy::eq_op)]

use float_eq::{assert_float_eq, assert_float_ne, float_eq, float_ne, FloatEq};

macro_rules! impl_tests {
    ($float:ident) => {
        mod $float {
            use super::*;

            #[test]
            fn eq_abs() {
                let eq = <$float as FloatEq>::eq_abs;
                let ne = <$float as FloatEq>::ne_abs;

                let check_eq = |a, b, max_diff| {
                    check(eq, ne, a, b, max_diff, true);
                    assert!(float_eq!(a, b, abs <= max_diff));
                    assert!(!float_ne!(a, b, abs <= max_diff));

                    assert_float_eq!(a, b, abs <= max_diff);

                    // trailing comma
                    assert!(float_eq!(a, b, abs <= max_diff,));
                    assert!(!float_ne!(a, b, abs <= max_diff,));
                    assert_float_eq!(a, b, abs <= max_diff,);
                };

                let check_ne = |a, b, max_diff| {
                    check(eq, ne, a, b, max_diff, false);
                    assert!(!float_eq!(a, b, abs <= max_diff));
                    assert!(float_ne!(a, b, abs <= max_diff));

                    assert_float_ne!(a, b, abs <= max_diff);

                    // trailing comma
                    assert_float_ne!(a, b, abs <= max_diff,);
                };

                // useful in range where epsilon is relevent
                let one: $float = 1.;
                let eps = std::$float::EPSILON;
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
                let nan = std::$float::NAN;
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
                    assert!(!float_ne!(a, b, rel <= max_diff));
                    assert_float_eq!(a, b, rel <= max_diff);
                };

                let check_ne = |a, b, max_diff| {
                    check(eq, ne, a, b, max_diff, false);
                    assert!(!float_eq!(a, b, rel <= max_diff));
                    assert!(float_ne!(a, b, rel <= max_diff));
                    assert_float_ne!(a, b, rel <= max_diff);
                };

                // useful in range where epsilon is relevent
                let one: $float = 1.;
                let eps = std::$float::EPSILON;
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
                let nan = std::$float::NAN;
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
                let eps = std::$float::EPSILON;
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
                let nan = std::$float::NAN;
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
    T: FloatEq + std::fmt::Display,
    U: std::fmt::Display,
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
