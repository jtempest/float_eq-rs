#![cfg(feature = "derive")]

use float_eq::{assert_float_eq, derive_float_eq};

#[derive_float_eq(ulps_tol = "MyComplex32Ulps", debug_ulps_diff = "MyComplex32UlpsDiff")]
#[derive(Debug, Clone, Copy, PartialEq)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

#[test]
#[should_panic(expected = "`float_eq!(left, right, ulps <= t)`
        left: `MyComplex32 { re: 1.0, im: 2.0 }`,
       right: `MyComplex32 { re: 1.0000005, im: -5.0 }`,
    abs_diff: `MyComplex32 { re: 0.00000047683716, im: 7.0 }`,
   ulps_diff: `MyComplex32UlpsDiff { re: Some(4), im: None }`,
    [ulps] t: `MyComplex32Ulps { re: 5, im: 2 }`")]
fn failed_assert() {
    assert_float_eq!(
        MyComplex32 { re: 1.0, im: 2.0 },
        MyComplex32 {
            re: 1.000_000_5,
            im: -5.0
        },
        ulps <= MyComplex32Ulps { re: 5, im: 2 },
    );
}
