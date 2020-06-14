use float_eq::{
    assert_float_eq, assert_float_ne, FloatDiff, FloatEq, FloatEqDebug, FloatUlps, Ulps,
};

#[derive(Debug, PartialEq, FloatUlps, FloatDiff, FloatEq, FloatEqDebug)]
#[float_eq(ulps = "MyTupleTypeUlps")]
struct MyTupleType(f32, f64);

fn float_ulps() {
    let ulps = MyTupleTypeUlps(1, 2);
    assert_eq!(ulps, Ulps::<MyTupleType> { 0: 1, 1: 2 });
}

fn float_diff() {
    let a = MyTupleType(1.0, 2.000_000_000_000_006_7);
    assert_eq!(a.abs_diff(&a), MyTupleType(0.0, 0.0));
    assert_eq!(a.ulps_diff(&a), Some(MyTupleTypeUlps(0, 0)));

    let b = MyTupleType(1.000_000_1, 2.0);
    assert_eq!(
        a.abs_diff(&b),
        MyTupleType(
            0.000_000_119_209_29,
            0.000_000_000_000_006_661_338_147_750_939
        )
    );
    assert_eq!(a.ulps_diff(&b), Some(MyTupleTypeUlps(1, 15)));
}

fn float_eq() {
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

fn float_eq_debug() {
    let a = MyTupleType(1.0, 200.0);
    let b = MyTupleType(50.0, 1.0);

    assert_eq!(
        a.debug_abs_epsilon(&b, &MyTupleType(0.1, 0.2)),
        MyTupleType(0.1, 0.2)
    );
    assert_eq!(
        a.debug_rel_epsilon(&b, &MyTupleType(0.1, 0.2)),
        MyTupleType(5.0, 40.0)
    );
    assert_eq!(
        a.debug_ulps_epsilon(&b, &MyTupleTypeUlps(1, 2)),
        MyTupleTypeUlps(1, 2)
    );
}

fn assert_float_eq() {
    let a = MyTupleType(2.0, -1_000_000.);
    let b = MyTupleType(2.000_000_5, -1_000_000.000_000_000_1);

    assert_float_eq!(a, b, abs <= MyTupleType(0.000_000_5, 0.000_000_000_2));
    assert_float_ne!(a, b, abs <= MyTupleType(0.000_000_4, 0.000_000_000_2));
    assert_float_ne!(a, b, abs <= MyTupleType(0.000_000_5, 0.000_000_000_1));

    assert_float_eq!(
        a,
        b,
        rel <= MyTupleType(0.000_000_25, 0.000_000_000_000_000_2)
    );
    assert_float_ne!(
        a,
        b,
        rel <= MyTupleType(0.000_000_15, 0.000_000_000_000_000_2)
    );
    assert_float_ne!(
        a,
        b,
        rel <= MyTupleType(0.000_000_25, 0.000_000_000_000_000_1)
    );

    assert_float_eq!(a, b, ulps <= MyTupleTypeUlps(2, 1));
    assert_float_ne!(a, b, ulps <= MyTupleTypeUlps(0, 1));
    assert_float_ne!(a, b, ulps <= MyTupleTypeUlps(2, 0));
}

fn main() {
    float_ulps();
    float_diff();
    float_eq();
    float_eq_debug();
    assert_float_eq();
}
