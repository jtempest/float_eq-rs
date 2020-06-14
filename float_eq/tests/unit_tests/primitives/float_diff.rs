macro_rules! impl_tests {
    ($float:ident, $uint:ident) => {
        mod $float {
            use crate::$float::*;
            use float_eq::FloatDiff;

            #[test]
            fn abs_diff() {
                let check = |a: $float, b, expected| assert!(a.abs_diff(&b) - expected <= EPSILON);

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
                assert!(INFINITY.abs_diff(&INFINITY).is_nan());
                assert_eq!(INFINITY.abs_diff(&(-INFINITY)), INFINITY);
                assert_eq!((-INFINITY).abs_diff(&(INFINITY)), INFINITY);
                assert!((-INFINITY).abs_diff(&(-INFINITY)).is_nan());

                // nans
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

                // zeroes
                check(0.0, 0.0, Some(0));
                check(0.0, -0.0, Some(0));
                check(-0.0, 0.0, Some(0));
                check(-0.0, -0.0, Some(0));

                // self
                check(1.0, 1.0, Some(0));
                check(-1.0, -1.0, Some(0));

                // finite numbers
                check(1.0, next_n(1.0, 10), Some(10));
                check(next(0.0), next(-0.0), None);
                check(next(-0.0), next(0.0), None);
                check(-1.0, next_n(-1.0, 10), Some(10));

                // infinities
                check(INFINITY, INFINITY, Some(0));
                check(INFINITY, -INFINITY, None);
                check(-INFINITY, INFINITY, None);
                check(-INFINITY, -INFINITY, Some(0));

                // nans
                let nans = nan_test_values();
                for a in &nans {
                    assert!(a.ulps_diff(&1.0).is_none());
                    assert!(1.0.ulps_diff(a).is_none());
                    for b in &nans {
                        assert!(a.ulps_diff(b).is_none());
                    }
                }
            }
        }
    };
}

impl_tests!(f32, u32);
impl_tests!(f64, u64);
