macro_rules! impl_tests {
    ($float:ident, $uint:ident) => {
        mod $float {
            use crate::unit_tests::primitives::$float::*;
            use float_eq::FloatDiff;

            #[test]
            fn abs_diff() {
                let check = |a: $float, b, expected| assert!(a.abs_diff(&b) - expected <= EPSILON);

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

impl_tests!(f32, u32);
impl_tests!(f64, u64);
