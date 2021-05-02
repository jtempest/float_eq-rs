use float_eq::{FloatEq, FloatEqAll, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq, FloatEqAll)]
#[float_eq(ulps_tol = "MyComplexUlps")]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
