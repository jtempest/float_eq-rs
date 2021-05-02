macro_rules! impl_tests {
    ($float:ident, $uint:ident) => {
        mod $float {
            use crate::$float::*;
            use float_eq::{AssertFloatEq, AssertFloatEqAll};

            #[test]
            fn debug_abs_diff() {
                let check =
                    |a: $float, b, expected| assert!(a.debug_abs_diff(&b) - expected <= EPSILON);

                // zeroes
                check(0.0, 0.0, 0.0);
                check(0.0, -0.0, 0.0);
                check(-0.0, 0.0, 0.0);
                check(-0.0, -0.0, 0.0);

                // self
                check(1.0, 1.0, 0.0);
                check(-1.0, -1.0, 0.0);

                // finite numbers
                check(1.0, 2.0, 1.0);
                check(1.0, -2.0, 3.0);
                check(-1.0, 2.0, 3.0);
                check(-1.0, -2.0, 1.0);

                // infinities
                assert!(INFINITY.debug_abs_diff(&INFINITY).is_nan());
                assert_eq!(INFINITY.debug_abs_diff(&(-INFINITY)), INFINITY);
                assert_eq!((-INFINITY).debug_abs_diff(&(INFINITY)), INFINITY);
                assert!((-INFINITY).debug_abs_diff(&(-INFINITY)).is_nan());

                // nans
                let nans = nan_test_values();
                for a in &nans {
                    assert!(a.debug_abs_diff(&1.0).is_nan());
                    assert!(1.0.debug_abs_diff(a).is_nan());
                    for b in &nans {
                        assert!(a.debug_abs_diff(b).is_nan());
                    }
                }
            }

            #[test]
            fn debug_ulps_diff() {
                let check = |a: $float, b, expected| assert_eq!(a.debug_ulps_diff(&b), expected);

                // zeroes
                check(0.0, 0.0, Some(0));
                check(0.0, -0.0, Some(0));
                check(-0.0, 0.0, Some(0));
                check(-0.0, -0.0, Some(0));

                // self
                check(1.0, 1.0, Some(0));
                check(-1.0, -1.0, Some(0));

                // denormals
                check(next(0.0), next_n(0.0, 10), Some(9));
                check(next(-0.0), next_n(-0.0, 10), Some(9));

                check(next(0.0), next(-0.0), None);
                check(next(-0.0), next(0.0), None);

                // normals
                check(1.0, next_n(1.0, 10), Some(10));
                check(-1.0, next_n(-1.0, 10), Some(10));

                check(1.0, -1.0, None);
                check(-1.0, 1.0, None);

                // infinities
                check(INFINITY, INFINITY, Some(0));
                check(INFINITY, -INFINITY, None);
                check(-INFINITY, INFINITY, None);
                check(-INFINITY, -INFINITY, Some(0));

                // nans
                let nans = nan_test_values();
                for a in &nans {
                    assert!(a.debug_ulps_diff(&1.0).is_none());
                    assert!(1.0.debug_ulps_diff(a).is_none());
                    for b in &nans {
                        assert!(a.debug_ulps_diff(b).is_none());
                    }
                }
            }

            #[test]
            fn debug_tol() {
                let a: $float = 10.0;
                let b: $float = 25.0;

                assert_eq!(a.debug_abs_tol(&b, &3.0), 3.0);
                assert_eq!(b.debug_abs_tol(&a, &3.0), 3.0);

                assert_eq!(a.debug_rel_tol(&b, &0.5), 12.5);
                assert_eq!(b.debug_rel_tol(&a, &0.5), 12.5);

                assert_eq!(a.debug_rmax_tol(&b, &0.5), 12.5);
                assert_eq!(b.debug_rmax_tol(&a, &0.5), 12.5);

                assert_eq!(a.debug_rmin_tol(&b, &0.5), 5.0);
                assert_eq!(b.debug_rmin_tol(&a, &0.5), 5.0);

                assert_eq!(a.debug_r1st_tol(&b, &0.5), 5.0);
                assert_eq!(b.debug_r1st_tol(&a, &0.5), 12.5);

                assert_eq!(a.debug_r2nd_tol(&b, &0.5), 12.5);
                assert_eq!(b.debug_r2nd_tol(&a, &0.5), 5.0);

                assert_eq!(a.debug_ulps_tol(&b, &5), 5);
                assert_eq!(b.debug_ulps_tol(&a, &5), 5);
            }

            #[test]
            fn debug_all_tol() {
                let a: $float = 10.0;
                let b: $float = 25.0;

                assert_eq!(a.debug_abs_all_tol(&b, &3.0), 3.0);
                assert_eq!(b.debug_abs_all_tol(&a, &3.0), 3.0);

                assert_eq!(a.debug_rel_all_tol(&b, &0.5), 12.5);
                assert_eq!(b.debug_rel_all_tol(&a, &0.5), 12.5);

                assert_eq!(a.debug_rmax_all_tol(&b, &0.5), 12.5);
                assert_eq!(b.debug_rmax_all_tol(&a, &0.5), 12.5);

                assert_eq!(a.debug_rmin_all_tol(&b, &0.5), 5.0);
                assert_eq!(b.debug_rmin_all_tol(&a, &0.5), 5.0);

                assert_eq!(a.debug_r1st_all_tol(&b, &0.5), 5.0);
                assert_eq!(b.debug_r1st_all_tol(&a, &0.5), 12.5);

                assert_eq!(a.debug_r2nd_all_tol(&b, &0.5), 12.5);
                assert_eq!(b.debug_r2nd_all_tol(&a, &0.5), 5.0);

                assert_eq!(a.debug_ulps_all_tol(&b, &5), 5);
                assert_eq!(b.debug_ulps_all_tol(&a, &5), 5);
            }
        }
    };
}

impl_tests!(f32, u32);
impl_tests!(f64, u64);
