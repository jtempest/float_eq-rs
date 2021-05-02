/// Systematic tests of rmin/rmin_all behaviour over primitives. Also tests the
/// relevant combinations of r1st/r1st_all and r2nd/r2nd_all.

macro_rules! impl_tests {
    ($float:ident) => {
        mod $float {
            use crate::$float::*;
            use float_eq::{
                assert_float_eq, assert_float_ne, float_eq, float_ne, FloatEq, FloatEqAll,
            };

            fn check_eq_rmin(a: $float, b: $float, tol: $float) {
                assert_float_eq!(a, b, rmin <= tol);
                assert_float_eq!(a, b, rmin_all <= tol);

                assert!(float_eq!(a, b, rmin <= tol));
                assert!(float_eq!(a, b, rmin_all <= tol));
                assert!(!float_ne!(a, b, rmin <= tol));
                assert!(!float_ne!(a, b, rmin_all <= tol));

                assert!(a.eq_rmin(&b, &tol));
                assert!(a.eq_rmin_all(&b, &tol));
                assert!(!a.ne_rmin(&b, &tol));
                assert!(!a.ne_rmin_all(&b, &tol));
            }

            fn check_ne_rmin(a: $float, b: $float, tol: $float) {
                assert_float_ne!(a, b, rmin <= tol);
                assert_float_ne!(a, b, rmin_all <= tol);

                assert!(!float_eq!(a, b, rmin <= tol));
                assert!(!float_eq!(a, b, rmin_all <= tol));
                assert!(float_ne!(a, b, rmin <= tol));
                assert!(float_ne!(a, b, rmin_all <= tol));

                assert!(!a.eq_rmin(&b, &tol));
                assert!(!a.eq_rmin_all(&b, &tol));
                assert!(a.ne_rmin(&b, &tol));
                assert!(a.ne_rmin_all(&b, &tol));
            }

            fn check_eq_r1st(a: $float, b: $float, tol: $float) {
                assert_float_eq!(a, b, r1st <= tol);
                assert_float_eq!(a, b, r1st_all <= tol);

                assert!(float_eq!(a, b, r1st <= tol));
                assert!(float_eq!(a, b, r1st_all <= tol));
                assert!(!float_ne!(a, b, r1st <= tol));
                assert!(!float_ne!(a, b, r1st_all <= tol));

                assert!(a.eq_r1st(&b, &tol));
                assert!(a.eq_r1st_all(&b, &tol));
                assert!(!a.ne_r1st(&b, &tol));
                assert!(!a.ne_r1st_all(&b, &tol));
            }

            fn check_ne_r1st(a: $float, b: $float, tol: $float) {
                assert_float_ne!(a, b, r1st <= tol);
                assert_float_ne!(a, b, r1st_all <= tol);

                assert!(!float_eq!(a, b, r1st <= tol));
                assert!(!float_eq!(a, b, r1st_all <= tol));
                assert!(float_ne!(a, b, r1st <= tol));
                assert!(float_ne!(a, b, r1st_all <= tol));

                assert!(!a.eq_r1st(&b, &tol));
                assert!(!a.eq_r1st_all(&b, &tol));
                assert!(a.ne_r1st(&b, &tol));
                assert!(a.ne_r1st_all(&b, &tol));
            }

            fn check_eq_r2nd(a: $float, b: $float, tol: $float) {
                assert_float_eq!(a, b, r2nd <= tol);
                assert_float_eq!(a, b, r2nd_all <= tol);

                assert!(float_eq!(a, b, r2nd <= tol));
                assert!(float_eq!(a, b, r2nd_all <= tol));
                assert!(!float_ne!(a, b, r2nd <= tol));
                assert!(!float_ne!(a, b, r2nd_all <= tol));

                assert!(a.eq_r2nd(&b, &tol));
                assert!(a.eq_r2nd_all(&b, &tol));
                assert!(!a.ne_r2nd(&b, &tol));
                assert!(!a.ne_r2nd_all(&b, &tol));
            }

            fn check_ne_r2nd(a: $float, b: $float, tol: $float) {
                assert_float_ne!(a, b, r2nd <= tol);
                assert_float_ne!(a, b, r2nd_all <= tol);

                assert!(!float_eq!(a, b, r2nd <= tol));
                assert!(!float_eq!(a, b, r2nd_all <= tol));
                assert!(float_ne!(a, b, r2nd <= tol));
                assert!(float_ne!(a, b, r2nd_all <= tol));

                assert!(!a.eq_r2nd(&b, &tol));
                assert!(!a.eq_r2nd_all(&b, &tol));
                assert!(a.ne_r2nd(&b, &tol));
                assert!(a.ne_r2nd_all(&b, &tol));
            }

            // also covers commutativity and negative values
            fn check_eq(a: $float, b: $float, tol: $float) {
                check_eq_rmin(a, b, tol);
                check_eq_rmin(b, a, tol);
                check_eq_rmin(-a, -b, tol);
                check_eq_rmin(-b, -a, tol);

                if $float::abs(a) <= $float::abs(b) {
                    check_eq_r1st(a, b, tol);
                    check_eq_r2nd(b, a, tol);
                } else {
                    check_eq_r1st(b, a, tol);
                    check_eq_r2nd(a, b, tol);
                }

                if $float::abs(-a) <= $float::abs(-b) {
                    check_eq_r1st(-a, -b, tol);
                    check_eq_r2nd(-b, -a, tol);
                } else {
                    check_eq_r1st(-b, -a, tol);
                    check_eq_r2nd(-a, -b, tol);
                }
            }

            // also covers commutativity and negative values
            fn check_ne(a: $float, b: $float, tol: $float) {
                check_ne_rmin(a, b, tol);
                check_ne_rmin(b, a, tol);
                check_ne_rmin(-a, -b, tol);
                check_ne_rmin(-b, -a, tol);

                if $float::abs(a) <= $float::abs(b) {
                    check_ne_r1st(a, b, tol);
                    check_ne_r2nd(b, a, tol);
                } else {
                    check_ne_r1st(b, a, tol);
                    check_ne_r2nd(a, b, tol);
                }

                if $float::abs(-a) <= $float::abs(-b) {
                    check_ne_r1st(-a, -b, tol);
                    check_ne_r2nd(-b, -a, tol);
                } else {
                    check_ne_r1st(-b, -a, tol);
                    check_ne_r2nd(-a, -b, tol);
                }
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
                // note: rmin can never be equal to zero unless it *is* zero
                check_ne(f, 0.0, INFINITY);
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

                check_ne(max_subnormal, MIN_NORMAL, 0.5 * EPSILON); // due to lack of precision in tolerance calculation
                check_eq(max_subnormal, MIN_NORMAL, EPSILON);

                // ranges of -f to f
                check_ne(min_subnormal, -min_subnormal, prev(1.5)); // due to lack of precision in tolerance calculation
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
                check_ne(MIN_NORMAL, next(MIN_NORMAL), 0.5 * EPSILON);
                check_eq(MIN_NORMAL, next(MIN_NORMAL), 1.0 * EPSILON);

                check_ne(MIN_NORMAL, next_n(MIN_NORMAL, 2), 1.0 * EPSILON);
                check_eq(MIN_NORMAL, next_n(MIN_NORMAL, 2), 2.0 * EPSILON);

                check_ne(MIN_NORMAL, MAX_NORMAL, MAX_NORMAL);
                check_eq(MIN_NORMAL, MAX_NORMAL, INFINITY);
                // above MAX_NORMAL is in infinities tests

                // ranges of -f to f
                check_ne(MIN_NORMAL, -MIN_NORMAL, prev(2.0));
                check_eq(MIN_NORMAL, -MIN_NORMAL, 2.0);
                check_ne(MAX_NORMAL, -MAX_NORMAL, 1.0);
                check_eq(MAX_NORMAL, -MAX_NORMAL, next(1.0)); // anything above 1.0 * MAX is INFINITY
            }

            #[test]
            fn one() {
                check_eq_zero(1.0);

                // range of -2 to +2 ULPs
                check_eq(1.0, prev_n(1.0, 2), 2.0 * EPSILON);
                check_ne(1.0, prev_n(1.0, 2), 1.0 * EPSILON);

                check_eq(1.0, prev(1.0), 1.0 * EPSILON);
                check_ne(1.0, prev(1.0), 0.5 * EPSILON);

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

                check_ne(2.0, next(2.0), 0.5 * EPSILON);
                check_eq(2.0, next(2.0), 1.0 * EPSILON);

                check_ne(2.0, next_n(2.0, 2), 1.0 * EPSILON);
                check_eq(2.0, next_n(2.0, 2), 2.0 * EPSILON);

                // ranges of -f to f
                check_ne(2.0, -2.0, prev(2.0));
                check_eq(2.0, -2.0, 2.0);
            }

            #[test]
            fn infinities() {
                check_eq_self(INFINITY);
                check_eq_zero(INFINITY);

                check_ne(INFINITY, MAX_NORMAL, 1.0);
                check_eq(INFINITY, MAX_NORMAL, next(1.0));

                // ranges of -f to f
                check_ne(INFINITY, -INFINITY, 0.0);
                check_eq(INFINITY, -INFINITY, next(0.0));
            }

            #[test]
            fn nans() {
                let nans = nan_test_values();
                for &a in &nans {
                    check_ne_rmin(a, a, 0.0);

                    check_ne_rmin(1.0, a, 1.0);
                    check_ne_rmin(a, 1.0, 1.0);

                    for &b in &nans {
                        check_ne_rmin(a, b, EPSILON);
                    }
                }
            }
        }
    };
}

impl_tests!(f32);
impl_tests!(f64);
