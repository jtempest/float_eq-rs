use float_eq::{AssertFloatEq, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsTol};

#[derive(
    Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq,
)]
#[float_eq(
    ulps_tol = "MyTupleTypeUlps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "MyTupleTypeDebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq"
)]
struct MyTupleType(f32, f64);

fn debug_diff() {
    let a = MyTupleType(1.0, 2.000_000_000_000_006_7);
    assert_eq!(a.debug_abs_diff(&a), MyTupleType(0.0, 0.0));
    assert_eq!(
        a.debug_ulps_diff(&a),
        MyTupleTypeDebugUlpsDiff(Some(0), Some(0))
    );

    let b = MyTupleType(1.000_000_1, 2.0);
    assert_eq!(
        a.debug_abs_diff(&b),
        MyTupleType(
            0.000_000_119_209_29,
            0.000_000_000_000_006_661_338_147_750_939
        )
    );
    assert_eq!(
        a.debug_ulps_diff(&b),
        MyTupleTypeDebugUlpsDiff(Some(1), Some(15))
    );

    let c = MyTupleType(1.000_000_1, -2.0);
    assert_eq!(
        a.debug_ulps_diff(&c),
        MyTupleTypeDebugUlpsDiff(Some(1), None)
    );
}

fn debug_tol() {
    let a = MyTupleType(1.0, 200.0);
    let b = MyTupleType(50.0, 1.0);

    assert_eq!(
        a.debug_abs_tol(&b, &MyTupleType(0.1, 0.2)),
        MyTupleType(0.1, 0.2)
    );
    assert_eq!(
        a.debug_rel_tol(&b, &MyTupleType(0.1, 0.2)),
        MyTupleType(5.0, 40.0)
    );
    assert_eq!(
        a.debug_ulps_tol(&b, &MyTupleTypeUlps(1, 2)),
        MyTupleTypeUlps(1, 2)
    );
}

fn main() {
    debug_diff();
    debug_tol();
}
