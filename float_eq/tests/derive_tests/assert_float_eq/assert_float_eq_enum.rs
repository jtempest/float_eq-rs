use float_eq::{AssertFloatEq, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsEpsilon};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    FloatEqUlpsEpsilon,
    FloatEq,
    FloatEqDebugUlpsDiff,
    AssertFloatEq,
)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
