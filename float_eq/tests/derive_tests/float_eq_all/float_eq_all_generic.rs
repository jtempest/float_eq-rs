use float_eq::{FloatEq, FloatEqAll, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq, FloatEqAll)]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
