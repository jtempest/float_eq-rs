use float_eq::FloatEqUlpsTol;

#[derive(Debug, PartialEq, FloatEqUlpsTol)]
#[float_eq(ulps_tol = "MyComplex32Ulps", cheese = "Hello")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
