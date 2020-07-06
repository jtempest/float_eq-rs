use float_eq::{DebugUlpsDiff, FloatEqDebugUlpsDiff};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
#[float_eq(debug_ulps_diff = "MyComplex32DebugUlpsDiff")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {
    let a = DebugUlpsDiff::<MyComplex32> {
        re: Some(1),
        im: Some(2),
    };
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
    assert_ne!(
        a,
        MyComplex32DebugUlpsDiff {
            re: None,
            im: Some(2)
        }
    );
    assert_ne!(
        a,
        MyComplex32DebugUlpsDiff {
            re: Some(1),
            im: Some(3)
        }
    );
}
