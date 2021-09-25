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
    ulps_tol = "MyComplex32Ulps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "MyComplex32DebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq",
    all_tol = "f32"
)]
struct MyComplex32(f32, f32);

fn main() {
    let a = MyComplex32(1.0, 200.0);
    let b = MyComplex32(50.0, 1.0);

    assert_eq!(a.debug_abs_all_tol(&b, &0.2), MyComplex32(0.2, 0.2));
    assert_eq!(a.debug_rel_all_tol(&b, &0.2), MyComplex32(10.0, 40.0));
    assert_eq!(a.debug_ulps_all_tol(&b, &2), MyComplex32Ulps(2, 2));
}
