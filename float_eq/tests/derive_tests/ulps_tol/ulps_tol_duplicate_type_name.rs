use float_eq::FloatEqUlpsTol;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol)]
#[float_eq(
    ulps_tol = "Name1",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    ulps_tol = "Name2"
)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
