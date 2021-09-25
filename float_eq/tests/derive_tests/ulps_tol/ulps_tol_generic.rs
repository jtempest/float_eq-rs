use float_eq::FloatEqUlpsTol;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol)]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
