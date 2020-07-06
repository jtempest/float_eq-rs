use float_eq::{assert_float_eq, assert_float_ne, derive_float_eq};

#[derive_float_eq(
    ulps_epsilon = "MyComplex32Ulps",
    debug_ulps_diff = "MyComplex32UlpsDiff",
    all_epsilon = "f32"
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
}
