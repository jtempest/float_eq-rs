use core::fmt;
use float_eq::{DebugUlpsDiff, FloatEqDebugUlpsDiff};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
#[float_eq(
    debug_ulps_diff = "MyComplex32DebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, PartialEq"
)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

impl fmt::Debug for MyComplex32DebugUlpsDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CustomDebug_MyComplex32DebugUlpsDiff({:?}, {:?})",
            self.re, self.im
        )
    }
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
    assert_eq!(
        format!(
            "{:?}",
            MyComplex32DebugUlpsDiff {
                re: Some(1),
                im: None
            }
        ),
        "CustomDebug_MyComplex32DebugUlpsDiff(Some(1), None)"
    );
}
