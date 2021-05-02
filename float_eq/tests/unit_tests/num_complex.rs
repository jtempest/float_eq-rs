#![allow(clippy::float_cmp)]

use core::f32;
use float_eq::{
    assert_float_eq, assert_float_ne, AssertFloatEq, AssertFloatEqAll, ComplexUlps32, DebugUlpsDiff,
};
use num_complex::Complex32;

#[test]
fn complex_ulps() {
    let a = ComplexUlps32::new(1, 2);
    assert_eq!(a, a);
    let mut b = a.clone();
    b.im = 3;
    assert_ne!(a, b);
}

#[test]
fn float_eq() {
    let a = Complex32::new(0.999_999_9f32, 4.0);
    let b = Complex32::new(1.0f32, 3.999_999_5);
    let eps = f32::EPSILON;

    assert_float_eq!(a, b, abs <= Complex32::new(1.0 * eps, 4.0 * eps));
    assert_float_ne!(a, b, abs <= Complex32::new(0.5 * eps, 4.0 * eps));
    assert_float_ne!(a, b, abs <= Complex32::new(1.0 * eps, 2.0 * eps));

    assert_float_eq!(a, b, rel <= Complex32::new(1.0 * eps, 1.0 * eps));
    assert_float_ne!(a, b, rel <= Complex32::new(0.5 * eps, 1.0 * eps));
    assert_float_ne!(a, b, rel <= Complex32::new(1.0 * eps, 0.5 * eps));

    assert_float_eq!(a, b, rmax <= Complex32::new(1.0 * eps, 1.0 * eps));
    assert_float_ne!(a, b, rmax <= Complex32::new(0.5 * eps, 1.0 * eps));
    assert_float_ne!(a, b, rmax <= Complex32::new(1.0 * eps, 0.5 * eps));

    assert_float_eq!(a, b, rmin <= Complex32::new(2.0 * eps, 2.0 * eps));
    assert_float_ne!(a, b, rmin <= Complex32::new(1.0 * eps, 2.0 * eps));
    assert_float_ne!(a, b, rmin <= Complex32::new(2.0 * eps, 1.0 * eps));

    assert_float_eq!(a, b, r1st <= Complex32::new(2.0 * eps, 1.0 * eps));
    assert_float_ne!(a, b, r1st <= Complex32::new(1.0 * eps, 1.0 * eps));
    assert_float_ne!(a, b, r1st <= Complex32::new(2.0 * eps, 0.5 * eps));

    assert_float_eq!(a, b, r2nd <= Complex32::new(1.0 * eps, 2.0 * eps));
    assert_float_ne!(a, b, r2nd <= Complex32::new(0.5 * eps, 2.0 * eps));
    assert_float_ne!(a, b, r2nd <= Complex32::new(1.0 * eps, 1.0 * eps));

    assert_float_eq!(a, b, ulps <= ComplexUlps32::new(2, 2));
    assert_float_ne!(a, b, ulps <= ComplexUlps32::new(1, 2));
    assert_float_ne!(a, b, ulps <= ComplexUlps32::new(2, 1));
}

#[test]
fn float_eq_all() {
    let a = Complex32::new(0.999_999_9f32, 4.0);
    let b = Complex32::new(1.0f32, 3.999_999_5);
    let eps = f32::EPSILON;

    assert_float_eq!(a, b, abs_all <= (4.0 * eps));
    assert_float_ne!(a, b, abs_all <= (2.0 * eps));

    assert_float_eq!(a, b, rel_all <= (1.0 * eps));
    assert_float_ne!(a, b, rel_all <= (0.5 * eps));

    assert_float_eq!(a, b, rmax_all <= (1.0 * eps));
    assert_float_ne!(a, b, rmax_all <= (0.5 * eps));

    assert_float_eq!(a, b, rmin_all <= (2.0 * eps));
    assert_float_ne!(a, b, rmin_all <= (1.0 * eps));

    assert_float_eq!(a, b, r1st_all <= (2.0 * eps));
    assert_float_ne!(a, b, r1st_all <= (1.0 * eps));

    assert_float_eq!(a, b, r2nd_all <= (2.0 * eps));
    assert_float_ne!(a, b, r2nd_all <= (1.0 * eps));

    assert_float_eq!(a, b, ulps_all <= 2);
    assert_float_ne!(a, b, ulps_all <= 1);
}

#[test]
fn debug_diff() {
    let a = Complex32::new(1.0f32, 2.0);
    let b = Complex32::new(1.5f32, 2.25);
    let ulps = DebugUlpsDiff::<Complex32>::new(Some(4_194_304), Some(1_048_576));

    assert_eq!(a.debug_abs_diff(&a), Complex32::new(0.0, 0.0));
    assert_eq!(
        a.debug_ulps_diff(&a),
        DebugUlpsDiff::<Complex32>::new(Some(0), Some(0))
    );

    assert_eq!(a.debug_abs_diff(&b), Complex32::new(0.5, 0.25));
    assert_eq!(b.debug_abs_diff(&a), Complex32::new(0.5, 0.25));

    assert_eq!(a.debug_ulps_diff(&b), ulps);
    assert_eq!(b.debug_ulps_diff(&a), ulps);
}

#[test]
fn debug_tol() {
    let a = Complex32::new(2.0f32, 4.25);
    let b = Complex32::new(2.5f32, 4.0);
    let eps = Complex32::new(0.1, 0.2);

    assert_eq!(a.debug_abs_tol(&b, &eps), Complex32::new(0.1, 0.2));
    assert_eq!(a.debug_rel_tol(&b, &eps), Complex32::new(0.25, 0.85));
    assert_eq!(a.debug_rmax_tol(&b, &eps), Complex32::new(0.25, 0.85));
    assert_eq!(a.debug_rmin_tol(&b, &eps), Complex32::new(0.2, 0.8));
    assert_eq!(a.debug_r1st_tol(&b, &eps), Complex32::new(0.2, 0.85));
    assert_eq!(a.debug_r2nd_tol(&b, &eps), Complex32::new(0.25, 0.8));
    assert_eq!(
        a.debug_ulps_tol(&b, &ComplexUlps32::new(1, 2)),
        ComplexUlps32::new(1, 2)
    );
}

#[test]
fn debug_all_tol() {
    let a = Complex32::new(2.0f32, 4.25);
    let b = Complex32::new(2.5f32, 4.0);

    assert_eq!(a.debug_abs_all_tol(&b, &0.2), Complex32::new(0.2, 0.2));
    assert_eq!(a.debug_rel_all_tol(&b, &0.2), Complex32::new(0.5, 0.85));
    assert_eq!(
        a.debug_rmax_all_tol(&b, &0.2),
        Complex32::new(0.5, 0.85)
    );
    assert_eq!(a.debug_rmin_all_tol(&b, &0.2), Complex32::new(0.4, 0.8));
    assert_eq!(
        a.debug_r1st_all_tol(&b, &0.2),
        Complex32::new(0.4, 0.85)
    );
    assert_eq!(a.debug_r2nd_all_tol(&b, &0.2), Complex32::new(0.5, 0.8));
    assert_eq!(a.debug_ulps_all_tol(&b, &2), ComplexUlps32::new(2, 2));
}

#[test]
#[should_panic(expected = r#"`float_eq!(left, right, abs <= t, rel <= t, ulps <= t)`
        left: `Complex { re: 1.0, im: 2.0 }`,
       right: `Complex { re: 3.0, im: -5.0 }`,
    abs_diff: `Complex { re: 2.0, im: 7.0 }`,
   ulps_diff: `ComplexUlps { re: Some(12582912), im: None }`,
     [abs] t: `Complex { re: 0.1, im: 0.25 }`,
     [rel] t: `Complex { re: 0.3, im: 1.25 }`,
    [ulps] t: `ComplexUlps { re: 1, im: 2 }`"#)]
fn assert_fail_message() {
    assert_float_eq!(
        Complex32::new(1., 2.),
        Complex32::new(3., -5.),
        abs <= Complex32::new(0.1, 0.25),
        rel <= Complex32::new(0.1, 0.25),
        ulps <= ComplexUlps32::new(1, 2)
    );
}

#[test]
#[should_panic(
    expected = r#"`float_eq!(left, right, abs_all <= t, rel_all <= t, ulps_all <= t)`
        left: `Complex { re: 1.0, im: 2.0 }`,
       right: `Complex { re: 3.0, im: -5.0 }`,
    abs_diff: `Complex { re: 2.0, im: 7.0 }`,
   ulps_diff: `ComplexUlps { re: Some(12582912), im: None }`,
 [abs_all] t: `Complex { re: 0.25, im: 0.25 }`,
 [rel_all] t: `Complex { re: 0.75, im: 1.25 }`,
[ulps_all] t: `ComplexUlps { re: 3, im: 3 }`"#
)]
fn assert_fail_all_message() {
    assert_float_eq!(
        Complex32::new(1., 2.),
        Complex32::new(3., -5.),
        abs_all <= 0.25,
        rel_all <= 0.25,
        ulps_all <= 3
    );
}
