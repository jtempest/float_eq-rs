use float_eq::{
    AssertFloatEq, AssertFloatEqAll, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff, FloatEqUlpsEpsilon,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    FloatEqUlpsEpsilon,
    FloatEq,
    FloatEqDebugUlpsDiff,
    AssertFloatEq,
    FloatEqAll,
    AssertFloatEqAll,
)]
#[float_eq(
    ulps_epsilon = "MyUnitTypeUlps",
    debug_ulps_diff = "MyUnitTypeDebugUlpsDiff",
    all_epsilon = "f32"
)]
struct MyUnitType();

fn main() {
    let a = MyUnitType {};
    let b = MyUnitType {};

    assert_eq!(a.debug_abs_all_epsilon(&b, &0.0), MyUnitType {});
    assert_eq!(a.debug_rel_all_epsilon(&b, &0.0), MyUnitType {});
    assert_eq!(a.debug_ulps_all_epsilon(&b, &0), MyUnitTypeUlps {});
}
