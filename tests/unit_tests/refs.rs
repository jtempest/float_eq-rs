use float_eq::{assert_float_eq, FloatDiff, FloatEqAllDebug, FloatEqDebug};

#[test]
fn refs_float_diff() {
    let a = &1.0f32;
    let b = &1.5f32;
    let mut ma = &mut 1.0f32;
    let mut mb = &mut 1.5f32;
    assert_eq!(FloatDiff::abs_diff(&a, &b), Some(0.5));
    assert_eq!(FloatDiff::abs_diff(&mut ma, &b), Some(0.5));
    assert_eq!(FloatDiff::abs_diff(&a, &mut mb), Some(0.5));
    assert_eq!(FloatDiff::abs_diff(&mut ma, &mut mb), Some(0.5));

    let c = &1.000_000_2f32;
    let mut mc = &mut 1.000_000_2f32;
    assert_eq!(FloatDiff::ulps_diff(&a, &c), Some(2));
    assert_eq!(FloatDiff::ulps_diff(&mut ma, &c), Some(2));
    assert_eq!(FloatDiff::ulps_diff(&a, &mut mc), Some(2));
    assert_eq!(FloatDiff::ulps_diff(&mut ma, &mut mc), Some(2));
}

#[test]
fn refs_float_eq() {
    let a = &1.0f32;
    let b = &1.5f32;
    let mut ma = &mut 1.0f32;
    let mut mb = &mut 1.5f32;
    assert_float_eq!(&a, &b, abs <= 1.0);
    assert_float_eq!(&mut ma, &b, abs <= 1.0);
    assert_float_eq!(&a, &mut mb, abs <= 1.0);
    assert_float_eq!(&mut ma, &mut mb, abs <= 1.0);
    assert_float_eq!(&a, &b, abs_all <= 1.0);
    assert_float_eq!(&mut ma, &b, abs_all <= 1.0);
    assert_float_eq!(&a, &mut mb, abs_all <= 1.0);
    assert_float_eq!(&mut ma, &mut mb, abs_all <= 1.0);

    let c = &1.000_000_2f32;
    let mut mc = &mut 1.000_000_2f32;
    let eps = 2.0 * f32::EPSILON;
    assert_float_eq!(&a, &c, rel <= eps);
    assert_float_eq!(&mut ma, &c, rel <= eps);
    assert_float_eq!(&a, &mut mc, rel <= eps);
    assert_float_eq!(&mut ma, &mut mc, rel <= eps);
    assert_float_eq!(&a, &c, rel_all <= eps);
    assert_float_eq!(&mut ma, &c, rel_all <= eps);
    assert_float_eq!(&a, &mut mc, rel_all <= eps);
    assert_float_eq!(&mut ma, &mut mc, rel_all <= eps);

    assert_float_eq!(&a, &c, ulps <= 2);
    assert_float_eq!(&mut ma, &c, ulps <= 2);
    assert_float_eq!(&a, &mut mc, ulps <= 2);
    assert_float_eq!(&mut ma, &mut mc, ulps <= 2);
    assert_float_eq!(&a, &c, ulps_all <= 2);
    assert_float_eq!(&mut ma, &c, ulps_all <= 2);
    assert_float_eq!(&a, &mut mc, ulps_all <= 2);
    assert_float_eq!(&mut ma, &mut mc, ulps_all <= 2);
}

#[test]
fn refs_debug_float_eq() {
    let a = &1.0f32;
    let b = &2.0f32;
    let mut ma = &mut 1.0f32;
    let mut mb = &mut 2.0f32;

    assert_eq!(FloatEqDebug::debug_abs_epsilon(&a, &b, &0.5), 0.5);
    assert_eq!(FloatEqDebug::debug_abs_epsilon(&mut ma, &b, &0.5), 0.5);
    assert_eq!(FloatEqDebug::debug_abs_epsilon(&a, &mut mb, &0.5), 0.5);
    assert_eq!(FloatEqDebug::debug_abs_epsilon(&mut ma, &mut mb, &0.5), 0.5);

    assert_eq!(FloatEqAllDebug::debug_abs_all_epsilon(&a, &b, &0.5), 0.5);
    assert_eq!(
        FloatEqAllDebug::debug_abs_all_epsilon(&mut ma, &b, &0.5),
        0.5
    );
    assert_eq!(
        FloatEqAllDebug::debug_abs_all_epsilon(&a, &mut mb, &0.5),
        0.5
    );
    assert_eq!(
        FloatEqAllDebug::debug_abs_all_epsilon(&mut ma, &mut mb, &0.5),
        0.5
    );

    assert_eq!(FloatEqDebug::debug_rel_epsilon(&a, &b, &0.1), 0.2);
    assert_eq!(FloatEqDebug::debug_rel_epsilon(&mut ma, &b, &0.1), 0.2);
    assert_eq!(FloatEqDebug::debug_rel_epsilon(&a, &mut mb, &0.1), 0.2);
    assert_eq!(FloatEqDebug::debug_rel_epsilon(&mut ma, &mut mb, &0.1), 0.2);

    assert_eq!(FloatEqAllDebug::debug_rel_all_epsilon(&a, &b, &0.1), 0.2);
    assert_eq!(
        FloatEqAllDebug::debug_rel_all_epsilon(&mut ma, &b, &0.1),
        0.2
    );
    assert_eq!(
        FloatEqAllDebug::debug_rel_all_epsilon(&a, &mut mb, &0.1),
        0.2
    );
    assert_eq!(
        FloatEqAllDebug::debug_rel_all_epsilon(&mut ma, &mut mb, &0.1),
        0.2
    );

    assert_eq!(FloatEqDebug::debug_ulps_epsilon(&a, &b, &1), 1);
    assert_eq!(FloatEqDebug::debug_ulps_epsilon(&mut ma, &b, &1), 1);
    assert_eq!(FloatEqDebug::debug_ulps_epsilon(&a, &mut mb, &1), 1);
    assert_eq!(FloatEqDebug::debug_ulps_epsilon(&mut ma, &mut mb, &1), 1);

    assert_eq!(FloatEqAllDebug::debug_ulps_all_epsilon(&a, &b, &1), 1);
    assert_eq!(FloatEqAllDebug::debug_ulps_all_epsilon(&mut ma, &b, &1), 1);
    assert_eq!(FloatEqAllDebug::debug_ulps_all_epsilon(&a, &mut mb, &1), 1);
    assert_eq!(
        FloatEqAllDebug::debug_ulps_all_epsilon(&mut ma, &mut mb, &1),
        1
    );
}
