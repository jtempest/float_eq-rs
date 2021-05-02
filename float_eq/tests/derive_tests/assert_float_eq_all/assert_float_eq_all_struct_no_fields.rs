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
    ulps_tol = "MyNoFieldsTypeUlps",
    debug_ulps_diff = "MyNoFieldsTypeDebugUlpsDiff",
    all_tol = "f32"
)]
struct MyNoFieldsType;

fn main() {
    let a = MyNoFieldsType {};
    let b = MyNoFieldsType {};

    assert_eq!(a.debug_abs_all_tol(&b, &0.0), MyNoFieldsType {});
    assert_eq!(a.debug_rel_all_tol(&b, &0.0), MyNoFieldsType {});
    assert_eq!(a.debug_ulps_all_tol(&b, &0), MyNoFieldsTypeUlps {});
}
