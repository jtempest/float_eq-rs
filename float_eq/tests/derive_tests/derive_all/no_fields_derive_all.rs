use float_eq::{assert_float_eq, FloatDiff, FloatEq, FloatEqDebug, FloatUlps};

#[derive(Debug, PartialEq, FloatUlps, FloatDiff, FloatEq, FloatEqDebug)]
#[float_eq(ulps = "MyNoFieldsTypeUlps")]
struct MyNoFieldsType {}

fn float_ulps() {
    let ulps = MyNoFieldsTypeUlps {};
    assert_eq!(ulps, MyNoFieldsTypeUlps {});
}

fn float_diff() {
    let a = MyNoFieldsType {};
    assert_eq!(a.abs_diff(&a), MyNoFieldsType {});
    assert_eq!(a.ulps_diff(&a), Some(MyNoFieldsTypeUlps {}));

    let b = MyNoFieldsType {};
    assert_eq!(a.abs_diff(&b), MyNoFieldsType {});
    assert_eq!(a.ulps_diff(&b), Some(MyNoFieldsTypeUlps {}));
}

fn float_eq() {
    let a = MyNoFieldsType {};
    let b = MyNoFieldsType {};

    assert!(a.eq_abs(&b, &MyNoFieldsType {}));
    assert!(a.eq_rel(&b, &MyNoFieldsType {}));
    assert!(a.eq_ulps(&b, &MyNoFieldsTypeUlps {}));
}

fn float_eq_debug() {
    let a = MyNoFieldsType {};
    let b = MyNoFieldsType {};

    assert_eq!(
        a.debug_abs_epsilon(&b, &MyNoFieldsType {}),
        MyNoFieldsType {}
    );
    assert_eq!(
        a.debug_rel_epsilon(&b, &MyNoFieldsType {}),
        MyNoFieldsType {}
    );
    assert_eq!(
        a.debug_ulps_epsilon(&b, &MyNoFieldsTypeUlps {}),
        MyNoFieldsTypeUlps {}
    );
}

fn assert_float_eq() {
    let a = MyNoFieldsType {};
    let b = MyNoFieldsType {};

    assert_float_eq!(a, b, abs <= MyNoFieldsType {});
    assert_float_eq!(a, b, rel <= MyNoFieldsType {});
    assert_float_eq!(a, b, ulps <= MyNoFieldsTypeUlps {});
}

fn main() {
    float_ulps();
    float_diff();
    float_eq();
    float_eq_debug();
    assert_float_eq();
}
