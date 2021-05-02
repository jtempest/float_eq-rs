use float_eq::{AssertFloatEq, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsTol};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    FloatEqUlpsTol,
    FloatEq,
    FloatEqDebugUlpsDiff,
    AssertFloatEq,
)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
