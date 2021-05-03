use crate::{f32, f64};
use float_eq::{assert_float_eq, assert_float_ne, AssertFloatEq};

#[test]
fn float_eq() {
    // ()
    assert_float_eq!((), (), abs <= ());
    assert_float_eq!((), (), rel <= ());
    assert_float_eq!((), (), rmax <= ());
    assert_float_eq!((), (), rmin <= ());
    assert_float_eq!((), (), r1st <= ());
    assert_float_eq!((), (), r2nd <= ());
    assert_float_eq!((), (), ulps <= ());

    // (A,)
    assert_float_eq!((1.0f32,), (1.5,), abs <= (0.5,));
    assert_float_ne!((1.0f32,), (1.5,), abs <= (f32::prev(0.5),));
    assert_float_eq!((4.0f32,), (4.000_000_5,), rel <= (f32::EPSILON,));
    assert_float_ne!((4.0f32,), (4.000_000_5,), rel <= (0.5 * f32::EPSILON,));
    assert_float_eq!((4.0f32,), (3.999_999_5,), rmax <= (1.0 * f32::EPSILON,));
    assert_float_ne!((4.0f32,), (3.999_999_5,), rmax <= (0.5 * f32::EPSILON,));
    assert_float_eq!((4.0f32,), (3.999_999_5,), rmin <= (2.0 * f32::EPSILON,));
    assert_float_ne!((4.0f32,), (3.999_999_5,), rmin <= (1.0 * f32::EPSILON,));
    assert_float_eq!((4.0f32,), (3.999_999_5,), r1st <= (1.0 * f32::EPSILON,));
    assert_float_ne!((4.0f32,), (3.999_999_5,), r1st <= (0.5 * f32::EPSILON,));
    assert_float_eq!((4.0f32,), (3.999_999_5,), r2nd <= (2.0 * f32::EPSILON,));
    assert_float_ne!((4.0f32,), (3.999_999_5,), r2nd <= (1.0 * f32::EPSILON,));
    assert_float_eq!((-4.0f32,), (-4.000_001,), ulps <= (2,));
    assert_float_ne!((-4.0f32,), (-4.000_001,), ulps <= (1,));

    // (A, B)
    let a = (1.0f32, 2.0f64);
    let b = (1.5, -3.0);
    assert_float_eq!(a, b, abs <= (0.5, 5.0));
    assert_float_ne!(a, b, abs <= (f32::prev(0.5), 5.0));
    assert_float_ne!(a, b, abs <= (0.5, f64::prev(5.0)));

    let a = (4.0f32, -8.0f64);
    let b = (4.000_000_5, -8.000_000_000_000_004);
    assert_float_eq!(a, b, rel <= (f32::EPSILON, 2.0 * f64::EPSILON));
    assert_float_ne!(a, b, rel <= (0.5 * f32::EPSILON, 2.0 * f64::EPSILON));
    assert_float_ne!(a, b, rel <= (f32::EPSILON, f64::EPSILON));

    let a = (4.0f32, -8.0f64);
    let b = (3.999_999_5, -7.999_999_999_999_996_4);

    assert_float_eq!(a, b, rmax <= (1.0 * f32::EPSILON, 2.0 * f64::EPSILON));
    assert_float_ne!(a, b, rmax <= (0.5 * f32::EPSILON, 2.0 * f64::EPSILON));
    assert_float_ne!(a, b, rmax <= (1.0 * f32::EPSILON, 1.0 * f64::EPSILON));

    assert_float_eq!(a, b, rmin <= (2.0 * f32::EPSILON, 4.0 * f64::EPSILON));
    assert_float_ne!(a, b, rmin <= (1.0 * f32::EPSILON, 4.0 * f64::EPSILON));
    assert_float_ne!(a, b, rmin <= (2.0 * f32::EPSILON, 2.0 * f64::EPSILON));

    assert_float_eq!(a, b, r1st <= (1.0 * f32::EPSILON, 2.0 * f64::EPSILON));
    assert_float_ne!(a, b, r1st <= (0.5 * f32::EPSILON, 2.0 * f64::EPSILON));
    assert_float_ne!(a, b, r1st <= (1.0 * f32::EPSILON, 1.0 * f64::EPSILON));

    assert_float_eq!(a, b, r2nd <= (2.0 * f32::EPSILON, 4.0 * f64::EPSILON));
    assert_float_ne!(a, b, r2nd <= (1.0 * f32::EPSILON, 4.0 * f64::EPSILON));
    assert_float_ne!(a, b, r2nd <= (2.0 * f32::EPSILON, 2.0 * f64::EPSILON));

    let a = (4.0f32, -8.0f64);
    let b = (4.000_000_5, -8.000_000_000_000_004);
    assert_float_eq!(a, b, ulps <= (1, 2));
    assert_float_ne!(a, b, ulps <= (0, 2));
    assert_float_ne!(a, b, ulps <= (1, 1));

    //...impl is by macro, so skip to the largest:

    // (A, B, C, D, E, F, G, H, I, J, K, L)
    let a = (
        1.0f32, -2.0f64, 3.0f32, -4.0f64, 5.0f32, 6.0f64, 7.0f32, 8.0f64, 9.0f32, 10.0f64, 11.0f32,
        16.0f64,
    );
    let b = (
        2.0f32, 2.0f64, -3.0f32, -4.5f64, 5.125f32, 6.25f64, 7.375f32, 8.5f64, 9.625f32, 10.75f64,
        11.875f32, 15.0f64,
    );
    let eps = (
        1.0f32, 4.0f64, 6.0f32, 0.5f64, 0.125f32, 0.25f64, 0.375f32, 0.5f64, 0.625f32, 0.75f64,
        0.875f32, 1.0f64,
    );

    assert_float_eq!(a, b, abs <= eps);

    let mut eps0 = eps;
    eps0.0 = f32::prev(eps.0);
    assert_float_ne!(a, b, abs <= eps0);

    let mut eps11 = eps;
    eps11.11 = f64::prev(eps.11);
    assert_float_ne!(a, b, abs <= eps11);

    let c = (
        f32::next_n(1.0f32, 1),
        f64::prev_n(-2.0f64, 4),
        f32::next_n(3.0f32, 3),
        f64::next_n(-4.0f64, 4),
        f32::next_n(5.0f32, 5),
        f64::next_n(6.0f64, 6),
        f32::next_n(7.0f32, 7),
        f64::next_n(8.0f64, 8),
        f32::next_n(9.0f32, 9),
        f64::next_n(10.0f64, 10),
        f32::next_n(11.0f32, 11),
        f64::prev_n(16.0f64, 24),
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

    assert_float_eq!(a, c, rmax <= eps);
    assert_float_eq!(c, a, rmax <= eps);

    let mut eps0 = eps;
    eps0.0 = eps.0 * 0.5;
    assert_float_ne!(a, c, rmax <= eps0);
    assert_float_ne!(c, a, rmax <= eps0);

    let mut eps11 = eps;
    eps11.11 = eps.11 * 0.5;
    assert_float_ne!(a, c, rmax <= eps11);
    assert_float_ne!(c, a, rmax <= eps11);

    let eps = (
        f32::EPSILON,
        4.0 * f64::EPSILON,
        3.0 * f32::EPSILON,
        4.0 * f64::EPSILON,
        5.0 * f32::EPSILON,
        6.0 * f64::EPSILON,
        7.0 * f32::EPSILON,
        8.0 * f64::EPSILON,
        9.0 * f32::EPSILON,
        10.0 * f64::EPSILON,
        11.0 * f32::EPSILON,
        24.0 * f64::EPSILON,
    );

    assert_float_eq!(a, c, rmin <= eps);
    assert_float_eq!(c, a, rmin <= eps);

    let mut eps0 = eps;
    eps0.0 = eps.0 * 0.5;
    assert_float_ne!(a, c, rmin <= eps0);
    assert_float_ne!(c, a, rmin <= eps0);

    let mut eps11 = eps;
    eps11.11 = eps.11 * 0.5;
    assert_float_ne!(a, c, rmin <= eps11);
    assert_float_ne!(c, a, rmin <= eps11);

    let eps_a = (
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
    let eps_c = (
        f32::EPSILON,
        4.0 * f64::EPSILON,
        3.0 * f32::EPSILON,
        4.0 * f64::EPSILON,
        5.0 * f32::EPSILON,
        6.0 * f64::EPSILON,
        7.0 * f32::EPSILON,
        8.0 * f64::EPSILON,
        9.0 * f32::EPSILON,
        10.0 * f64::EPSILON,
        11.0 * f32::EPSILON,
        24.0 * f64::EPSILON,
    );

    assert_float_eq!(a, c, r1st <= eps_a);
    assert_float_eq!(c, a, r1st <= eps_c);

    assert_float_eq!(a, c, r2nd <= eps_c);
    assert_float_eq!(c, a, r2nd <= eps_a);

    let mut eps_a0 = eps_a;
    eps_a0.0 = eps_a.0 * 0.5;
    let mut eps_c0 = eps_c;
    eps_c0.0 = eps_c.0 * 0.5;

    assert_float_ne!(a, c, r1st <= eps_a0);
    assert_float_ne!(c, a, r1st <= eps_c0);

    assert_float_ne!(a, c, r2nd <= eps_c0);
    assert_float_ne!(c, a, r2nd <= eps_a0);

    let mut eps_a11 = eps_a;
    eps_a11.11 = eps_a.11 * 0.5;
    let mut eps_c11 = eps_c;
    eps_c11.11 = eps_c.11 * 0.5;

    assert_float_ne!(a, c, r1st <= eps_a11);
    assert_float_ne!(c, a, r1st <= eps_c11);

    assert_float_ne!(a, c, r2nd <= eps_c11);
    assert_float_ne!(c, a, r2nd <= eps_a11);

    let eps = (1, 4, 3, 4, 5, 6, 7, 8, 9, 10, 11, 24);
    assert_float_eq!(a, c, ulps <= eps);

    let mut eps0 = eps;
    eps0.0 = eps.0 - 1;
    assert_float_ne!(a, c, ulps <= eps0);

    let mut eps11 = eps;
    eps11.11 = eps.11 - 1;
    assert_float_ne!(a, c, ulps <= eps11);
}

#[test]
fn debug_diff() {
    // ()
    assert_eq!(().debug_abs_diff(&()), ());
    assert_eq!(().debug_ulps_diff(&()), ());

    // (A,)
    assert_eq!((1.0f32,).debug_abs_diff(&(2.0,)), (1.0,));
    assert_eq!((1.0f32,).debug_ulps_diff(&(1.000_000_1,)), (Some(1),));
    assert_eq!((-1.0f32,).debug_ulps_diff(&(1.000_000_1,)), (None,));

    // (A, B)
    assert_eq!(
        (1.0f32, -2.0f64).debug_abs_diff(&(2.0, -4.0)),
        (1.0f32, 2.0f64)
    );
    assert_eq!(
        (1.0f32, 2.0f64).debug_ulps_diff(&(1.000_000_1, -2.000_000_000_000_001)),
        (Some(1), None)
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
        a.debug_abs_diff(&b),
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
        -9f32,
        -10.0f64,
        f32::next_n(11.0f32, 11),
        f64::next_n(12.0f64, 12),
    );
    assert_eq!(
        a.debug_ulps_diff(&c),
        (
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            None,
            None,
            Some(11),
            Some(12)
        )
    );
}

#[test]
fn debug_tol() {
    // ()
    assert_eq!(().debug_abs_tol(&(), &()), ());
    assert_eq!(().debug_rel_tol(&(), &()), ());
    assert_eq!(().debug_rmax_tol(&(), &()), ());
    assert_eq!(().debug_rmin_tol(&(), &()), ());
    assert_eq!(().debug_r1st_tol(&(), &()), ());
    assert_eq!(().debug_r2nd_tol(&(), &()), ());
    assert_eq!(().debug_ulps_tol(&(), &()), ());

    // (A,)
    assert_eq!((1.0f32,).debug_abs_tol(&(1.0,), &(0.1,)), (0.1,));

    assert_eq!((1.0f32,).debug_rel_tol(&(2.0,), &(0.1,)), (0.2,));
    assert_eq!((2.0f32,).debug_rel_tol(&(1.0,), &(0.1,)), (0.2,));

    assert_eq!((1.0f32,).debug_rmax_tol(&(2.0,), &(0.1,)), (0.2,));
    assert_eq!((2.0f32,).debug_rmax_tol(&(1.0,), &(0.1,)), (0.2,));

    assert_eq!((1.0f32,).debug_rmin_tol(&(2.0,), &(0.1,)), (0.1,));
    assert_eq!((2.0f32,).debug_rmin_tol(&(1.0,), &(0.1,)), (0.1,));

    assert_eq!((1.0f32,).debug_r1st_tol(&(2.0,), &(0.1,)), (0.1,));
    assert_eq!((2.0f32,).debug_r1st_tol(&(1.0,), &(0.1,)), (0.2,));

    assert_eq!((1.0f32,).debug_r2nd_tol(&(2.0,), &(0.1,)), (0.2,));
    assert_eq!((2.0f32,).debug_r2nd_tol(&(1.0,), &(0.1,)), (0.1,));

    assert_eq!((1.0f32,).debug_ulps_tol(&(1.0f32,), &(1,)), (1,));

    // (A, B)
    assert_eq!(
        (1.0f32, -2.0f64).debug_abs_tol(&(1.0, -2.0f64), &(0.1, 0.2)),
        (0.1, 0.2)
    );

    assert_eq!(
        (1.0f32, -4.0f64).debug_rel_tol(&(2.0, -2.0f64), &(0.1, 0.2)),
        (0.2, 0.8)
    );
    assert_eq!(
        (2.0f32, -2.0f64).debug_rel_tol(&(1.0, -4.0f64), &(0.1, 0.2)),
        (0.2, 0.8)
    );

    assert_eq!(
        (1.0f32, -4.0f64).debug_rmax_tol(&(2.0, -2.0f64), &(0.1, 0.2)),
        (0.2, 0.8)
    );
    assert_eq!(
        (2.0f32, -2.0f64).debug_rmax_tol(&(1.0, -4.0f64), &(0.1, 0.2)),
        (0.2, 0.8)
    );

    assert_eq!(
        (1.0f32, -4.0f64).debug_rmin_tol(&(2.0, -2.0f64), &(0.1, 0.2)),
        (0.1, 0.4)
    );
    assert_eq!(
        (2.0f32, -2.0f64).debug_rmin_tol(&(1.0, -4.0f64), &(0.1, 0.2)),
        (0.1, 0.4)
    );

    assert_eq!(
        (1.0f32, -4.0f64).debug_r1st_tol(&(2.0, -2.0f64), &(0.1, 0.2)),
        (0.1, 0.8)
    );
    assert_eq!(
        (2.0f32, -2.0f64).debug_r1st_tol(&(1.0, -4.0f64), &(0.1, 0.2)),
        (0.2, 0.4)
    );

    assert_eq!(
        (1.0f32, -4.0f64).debug_r2nd_tol(&(2.0, -2.0f64), &(0.1, 0.2)),
        (0.2, 0.4)
    );
    assert_eq!(
        (2.0f32, -2.0f64).debug_r2nd_tol(&(1.0, -4.0f64), &(0.1, 0.2)),
        (0.1, 0.8)
    );

    assert_eq!(
        (2.0f32, -2.0f64).debug_ulps_tol(&(1.0, -4.0f64), &(1, 2)),
        (1, 2)
    );

    //...impl is by macro, so skip to the largest:

    // (A, B, C, D, E, F, G, H, I, J, K, L)
    let a = (
        1.0f32, -4.0f64, 3.0f32, -4.0f64, 5.0f32, 6.0f64, 7.0f32, 8.0f64, 9.0f32, 10.0f64, 11.0f32,
        24.0f64,
    );
    let b = (
        2.0f32, -2.0f64, 6.0f32, -8.0f64, 10.0f32, 12.0f64, 14.0f32, 16.0f64, 18.0f32, 20.0f64,
        22.0f32, 12.0f64,
    );

    assert_eq!(
        a.debug_abs_tol(
            &b,
            &(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2)
        ),
        (0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2)
    );

    assert_eq!(
        a.debug_rmax_tol(
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
        a.debug_rmin_tol(
            &b,
            &(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2)
        ),
        (
            0.1,
            0.4,
            0.900_000_04,
            1.6,
            2.5,
            3.599_999_999_999_999_6,
            4.9,
            6.4,
            8.099_999,
            10.0,
            12.1,
            14.399_999_999_999_999
        )
    );

    assert_eq!(
        a.debug_r1st_tol(
            &b,
            &(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2)
        ),
        (
            0.1,
            0.8,
            0.900_000_04,
            1.6,
            2.5,
            3.599_999_999_999_999_6,
            4.9,
            6.4,
            8.099_999,
            10.0,
            12.1,
            28.799_999_999_999_997
        )
    );

    assert_eq!(
        a.debug_r2nd_tol(
            &b,
            &(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2)
        ),
        (
            0.2,
            0.4,
            1.800_000_1,
            3.2,
            5.0,
            7.199_999_999_999_999,
            9.8,
            12.8,
            16.199_999,
            20.0,
            24.2,
            14.399_999_999_999_999
        )
    );

    assert_eq!(
        a.debug_ulps_tol(&b, &(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)),
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
    );
}

#[test]
#[should_panic(expected = r#"`float_eq!(left, right, abs <= t)`
        left: `(1.0, 2.0)`,
       right: `(1.5, -2.5)`,
    abs_diff: `(0.5, 4.5)`,
   ulps_diff: `(Some(4194304), None)`,
     [abs] t: `(0.1, 0.2)"#)]
fn test_assert_fail_message() {
    assert_float_eq!((1.0f32, 2.0f64), (1.5f32, -2.5f64), abs <= (0.1, 0.2))
}
