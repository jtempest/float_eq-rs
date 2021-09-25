use float_eq::FloatEqDebugUlpsDiff;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
