use float_eq::{FloatEq, FloatEqAll, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq, FloatEqAll)]
#[float_eq(ulps_tol = "MyUnitTypeUlps", all_tol = "f32")]
struct MyUnitType();

fn main() {
    let a = MyUnitType {};
    let b = MyUnitType {};

    assert!(a.eq_abs_all(&b, &0.0));
    assert!(a.eq_rel_all(&b, &0.0));
    assert!(a.eq_ulps_all(&b, &0));
}
