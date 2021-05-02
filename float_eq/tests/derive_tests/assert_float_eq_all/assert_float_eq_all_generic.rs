use float_eq::{
    AssertFloatEq, AssertFloatEqAll, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff, FloatEqUlpsTol,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    FloatEqUlpsTol,
    FloatEq,
    FloatEqDebugUlpsDiff,
    AssertFloatEq,
    FloatEqAll,
    AssertFloatEqAll,
)]
#[float_eq(ulps_tol = "MyComplexUlps")]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
