use float_eq::FloatEq;

#[derive(Debug, PartialEq, FloatEq)]
#[float_eq(ulps = "MyComplexGenericUlps")]
struct MyComplexGeneric<T> {
    re: T,
    im: T,
}

fn main() {}
