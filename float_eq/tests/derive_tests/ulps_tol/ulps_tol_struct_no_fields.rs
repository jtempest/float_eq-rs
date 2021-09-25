use float_eq::{FloatEqUlpsTol, UlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol)]
#[float_eq(
    ulps_tol = "MyComplex32Ulps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq"
)]
struct MyComplex32;

fn main() {
    let a = UlpsTol::<MyComplex32> {};
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
}
