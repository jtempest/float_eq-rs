use float_eq::{
    AssertFloatEq, AssertFloatEqAll, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff, FloatEqUlpsEpsilon,
    UlpsEpsilon,
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
struct MyComplex32 {
    re: f32,
    im: f32,
}

impl MyComplex32 {
    fn new(re: f32, im: f32) -> MyComplex32 {
        MyComplex32 { re, im }
    }
}

impl MyComplex32Ulps {
    fn new(re: UlpsEpsilon<f32>, im: UlpsEpsilon<f32>) -> MyComplex32Ulps {
        MyComplex32Ulps { re, im }
    }
}

fn main() {
    let a = MyComplex32 { re: 1.0, im: 200.0 };
    let b = MyComplex32 { re: 50.0, im: 1.0 };

    assert_eq!(
        a.debug_abs_all_epsilon(&b, &0.2),
        MyComplex32::new(0.2, 0.2)
    );
    assert_eq!(
        a.debug_rel_all_epsilon(&b, &0.2),
        MyComplex32::new(10.0, 40.0)
    );
    assert_eq!(a.debug_ulps_all_epsilon(&b, &2), MyComplex32Ulps::new(2, 2));
}
