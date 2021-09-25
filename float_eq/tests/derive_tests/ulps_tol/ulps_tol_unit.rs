use float_eq::FloatEqUlpsTol;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol)]
#[float_eq(
    ulps_tol = "MyComplex32Ulps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq"
)]
struct MyComplex32();

fn main() {
    let a = MyComplex32Ulps();
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
}
