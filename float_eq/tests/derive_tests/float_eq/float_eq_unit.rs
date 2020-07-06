use float_eq::{FloatEq, FloatEqUlpsEpsilon};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon, FloatEq)]
#[float_eq(ulps_epsilon = "MyUnitTypeUlps")]
struct MyUnitType();

fn main() {
    let a = MyUnitType {};
    let b = MyUnitType {};

    assert!(a.eq_abs(&b, &MyUnitType {}));
    assert!(a.eq_rel(&b, &MyUnitType {}));
    assert!(a.eq_ulps(&b, &MyUnitTypeUlps {}));
}
