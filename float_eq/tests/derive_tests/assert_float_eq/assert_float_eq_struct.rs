use float_eq::{
    AssertFloatEq, DebugUlpsDiff, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsTol, UlpsTol,
};

#[derive(
    Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq,
)]
#[float_eq(
    ulps_tol = "MyComplex32Ulps",
    debug_ulps_diff = "MyComplex32DebugUlpsDiff"
)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

impl MyComplex32 {
    fn new(re: f32, im: f32) -> MyComplex32 {
        MyComplex32 { re, im }
    }
}

impl MyComplex32Ulps {
    fn new(re: UlpsTol<f32>, im: UlpsTol<f32>) -> MyComplex32Ulps {
        MyComplex32Ulps { re, im }
    }
}

fn debug_diff() {
    let a = MyComplex32::new(1.0, 2.000_003_6);
    assert_eq!(a.debug_abs_diff(&a), MyComplex32::new(0.0, 0.0));
    assert_eq!(
        a.debug_ulps_diff(&a),
        DebugUlpsDiff::<MyComplex32> {
            re: Some(0),
            im: Some(0)
        }
    );

    let b = MyComplex32::new(1.000_000_1, 2.0);
    assert_eq!(
        a.debug_abs_diff(&b),
        MyComplex32::new(0.000_000_119_209_29, 0.000_003_576_278_7)
    );
    assert_eq!(
        a.debug_ulps_diff(&b),
        DebugUlpsDiff::<MyComplex32> {
            re: Some(1),
            im: Some(15)
        }
    );

    let c = MyComplex32::new(1.000_000_2, -2.0);
    assert_eq!(
        a.debug_ulps_diff(&c),
        DebugUlpsDiff::<MyComplex32> {
            re: Some(2),
            im: None
        }
    );
}

fn debug_tol() {
    let a = MyComplex32::new(1.0, 200.0);
    let b = MyComplex32::new(50.0, 1.0);

    assert_eq!(
        a.debug_abs_tol(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(0.1, 0.2)
    );
    assert_eq!(
        a.debug_rel_tol(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(5.0, 40.0)
    );
    assert_eq!(
        a.debug_ulps_tol(&b, &UlpsTol::<MyComplex32> { re: 1, im: 2 }),
        UlpsTol::<MyComplex32> { re: 1, im: 2 }
    );
}

fn main() {
    debug_diff();
    debug_tol();
}
