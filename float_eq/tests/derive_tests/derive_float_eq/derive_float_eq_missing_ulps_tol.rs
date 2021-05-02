use float_eq::derive_float_eq;

#[derive_float_eq(
    //ulps_tol = "MyComplex32Ulps",
    debug_ulps_diff = "MyComplex32UlpsDiff"
)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
