use float_eq::FloatEqDebugUlpsDiff;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
#[float_eq(debug_ulps_diff = "MyComplex32DebugUlpsDiff")]
struct MyComplex32();

fn main() {
    let a = MyComplex32DebugUlpsDiff();
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
}
