use float_eq::{DebugUlpsDiff, FloatEqDebugUlpsDiff};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
#[float_eq(debug_ulps_diff = "MyComplex32DebugUlpsDiff")]
struct MyComplex32;

fn main() {
    let a = DebugUlpsDiff::<MyComplex32> {};
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
}
