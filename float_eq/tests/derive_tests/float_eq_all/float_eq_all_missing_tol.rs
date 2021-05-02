use float_eq::{FloatEq, FloatEqAll, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq, FloatEqAll)]
#[float_eq(ulps_tol = "MyComplex32Ulps")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
