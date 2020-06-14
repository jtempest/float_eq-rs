use float_eq::{assert_float_eq, FloatDiff, FloatEq, FloatEqDebug, FloatUlps};

#[derive(Debug, PartialEq, FloatUlps, FloatDiff, FloatEq, FloatEqDebug)]
#[float_eq(ulps = "MyUnitTypeUlps")]
struct MyUnitType;

fn float_ulps() {
    let ulps = MyUnitTypeUlps {};
    assert_eq!(ulps, MyUnitTypeUlps {});
}

fn float_diff() {
    let a = MyUnitType {};
    assert_eq!(a.abs_diff(&a), MyUnitType {});
    assert_eq!(a.ulps_diff(&a), Some(MyUnitTypeUlps {}));

    let b = MyUnitType {};
    assert_eq!(a.abs_diff(&b), MyUnitType {});
    assert_eq!(a.ulps_diff(&b), Some(MyUnitTypeUlps {}));
}

fn float_eq() {
    let a = MyUnitType {};
    let b = MyUnitType {};

    assert!(a.eq_abs(&b, &MyUnitType {}));
    assert!(a.eq_rel(&b, &MyUnitType {}));
    assert!(a.eq_ulps(&b, &MyUnitTypeUlps {}));
}

fn float_eq_debug() {
    let a = MyUnitType {};
    let b = MyUnitType {};

    assert_eq!(a.debug_abs_epsilon(&b, &MyUnitType {}), MyUnitType {});
    assert_eq!(a.debug_rel_epsilon(&b, &MyUnitType {}), MyUnitType {});
    assert_eq!(
        a.debug_ulps_epsilon(&b, &MyUnitTypeUlps {}),
        MyUnitTypeUlps {}
    );
}

fn assert_float_eq() {
    let a = MyUnitType {};
    let b = MyUnitType {};

    assert_float_eq!(a, b, abs <= MyUnitType {});
    assert_float_eq!(a, b, rel <= MyUnitType {});
    assert_float_eq!(a, b, ulps <= MyUnitTypeUlps {});
}

fn main() {
    float_ulps();
    float_diff();
    float_eq();
    float_eq_debug();
    assert_float_eq();
}
