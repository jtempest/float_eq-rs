use float_eq::{FloatEq, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq)]
#[float_eq(ulps_tol = "MyComplexUlps")]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
