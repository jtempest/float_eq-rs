use crate::{f32, f64};
use float_eq::{assert_float_eq, float_eq, float_ne, FloatDiff, FloatEqDebug};

#[test]
fn float_diff() {
    // ()
    assert_eq!(().abs_diff(&()), ());
    assert_eq!(().ulps_diff(&()), Some(()));

    // (A,)
    assert_eq!((1.0f32,).abs_diff(&(2.0,)), (1.0,));
    assert_eq!((1.0f32,).ulps_diff(&(1.000_000_1,)), Some((1,)));
    assert_eq!((-1.0f32,).ulps_diff(&(1.000_000_1,)), None);

    // (A, B)
    assert_eq!((1.0f32, -2.0f64).abs_diff(&(2.0, -4.0)), (1.0f32, 2.0f64));
    assert_eq!(
        (1.0f32, 2.0f64).ulps_diff(&(1.000_000_1, 2.000_000_000_000_001)),
        Some((1, 2,))
    );
    assert_eq!(
        (-1.0f32, 2.0f64).ulps_diff(&(1.000_000_1, 2.000_000_000_000_001)),
        None
    );

    //...impl is by macro, so skip to the largest:

    // (A, B, C, D, E, F, G, H, I, J, K, L)
    let a = (
        1.0f32, -2.0f64, 3.0f32, -4.0f64, 5.0f32, 6.0f64, 7.0f32, 8.0f64, 9.0f32, 10.0f64, 11.0f32,
        12.0f64,
    );
    let b = (
        2.0f32, 2.0f64, -3.0f32, -4.5f64, 5.125f32, 6.25f64, 7.375f32, 8.5f64, 9.625f32, 10.75f64,
        11.875f32, 13.0f64,
    );
    assert_eq!(
        a.abs_diff(&b),
        (
            1.0f32, 4.0f64, 6.0f32, 0.5f64, 0.125f32, 0.25f64, 0.375f32, 0.5f64, 0.625f32, 0.75f64,
            0.875f32, 1.0f64,
        )
    );

    let c = (
        f32::next_n(1.0f32, 1),
        f64::next_n(-2.0f64, 2),
        f32::next_n(3.0f32, 3),
        f64::next_n(-4.0f64, 4),
        f32::next_n(5.0f32, 5),
        f64::next_n(6.0f64, 6),
        f32::next_n(7.0f32, 7),
        f64::next_n(8.0f64, 8),
        f32::next_n(9.0f32, 9),
        f64::next_n(10.0f64, 10),
        f32::next_n(11.0f32, 11),
        f64::next_n(12.0f64, 12),
    );
    assert_eq!(a.ulps_diff(&b), None);
    assert_eq!(
        a.ulps_diff(&c),
        Some((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12))
    );
}

#[test]
fn float_eq() {
    // ()
    assert!(float_eq!((), (), abs <= ()));
    assert!(float_eq!((), (), rel <= ()));
    assert!(float_eq!((), (), ulps <= ()));

    // (A,)
    assert!(float_eq!((1.0f32,), (1.5,), abs <= (0.5,)));
    assert!(float_ne!((1.0f32,), (1.5,), abs <= (f32::prev(0.5),)));
    assert!(float_eq!((4.0f32,), (4.000_000_5,), rel <= (f32::EPSILON,)));
    assert!(float_ne!(
        (4.0f32,),
        (4.000_000_5,),
        rel <= (0.5 * f32::EPSILON,)
    ));
    assert!(float_eq!((-4.0f32,), (-4.000_001,), ulps <= (2,)));
    assert!(float_ne!((-4.0f32,), (-4.000_001,), ulps <= (1,)));

    // (A, B)
    let a = (1.0f32, 2.0f64);
    let b = (1.5, -3.0);
    assert!(float_eq!(a, b, abs <= (0.5, 5.0)));
    assert!(float_ne!(a, b, abs <= (f32::prev(0.5), 5.0)));
    assert!(float_ne!(a, b, abs <= (0.5, f64::prev(5.0))));
    let a = (4.0f32, -8.0f64);
    let b = (4.000_000_5, -8.000_000_000_000_004);
    assert!(float_eq!(a, b, rel <= (f32::EPSILON, 2.0 * f64::EPSILON)));
    assert!(float_ne!(
        a,
        b,
        rel <= (0.5 * f32::EPSILON, 2.0 * f64::EPSILON)
    ));
    assert!(float_ne!(a, b, rel <= (f32::EPSILON, f64::EPSILON)));
    assert!(float_eq!(a, b, ulps <= (1, 2)));
    assert!(float_ne!(a, b, ulps <= (0, 2)));
    assert!(float_ne!(a, b, ulps <= (1, 1)));

    //...impl is by macro, so skip to the largest:

    // (A, B, C, D, E, F, G, H, I, J, K, L)
    let a = (
        1.0f32, -2.0f64, 3.0f32, -4.0f64, 5.0f32, 6.0f64, 7.0f32, 8.0f64, 9.0f32, 10.0f64, 11.0f32,
        12.0f64,
    );
    let b = (
        2.0f32, 2.0f64, -3.0f32, -4.5f64, 5.125f32, 6.25f64, 7.375f32, 8.5f64, 9.625f32, 10.75f64,
        11.875f32, 13.0f64,
    );
    let eps = (
        1.0f32, 4.0f64, 6.0f32, 0.5f64, 0.125f32, 0.25f64, 0.375f32, 0.5f64, 0.625f32, 0.75f64,
        0.875f32, 1.0f64,
    );
    assert!(float_eq!(a, b, abs <= eps));
    let mut eps0 = eps;
    eps0.0 = f32::prev(eps.0);
    assert!(float_ne!(a, b, abs <= eps0));
    let mut eps11 = eps;
    eps11.11 = f64::prev(eps.11);
    assert!(float_ne!(a, b, abs <= eps11));

    let c = (
        f32::next_n(1.0f32, 1),
        f64::next_n(-2.0f64, 2),
        f32::next_n(3.0f32, 3),
        f64::next_n(-4.0f64, 4),
        f32::next_n(5.0f32, 5),
        f64::next_n(6.0f64, 6),
        f32::next_n(7.0f32, 7),
        f64::next_n(8.0f64, 8),
        f32::next_n(9.0f32, 9),
        f64::next_n(10.0f64, 10),
        f32::next_n(11.0f32, 11),
        f64::next_n(12.0f64, 12),
    );
    let eps = (
        f32::EPSILON,
        2.0 * f64::EPSILON,
        3.0 * f32::EPSILON,
        4.0 * f64::EPSILON,
        5.0 * f32::EPSILON,
        6.0 * f64::EPSILON,
        7.0 * f32::EPSILON,
        8.0 * f64::EPSILON,
        9.0 * f32::EPSILON,
        10.0 * f64::EPSILON,
        11.0 * f32::EPSILON,
        12.0 * f64::EPSILON,
    );
    assert!(float_eq!(a, c, rel <= eps));
    let mut eps0 = eps;
    eps0.0 = eps.0 * 0.5;
    assert!(float_ne!(a, c, rel <= eps0));
    let mut eps11 = eps;
    eps11.11 = eps.11 * 0.5;
    assert!(float_ne!(a, c, rel <= eps11));

    let eps = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    assert!(float_eq!(a, c, ulps <= eps));
    let mut eps0 = eps;
    eps0.0 = eps.0 - 1;
    assert!(float_ne!(a, c, ulps <= eps0));
    let mut eps11 = eps;
    eps11.11 = eps.11 - 1;
    assert!(float_ne!(a, c, ulps <= eps11));
}

#[test]
fn float_eq_debug() {
    // ()
    assert_eq!(().debug_abs_epsilon(&(), &()), ());
    assert_eq!(().debug_rel_epsilon(&(), &()), ());
    assert_eq!(().debug_ulps_epsilon(&(), &()), ());

    // (A,)
    assert_eq!((1.0f32,).debug_abs_epsilon(&(1.0,), &(0.1,)), (0.1,));
    assert_eq!((1.0f32,).debug_rel_epsilon(&(2.0,), &(0.1,)), (0.2,));
    assert_eq!((2.0f32,).debug_rel_epsilon(&(1.0,), &(0.1,)), (0.2,));
    assert_eq!((1.0f32,).debug_ulps_epsilon(&(1.0f32,), &(1,)), (1,));

    // (A, B)
    assert_eq!(
        (1.0f32, -2.0f64).debug_abs_epsilon(&(1.0, -2.0f64), &(0.1, 0.2)),
        (0.1, 0.2)
    );
    assert_eq!(
        (1.0f32, -4.0f64).debug_rel_epsilon(&(2.0, -2.0f64), &(0.1, 0.2)),
        (0.2, 0.8)
    );
    assert_eq!(
        (2.0f32, -2.0f64).debug_rel_epsilon(&(1.0, -4.0f64), &(0.1, 0.2)),
        (0.2, 0.8)
    );
    assert_eq!(
        (2.0f32, -2.0f64).debug_ulps_epsilon(&(1.0, -4.0f64), &(1, 2)),
        (1, 2)
    );

    //...impl is by macro, so skip to the largest:

    // (A, B, C, D, E, F, G, H, I, J, K, L)
    let a = (
        1.0f32, -2.0f64, 3.0f32, -4.0f64, 5.0f32, 6.0f64, 7.0f32, 8.0f64, 9.0f32, 10.0f64, 11.0f32,
        12.0f64,
    );
    let b = (
        2.0f32, -4.0f64, 6.0f32, -8.0f64, 10.0f32, 12.0f64, 14.0f32, 16.0f64, 18.0f32, 20.0f64,
        22.0f32, 24.0f64,
    );
    assert_eq!(
        a.debug_abs_epsilon(
            &b,
            &(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2)
        ),
        (0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2)
    );
    assert_eq!(
        a.debug_rel_epsilon(
            &b,
            &(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2)
        ),
        (
            0.2,
            0.8,
            1.800_000_1,
            3.2,
            5.0,
            7.199_999_999_999_999,
            9.8,
            12.8,
            16.199_999,
            20.0,
            24.2,
            28.799_999_999_999_997
        )
    );
    assert_eq!(
        a.debug_ulps_epsilon(&b, &(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)),
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
    );
}

#[test]
#[should_panic(expected = r#"`float_eq!(left, right, abs <= ε)`
        left: `(1.0, 2.0)`,
       right: `(1.5, 2.5)`,
    abs_diff: `(0.5, 0.5)`,
   ulps_diff: `Some((4194304, 1125899906842624))`,
     [abs] ε: `(0.1, 0.2)"#)]
fn test_assert_fail_message() {
    assert_float_eq!((1.0f32, 2.0f64), (1.5f32, 2.5f64), abs <= (0.1, 0.2))
}
