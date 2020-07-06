use float_eq::{
    AssertFloatEq, AssertFloatEqAll, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff, FloatEqUlpsEpsilon,
};

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
    FloatEqAll,
    AssertFloatEqAll,
)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
