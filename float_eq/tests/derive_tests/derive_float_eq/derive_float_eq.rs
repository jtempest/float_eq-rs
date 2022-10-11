mod my_module {
    use float_eq::{derive_float_eq, UlpsTol};

    #[derive_float_eq(
        ulps_tol = "MyComplex32Ulps",
        ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
        debug_ulps_diff = "MyComplex32UlpsDiff",
        debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq"
    )]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct MyComplex32 {
        pub re: f32,
        pub im: f32,
    }

    impl MyComplex32 {
        pub fn new(re: f32, im: f32) -> MyComplex32 {
            MyComplex32 { re, im }
        }
    }

    impl MyComplex32Ulps {
        pub fn new(re: UlpsTol<f32>, im: UlpsTol<f32>) -> MyComplex32Ulps {
            MyComplex32Ulps { re, im }
        }
    }
}

fn main() {
    use float_eq::{assert_float_eq, assert_float_ne};
    use my_module::{MyComplex32, MyComplex32Ulps};

    let a = MyComplex32 { re: 1.0, im: -2.0 };
    assert_float_eq!(a, a, abs <= MyComplex32 { re: 0.0, im: 0.0 });
    assert_float_eq!(a, a, rel <= MyComplex32 { re: 0.0, im: 0.0 });
    assert_float_eq!(a, a, ulps <= MyComplex32Ulps { re: 0, im: 0 });

    let b = MyComplex32 {
        re: 1.000_000_1,
        im: -2.000_000_5,
    };

    assert_float_eq!(a, b, abs <= MyComplex32::new(0.000_000_15, 0.000_000_55));
    assert_float_ne!(a, b, abs <= MyComplex32::new(0.000_000_05, 0.000_000_55));
    assert_float_ne!(a, b, abs <= MyComplex32::new(0.000_000_15, 0.000_000_45));

    assert_float_eq!(a, b, rel <= MyComplex32::new(0.000_000_15, 0.000_000_25));
    assert_float_ne!(a, b, rel <= MyComplex32::new(0.000_000_05, 0.000_000_25));
    assert_float_ne!(a, b, rel <= MyComplex32::new(0.000_000_15, 0.000_000_15));

    assert_float_eq!(a, b, ulps <= MyComplex32Ulps::new(1, 2));
    assert_float_ne!(a, b, ulps <= MyComplex32Ulps::new(0, 2));
    assert_float_ne!(a, b, ulps <= MyComplex32Ulps::new(1, 1));
}
