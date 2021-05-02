use float_eq::{FloatEq, FloatEqUlpsTol};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, FloatEqUlpsTol, FloatEq)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
