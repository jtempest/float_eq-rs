use float_eq::FloatEqUlpsTol;

#[derive(Debug, PartialEq, FloatEqUlpsTol)]
#[float_eq]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
