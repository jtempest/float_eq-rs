use float_eq::FloatEqUlpsTol;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
