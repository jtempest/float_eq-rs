use float_eq::{AssertFloatEq, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsEpsilon};

#[derive(
    Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq,
)]
#[float_eq(
    ulps_epsilon = "MyUnitTypeUlps",
    debug_ulps_diff = "MyUnitTypeDebugUlpsDiff"
)]
struct MyUnitType();

fn debug_diff() {
    let a = MyUnitType {};
    assert_eq!(a.debug_abs_diff(&a), MyUnitType {});
    assert_eq!(a.debug_ulps_diff(&a), MyUnitTypeDebugUlpsDiff {});

    let b = MyUnitType {};
    assert_eq!(a.debug_abs_diff(&b), MyUnitType {});
    assert_eq!(a.debug_ulps_diff(&b), MyUnitTypeDebugUlpsDiff {});
}

fn debug_epsilon() {
    let a = MyUnitType {};
    let b = MyUnitType {};

    assert_eq!(a.debug_abs_epsilon(&b, &MyUnitType {}), MyUnitType {});
    assert_eq!(a.debug_rel_epsilon(&b, &MyUnitType {}), MyUnitType {});
    assert_eq!(
        a.debug_ulps_epsilon(&b, &MyUnitTypeUlps {}),
        MyUnitTypeUlps {}
    );
}

fn main() {
    debug_diff();
    debug_epsilon();
}
