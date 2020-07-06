use float_eq::FloatEqDebugUlpsDiff;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
#[float_eq(debug_ulps_diff = "MyComplex32DebugUlpsDiff")]
struct MyComplex32(f32, f32);

fn main() {
    let a = MyComplex32DebugUlpsDiff(Some(1), Some(2));
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
    assert_ne!(a, MyComplex32DebugUlpsDiff(None, Some(2)));
    assert_ne!(a, MyComplex32DebugUlpsDiff(Some(1), Some(3)));
}
