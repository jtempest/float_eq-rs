mod assert_float_eq {
    use float_eq::assert_float_eq;

    #[test]
    #[should_panic]
    fn abs_fail() {
        assert_float_eq!(0_f32, 1., abs <= 0.1);
    }

    #[test]
    #[should_panic]
    fn array_abs_fail() {
        assert_float_eq!([1.0_f32, 2.], [1.0000001, 3.], abs <= 0.0000001);
    }

    #[test]
    #[should_panic]
    fn rel_fail() {
        assert_float_eq!(0_f32, 1., rel <= 0.1);
    }

    #[test]
    #[should_panic]
    fn array_rel_fail() {
        assert_float_eq!([1.0_f32, 2.], [1.0000001, 3.], rel <= std::f32::EPSILON);
    }

    #[test]
    #[should_panic]
    fn ulps_fail() {
        assert_float_eq!(1_f32, 1.000_000_2, ulps <= 1);
    }

    #[test]
    #[should_panic]
    fn array_ulps_fail() {
        assert_float_eq!([1.0_f32, 2.], [1.0000001, 3.], ulps <= 1);
    }

    #[test]
    #[should_panic]
    fn fail_with_message() {
        assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[should_panic]
    fn fail_with_message_trailing_comma() {
        assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32,);
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
    use float_eq::assert_float_ne;

    #[test]
    #[should_panic]
    fn abs_fail() {
        assert_float_ne!(0_f32, 1., abs <= 1.);
    }

    #[test]
    #[should_panic]
    fn rel_fail() {
        assert_float_ne!(0_f32, 1., rel <= 1.);
    }

    #[test]
    #[should_panic]
    fn ulps_fail() {
        assert_float_ne!(1_f32, 1.000_000_1, ulps <= 1);
    }

    #[test]
    #[should_panic]
    fn fail_with_message() {
        assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[should_panic]
    fn fail_with_message_trailing_comma() {
        assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32,);
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
    use float_eq::debug_assert_float_eq;

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn abs_fail() {
        debug_assert_float_eq!(0_f32, 1., abs <= 0.1);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn rel_fail() {
        debug_assert_float_eq!(0_f32, 1., rel <= 0.1);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn ulps_fail() {
        debug_assert_float_eq!(1_f32, 1.000_000_2, ulps <= 1);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn fail_with_message() {
        debug_assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn fail_with_message_trailing_comma() {
        debug_assert_float_eq!(0_f32, 1., abs <= 0.1, "testing: {} != {}", 0_f32, 1_f32,);
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
    use float_eq::debug_assert_float_ne;

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn abs_fail() {
        debug_assert_float_ne!(0_f32, 1., abs <= 1.);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn rel_fail() {
        debug_assert_float_ne!(0_f32, 1., rel <= 1.);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn ulps_fail() {
        debug_assert_float_ne!(1_f32, 1.000_000_1, ulps <= 1);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn fail_with_message() {
        debug_assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn fail_with_message_trailing_comma() {
        debug_assert_float_ne!(0_f32, 1., abs <= 1., "testing: {} != {}", 0_f32, 1_f32,);
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
