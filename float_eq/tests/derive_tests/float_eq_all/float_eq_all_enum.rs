use float_eq::{FloatEq, FloatEqAll, FloatEqUlpsEpsilon};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, FloatEqUlpsEpsilon, FloatEq, FloatEqAll)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
