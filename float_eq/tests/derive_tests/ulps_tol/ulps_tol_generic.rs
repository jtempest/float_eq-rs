use float_eq::FloatEqUlpsTol;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol)]
#[float_eq(ulps_tol = "MyComplexUlps")]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
