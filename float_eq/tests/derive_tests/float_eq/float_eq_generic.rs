use float_eq::{FloatEq, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq)]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
