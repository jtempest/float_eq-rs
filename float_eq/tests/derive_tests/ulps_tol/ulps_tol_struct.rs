use float_eq::{FloatEqUlpsTol, UlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol)]
#[float_eq(ulps_tol = "MyComplex32Ulps")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {
    let a = UlpsTol::<MyComplex32> { re: 1, im: 2 };
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
    assert_ne!(a, MyComplex32Ulps { re: 3, im: 2 });
    assert_ne!(a, MyComplex32Ulps { re: 1, im: 3 });
}
