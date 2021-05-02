use float_eq::{assert_float_eq, assert_float_ne, derive_float_eq, UlpsTol};

#[derive_float_eq(
    ulps_tol = "MyComplex32Ulps",
    debug_ulps_diff = "MyComplex32UlpsDiff"
)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

impl MyComplex32 {
    fn new(re: f32, im: f32) -> MyComplex32 {
        MyComplex32 { re, im }
    }
}

impl MyComplex32Ulps {
    fn new(re: UlpsTol<f32>, im: UlpsTol<f32>) -> MyComplex32Ulps {
        MyComplex32Ulps { re, im }
    }
}

fn main() {
    let a = MyComplex32 { re: 1.0, im: -2.0 };
    assert_float_eq!(a, a, abs <= MyComplex32 { re: 0.0, im: 0.0 });
    assert_float_eq!(a, a, rel <= MyComplex32 { re: 0.0, im: 0.0 });
    assert_float_eq!(a, a, ulps <= MyComplex32Ulps { re: 0, im: 0 });

    let b = MyComplex32 {
        re: 1.000_000_1,
        im: -2.000_000_5,
    };

    assert_float_eq!(a, b, abs <= MyComplex32::new(0.000_000_15, 0.000_000_55));
    assert_float_ne!(a, b, abs <= MyComplex32::new(0.000_000_05, 0.000_000_55));
    assert_float_ne!(a, b, abs <= MyComplex32::new(0.000_000_15, 0.000_000_45));

    assert_float_eq!(a, b, rel <= MyComplex32::new(0.000_000_15, 0.000_000_25));
    assert_float_ne!(a, b, rel <= MyComplex32::new(0.000_000_05, 0.000_000_25));
    assert_float_ne!(a, b, rel <= MyComplex32::new(0.000_000_15, 0.000_000_15));

    assert_float_eq!(a, b, ulps <= MyComplex32Ulps::new(1, 2));
    assert_float_ne!(a, b, ulps <= MyComplex32Ulps::new(0, 2));
    assert_float_ne!(a, b, ulps <= MyComplex32Ulps::new(1, 1));
}
