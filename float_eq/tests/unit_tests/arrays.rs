#![allow(clippy::float_cmp, clippy::reversed_empty_ranges)]

use float_eq::{assert_float_eq, assert_float_ne, AssertFloatEq, AssertFloatEqAll};

macro_rules! impl_tests {
    ($float:ident) => {
        mod $float {
            use super::*;
            use crate::$float::*;

            macro_rules! check_debug_float_diff {
                ($n:literal) => {{
                    let mut a: [$float; $n] = [0.; $n];
                    for i in 0..$n {
                        a[i] = (i as $float + 1.);
                    }

                    let mut b = [0.; $n];
                    for i in 0..$n {
                        b[i] = a[i] + 0.5;
                    }

                    // test diffs calculated correctly
                    let abs_diff = a.debug_abs_diff(&b);
                    let ulps_diff = a.debug_ulps_diff(&b);
                    for i in 0..$n {
                        assert_eq!(abs_diff[i], a[i].debug_abs_diff(&b[i]));
                        assert_eq!(ulps_diff[i], a[i].debug_ulps_diff(&b[i]));
                    }

                    // test None is used judiciously
                    for i in 0..$n {
                        let mut b = [0.; $n];
                        b[i] = -a[i];
                        let diff = a.debug_ulps_diff(&b);
                        for j in 0..$n {
                            if i == j {
                                assert!(diff[j].is_none());
                            } else {
                                assert!(diff[j].is_some());
                            }
                        }
                    }
                }};
            }

            #[test]

            fn debug_float_diff() {
                check_debug_float_diff!(0);
                check_debug_float_diff!(1);
                check_debug_float_diff!(2);
                //we can infer the checks in between work
                check_debug_float_diff!(32);

                // nested
                let a = [[1_f32, 2.], [1., 2.]];
                let b = [[1_f32, 2.], [-1., -2.]];
                assert_eq!(a.debug_abs_diff(&b), [[0.0, 0.0], [2., 4.]]);
            }

            macro_rules! check_float_eq {
                ($n:literal) => {{
                    let mut a: [$float; $n] = [0.; $n];
                    for i in 0..$n {
                        a[i] = (i as $float + 1.);
                    }

                    // test equality with self
                    assert_float_eq!(a, a, abs <= [0.0; $n]);
                    assert_float_eq!(a, a, rel <= [0.0; $n]);
                    assert_float_eq!(a, a, rmax <= [0.0; $n]);
                    assert_float_eq!(a, a, rmin <= [0.0; $n]);
                    assert_float_eq!(a, a, r1st <= [0.0; $n]);
                    assert_float_eq!(a, a, r2nd <= [0.0; $n]);
                    assert_float_eq!(a, a, ulps <= [0; $n]);

                    // test different types of equality, by changing one index
                    // at a time.
                    for i in 0..$n {
                        let mut b = a;
                        b[i] = a[i] + 0.5;

                        let mut eps = [prev(0.5); $n];
                        assert_float_ne!(a, b, abs <= eps);
                        eps[i] = 0.5;
                        assert_float_eq!(a, b, abs <= eps);

                        let mut b = a;
                        b[i] = prev_n(a[i], 2);

                        let mut eps = [0.5 * EPSILON; $n];
                        assert_float_ne!(a, b, rel <= eps);
                        eps[i] = 2.0 * EPSILON;
                        assert_float_eq!(a, b, rel <= eps);

                        let mut eps = [0.5 * EPSILON; $n];
                        assert_float_ne!(a, b, rmax <= eps);
                        eps[i] = 2.0 * EPSILON;
                        assert_float_eq!(a, b, rmax <= eps);

                        let mut eps = [EPSILON; $n];
                        assert_float_ne!(a, b, rmin <= eps);
                        eps[i] = 2.0 * EPSILON;
                        assert_float_eq!(a, b, rmin <= eps);

                        let mut eps = [0.5 * EPSILON; $n];
                        assert_float_ne!(a, b, r1st <= eps);
                        eps[i] = 2.0 * EPSILON;
                        assert_float_eq!(a, b, r1st <= eps);

                        let mut eps = [EPSILON; $n];
                        assert_float_ne!(a, b, r2nd <= eps);
                        eps[i] = 2.0 * EPSILON;
                        assert_float_eq!(a, b, r2nd <= eps);

                        let mut eps = [1; $n];
                        assert_float_ne!(a, b, ulps <= eps);
                        eps[i] = 2;
                        assert_float_eq!(a, b, ulps <= eps);
                    }
                }};
            }

            #[test]

            fn float_eq() {
                check_float_eq!(0);
                check_float_eq!(1);
                check_float_eq!(2);
                //we can infer the checks in between work
                check_float_eq!(32);

                // nested
                let a = [[1_f32, 2.], [1., -2.]];
                let b = [[1_f32, 3.], [-1., 2.]];
                assert_float_eq!(a, b, abs <= [[0.0, 1.], [2., 4.]]);
            }

            macro_rules! check_float_eq_all {
                ($n:literal) => {{
                    let mut a: [$float; $n] = [0.; $n];
                    for i in 0..$n {
                        a[i] = (i as $float + 1.);
                    }

                    // test equality with self
                    assert_float_eq!(a, a, abs_all <= 0.0);
                    assert_float_eq!(a, a, rel_all <= 0.0);
                    assert_float_eq!(a, a, rmax_all <= 0.0);
                    assert_float_eq!(a, a, rmin_all <= 0.0);
                    assert_float_eq!(a, a, r1st_all <= 0.0);
                    assert_float_eq!(a, a, r2nd_all <= 0.0);
                    assert_float_eq!(a, a, ulps_all <= 0);

                    // test different types of equality, by changing one index
                    // at a time.
                    for i in 0..$n {
                        let mut b = a;
                        b[i] = a[i] + 0.5;

                        assert_float_ne!(a, b, abs_all <= prev(0.5));
                        assert_float_eq!(a, b, abs_all <= 0.5);

                        let mut b = a;
                        b[i] = prev_n(a[i], 2);

                        assert_float_ne!(a, b, rel_all <= 0.5 * EPSILON);
                        assert_float_eq!(a, b, rel_all <= 2.0 * EPSILON);

                        assert_float_ne!(a, b, rmax_all <= 0.5 * EPSILON);
                        assert_float_eq!(a, b, rmax_all <= 2.0 * EPSILON);

                        assert_float_ne!(a, b, rmin_all <= 1.0 * EPSILON);
                        assert_float_eq!(a, b, rmin_all <= 2.0 * EPSILON);

                        assert_float_ne!(a, b, r1st_all <= 0.5 * EPSILON);
                        assert_float_eq!(a, b, r1st_all <= 2.0 * EPSILON);

                        assert_float_ne!(a, b, r2nd_all <= 1.0 * EPSILON);
                        assert_float_eq!(a, b, r2nd_all <= 2.0 * EPSILON);

                        assert_float_ne!(a, b, ulps_all <= 1);
                        assert_float_eq!(a, b, ulps_all <= 2);
                    }
                }};
            }

            #[test]

            fn float_eq_all() {
                check_float_eq_all!(0);
                check_float_eq_all!(1);
                check_float_eq_all!(2);
                //we can infer the checks in between work
                check_float_eq_all!(32);

                // nested
                let a = [[1_f32, 2.], [1., -2.]];
                let b = [[1_f32, 3.], [-1., 2.]];
                assert_float_eq!(a, b, abs_all <= 4.);
            }

            macro_rules! check_debug_tol {
                ($n:literal) => {{
                    let mut a: [$float; $n] = [0.; $n];
                    for i in 0..$n {
                        a[i] = (i as $float + 1.);
                    }

                    assert_eq!(a.debug_abs_tol(&a, &[0.0; $n]), [0.0; $n]);
                    assert_eq!(a.debug_rel_tol(&a, &[0.0; $n]), [0.0; $n]);
                    assert_eq!(a.debug_rmax_tol(&a, &[0.0; $n]), [0.0; $n]);
                    assert_eq!(a.debug_rmin_tol(&a, &[0.0; $n]), [0.0; $n]);
                    assert_eq!(a.debug_r1st_tol(&a, &[0.0; $n]), [0.0; $n]);
                    assert_eq!(a.debug_r2nd_tol(&a, &[0.0; $n]), [0.0; $n]);
                    assert_eq!(a.debug_ulps_tol(&a, &[0; $n]), [0; $n]);

                    for i in 0..$n {
                        let mut b = a;
                        b[i] = a[i] + 0.5;

                        let mut eps = [0.0; $n];
                        eps[i] = 0.5;
                        assert_eq!(a.debug_abs_tol(&b, &eps), eps);

                        let mut b = a;
                        b[i] = $float::from_bits(a[i].to_bits() + 1);

                        let mut eps = [0.0; $n];
                        eps[i] = EPSILON;
                        let mut expected = [0.0; $n];
                        expected[i] = a[i].debug_rel_tol(&b[i], &eps[i]);
                        assert_eq!(a.debug_rel_tol(&b, &eps), expected);

                        let mut eps = [0.0; $n];
                        eps[i] = EPSILON;
                        let mut expected = [0.0; $n];
                        expected[i] = a[i].debug_rmax_tol(&b[i], &eps[i]);
                        assert_eq!(a.debug_rmax_tol(&b, &eps), expected);

                        let mut eps = [0.0; $n];
                        eps[i] = EPSILON;
                        let mut expected = [0.0; $n];
                        expected[i] = a[i].debug_rmin_tol(&b[i], &eps[i]);
                        assert_eq!(a.debug_rmin_tol(&b, &eps), expected);

                        let mut eps = [0.0; $n];
                        eps[i] = EPSILON;
                        let mut expected = [0.0; $n];
                        expected[i] = a[i].debug_r1st_tol(&b[i], &eps[i]);
                        assert_eq!(a.debug_r1st_tol(&b, &eps), expected);

                        let mut eps = [0.0; $n];
                        eps[i] = EPSILON;
                        let mut expected = [0.0; $n];
                        expected[i] = a[i].debug_r2nd_tol(&b[i], &eps[i]);
                        assert_eq!(a.debug_r2nd_tol(&b, &eps), expected);

                        let mut eps = [0; $n];
                        eps[i] = 1;
                        assert_eq!(a.debug_ulps_tol(&b, &eps), eps);
                    }
                }};
            }

            #[test]
            fn debug_tol() {
                check_debug_tol!(0);
                check_debug_tol!(1);
                check_debug_tol!(2);
                //we can infer the checks in between work
                check_debug_tol!(32);
            }

            macro_rules! check_debug_all_tol {
                ($n:literal) => {{
                    let mut a: [$float; $n] = [0.; $n];
                    for i in 0..$n {
                        a[i] = (i as $float + 1.);
                    }

                    assert_eq!(a.debug_abs_all_tol(&a, &0.0), [0.0; $n]);
                    assert_eq!(a.debug_rel_all_tol(&a, &0.0), [0.0; $n]);
                    assert_eq!(a.debug_rmax_all_tol(&a, &0.0), [0.0; $n]);
                    assert_eq!(a.debug_rmin_all_tol(&a, &0.0), [0.0; $n]);
                    assert_eq!(a.debug_r1st_all_tol(&a, &0.0), [0.0; $n]);
                    assert_eq!(a.debug_r2nd_all_tol(&a, &0.0), [0.0; $n]);
                    assert_eq!(a.debug_ulps_all_tol(&a, &0), [0; $n]);

                    for i in 0..$n {
                        let mut b = a;
                        b[i] = a[i] + 0.5;

                        let eps = 0.5;
                        assert_eq!(a.debug_abs_all_tol(&b, &eps), [eps; $n]);

                        let mut b = a;
                        b[i] = $float::from_bits(a[i].to_bits() + 1);

                        let eps = EPSILON;
                        let mut expected = [0.0; $n];
                        for j in 0..$n {
                            expected[j] = a[j].debug_rel_all_tol(&b[j], &eps);
                        }
                        assert_eq!(a.debug_rel_all_tol(&b, &eps), expected);

                        let eps = EPSILON;
                        let mut expected = [0.0; $n];
                        for j in 0..$n {
                            expected[j] = a[j].debug_rmax_all_tol(&b[j], &eps);
                        }
                        assert_eq!(a.debug_rmax_all_tol(&b, &eps), expected);

                        let eps = EPSILON;
                        let mut expected = [0.0; $n];
                        for j in 0..$n {
                            expected[j] = a[j].debug_rmin_all_tol(&b[j], &eps);
                        }
                        assert_eq!(a.debug_rmin_all_tol(&b, &eps), expected);

                        let eps = EPSILON;
                        let mut expected = [0.0; $n];
                        for j in 0..$n {
                            expected[j] = a[j].debug_r1st_all_tol(&b[j], &eps);
                        }
                        assert_eq!(a.debug_r1st_all_tol(&b, &eps), expected);

                        let eps = EPSILON;
                        let mut expected = [0.0; $n];
                        for j in 0..$n {
                            expected[j] = a[j].debug_r2nd_all_tol(&b[j], &eps);
                        }
                        assert_eq!(a.debug_r2nd_all_tol(&b, &eps), expected);

                        let eps = 1;
                        assert_eq!(a.debug_ulps_all_tol(&b, &eps), [eps; $n]);
                    }
                }};
            }

            #[test]

            fn debug_all_tol() {
                check_debug_all_tol!(0);
                check_debug_all_tol!(1);
                check_debug_all_tol!(2);
                //we can infer the checks in between work
                check_debug_all_tol!(32);
            }
        }
    };
}

impl_tests!(f32);
impl_tests!(f64);

#[test]
#[should_panic(expected = r#"`float_eq!(left, right, abs <= t, rel <= t, ulps <= t)`
        left: `[1.0, 2.0]`,
       right: `[3.0, -5.0]`,
    abs_diff: `[2.0, 7.0]`,
   ulps_diff: `[Some(6755399441055744), None]`,
     [abs] t: `[0.1, 0.25]`,
     [rel] t: `[0.30000000000000004, 1.25]`,
    [ulps] t: `[1, 2]`"#)]
fn assert_fail_message() {
    assert_float_eq!(
        [1., 2.],
        [3., -5.],
        abs <= [0.1, 0.25],
        rel <= [0.1, 0.25],
        ulps <= [1u64, 2]
    );
}

#[test]
#[should_panic(
    expected = r#"`float_eq!(left, right, abs_all <= t, rel_all <= t, ulps_all <= t)`
        left: `[1.0, 2.0]`,
       right: `[3.0, -5.0]`,
    abs_diff: `[2.0, 7.0]`,
   ulps_diff: `[Some(6755399441055744), None]`,
 [abs_all] t: `[0.25, 0.25]`,
 [rel_all] t: `[0.75, 1.25]`,
[ulps_all] t: `[3, 3]"#
)]
fn assert_fail_all_message() {
    assert_float_eq!(
        [1., 2.],
        [3., -5.],
        abs_all <= 0.25,
        rel_all <= 0.25,
        ulps_all <= 3u64
    );
}
