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
    ulps_epsilon = "MyNoFieldsTypeUlps",
    debug_ulps_diff = "MyNoFieldsTypeDebugUlpsDiff",
    all_epsilon = "f32"
)]
struct MyNoFieldsType;

fn main() {
    let a = MyNoFieldsType {};
    let b = MyNoFieldsType {};

    assert_eq!(a.debug_abs_all_epsilon(&b, &0.0), MyNoFieldsType {});
    assert_eq!(a.debug_rel_all_epsilon(&b, &0.0), MyNoFieldsType {});
    assert_eq!(a.debug_ulps_all_epsilon(&b, &0), MyNoFieldsTypeUlps {});
}
