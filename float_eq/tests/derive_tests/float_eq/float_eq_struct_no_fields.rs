use float_eq::{FloatEq, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq)]
#[float_eq(ulps_tol = "MyNoFieldsTypeUlps")]
struct MyNoFieldsType;

fn main() {
    let a = MyNoFieldsType {};
    let b = MyNoFieldsType {};

    assert!(a.eq_abs(&b, &MyNoFieldsType {}));
    assert!(a.eq_rel(&b, &MyNoFieldsType {}));
    assert!(a.eq_ulps(&b, &MyNoFieldsTypeUlps {}));
}
