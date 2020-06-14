/// Systematic tests of eq_rel/eq_rel_all behaviour over primitives.

macro_rules! impl_tests {
    ($float:ident) => {
        mod $float {
            use crate::$float::*;
            use float_eq::{
                assert_float_eq, assert_float_ne, float_eq, float_ne, FloatEq, FloatEqAll,
            };

            fn check_eq_rel(a: $float, b: $float, max_diff: $float) {
                assert_float_eq!(a, b, rel <= max_diff);
                assert_float_eq!(a, b, rel_all <= max_diff);

                assert!(float_eq!(a, b, rel <= max_diff));
                assert!(float_eq!(a, b, rel_all <= max_diff));
                assert!(!float_ne!(a, b, rel <= max_diff));
                assert!(!float_ne!(a, b, rel_all <= max_diff));

                assert!(a.eq_rel(&b, &max_diff));
                assert!(a.eq_rel_all(&b, &max_diff));
                assert!(!a.ne_rel(&b, &max_diff));
                assert!(!a.ne_rel_all(&b, &max_diff));
            }

            fn check_ne_rel(a: $float, b: $float, max_diff: $float) {
                assert_float_ne!(a, b, rel <= max_diff);
                assert_float_ne!(a, b, rel_all <= max_diff);

                assert!(!float_eq!(a, b, rel <= max_diff));
                assert!(!float_eq!(a, b, rel_all <= max_diff));
                assert!(float_ne!(a, b, rel <= max_diff));
                assert!(float_ne!(a, b, rel_all <= max_diff));

                assert!(!a.eq_rel(&b, &max_diff));
                assert!(!a.eq_rel_all(&b, &max_diff));
                assert!(a.ne_rel(&b, &max_diff));
                assert!(a.ne_rel_all(&b, &max_diff));
            }

            // also covers commutativity and negative values
            fn check_eq(a: $float, b: $float, max_diff: $float) {
                check_eq_rel(a, b, max_diff);
                check_eq_rel(b, a, max_diff);
                check_eq_rel(-a, -b, max_diff);
                check_eq_rel(-b, -a, max_diff);
            }

            // also covers commutativity and negative values
            fn check_ne(a: $float, b: $float, max_diff: $float) {
                check_ne_rel(a, b, max_diff);
                check_ne_rel(b, a, max_diff);
                check_ne_rel(-a, -b, max_diff);
                check_ne_rel(-b, -a, max_diff);
            }

            // also covers tests over -f
            fn check_eq_self(f: $float) {
                check_eq(f, f, 0.0);
                check_eq(f, f, 1.0 / f);
                check_eq(f, f, 1.0);
                check_eq(f, f, INFINITY);
            }

            // also covers tests over -f
            fn check_eq_zero(f: $float) {
                check_ne(f, 0.0, prev(f) / f);
                check_eq(f, 0.0, 1.0);
            }

            #[test]
            fn zero() {
                check_eq_self(0.0);

                // negative zero is a different representation
                check_eq(0.0, -0.0, 0.0);
                check_eq(0.0, -0.0, 1.0);
                check_eq(0.0, -0.0, INFINITY);
            }

            #[test]
            fn subnormals() {
                let min_subnormal: $float = next(0.0);
                let max_subnormal: $float = prev(MIN_NORMAL);

                check_eq_self(min_subnormal);
                check_eq_self(max_subnormal);

                check_eq_zero(min_subnormal);
                check_eq_zero(max_subnormal);

                check_ne(max_subnormal, MIN_NORMAL, 0.5 * EPSILON); // due to lack of precision in epsilon calculation
                check_eq(max_subnormal, MIN_NORMAL, EPSILON);

                // ranges of -f to f
                check_ne(min_subnormal, -min_subnormal, prev(1.5)); // due to lack of precision in epsilon calculation
                check_eq(min_subnormal, -min_subnormal, 2.0);
                check_ne(max_subnormal, -max_subnormal, prev(2.0));
                check_eq(max_subnormal, -max_subnormal, 2.0);
            }

            #[test]
            fn normals() {
                check_eq_self(MIN_NORMAL);
                check_eq_self(MAX_NORMAL);

                check_eq_zero(MIN_NORMAL);
                check_eq_zero(MAX_NORMAL);

                // below MIN_NORMAL is in subnormal tests
                check_ne(MIN_NORMAL, next(MIN_NORMAL), 0.0);
                check_eq(MIN_NORMAL, next(MIN_NORMAL), EPSILON);

                check_ne(MIN_NORMAL, next_n(MIN_NORMAL, 2), EPSILON);
                check_eq(MIN_NORMAL, next_n(MIN_NORMAL, 2), 2.0 * EPSILON);

                check_ne(MIN_NORMAL, MAX_NORMAL, prev(MAX_NORMAL) / MAX_NORMAL);
                check_eq(MIN_NORMAL, MAX_NORMAL, 1.0);
                // above MAX_NORMAL is in infinities tests

                // ranges of -f to f
                check_ne(MIN_NORMAL, -MIN_NORMAL, prev(2.0));
                check_eq(MIN_NORMAL, -MIN_NORMAL, 2.0);
                check_ne(MAX_NORMAL, -MAX_NORMAL, 1.0); // anything above 1.0 * MAX is INFINITY
                check_eq(MAX_NORMAL, -MAX_NORMAL, 2.0);
            }

            #[test]
            fn one() {
                check_eq_self(1.0);
                check_eq_zero(1.0);

                check_ne(1.0, prev_n(1.0, 2), 0.5 * EPSILON); // precision below power of two...
                check_eq(1.0, prev_n(1.0, 2), EPSILON);

                check_ne(1.0, prev(1.0), 0.0);
                check_eq(1.0, prev(1.0), EPSILON);

                check_ne(1.0, next(1.0), 0.0);
                check_eq(1.0, next(1.0), EPSILON);

                check_ne(1.0, next_n(1.0, 2), EPSILON);
                check_eq(1.0, next_n(1.0, 2), 2.0 * EPSILON); // ..is double that of above it

                // ranges of -f to f
                check_ne(1.0, -1.0, prev(2.0));
                check_eq(1.0, -1.0, 2.0);
            }

            #[test]
            fn two() {
                check_eq_self(2.0);
                check_eq_zero(2.0);

                check_ne(2.0, prev_n(2.0, 2), 0.5 * EPSILON); // precision below power of two...
                check_eq(2.0, prev_n(2.0, 2), EPSILON);

                check_ne(2.0, prev(2.0), 0.0);
                check_eq(2.0, prev(2.0), EPSILON);

                check_ne(2.0, next(2.0), 0.0);
                check_eq(2.0, next(2.0), EPSILON);

                check_ne(2.0, next_n(2.0, 2), EPSILON);
                check_eq(2.0, next_n(2.0, 2), 2.0 * EPSILON); // ..is double that of above it

                // ranges of -f to f
                check_ne(2.0, -2.0, prev(2.0));
                check_eq(2.0, -2.0, 2.0);
            }

            #[test]
            fn infinities() {
                check_eq_self(INFINITY);
                check_eq_zero(INFINITY);

                check_ne(INFINITY, MAX_NORMAL, 0.0);
                check_eq(INFINITY, MAX_NORMAL, next(0.0)); // any value * inf => inf

                // ranges of -f to f
                check_ne(INFINITY, -INFINITY, 0.0);
                check_eq(INFINITY, -INFINITY, next(0.0)); // any value * inf => inf
                check_eq(INFINITY, -INFINITY, 1.0);
            }

            #[test]
            fn nans() {
                let nans = nan_test_values();
                for &a in &nans {
                    check_ne_rel(a, a, 0.0);

                    check_ne_rel(1.0, a, 1.0);
                    check_ne_rel(a, 1.0, 1.0);

                    for &b in &nans {
                        check_ne_rel(a, b, EPSILON);
                    }
                }
            }
        }
    };
}

impl_tests!(f32);
impl_tests!(f64);
