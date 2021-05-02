use float_eq::{
    AssertFloatEq, AssertFloatEqAll, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff, FloatEqUlpsTol,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    FloatEqUlpsTol,
    FloatEq,
    FloatEqDebugUlpsDiff,
    AssertFloatEq,
    FloatEqAll,
    AssertFloatEqAll,
)]
#[float_eq(
    ulps_tol = "MyUnitTypeUlps",
    debug_ulps_diff = "MyUnitTypeDebugUlpsDiff",
    all_tol = "f32"
)]
struct MyUnitType();

fn main() {
    let a = MyUnitType {};
    let b = MyUnitType {};

    assert_eq!(a.debug_abs_all_tol(&b, &0.0), MyUnitType {});
    assert_eq!(a.debug_rel_all_tol(&b, &0.0), MyUnitType {});
    assert_eq!(a.debug_ulps_all_tol(&b, &0), MyUnitTypeUlps {});
}
