use core::fmt;
use float_eq::{FloatEqUlpsTol, UlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol)]
#[float_eq(
    ulps_tol = "MyComplex32Ulps",
    ulps_tol_derive = "Clone, Copy, PartialEq"
)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

impl fmt::Debug for MyComplex32Ulps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomDebug_MyComplex32Ulps({}, {})", self.re, self.im)
    }
}

fn main() {
    let a = UlpsTol::<MyComplex32> { re: 1, im: 2 };
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
    assert_ne!(a, MyComplex32Ulps { re: 3, im: 2 });
    assert_ne!(a, MyComplex32Ulps { re: 1, im: 3 });
    assert_eq!(
        format!("{:?}", MyComplex32Ulps { re: 1, im: 2 }),
        "CustomDebug_MyComplex32Ulps(1, 2)"
    );
}
