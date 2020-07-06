use float_eq::FloatEqDebugUlpsDiff;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
#[float_eq(debug_ulps_diff = "MyComplexUlpsDiff")]
struct MyComplexUlps<T> {
    re: T,
    im: T,
}

fn main() {}
