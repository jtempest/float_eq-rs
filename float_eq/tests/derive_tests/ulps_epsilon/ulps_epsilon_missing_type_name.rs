use float_eq::FloatEqUlpsEpsilon;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
