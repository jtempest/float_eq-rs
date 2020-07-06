use float_eq::FloatEqUlpsEpsilon;

#[derive(Debug, PartialEq, FloatEqUlpsEpsilon)]
#[float_eq]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
