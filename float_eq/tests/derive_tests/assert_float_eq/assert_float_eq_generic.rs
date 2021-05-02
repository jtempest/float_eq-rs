use float_eq::{AssertFloatEq, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsTol};

#[derive(
    Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq,
)]
#[float_eq(ulps_tol = "MyComplexUlps")]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
