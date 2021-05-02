use float_eq::{FloatEq, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq)]
#[float_eq(ulps_tol = "MyUnitTypeUlps")]
struct MyUnitType();

fn main() {
    let a = MyUnitType {};
    let b = MyUnitType {};

    assert!(a.eq_abs(&b, &MyUnitType {}));
    assert!(a.eq_rel(&b, &MyUnitType {}));
    assert!(a.eq_ulps(&b, &MyUnitTypeUlps {}));
}
