use float_eq::{
    assert_float_eq, assert_float_ne, FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug,
    FloatEqDebug, FloatUlps, Ulps,
};

#[derive(Debug, PartialEq, FloatDiff, FloatEq, FloatEqDebug, FloatEqAll, FloatEqAllDebug)]
#[float_eq(ulps = "MyComplex32Ulps", all_epsilon = "f32")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

#[derive(Debug, PartialEq)]
struct MyComplex32Ulps {
    re: u32,
    im: u32,
}

impl FloatUlps for MyComplex32 {
    type Ulps = MyComplex32Ulps;
}

impl MyComplex32 {
    fn new(re: f32, im: f32) -> MyComplex32 {
        MyComplex32 { re, im }
    }
}

impl MyComplex32Ulps {
    fn new(re: Ulps<f32>, im: Ulps<f32>) -> MyComplex32Ulps {
        MyComplex32Ulps { re, im }
    }
}

fn float_ulps() {
    let ulps = MyComplex32Ulps { re: 1, im: 2 };
    assert_eq!(ulps, Ulps::<MyComplex32> { re: 1, im: 2 });
}

fn float_diff() {
    let a = MyComplex32::new(1.0, 2.000_003_6);
    assert_eq!(a.abs_diff(&a), MyComplex32::new(0.0, 0.0));
    assert_eq!(a.ulps_diff(&a), Some(Ulps::<MyComplex32> { re: 0, im: 0 }));

    let b = MyComplex32::new(1.000_000_1, 2.0);
    assert_eq!(
        a.abs_diff(&b),
        MyComplex32::new(0.000_000_119_209_29, 0.000_003_576_278_7)
    );
    assert_eq!(a.ulps_diff(&b), Some(Ulps::<MyComplex32> { re: 1, im: 15 }));
}

fn float_eq() {
    let a = MyComplex32::new(2.0, -1_000_000.);
    let b = MyComplex32::new(2.000_000_5, -1_000_000.06);

    assert!(a.eq_abs(&b, &MyComplex32::new(0.000_000_5, 0.07)));
    assert!(a.ne_abs(&b, &MyComplex32::new(0.000_000_4, 0.07)));
    assert!(a.ne_abs(&b, &MyComplex32::new(0.000_000_5, 0.06)));

    assert!(a.eq_rel(&b, &MyComplex32::new(0.000_000_25, 0.000_000_1)));
    assert!(a.ne_rel(&b, &MyComplex32::new(0.000_000_15, 0.000_000_1)));
    assert!(a.ne_rel(&b, &MyComplex32::new(0.000_000_25, 0.000_000_05)));

    assert!(a.eq_ulps(&b, &Ulps::<MyComplex32> { re: 2, im: 1 }));
    assert!(a.ne_ulps(&b, &Ulps::<MyComplex32> { re: 1, im: 1 }));
    assert!(a.ne_ulps(&b, &Ulps::<MyComplex32> { re: 2, im: 0 }));
}

fn float_eq_debug() {
    let a = MyComplex32::new(1.0, 200.0);
    let b = MyComplex32::new(50.0, 1.0);

    assert_eq!(
        a.debug_abs_epsilon(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(0.1, 0.2)
    );
    assert_eq!(
        a.debug_rel_epsilon(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(5.0, 40.0)
    );
    assert_eq!(
        a.debug_ulps_epsilon(&b, &Ulps::<MyComplex32> { re: 1, im: 2 }),
        Ulps::<MyComplex32> { re: 1, im: 2 }
    );
}

fn float_eq_all() {
    let a = MyComplex32::new(2.0, -1_000_000.);
    let b = MyComplex32::new(2.000_000_5, -1_000_000.06);

    assert!(a.eq_abs_all(&b, &0.07));
    assert!(a.ne_abs_all(&b, &0.06));

    assert!(a.eq_rel_all(&b, &0.000_000_25));
    assert!(a.ne_rel_all(&b, &0.000_000_15));

    assert!(a.eq_ulps_all(&b, &2));
    assert!(a.ne_ulps_all(&b, &1));
}

fn float_eq_all_debug() {
    let a = MyComplex32 { re: 1.0, im: 200.0 };
    let b = MyComplex32 { re: 50.0, im: 1.0 };

    assert_eq!(
        a.debug_abs_all_epsilon(&b, &0.2),
        MyComplex32::new(0.2, 0.2)
    );
    assert_eq!(
        a.debug_rel_all_epsilon(&b, &0.2),
        MyComplex32::new(10.0, 40.0)
    );
    assert_eq!(a.debug_ulps_all_epsilon(&b, &2), MyComplex32Ulps::new(2, 2));
}

fn assert_float_eq() {
    let a = MyComplex32::new(2.0, -1_000_000.);
    let b = MyComplex32::new(2.000_000_5, -1_000_000.06);

    assert_float_eq!(a, b, abs <= MyComplex32::new(0.000_000_5, 0.07));
    assert_float_ne!(a, b, abs <= MyComplex32::new(0.000_000_4, 0.07));
    assert_float_ne!(a, b, abs <= MyComplex32::new(0.000_000_5, 0.06));
    assert_float_eq!(a, b, abs_all <= 0.07);
    assert_float_ne!(a, b, abs_all <= 0.06);

    assert_float_eq!(a, b, rel <= MyComplex32::new(0.000_000_25, 0.000_000_1));
    assert_float_ne!(a, b, rel <= MyComplex32::new(0.000_000_15, 0.000_000_1));
    assert_float_ne!(a, b, rel <= MyComplex32::new(0.000_000_25, 0.000_000_05));
    assert_float_eq!(a, b, rel_all <= 0.000_000_25);
    assert_float_ne!(a, b, rel_all <= 0.000_000_15);

    assert_float_eq!(a, b, ulps <= MyComplex32Ulps::new(2, 1));
    assert_float_ne!(a, b, ulps <= MyComplex32Ulps::new(0, 1));
    assert_float_ne!(a, b, ulps <= MyComplex32Ulps::new(2, 0));
    assert_float_eq!(a, b, ulps_all <= 2);
    assert_float_ne!(a, b, ulps_all <= 1);
}

fn main() {
    float_ulps();
    float_diff();
    float_eq();
    float_eq_debug();
    float_eq_all();
    float_eq_all_debug();
    assert_float_eq();
}
