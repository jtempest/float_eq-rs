use float_eq::{FloatEq, FloatEqAll, FloatEqUlpsEpsilon};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon, FloatEq, FloatEqAll)]
#[float_eq(
    ulps_epsilon = "MyComplex32Ulps",
    all_epsilon = "Epsilon1",
    all_epsilon = "Epsilon2"
)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
