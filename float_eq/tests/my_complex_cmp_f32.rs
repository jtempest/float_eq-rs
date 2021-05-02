//TODO: update this to have proper tests for rmax/rmin/r1st/r2nd?

#![allow(clippy::float_cmp)]

use float_eq::{
    assert_float_eq, assert_float_ne, float_eq, float_ne, AssertFloatEq, DebugUlpsDiff, FloatEq,
    FloatEqDebugUlpsDiff, FloatEqUlpsTol, UlpsTol,
};

//------------------------------------------------------------------------------
// MyComplex32
//------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

impl MyComplex32 {
    fn new(re: f32, im: f32) -> MyComplex32 {
        MyComplex32 { re, im }
    }
}

//------------------------------------------------------------------------------
// MyComplex32Ulps
//------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct MyComplex32Ulps {
    re: UlpsTol<f32>,
    im: UlpsTol<f32>,
}

impl MyComplex32Ulps {
    fn new(re: UlpsTol<f32>, im: UlpsTol<f32>) -> MyComplex32Ulps {
        MyComplex32Ulps { re, im }
    }
}

impl FloatEqUlpsTol for MyComplex32 {
    type UlpsTol = MyComplex32Ulps;
}

//------------------------------------------------------------------------------
// MyComplex32DebugUlpsDiff
//------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct MyComplex32DebugUlpsDiff {
    re: Option<u32>,
    im: Option<u32>,
}

impl MyComplex32DebugUlpsDiff {
    fn new(re: Option<u32>, im: Option<u32>) -> MyComplex32DebugUlpsDiff {
        MyComplex32DebugUlpsDiff { re, im }
    }
}

impl FloatEqDebugUlpsDiff for MyComplex32 {
    type DebugUlpsDiff = MyComplex32DebugUlpsDiff;
}

//------------------------------------------------------------------------------
// FloatEq
//------------------------------------------------------------------------------
impl FloatEq<f32> for MyComplex32 {
    type Tol = f32;

    fn eq_abs(&self, other: &f32, tol: &Self::Tol) -> bool {
        self.re.eq_abs(other, tol) && self.im.eq_abs(&0.0, tol)
    }

    fn eq_rmax(&self, other: &f32, tol: &Self::Tol) -> bool {
        self.re.eq_rmax(other, tol) && self.im.eq_rmax(&0.0, tol)
    }

    fn eq_rmin(&self, other: &f32, tol: &Self::Tol) -> bool {
        self.re.eq_rmin(other, tol) && self.im.eq_rmin(&0.0, tol)
    }

    fn eq_r1st(&self, other: &f32, tol: &Self::Tol) -> bool {
        self.re.eq_r1st(other, tol) && self.im.eq_r1st(&0.0, tol)
    }

    fn eq_r2nd(&self, other: &f32, tol: &Self::Tol) -> bool {
        self.re.eq_r2nd(other, tol) && self.im.eq_r2nd(&0.0, tol)
    }

    fn eq_ulps(&self, other: &f32, tol: &UlpsTol<Self::Tol>) -> bool {
        self.re.eq_ulps(other, tol) && self.im.eq_ulps(&0.0, tol)
    }
}

impl FloatEq<MyComplex32> for f32 {
    type Tol = <MyComplex32 as FloatEq<f32>>::Tol;

    fn eq_abs(&self, other: &MyComplex32, tol: &Self::Tol) -> bool {
        other.eq_abs(self, tol)
    }

    fn eq_rmax(&self, other: &MyComplex32, tol: &Self::Tol) -> bool {
        other.eq_rmax(self, tol)
    }

    fn eq_rmin(&self, other: &MyComplex32, tol: &Self::Tol) -> bool {
        other.eq_rmin(self, tol)
    }

    fn eq_r1st(&self, other: &MyComplex32, tol: &Self::Tol) -> bool {
        other.eq_r1st(self, tol)
    }

    fn eq_r2nd(&self, other: &MyComplex32, tol: &Self::Tol) -> bool {
        other.eq_r2nd(self, tol)
    }

    fn eq_ulps(&self, other: &MyComplex32, tol: &UlpsTol<Self::Tol>) -> bool {
        other.eq_ulps(self, tol)
    }
}

#[test]
fn float_eq_f32() {
    let a = 1_000_000.06;
    let b = MyComplex32 {
        re: 1_000_000.0,
        im: 2.0,
    };

    //todo: rmax, rmin, r1st, r2nd

    assert!(a.ne_abs(&b, &0.07));
    assert!(a.ne_rel(&b, &0.000_000_12));
    assert!(a.ne_ulps(&b, &1));

    assert!(a.eq_abs(&b, &2.0));
    assert!(a.eq_rel(&b, &2.0));
    assert!(a.eq_ulps(&b, &1_073_741_824));

    assert!(b.ne_abs(&a, &0.07));
    assert!(b.ne_rel(&a, &0.000_000_12));
    assert!(b.ne_ulps(&a, &1));

    assert!(b.eq_abs(&a, &2.0));
    assert!(b.eq_rel(&a, &2.0));
    assert!(b.eq_ulps(&a, &1_073_741_824));

    let c = 2.000_000_5;
    let d = MyComplex32 { re: 2.0, im: 0.0 };

    //todo: rmax, rmin, r1st, r2nd

    assert!(c.ne_abs(&d, &0.000_000_4));
    assert!(c.ne_rel(&d, &0.000_000_23));
    assert!(c.ne_ulps(&d, &1));

    assert!(c.eq_abs(&d, &0.000_000_5));
    assert!(c.eq_rel(&d, &0.000_000_24));
    assert!(c.eq_ulps(&d, &2));

    assert!(d.ne_abs(&c, &0.000_000_4));
    assert!(d.ne_rel(&c, &0.000_000_23));
    assert!(d.ne_ulps(&c, &1));

    assert!(d.eq_abs(&c, &0.000_000_5));
    assert!(d.eq_rel(&c, &0.000_000_24));
    assert!(d.eq_ulps(&c, &2));
}

#[test]
fn float_eq_macro_f32() {
    let a = 1_000_000.06_f32;
    let b = MyComplex32 {
        re: 1_000_000.0,
        im: 2.0,
    };

    //todo: rmax, rmin, r1st, r2nd

    assert!(float_ne!(a, b, abs <= 0.07));
    assert!(float_ne!(a, b, rel <= 0.000_000_12));
    assert!(float_ne!(a, b, ulps <= 1));

    assert!(float_eq!(a, b, abs <= 2.0));
    assert!(float_eq!(a, b, rel <= 2.0));
    assert!(float_eq!(a, b, ulps <= 1_073_741_824));

    assert!(float_ne!(b, a, abs <= 0.07));
    assert!(float_ne!(b, a, rel <= 0.000_000_12));
    assert!(float_ne!(b, a, ulps <= 1));

    assert!(float_eq!(b, a, abs <= 2.0));
    assert!(float_eq!(b, a, rel <= 2.0));
    assert!(float_eq!(b, a, ulps <= 1_073_741_824));

    let c = 2.000_000_5_f32;
    let d = MyComplex32 { re: 2.0, im: 0.0 };

    //todo: rmax, rmin, r1st, r2nd

    assert!(float_ne!(c, d, abs <= 0.000_000_4));
    assert!(float_ne!(c, d, rel <= 0.000_000_23));
    assert!(float_ne!(c, d, ulps <= 1));

    assert!(float_eq!(c, d, abs <= 0.000_000_5));
    assert!(float_eq!(c, d, rel <= 0.000_000_24));
    assert!(float_eq!(c, d, ulps <= 2));

    assert!(float_ne!(d, c, abs <= 0.000_000_4));
    assert!(float_ne!(d, c, rel <= 0.000_000_23));
    assert!(float_ne!(d, c, ulps <= 1));

    assert!(float_eq!(d, c, abs <= 0.000_000_5));
    assert!(float_eq!(d, c, rel <= 0.000_000_24));
    assert!(float_eq!(d, c, ulps <= 2));
}

//------------------------------------------------------------------------------
// AssertFloatEq
//------------------------------------------------------------------------------
impl AssertFloatEq<f32> for MyComplex32 {
    type DebugAbsDiff = MyComplex32;
    type DebugTol = MyComplex32;

    fn debug_abs_diff(&self, other: &f32) -> Self::DebugAbsDiff {
        MyComplex32 {
            re: self.re.debug_abs_diff(other),
            im: self.im.debug_abs_diff(&0.0),
        }
    }

    fn debug_ulps_diff(&self, other: &f32) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        MyComplex32DebugUlpsDiff {
            re: self.re.debug_ulps_diff(other),
            im: self.im.debug_ulps_diff(&0.0),
        }
    }

    fn debug_abs_tol(&self, other: &f32, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.re.debug_abs_tol(other, tol),
            im: self.im.debug_abs_tol(&0.0, tol),
        }
    }

    fn debug_rmax_tol(&self, other: &f32, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.re.debug_rmax_tol(other, tol),
            im: self.im.debug_rmax_tol(&0.0, tol),
        }
    }

    fn debug_rmin_tol(&self, other: &f32, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.re.debug_rmin_tol(other, tol),
            im: self.im.debug_rmin_tol(&0.0, tol),
        }
    }

    fn debug_r1st_tol(&self, other: &f32, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.re.debug_r1st_tol(other, tol),
            im: self.im.debug_r1st_tol(&0.0, tol),
        }
    }

    fn debug_r2nd_tol(&self, other: &f32, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.re.debug_r2nd_tol(other, tol),
            im: self.im.debug_r2nd_tol(&0.0, tol),
        }
    }

    fn debug_ulps_tol(
        &self,
        other: &f32,
        tol: &UlpsTol<Self::Tol>,
    ) -> UlpsTol<Self::DebugTol> {
        UlpsTol::<Self::DebugTol> {
            re: self.re.debug_ulps_tol(other, tol),
            im: self.im.debug_ulps_tol(&0.0, tol),
        }
    }
}

impl AssertFloatEq<MyComplex32> for f32 {
    type DebugAbsDiff = <MyComplex32 as AssertFloatEq<f32>>::DebugAbsDiff;
    type DebugTol = <MyComplex32 as AssertFloatEq<f32>>::DebugTol;

    fn debug_abs_diff(&self, other: &MyComplex32) -> Self::DebugAbsDiff {
        other.debug_abs_diff(self)
    }

    fn debug_ulps_diff(&self, other: &MyComplex32) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        other.debug_ulps_diff(self)
    }

    fn debug_abs_tol(
        &self,
        other: &MyComplex32,
        tol: &Self::Tol,
    ) -> Self::DebugTol {
        other.debug_abs_tol(self, tol)
    }

    fn debug_rmax_tol(
        &self,
        other: &MyComplex32,
        tol: &Self::Tol,
    ) -> Self::DebugTol {
        other.debug_rmax_tol(self, tol)
    }

    fn debug_rmin_tol(
        &self,
        other: &MyComplex32,
        tol: &Self::Tol,
    ) -> Self::DebugTol {
        other.debug_rmin_tol(self, tol)
    }

    fn debug_r1st_tol(
        &self,
        other: &MyComplex32,
        tol: &Self::Tol,
    ) -> Self::DebugTol {
        other.debug_r1st_tol(self, tol)
    }

    fn debug_r2nd_tol(
        &self,
        other: &MyComplex32,
        tol: &Self::Tol,
    ) -> Self::DebugTol {
        other.debug_r2nd_tol(self, tol)
    }

    fn debug_ulps_tol(
        &self,
        other: &MyComplex32,
        tol: &UlpsTol<Self::Tol>,
    ) -> UlpsTol<Self::DebugTol> {
        other.debug_ulps_tol(self, tol)
    }
}

#[test]
fn float_diff_f32() {
    let a = 1.000_000_1_f32;
    let b = MyComplex32 {
        re: 1.0,
        im: 2.000_003_6,
    };

    //todo: rmax, rmin, r1st, r2nd

    assert_eq!(
        a.debug_abs_diff(&b),
        MyComplex32::new(0.000_000_119_209_29, 2.000_003_6)
    );
    assert_eq!(
        b.debug_abs_diff(&a),
        MyComplex32::new(0.000_000_119_209_29, 2.000_003_6)
    );

    assert_eq!(
        a.debug_ulps_diff(&b),
        MyComplex32DebugUlpsDiff::new(Some(1), Some(1_073_741_839))
    );
    assert_eq!(
        b.debug_ulps_diff(&a),
        MyComplex32DebugUlpsDiff::new(Some(1), Some(1_073_741_839))
    );
}

#[test]
fn float_eq_debug_f32() {
    let a = MyComplex32 {
        re: 150.0,
        im: 200.0,
    };
    let b = 1.0_f32;

    //todo: rmax, rmin, r1st, r2nd

    assert_eq!(
        a.debug_abs_tol(&b, &0.1),
        MyComplex32 { re: 0.1, im: 0.1 }
    );
    assert_eq!(
        b.debug_abs_tol(&a, &0.1),
        MyComplex32 { re: 0.1, im: 0.1 }
    );

    assert_eq!(
        a.debug_rel_tol(&b, &0.1),
        MyComplex32 { re: 15.0, im: 20.0 }
    );
    assert_eq!(
        b.debug_rel_tol(&a, &0.1),
        MyComplex32 { re: 15.0, im: 20.0 }
    );

    assert_eq!(a.debug_ulps_tol(&b, &42), MyComplex32Ulps::new(42, 42));
    assert_eq!(b.debug_ulps_tol(&a, &42), MyComplex32Ulps::new(42, 42));

    let c = 9000.0_f32;

    assert_eq!(
        a.debug_abs_tol(&c, &0.1),
        MyComplex32 { re: 0.1, im: 0.1 }
    );
    assert_eq!(
        c.debug_abs_tol(&a, &0.1),
        MyComplex32 { re: 0.1, im: 0.1 }
    );

    assert_eq!(
        a.debug_rel_tol(&c, &0.1),
        MyComplex32 {
            re: 900.0,
            im: 20.0
        }
    );
    assert_eq!(
        c.debug_rel_tol(&a, &0.1),
        MyComplex32 {
            re: 900.0,
            im: 20.0
        }
    );

    //todo: rmax, rmin, r1st, r2nd

    assert_eq!(
        a.debug_ulps_tol(&c, &42),
        MyComplex32Ulps { re: 42, im: 42 }
    );
    assert_eq!(
        c.debug_ulps_tol(&a, &42),
        MyComplex32Ulps { re: 42, im: 42 }
    );
}

//------------------------------------------------------------------------------
// assert_float_eq!/assert_float_ne! with f32
//------------------------------------------------------------------------------
#[test]
fn assert_float_eq_f32() {
    let a = 1_000_000.06_f32;
    let b = MyComplex32 {
        re: 1_000_000.0,
        im: 2.0,
    };

    //todo: rmax, rmin, r1st, r2nd

    assert_float_ne!(a, b, abs <= 0.07);
    assert_float_ne!(a, b, rel <= 0.000_000_12);
    assert_float_ne!(a, b, ulps <= 1);

    assert_float_eq!(a, b, abs <= 2.0);
    assert_float_eq!(a, b, rel <= 2.0);
    assert_float_eq!(a, b, ulps <= 1_073_741_824);

    assert_float_ne!(b, a, abs <= 0.07);
    assert_float_ne!(b, a, rel <= 0.000_000_12);
    assert_float_ne!(b, a, ulps <= 1);

    assert_float_eq!(b, a, abs <= 2.0);
    assert_float_eq!(b, a, rel <= 2.0);
    assert_float_eq!(b, a, ulps <= 1_073_741_824);

    let c = 2.000_000_5_f32;
    let d = MyComplex32 { re: 2.0, im: 0.0 };

    //todo: rmax, rmin, r1st, r2nd

    assert_float_ne!(c, d, abs <= 0.000_000_4);
    assert_float_ne!(c, d, rel <= 0.000_000_23);
    assert_float_ne!(c, d, ulps <= 1);

    assert_float_eq!(c, d, abs <= 0.000_000_5);
    assert_float_eq!(c, d, rel <= 0.000_000_24);
    assert_float_eq!(c, d, ulps <= 2);

    assert_float_ne!(d, c, abs <= 0.000_000_4);
    assert_float_ne!(d, c, rel <= 0.000_000_23);
    assert_float_ne!(d, c, ulps <= 1);

    assert_float_eq!(d, c, abs <= 0.000_000_5);
    assert_float_eq!(d, c, rel <= 0.000_000_24);
    assert_float_eq!(d, c, ulps <= 2);
}
