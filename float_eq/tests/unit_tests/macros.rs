use float_eq::{
    assert_float_eq, assert_float_ne, debug_assert_float_eq, debug_assert_float_ne, float_eq,
    float_ne,
};

#[test]
fn trailing_commas() {
    assert!(float_eq!(1.0, 1.5, abs <= 0.5,));
    assert!(float_ne!(1.0, 1.5, abs <= 0.4,));
    assert_float_eq!(1.0, 1.5, abs <= 0.5,);
    assert_float_ne!(1.0, 1.5, abs <= 0.4);
}

mod assert_float_eq {
    use super::*;

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `0.1`"#)]
    fn abs_fail() {
        assert_float_eq!(0_f32, 1., abs <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, abs_all <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
 [abs_all] t: `0.1`"#)]
    fn abs_all_fail() {
        assert_float_eq!(0_f32, 1., abs_all <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, rel <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [rel] t: `0.1`"#)]
    fn rel_fail() {
        assert_float_eq!(0_f32, 1., rel <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, rel_all <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
 [rel_all] t: `0.1`"#)]
    fn rel_all_fail() {
        assert_float_eq!(0_f32, 1., rel_all <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, rmax <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
    [rmax] t: `0.1`"#)]
    fn rmax_fail() {
        assert_float_eq!(0_f32, 1., rmax <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, rmax_all <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
[rmax_all] t: `0.1`"#)]
    fn rmax_all_fail() {
        assert_float_eq!(0_f32, 1., rmax_all <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, rmin <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
    [rmin] t: `0.0`"#)]
    fn rmin_fail() {
        assert_float_eq!(0_f32, 1., rmin <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, rmin_all <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
[rmin_all] t: `0.0`"#)]
    fn rmin_all_fail() {
        assert_float_eq!(0_f32, 1., rmin_all <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, r1st <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
    [r1st] t: `0.0`"#)]
    fn r1st_fail() {
        assert_float_eq!(0_f32, 1., r1st <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, r1st_all <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
[r1st_all] t: `0.0`"#)]
    fn r1st_all_fail() {
        assert_float_eq!(0_f32, 1., r1st_all <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, r2nd <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
    [r2nd] t: `0.1`"#)]
    fn r2nd_fail() {
        assert_float_eq!(0_f32, 1., r2nd <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, r2nd_all <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
[r2nd_all] t: `0.1`"#)]
    fn r2nd_all_fail() {
        assert_float_eq!(0_f32, 1., r2nd_all <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, ulps <= t)`
        left: `1.0`,
       right: `1.0000002`,
    abs_diff: `2.3841858e-7`,
   ulps_diff: `Some(2)`,
    [ulps] t: `1`"#)]
    fn ulps_fail() {
        assert_float_eq!(1_f32, 1.000_000_2, ulps <= 1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, ulps_all <= t)`
        left: `1.0`,
       right: `1.0000002`,
    abs_diff: `2.3841858e-7`,
   ulps_diff: `Some(2)`,
[ulps_all] t: `1`"#)]
    fn ulps_all_fail() {
        assert_float_eq!(1_f32, 1.000_000_2, ulps_all <= 1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `0.1`: testing: 0 != 1"#)]
    fn fail_with_message() {
        assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `0.1`: testing: 0 != 1"#)]
    fn fail_with_message_trailing_comma() {
        assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32,);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, abs <= t, ulps <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `0.1`,
    [ulps] t: `1`: testing: 0 != 1"#)]
    fn chaining_fail_with_message() {
        assert_float_eq!(
            0_f32,
            1.,
            abs <= 0.1,
            ulps <= 1,
            "testing: {} != {}",
            0_f32,
            1_f32
        );
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, abs <= t, ulps <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `0.1`,
    [ulps] t: `1`: testing: 0 != 1"#)]
    fn chaining_fail_with_message_trailing_comma() {
        assert_float_eq!(
            0_f32,
            1.,
            abs <= 0.1,
            ulps <= 1,
            "testing: {} != {}",
            0_f32,
            1_f32,
        );
    }

    #[test]
    fn chaining() {
        // first succeeds
        assert_float_eq!(1_f32, 1.000_000_2, abs <= 0.000_000_3, ulps <= 1);
        assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_3,
            rel <= 0.000_000_1,
            ulps <= 1
        );

        // second succeeds
        assert_float_eq!(1_f32, 1.000_000_2, abs <= 0.000_000_1, ulps <= 2);
        assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_3,
            ulps <= 1
        );

        // third succeeds
        assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_1,
            ulps <= 2
        );
    }

    #[test]
    fn chaining_with_messages() {
        // first succeeds
        assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_3,
            ulps <= 1,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );
        assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_3,
            rel <= 0.000_000_1,
            ulps <= 1,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );

        // second succeeds
        assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            ulps <= 2,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );
        assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_3,
            ulps <= 1,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );

        // third succeeds
        assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_1,
            ulps <= 2,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );
    }
}

mod assert_float_ne {
    use super::*;

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `1.0`"#)]
    fn abs_fail() {
        assert_float_ne!(0_f32, 1., abs <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, abs_all <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
 [abs_all] t: `1.0`"#)]
    fn abs_all_fail() {
        assert_float_ne!(0_f32, 1., abs_all <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, rel <= t)`
        left: `0.0`,
       right: `2.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(1073741824)`,
     [rel] t: `2.0`"#)]
    fn rel_fail() {
        assert_float_ne!(0_f32, 2., rel <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, rel_all <= t)`
        left: `0.0`,
       right: `2.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(1073741824)`,
 [rel_all] t: `2.0`"#)]
    fn rel_all_fail() {
        assert_float_ne!(0_f32, 2., rel_all <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, rmax <= t)`
        left: `0.0`,
       right: `2.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(1073741824)`,
    [rmax] t: `2.0`"#)]
    fn rmax_fail() {
        assert_float_ne!(0_f32, 2., rmax <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, rmax_all <= t)`
        left: `0.0`,
       right: `2.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(1073741824)`,
[rmax_all] t: `2.0`"#)]
    fn rmax_all_fail() {
        assert_float_ne!(0_f32, 2., rmax_all <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, rmin <= t)`
        left: `4.0`,
       right: `2.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(8388608)`,
    [rmin] t: `2.0`"#)]
    fn rmin_fail() {
        assert_float_ne!(4_f32, 2., rmin <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, rmin_all <= t)`
        left: `4.0`,
       right: `2.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(8388608)`,
[rmin_all] t: `2.0`"#)]
    fn rmin_all_fail() {
        assert_float_ne!(4_f32, 2., rmin_all <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, r1st <= t)`
        left: `3.0`,
       right: `1.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(12582912)`,
    [r1st] t: `3.0`"#)]
    fn r1st_fail() {
        assert_float_ne!(3_f32, 1., r1st <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, r1st_all <= t)`
        left: `3.0`,
       right: `1.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(12582912)`,
[r1st_all] t: `3.0`"#)]
    fn r1st_all_fail() {
        assert_float_ne!(3_f32, 1., r1st_all <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, r2nd <= t)`
        left: `0.0`,
       right: `2.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(1073741824)`,
    [r2nd] t: `2.0`"#)]
    fn r2nd_fail() {
        assert_float_ne!(0_f32, 2., r2nd <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, r2nd_all <= t)`
        left: `0.0`,
       right: `2.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(1073741824)`,
[r2nd_all] t: `2.0`"#)]
    fn r2nd_all_fail() {
        assert_float_ne!(0_f32, 2., r2nd_all <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, ulps <= t)`
        left: `1.0`,
       right: `1.0000001`,
    abs_diff: `1.1920929e-7`,
   ulps_diff: `Some(1)`,
    [ulps] t: `1`"#)]
    fn ulps_fail() {
        assert_float_ne!(1_f32, 1.000_000_1, ulps <= 1);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `1.0`: testing: 0 != 1"#)]
    fn fail_with_message() {
        assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `1.0`: testing: 0 != 1"#)]
    fn fail_with_message_trailing_comma() {
        assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32,);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, abs <= t, ulps <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `1.0`,
    [ulps] t: `1`: testing: 0 != 1"#)]
    fn chaining_fail_with_message() {
        assert_float_ne!(
            0_f32,
            1.,
            abs <= 1.,
            ulps <= 1,
            "testing: {} != {}",
            0_f32,
            1_f32
        );
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, abs <= t, ulps <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `1.0`,
    [ulps] t: `1`: testing: 0 != 1"#)]
    fn chaining_fail_with_message_trailing_comma() {
        assert_float_ne!(
            0_f32,
            1.,
            abs <= 1.,
            ulps <= 1,
            "testing: {} != {}",
            0_f32,
            1_f32,
        );
    }

    #[test]
    fn chaining() {
        assert_float_ne!(1_f32, 1.000_000_2, abs <= 0.000_000_1, ulps <= 1);
        assert_float_ne!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_1,
            ulps <= 1
        );
    }

    #[test]
    fn chaining_with_messages() {
        assert_float_ne!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            ulps <= 1,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );
        assert_float_ne!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_1,
            ulps <= 1,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );
    }
}

mod debug_assert_float_eq {
    use super::*;

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `0.1`"#)
    )]
    fn abs_fail() {
        debug_assert_float_eq!(0_f32, 1., abs <= 0.1);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, rel <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [rel] t: `0.1`"#)
    )]
    fn rel_fail() {
        debug_assert_float_eq!(0_f32, 1., rel <= 0.1);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, ulps <= t)`
        left: `1.0`,
       right: `1.0000002`,
    abs_diff: `2.3841858e-7`,
   ulps_diff: `Some(2)`,
    [ulps] t: `1`"#)
    )]
    fn ulps_fail() {
        debug_assert_float_eq!(1_f32, 1.000_000_2, ulps <= 1);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, abs <= t)`
        left: `-1.0`,
       right: `1.0`,
    abs_diff: `2.0`,
   ulps_diff: `None`,
     [abs] t: `0.1`"#)
    )]
    fn fail_ulps_diff_none() {
        debug_assert_float_eq!(-1.0, 1.0, abs <= 0.1);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `0.1`: testing: 0 != 1"#)
    )]
    fn fail_with_message() {
        debug_assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `0.1`: testing: 0 != 1"#)
    )]
    fn fail_with_message_trailing_comma() {
        debug_assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32,);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, abs <= t, ulps <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `0.1`,
    [ulps] t: `1`: testing: 0 != 1"#)
    )]
    fn chaining_fail_with_message() {
        debug_assert_float_eq!(
            0_f32,
            1.,
            abs <= 0.1,
            ulps <= 1,
            "testing: {} != {}",
            0_f32,
            1_f32
        );
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, abs <= t, ulps <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `0.1`,
    [ulps] t: `1`: testing: 0 != 1"#)
    )]
    fn chaining_fail_with_message_trailing_comma() {
        debug_assert_float_eq!(
            0_f32,
            1.,
            abs <= 0.1,
            ulps <= 1,
            "testing: {} != {}",
            0_f32,
            1_f32,
        );
    }

    #[test]
    fn chaining() {
        // first succeeds
        debug_assert_float_eq!(1_f32, 1.000_000_2, abs <= 0.000_000_3, ulps <= 1);
        debug_assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_3,
            rel <= 0.000_000_1,
            ulps <= 1
        );

        // second succeeds
        debug_assert_float_eq!(1_f32, 1.000_000_2, abs <= 0.000_000_1, ulps <= 2);
        debug_assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_3,
            ulps <= 1
        );

        // third succeeds
        debug_assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_1,
            ulps <= 2
        );
    }

    #[test]
    fn chaining_with_messages() {
        // first succeeds
        debug_assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_3,
            ulps <= 1,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );
        debug_assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_3,
            rel <= 0.000_000_1,
            ulps <= 1,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );

        // second succeeds
        debug_assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            ulps <= 2,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );
        debug_assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_3,
            ulps <= 1,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );

        // third succeeds
        debug_assert_float_eq!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_1,
            ulps <= 2,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );
    }
}

mod debug_assert_float_ne {
    use super::*;

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `1.0`"#)
    )]
    fn abs_fail() {
        debug_assert_float_ne!(0_f32, 1., abs <= 1.);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, rel <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [rel] t: `1.0`"#)
    )]
    fn rel_fail() {
        debug_assert_float_ne!(0_f32, 1., rel <= 1.);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, ulps <= t)`
        left: `1.0`,
       right: `1.0000001`,
    abs_diff: `1.1920929e-7`,
   ulps_diff: `Some(1)`,
    [ulps] t: `1`"#)
    )]
    fn ulps_fail() {
        debug_assert_float_ne!(1_f32, 1.000_000_1, ulps <= 1);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `1.0`: testing: 0 != 1"#)
    )]
    fn fail_with_message() {
        debug_assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, abs <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `1.0`: testing: 0 != 1"#)
    )]
    fn fail_with_message_trailing_comma() {
        debug_assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32,);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, abs <= t, ulps <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `1.0`,
    [ulps] t: `1`: testing: 0 != 1"#)
    )]
    fn chaining_fail_with_message() {
        debug_assert_float_ne!(
            0_f32,
            1.,
            abs <= 1.,
            ulps <= 1,
            "testing: {} != {}",
            0_f32,
            1_f32
        );
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, abs <= t, ulps <= t)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] t: `1.0`,
    [ulps] t: `1`: testing: 0 != 1"#)
    )]
    fn chaining_fail_with_message_trailing_comma() {
        debug_assert_float_ne!(
            0_f32,
            1.,
            abs <= 1.,
            ulps <= 1,
            "testing: {} != {}",
            0_f32,
            1_f32,
        );
    }

    #[test]
    fn chaining() {
        debug_assert_float_ne!(1_f32, 1.000_000_2, abs <= 0.000_000_1, ulps <= 1);
        debug_assert_float_ne!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_1,
            ulps <= 1
        );
    }

    #[test]
    fn chaining_with_messages() {
        debug_assert_float_ne!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            ulps <= 1,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );
        debug_assert_float_ne!(
            1_f32,
            1.000_000_2,
            abs <= 0.000_000_1,
            rel <= 0.000_000_1,
            ulps <= 1,
            "testing: {} != {}",
            1_f32,
            1.000_000_2
        );
    }
}
