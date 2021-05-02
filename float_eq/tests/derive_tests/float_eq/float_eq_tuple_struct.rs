use float_eq::{FloatEq, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq)]
#[float_eq(ulps_tol = "MyTupleTypeUlps")]
struct MyTupleType(f32, f64);

fn main() {
    let a = MyTupleType(2.0, -1_000_000.0);
    let b = MyTupleType(2.000_000_5, -1_000_000.000_000_000_1);

    assert!(a.eq_abs(&b, &MyTupleType(0.000_000_5, 0.000_000_000_2)));
    assert!(a.ne_abs(&b, &MyTupleType(0.000_000_4, 0.000_000_000_2)));
    assert!(a.ne_abs(&b, &MyTupleType(0.000_000_5, 0.000_000_000_05)));

    assert!(a.eq_rel(&b, &MyTupleType(0.000_000_25, 0.000_000_000_000_000_2)));
    assert!(a.ne_rel(&b, &MyTupleType(0.000_000_15, 0.000_000_000_000_000_2)));
    assert!(a.ne_rel(&b, &MyTupleType(0.000_000_25, 0.000_000_000_000_000_1)));

    assert!(a.eq_ulps(&b, &MyTupleTypeUlps(2, 1)));
    assert!(a.ne_ulps(&b, &MyTupleTypeUlps(1, 1)));
    assert!(a.ne_ulps(&b, &MyTupleTypeUlps(2, 0)));
}
