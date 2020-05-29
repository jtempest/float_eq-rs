/// Systematic tests of eq_ulps/eq_ulps_all behaviour over primitives.

macro_rules! impl_tests {
    ($float:ident, $uint:ident) => {
        mod $float {
            use crate::unit_tests::primitives::$float::*;
            use float_eq::{
                assert_float_eq, assert_float_ne, float_eq, float_ne, FloatEq, FloatEqAll,
            };

            fn check_eq_ulps(a: $float, b: $float, max_diff: $uint) {
                assert_float_eq!(a, b, ulps <= max_diff);
                assert_float_eq!(a, b, ulps_all <= max_diff);

                assert!(float_eq!(a, b, ulps <= max_diff));
                assert!(float_eq!(a, b, ulps_all <= max_diff));
                assert!(!float_ne!(a, b, ulps <= max_diff));
                assert!(!float_ne!(a, b, ulps_all <= max_diff));

                assert!(a.eq_ulps(&b, &max_diff));
                assert!(a.eq_ulps_all(&b, &max_diff));
                assert!(!a.ne_ulps(&b, &max_diff));
                assert!(!a.ne_ulps_all(&b, &max_diff));
            }

            fn check_ne_ulps(a: $float, b: $float, max_diff: $uint) {
                assert_float_ne!(a, b, ulps <= max_diff);
                assert_float_ne!(a, b, ulps_all <= max_diff);

                assert!(!float_eq!(a, b, ulps <= max_diff));
                assert!(!float_eq!(a, b, ulps_all <= max_diff));
                assert!(float_ne!(a, b, ulps <= max_diff));
                assert!(float_ne!(a, b, ulps_all <= max_diff));

                assert!(!a.eq_ulps(&b, &max_diff));
                assert!(!a.eq_ulps_all(&b, &max_diff));
                assert!(a.ne_ulps(&b, &max_diff));
                assert!(a.ne_ulps_all(&b, &max_diff));
            }

            // also covers commutativity and negative values
            fn check_eq(a: $float, b: $float, max_diff: $uint) {
                check_eq_ulps(a, b, max_diff);
                check_eq_ulps(b, a, max_diff);
                check_eq_ulps(-a, -b, max_diff);
                check_eq_ulps(-b, -a, max_diff);
            }

            // also covers commutativity and negative values
            fn check_ne(a: $float, b: $float, max_diff: $uint) {
                check_ne_ulps(a, b, max_diff);
                check_ne_ulps(b, a, max_diff);
                check_ne_ulps(-a, -b, max_diff);
                check_ne_ulps(-b, -a, max_diff);
            }

            // also covers tests over -f
            fn check_eq_self(f: $float) {
                check_eq(f, f, 0);
                check_eq(f, f, 1);
                check_eq(f, f, MAX_ULPS);
            }

            // also covers tests over -f
            fn check_eq_zero(f: $float) {
                check_ne(f, 0.0, prev(f).abs().to_bits());
                check_eq(f, 0.0, f.abs().to_bits());
            }

            #[test]
            fn zero() {
                check_eq_self(0.0);

                // negative zero is a different representation
                check_eq(0.0, -0.0, 0);
                check_eq(0.0, -0.0, 1);
                check_eq(0.0, -0.0, MAX_ULPS);
            }

            #[test]
            fn subnormals() {
                let min_subnormal: $float = next(0.0);
                let max_subnormal: $float = prev(MIN_NORMAL);

                check_eq_self(min_subnormal);
                check_eq_self(max_subnormal);

                check_eq_zero(min_subnormal);
                check_eq_zero(max_subnormal);

                check_ne(max_subnormal, MIN_NORMAL, 0);
                check_eq(max_subnormal, MIN_NORMAL, 1);

                // ranges of -f to f
                check_ne(min_subnormal, -min_subnormal, MAX_ULPS); // numbers with different signs aren't ever equal
                check_ne(max_subnormal, -max_subnormal, MAX_ULPS);
            }

            #[test]
            fn normals() {
                check_eq_self(MIN_NORMAL);
                check_eq_self(MAX_NORMAL);

                check_eq_zero(MIN_NORMAL);
                check_eq_zero(MAX_NORMAL);

                // below MIN_NORMAL is in subnormal tests
                check_ne(MIN_NORMAL, next(MIN_NORMAL), 0);
                check_eq(MIN_NORMAL, next(MIN_NORMAL), 1);

                check_ne(MIN_NORMAL, next_n(MIN_NORMAL, 2), 1);
                check_eq(MIN_NORMAL, next_n(MIN_NORMAL, 2), 2);

                check_ne(
                    MIN_NORMAL,
                    MAX_NORMAL,
                    MAX_NORMAL.to_bits() - MIN_NORMAL.to_bits() - 1,
                );
                check_eq(
                    MIN_NORMAL,
                    MAX_NORMAL,
                    MAX_NORMAL.to_bits() - MIN_NORMAL.to_bits(),
                );
                // above MAX_NORMAL is in infinities tests

                // ranges of -f to f
                check_ne(MIN_NORMAL, -MIN_NORMAL, MAX_ULPS); // numbers with different signs aren't ever equal
                check_ne(MAX_NORMAL, -MAX_NORMAL, MAX_ULPS);
            }

            #[test]
            fn one() {
                check_eq_self(1.0);
                check_eq_zero(1.0);

                check_ne(1.0, prev_n(1.0, 2), 1);
                check_eq(1.0, prev_n(1.0, 2), 2);

                check_ne(1.0, prev(1.0), 0);
                check_eq(1.0, prev(1.0), 1);

                check_ne(1.0, next(1.0), 0);
                check_eq(1.0, next(1.0), 1);

                check_ne(1.0, next_n(1.0, 2), 1);
                check_eq(1.0, next_n(1.0, 2), 2);

                // ranges of -f to f
                check_ne(1.0, -1.0, MAX_ULPS); // numbers with different signs aren't ever equal
            }

            #[test]
            fn two() {
                check_eq_self(2.0);
                check_eq_zero(2.0);

                check_ne(2.0, prev_n(2.0, 2), 1);
                check_eq(2.0, prev_n(2.0, 2), 2);

                check_ne(2.0, prev(2.0), 0);
                check_eq(2.0, prev(2.0), 1);

                check_ne(2.0, next(2.0), 0);
                check_eq(2.0, next(2.0), 1);

                check_ne(2.0, next_n(2.0, 2), 1);
                check_eq(2.0, next_n(2.0, 2), 2);

                // ranges of -f to f
                check_ne(2.0, -2.0, MAX_ULPS); // numbers with different signs aren't ever equal
            }

            #[test]
            fn infinities() {
                check_eq_self(INFINITY);
                check_eq_zero(INFINITY);

                check_ne(INFINITY, MAX_NORMAL, 0);
                check_eq(INFINITY, MAX_NORMAL, 1);

                // ranges of -f to f
                check_ne(INFINITY, -INFINITY, MAX_ULPS); // numbers with different signs aren't ever equal
            }

            #[test]
            fn nans() {
                let nans = nan_test_values();
                for &a in &nans {
                    check_ne_ulps(a, a, MAX_ULPS);

                    check_ne_ulps(1.0, a, MAX_ULPS);
                    check_ne_ulps(a, 1.0, MAX_ULPS);

                    for &b in &nans {
                        check_ne_ulps(a, b, MAX_ULPS);
                    }
                }
            }
        }
    };
}

impl_tests!(f32, u32);
impl_tests!(f64, u64);
