use float_eq::{FloatEq, FloatEqAll, FloatEqUlpsEpsilon};

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon, FloatEq, FloatEqAll)]
#[float_eq(ulps_epsilon = "MyComplex32Ulps", all_epsilon = "f32")]
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

    assert!(a.eq_abs_all(&b, &0.07));
    assert!(a.ne_abs_all(&b, &0.06));

    assert!(a.eq_rel_all(&b, &0.000_000_25));
    assert!(a.ne_rel_all(&b, &0.000_000_15));

    assert!(a.eq_ulps_all(&b, &2));
    assert!(a.ne_ulps_all(&b, &1));
}
