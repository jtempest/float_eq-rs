use float_eq::{FloatEq, FloatEqUlpsEpsilon};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, FloatEqUlpsEpsilon, FloatEq)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
