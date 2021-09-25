use float_eq::{AssertFloatEq, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsTol};

#[derive(
    Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq,
)]
#[float_eq(
    ulps_tol = "MyUnitTypeUlps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "MyUnitTypeDebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq"
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

fn debug_tol() {
    let a = MyUnitType {};
    let b = MyUnitType {};

    assert_eq!(a.debug_abs_tol(&b, &MyUnitType {}), MyUnitType {});
    assert_eq!(a.debug_rel_tol(&b, &MyUnitType {}), MyUnitType {});
    assert_eq!(a.debug_ulps_tol(&b, &MyUnitTypeUlps {}), MyUnitTypeUlps {});
}

fn main() {
    debug_diff();
    debug_tol();
}
