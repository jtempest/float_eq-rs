use float_eq::{FloatEq, FloatEqUlpsTol};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsTol, FloatEq)]
#[float_eq(ulps_tol = "MyComplex32Ulps")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

impl MyComplex32 {
    fn new(re: f32, im: f32) -> MyComplex32 {
        MyComplex32 { re, im }
    }
}

fn main() {
    let a = MyComplex32::new(2.0, -1_000_000.0);
    let b = MyComplex32::new(2.000_000_5, -1_000_000.06);

    assert!(a.eq_abs(&b, &MyComplex32::new(0.000_000_5, 0.07)));
    assert!(a.ne_abs(&b, &MyComplex32::new(0.000_000_4, 0.07)));
    assert!(a.ne_abs(&b, &MyComplex32::new(0.000_000_5, 0.06)));

    assert!(a.eq_rel(&b, &MyComplex32::new(0.000_000_25, 0.000_000_1)));
    assert!(a.ne_rel(&b, &MyComplex32::new(0.000_000_15, 0.000_000_1)));
    assert!(a.ne_rel(&b, &MyComplex32::new(0.000_000_25, 0.000_000_05)));

    assert!(a.eq_ulps(&b, &MyComplex32Ulps { re: 2, im: 1 }));
    assert!(a.ne_ulps(&b, &MyComplex32Ulps { re: 1, im: 1 }));
    assert!(a.ne_ulps(&b, &MyComplex32Ulps { re: 2, im: 0 }));
}
