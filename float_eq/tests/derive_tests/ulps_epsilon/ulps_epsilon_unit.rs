use float_eq::FloatEqUlpsEpsilon;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon)]
#[float_eq(ulps_epsilon = "MyComplex32Ulps")]
struct MyComplex32();

fn main() {
    let a = MyComplex32Ulps();
    let b = a; // Clone, Copy

    // Debug, PartialEq
    assert_eq!(a, b);
}
