use float_eq::FloatEqDebug;

#[derive(Debug, PartialEq, FloatEqDebug)]
#[float_eq(ulps = "MyComplexGenericUlps")]
struct MyComplexGeneric<T> {
    re: T,
    im: T,
}

fn main() {}
