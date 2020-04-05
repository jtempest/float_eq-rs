use float_eq::FloatDiff;

macro_rules! impl_tests {
    ($float:ident) => {
        mod $float {
            use super::*;

            #[test]
            fn abs_diff() {
                let check =
                    |a: f32, b, expected| assert!(a.abs_diff(&b) - expected <= std::f32::EPSILON);

                check(1., 1., 0.);
                check(1., 1.5, 0.5);
                check(1., -1., 2.);
            }

            #[test]
            fn ulps_diff() {
                let check = |a: f32, b, expected| assert_eq!(a.ulps_diff(&b), expected);

                let one: f32 = 1.;
                check(one, one, 0);

                let next = f32::from_bits(one.to_bits() + 1);
                check(one, next, 1);
                check(next, one, 1);

                let prev = f32::from_bits(one.to_bits() - 1);
                check(one, prev, 1);
                check(prev, one, 1);
                check(next, prev, 2);
                check(prev, next, 2);
            }
        }
    };
}

impl_tests!(f32);
impl_tests!(f64);
