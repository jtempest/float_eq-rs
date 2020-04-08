use float_eq::FloatDiff;

macro_rules! impl_tests {
    ($float:ident) => {
        mod $float {
            use super::*;

            #[test]
            fn abs_diff() {
                let check = |a: $float, b, expected| {
                    assert!(a.abs_diff(&b) - expected <= std::$float::EPSILON)
                };

                check(1., 1., 0.);
                check(1., 1.5, 0.5);
                check(1., -1., 2.);

                let nan = std::$float::NAN;
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

            macro_rules! check_array {
                ($n:literal) => {{
                    let mut a: [$float; $n] = [0.; $n];
                    for i in 0..$n {
                        a[i] = (i as $float + 1.);
                    }

                    let mut b = [0.; $n];
                    for i in 0..$n {
                        b[i] = -a[i];
                    }

                    let abs_diff = a.abs_diff(&b);
                    let ulps_diff = a.ulps_diff(&b);
                    for i in 0..$n {
                        assert_eq!(abs_diff[i], a[i].abs_diff(&b[i]));
                        assert_eq!(ulps_diff[i], a[i].ulps_diff(&b[i]));
                    }
                }};
            }

            #[test]
            fn float_diff_array() {
                //TODO: Use const generics once they're stable
                check_array!(0);
                check_array!(1);
                check_array!(2);
                check_array!(3);
                check_array!(4);
                check_array!(5);
                check_array!(6);
                check_array!(7);
                check_array!(8);
                check_array!(9);
                check_array!(10);
                check_array!(11);
                check_array!(12);
                check_array!(13);
                check_array!(14);
                check_array!(15);
                check_array!(16);
                check_array!(17);
                check_array!(18);
                check_array!(19);
                check_array!(20);
                check_array!(21);
                check_array!(22);
                check_array!(23);
                check_array!(24);
                check_array!(25);
                check_array!(26);
                check_array!(27);
                check_array!(28);
                check_array!(29);
                check_array!(30);
                check_array!(31);
                check_array!(32);
            }
        }
    };
}

impl_tests!(f32);
impl_tests!(f64);
