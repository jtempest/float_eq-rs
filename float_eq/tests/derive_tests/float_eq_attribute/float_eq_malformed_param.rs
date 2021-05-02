use float_eq::FloatEqUlpsTol;

#[derive(Debug, PartialEq, FloatEqUlpsTol)]
#[float_eq(ulps = "MyComplex32Ulps", cheese)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
