use core::fmt;
use float_eq::{assert_float_eq, assert_float_ne, derive_float_eq};

#[derive_float_eq(
    ulps_tol = "MyComplex32Ulps",
    ulps_tol_derive = "Clone, Copy, PartialEq",
    debug_ulps_diff = "MyComplex32DebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, PartialEq",
    all_tol = "f32"
)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

impl fmt::Debug for MyComplex32Ulps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomDebug_MyComplex32Ulps({}, {})", self.re, self.im)
    }
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

impl MyComplex32 {
    fn new(re: f32, im: f32) -> MyComplex32 {
        MyComplex32 { re, im }
    }
}

fn main() {
    let a = MyComplex32 { re: 1.0, im: -2.0 };
    assert_float_eq!(a, a, abs_all <= 0.0);
    assert_float_eq!(a, a, rel_all <= 0.0);
    assert_float_eq!(a, a, ulps_all <= 0);

    let b = MyComplex32 {
        re: 1.000_000_1,
        im: -2.000_000_5,
    };

    assert_float_eq!(a, b, abs_all <= 0.000_000_55);
    assert_float_ne!(a, b, abs_all <= 0.000_000_45);

    assert_float_eq!(a, b, rel_all <= 0.000_000_25);
    assert_float_ne!(a, b, rel_all <= 0.000_000_15);

    assert_float_eq!(a, b, ulps_all <= 2);
    assert_float_ne!(a, b, ulps_all <= 1);

    assert_eq!(
        format!("{:?}", MyComplex32Ulps { re: 1, im: 2 }),
        "CustomDebug_MyComplex32Ulps(1, 2)"
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
