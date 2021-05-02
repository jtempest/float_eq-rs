use float_eq::{AssertFloatEq, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsTol};

#[derive(
    Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq,
)]
#[float_eq(
    ulps_tol = "MyNoFieldsTypeUlps",
    debug_ulps_diff = "MyNoFieldsTypeDebugUlpsDiff"
)]
struct MyNoFieldsType;

fn debug_diff() {
    let a = MyNoFieldsType {};
    assert_eq!(a.debug_abs_diff(&a), MyNoFieldsType {});
    assert_eq!(a.debug_ulps_diff(&a), MyNoFieldsTypeDebugUlpsDiff {});

    let b = MyNoFieldsType {};
    assert_eq!(a.debug_abs_diff(&b), MyNoFieldsType {});
    assert_eq!(a.debug_ulps_diff(&b), MyNoFieldsTypeDebugUlpsDiff {});
}

fn debug_tol() {
    let a = MyNoFieldsType {};
    let b = MyNoFieldsType {};

    assert_eq!(
        a.debug_abs_tol(&b, &MyNoFieldsType {}),
        MyNoFieldsType {}
    );
    assert_eq!(
        a.debug_rel_tol(&b, &MyNoFieldsType {}),
        MyNoFieldsType {}
    );
    assert_eq!(
        a.debug_ulps_tol(&b, &MyNoFieldsTypeUlps {}),
        MyNoFieldsTypeUlps {}
    );
}

fn main() {
    debug_diff();
    debug_tol();
}
