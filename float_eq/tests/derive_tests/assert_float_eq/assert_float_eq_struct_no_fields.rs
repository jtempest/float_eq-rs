use float_eq::{AssertFloatEq, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsEpsilon};

#[derive(
    Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq,
)]
#[float_eq(
    ulps_epsilon = "MyNoFieldsTypeUlps",
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

fn debug_epsilon() {
    let a = MyNoFieldsType {};
    let b = MyNoFieldsType {};

    assert_eq!(
        a.debug_abs_epsilon(&b, &MyNoFieldsType {}),
        MyNoFieldsType {}
    );
    assert_eq!(
        a.debug_rel_epsilon(&b, &MyNoFieldsType {}),
        MyNoFieldsType {}
    );
    assert_eq!(
        a.debug_ulps_epsilon(&b, &MyNoFieldsTypeUlps {}),
        MyNoFieldsTypeUlps {}
    );
}

fn main() {
    debug_diff();
    debug_epsilon();
}
