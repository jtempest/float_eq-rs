use float_eq::{FloatEq, FloatEqAll, FloatEqUlpsTol};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, FloatEqUlpsTol, FloatEq, FloatEqAll)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
