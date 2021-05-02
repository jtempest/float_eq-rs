/// Systematic tests of eq_abs/eq_abs_all behaviour over primitives.

macro_rules! impl_tests {
    ($float:ident) => {
        mod $float {
            use crate::$float::*;
            use float_eq::{
                assert_float_eq, assert_float_ne, float_eq, float_ne, FloatEq, FloatEqAll,
            };

            fn check_eq_abs(a: $float, b: $float, tol: $float) {
                assert_float_eq!(a, b, abs <= tol);
                assert_float_eq!(a, b, abs_all <= tol);

                assert!(float_eq!(a, b, abs <= tol));
                assert!(float_eq!(a, b, abs_all <= tol));
                assert!(!float_ne!(a, b, abs <= tol));
                assert!(!float_ne!(a, b, abs_all <= tol));

                assert!(a.eq_abs(&b, &tol));
                assert!(a.eq_abs_all(&b, &tol));
                assert!(!a.ne_abs(&b, &tol));
                assert!(!a.ne_abs_all(&b, &tol));
            }

            fn check_ne_abs(a: $float, b: $float, tol: $float) {
                assert_float_ne!(a, b, abs <= tol);
                assert_float_ne!(a, b, abs_all <= tol);

                assert!(!float_eq!(a, b, abs <= tol));
                assert!(!float_eq!(a, b, abs_all <= tol));
                assert!(float_ne!(a, b, abs <= tol));
                assert!(float_ne!(a, b, abs_all <= tol));

                assert!(!a.eq_abs(&b, &tol));
                assert!(!a.eq_abs_all(&b, &tol));
                assert!(a.ne_abs(&b, &tol));
                assert!(a.ne_abs_all(&b, &tol));
            }

            // also covers commutativity and negative values
            fn check_eq(a: $float, b: $float, tol: $float) {
                check_eq_abs(a, b, tol);
                check_eq_abs(b, a, tol);
                check_eq_abs(-a, -b, tol);
                check_eq_abs(-b, -a, tol);
            }

            // also covers commutativity and negative values
            fn check_ne(a: $float, b: $float, tol: $float) {
                check_ne_abs(a, b, tol);
                check_ne_abs(b, a, tol);
                check_ne_abs(-a, -b, tol);
                check_ne_abs(-b, -a, tol);
            }

            // also covers tests over -f
            fn check_eq_self(f: $float) {
                check_eq(f, f, 0.0);
                check_eq(f, f, next(0.0));
                check_eq(f, f, prev(MIN_NORMAL));
                check_eq(f, f, MIN_NORMAL);
                check_eq(f, f, MAX_NORMAL);
                check_eq(f, f, INFINITY);
            }

            // also covers tests over -f
            fn check_eq_zero(f: $float) {
                check_ne(f, 0.0, prev(f));
                check_eq(f, 0.0, f);
            }

            #[test]
            fn zero() {
                check_eq_self(0.0);

                // negative zero is a different representation
                check_eq(0.0, -0.0, next(0.0));
                check_eq(0.0, -0.0, prev(MIN_NORMAL));
                check_eq(0.0, -0.0, MIN_NORMAL);
                check_eq(0.0, -0.0, MAX_NORMAL);
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

                check_ne(max_subnormal, MIN_NORMAL, 0.0);
                check_eq(max_subnormal, MIN_NORMAL, min_subnormal); // due to linear spacing

                // ranges of -f to f
                check_ne(min_subnormal, -min_subnormal, prev(2.0 * min_subnormal));
                check_eq(min_subnormal, -min_subnormal, 2.0 * min_subnormal);
                check_ne(max_subnormal, -max_subnormal, prev(2.0 * max_subnormal));
                check_eq(max_subnormal, -max_subnormal, 2.0 * max_subnormal);
            }

            #[test]
            fn normals() {
                check_eq_self(MIN_NORMAL);
                check_eq_self(MAX_NORMAL);

                check_eq_zero(MIN_NORMAL);
                check_eq_zero(MAX_NORMAL);

                // below MIN_NORMAL is in subnormal tests
                check_ne(MIN_NORMAL, next(MIN_NORMAL), 0.0);
                check_eq(MIN_NORMAL, next(MIN_NORMAL), next(0.0));

                check_ne(MIN_NORMAL, next_n(MIN_NORMAL, 2), next(0.0));
                check_eq(MIN_NORMAL, next_n(MIN_NORMAL, 2), next_n(0.0, 2));

                check_ne(MIN_NORMAL, MAX_NORMAL, prev(MAX_NORMAL)); // due to of loss of precision
                check_eq(MIN_NORMAL, MAX_NORMAL, MAX_NORMAL);
                // above MAX_NORMAL is in infinities tests

                // ranges of -f to f
                check_ne(MIN_NORMAL, -MIN_NORMAL, prev(2.0 * MIN_NORMAL));
                check_eq(MIN_NORMAL, -MIN_NORMAL, 2.0 * MIN_NORMAL);
                check_ne(MAX_NORMAL, -MAX_NORMAL, MAX_NORMAL);
                check_eq(MAX_NORMAL, -MAX_NORMAL, INFINITY);
            }

            #[test]
            fn one() {
                check_eq_zero(1.0);

                // range of -2 to +2 ULPs
                check_eq(1.0, prev_n(1.0, 2), 1.0 * EPSILON);
                check_ne(1.0, prev_n(1.0, 2), 0.5 * EPSILON);

                check_eq(1.0, prev(1.0), 0.5 * EPSILON);
                check_ne(1.0, prev(1.0), 0.25 * EPSILON);

                check_eq_self(1.0);

                check_ne(1.0, next(1.0), 0.5 * EPSILON);
                check_eq(1.0, next(1.0), 1.0 * EPSILON);

                check_ne(1.0, next_n(1.0, 2), 1.0 * EPSILON);
                check_eq(1.0, next_n(1.0, 2), 2.0 * EPSILON);

                // ranges of -f to f
                check_ne(1.0, -1.0, prev(2.0));
                check_eq(1.0, -1.0, 2.0);
            }

            #[test]
            fn two() {
                check_eq_zero(2.0);

                // range of -2 to +2 ULPs
                check_eq(2.0, prev_n(2.0, 2), 2.0 * EPSILON);
                check_ne(2.0, prev_n(2.0, 2), 1.0 * EPSILON);

                check_eq(2.0, prev(2.0), 1.0 * EPSILON);
                check_ne(2.0, prev(2.0), 0.5 * EPSILON);

                check_eq_self(2.0);

                check_ne(2.0, next(2.0), 1.0 * EPSILON);
                check_eq(2.0, next(2.0), 2.0 * EPSILON);

                check_ne(2.0, next_n(2.0, 2), 2.0 * EPSILON);
                check_eq(2.0, next_n(2.0, 2), 4.0 * EPSILON);

                // ranges of -f to f
                check_ne(2.0, -2.0, prev(4.0));
                check_eq(2.0, -2.0, 4.0);
            }
            #[test]
            fn infinities() {
                check_eq_self(INFINITY);
                check_eq_zero(INFINITY);

                check_ne(INFINITY, MAX_NORMAL, MAX_NORMAL);
                check_eq(INFINITY, MAX_NORMAL, INFINITY);

                // ranges of -f to f
                check_ne(INFINITY, -INFINITY, MAX_NORMAL);
                check_eq(INFINITY, -INFINITY, INFINITY);
            }

            #[test]
            fn nans() {
                let nans = nan_test_values();
                for &a in &nans {
                    check_ne_abs(a, a, 0.0);

                    check_ne_abs(1.0, a, 1.0);
                    check_ne_abs(a, 1.0, 1.0);

                    for &b in &nans {
                        check_ne_abs(a, b, EPSILON);
                    }
                }
            }
        }
    };
}

impl_tests!(f32);
impl_tests!(f64);
