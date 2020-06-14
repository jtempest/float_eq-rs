use float_eq::FloatUlps;

#[derive(Debug, PartialEq, FloatUlps)]
#[float_eq(ulps = "MyComplexGenericUlps")]
struct MyComplexGeneric<T> {
    re: T,
    im: T,
}

fn main() {}
