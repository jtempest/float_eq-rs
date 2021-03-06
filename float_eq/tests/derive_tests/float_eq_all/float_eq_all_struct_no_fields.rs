use float_eq::{FloatEq, FloatEqAll, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq, FloatEqAll)]
#[float_eq(ulps_tol = "MyNoFieldsTypeUlps", all_tol = "f64")]
struct MyNoFieldsType;

fn main() {
    let a = MyNoFieldsType {};
    let b = MyNoFieldsType {};

    assert!(a.eq_abs_all(&b, &0.0));
    assert!(a.eq_rel_all(&b, &0.0));
    assert!(a.eq_ulps_all(&b, &0));
}
