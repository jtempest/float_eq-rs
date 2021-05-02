use float_eq::FloatEqUlpsTol;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol)]
#[float_eq(ulps_tol = "MyComplex32Ulps")]
struct MyComplex32(f32, f32);

fn main() {
    let a = MyComplex32Ulps(1, 2);
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
    assert_ne!(a, MyComplex32Ulps(3, 2));
    assert_ne!(a, MyComplex32Ulps(1, 3));
}
