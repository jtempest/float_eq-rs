use float_eq::FloatEqUlpsTol;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol)]
#[float_eq(ulps_tol = "MyComplex32Ulps")]
struct MyComplex32();

fn main() {
    let a = MyComplex32Ulps();
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
}
