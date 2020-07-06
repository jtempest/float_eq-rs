use float_eq::{AssertFloatEq, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsEpsilon};

#[derive(
    Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq,
)]
#[float_eq(ulps_epsilon = "MyComplexUlps")]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
