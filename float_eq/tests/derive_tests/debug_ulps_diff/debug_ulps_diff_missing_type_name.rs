use float_eq::FloatEqDebugUlpsDiff;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
