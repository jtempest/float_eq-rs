use float_eq::{assert_float_eq, FloatDiff, FloatEq, FloatEqDebug, FloatUlps};

#[derive(Debug, PartialEq, FloatUlps, FloatDiff, FloatEq, FloatEqDebug)]
#[float_eq(ulps = "MyTupleNoFieldsTypeUlps")]
struct MyTupleNoFieldsType();

fn float_ulps() {
    let ulps = MyTupleNoFieldsTypeUlps();
    assert_eq!(ulps, MyTupleNoFieldsTypeUlps());
}

fn float_diff() {
    let a = MyTupleNoFieldsType();
    assert_eq!(a.abs_diff(&a), MyTupleNoFieldsType());
    assert_eq!(a.ulps_diff(&a), Some(MyTupleNoFieldsTypeUlps()));

    let b = MyTupleNoFieldsType();
    assert_eq!(a.abs_diff(&b), MyTupleNoFieldsType());
    assert_eq!(a.ulps_diff(&b), Some(MyTupleNoFieldsTypeUlps()));
}

fn float_eq() {
    let a = MyTupleNoFieldsType();
    let b = MyTupleNoFieldsType();

    assert!(a.eq_abs(&b, &MyTupleNoFieldsType()));
    assert!(a.eq_rel(&b, &MyTupleNoFieldsType()));
    assert!(a.eq_ulps(&b, &MyTupleNoFieldsTypeUlps()));
}

fn float_eq_debug() {
    let a = MyTupleNoFieldsType();
    let b = MyTupleNoFieldsType();

    assert_eq!(
        a.debug_abs_epsilon(&b, &MyTupleNoFieldsType()),
        MyTupleNoFieldsType()
    );
    assert_eq!(
        a.debug_rel_epsilon(&b, &MyTupleNoFieldsType()),
        MyTupleNoFieldsType()
    );
    assert_eq!(
        a.debug_ulps_epsilon(&b, &MyTupleNoFieldsTypeUlps()),
        MyTupleNoFieldsTypeUlps()
    );
}

fn assert_float_eq() {
    let a = MyTupleNoFieldsType();
    let b = MyTupleNoFieldsType();

    assert_float_eq!(a, b, abs <= MyTupleNoFieldsType());
    assert_float_eq!(a, b, rel <= MyTupleNoFieldsType());
    assert_float_eq!(a, b, ulps <= MyTupleNoFieldsTypeUlps());
}

fn main() {
    float_ulps();
    float_diff();
    float_eq();
    float_eq_debug();
    assert_float_eq();
}
