use float_eq::FloatUlps;

#[derive(Debug, PartialEq, FloatUlps)]
#[float_eq(ulps = "MyComplexGenericUlps")]
struct MyComplexGeneric<T>(T, T, f64);

fn main() {}
