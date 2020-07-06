use float_eq::FloatEqUlpsEpsilon;

#[derive(Debug, PartialEq, FloatEqUlpsEpsilon)]
#[float_eq(ulps_epsilon = "MyComplex32Ulps", cheese = "Hello")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
