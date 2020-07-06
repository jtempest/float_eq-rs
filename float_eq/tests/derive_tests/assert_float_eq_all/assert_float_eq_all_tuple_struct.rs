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
    ulps_epsilon = "MyComplex32Ulps",
    debug_ulps_diff = "MyComplex32DebugUlpsDiff",
    all_epsilon = "f32"
)]
struct MyComplex32(f32, f32);

fn main() {
    let a = MyComplex32(1.0, 200.0);
    let b = MyComplex32(50.0, 1.0);

    assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), MyComplex32(0.2, 0.2));
    assert_eq!(a.debug_rel_all_epsilon(&b, &0.2), MyComplex32(10.0, 40.0));
    assert_eq!(a.debug_ulps_all_epsilon(&b, &2), MyComplex32Ulps(2, 2));
}
