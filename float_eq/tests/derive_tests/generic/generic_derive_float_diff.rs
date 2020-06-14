use float_eq::FloatDiff;

#[derive(Debug, PartialEq, FloatDiff)]
#[float_eq(ulps = "MyComplexGenericUlps")]
struct MyComplexGeneric<T> {
    re: T,
    im: T,
}

fn main() {}
