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
    #[should_panic(expected = r#"`float_eq!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `0.1`"#)]
    fn abs_fail() {
        assert_float_eq!(0_f32, 1., abs <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, abs_all <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
 [abs_all] ε: `0.1`"#)]
    fn abs_all_fail() {
        assert_float_eq!(0_f32, 1., abs_all <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, rel <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [rel] ε: `0.1`"#)]
    fn rel_fail() {
        assert_float_eq!(0_f32, 1., rel <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, rel_all <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
 [rel_all] ε: `0.1`"#)]
    fn rel_all_fail() {
        assert_float_eq!(0_f32, 1., rel_all <= 0.1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, ulps <= ε)`
        left: `1.0`,
       right: `1.0000002`,
    abs_diff: `0.00000023841858`,
   ulps_diff: `Some(2)`,
    [ulps] ε: `1`"#)]
    fn ulps_fail() {
        assert_float_eq!(1_f32, 1.000_000_2, ulps <= 1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, ulps_all <= ε)`
        left: `1.0`,
       right: `1.0000002`,
    abs_diff: `0.00000023841858`,
   ulps_diff: `Some(2)`,
[ulps_all] ε: `1`"#)]
    fn ulps_all_fail() {
        assert_float_eq!(1_f32, 1.000_000_2, ulps_all <= 1);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `0.1`: testing: 0 != 1"#)]
    fn fail_with_message() {
        assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `0.1`: testing: 0 != 1"#)]
    fn fail_with_message_trailing_comma() {
        assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32,);
    }

    #[test]
    #[should_panic(expected = r#"`float_eq!(left, right, abs <= ε, ulps <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `0.1`,
    [ulps] ε: `1`: testing: 0 != 1"#)]
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
    #[should_panic(expected = r#"`float_eq!(left, right, abs <= ε, ulps <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `0.1`,
    [ulps] ε: `1`: testing: 0 != 1"#)]
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
    #[should_panic(expected = r#"`float_ne!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `1.0`"#)]
    fn abs_fail() {
        assert_float_ne!(0_f32, 1., abs <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, abs_all <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
 [abs_all] ε: `1.0`"#)]
    fn abs_all_fail() {
        assert_float_ne!(0_f32, 1., abs_all <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, rel <= ε)`
        left: `0.0`,
       right: `2.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(1073741824)`,
     [rel] ε: `2.0`"#)]
    fn rel_fail() {
        assert_float_ne!(0_f32, 2., rel <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, rel_all <= ε)`
        left: `0.0`,
       right: `2.0`,
    abs_diff: `2.0`,
   ulps_diff: `Some(1073741824)`,
 [rel_all] ε: `2.0`"#)]
    fn rel_all_fail() {
        assert_float_ne!(0_f32, 2., rel_all <= 1.);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, ulps <= ε)`
        left: `1.0`,
       right: `1.0000001`,
    abs_diff: `0.00000011920929`,
   ulps_diff: `Some(1)`,
    [ulps] ε: `1`"#)]
    fn ulps_fail() {
        assert_float_ne!(1_f32, 1.000_000_1, ulps <= 1);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `1.0`: testing: 0 != 1"#)]
    fn fail_with_message() {
        assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `1.0`: testing: 0 != 1"#)]
    fn fail_with_message_trailing_comma() {
        assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32,);
    }

    #[test]
    #[should_panic(expected = r#"`float_ne!(left, right, abs <= ε, ulps <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `1.0`,
    [ulps] ε: `1`: testing: 0 != 1"#)]
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
    #[should_panic(expected = r#"`float_ne!(left, right, abs <= ε, ulps <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `1.0`,
    [ulps] ε: `1`: testing: 0 != 1"#)]
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
        should_panic(expected = r#"`float_eq!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `0.1`"#)
    )]
    fn abs_fail() {
        debug_assert_float_eq!(0_f32, 1., abs <= 0.1);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, rel <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [rel] ε: `0.1`"#)
    )]
    fn rel_fail() {
        debug_assert_float_eq!(0_f32, 1., rel <= 0.1);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, ulps <= ε)`
        left: `1.0`,
       right: `1.0000002`,
    abs_diff: `0.00000023841858`,
   ulps_diff: `Some(2)`,
    [ulps] ε: `1`"#)
    )]
    fn ulps_fail() {
        debug_assert_float_eq!(1_f32, 1.000_000_2, ulps <= 1);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, abs <= ε)`
        left: `-1.0`,
       right: `1.0`,
    abs_diff: `2.0`,
   ulps_diff: `None`,
     [abs] ε: `0.1`"#)
    )]
    fn fail_ulps_diff_none() {
        debug_assert_float_eq!(-1.0, 1.0, abs <= 0.1);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `0.1`: testing: 0 != 1"#)
    )]
    fn fail_with_message() {
        debug_assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `0.1`: testing: 0 != 1"#)
    )]
    fn fail_with_message_trailing_comma() {
        debug_assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32,);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_eq!(left, right, abs <= ε, ulps <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `0.1`,
    [ulps] ε: `1`: testing: 0 != 1"#)
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
        should_panic(expected = r#"`float_eq!(left, right, abs <= ε, ulps <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `0.1`,
    [ulps] ε: `1`: testing: 0 != 1"#)
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
        should_panic(expected = r#"`float_ne!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `1.0`"#)
    )]
    fn abs_fail() {
        debug_assert_float_ne!(0_f32, 1., abs <= 1.);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, rel <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [rel] ε: `1.0`"#)
    )]
    fn rel_fail() {
        debug_assert_float_ne!(0_f32, 1., rel <= 1.);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, ulps <= ε)`
        left: `1.0`,
       right: `1.0000001`,
    abs_diff: `0.00000011920929`,
   ulps_diff: `Some(1)`,
    [ulps] ε: `1`"#)
    )]
    fn ulps_fail() {
        debug_assert_float_ne!(1_f32, 1.000_000_1, ulps <= 1);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `1.0`: testing: 0 != 1"#)
    )]
    fn fail_with_message() {
        debug_assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, abs <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `1.0`: testing: 0 != 1"#)
    )]
    fn fail_with_message_trailing_comma() {
        debug_assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32,);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = r#"`float_ne!(left, right, abs <= ε, ulps <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `1.0`,
    [ulps] ε: `1`: testing: 0 != 1"#)
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
        should_panic(expected = r#"`float_ne!(left, right, abs <= ε, ulps <= ε)`
        left: `0.0`,
       right: `1.0`,
    abs_diff: `1.0`,
   ulps_diff: `Some(1065353216)`,
     [abs] ε: `1.0`,
    [ulps] ε: `1`: testing: 0 != 1"#)
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
