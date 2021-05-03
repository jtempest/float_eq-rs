#![allow(clippy::float_cmp)]

use float_eq::{
    assert_float_eq, assert_float_ne, float_eq, float_ne, AssertFloatEq, DebugUlpsDiff, FloatEq,
    FloatEqDebugUlpsDiff, FloatEqUlpsTol, UlpsTol,
};

//------------------------------------------------------------------------------
// Helpers
//------------------------------------------------------------------------------

// next representable float
fn next(f: f32) -> f32 {
    next_n(f, 1)
}

// previous representable float
fn prev(f: f32) -> f32 {
    prev_n(f, 1)
}

fn next_n(f: f32, n: u32) -> f32 {
    f32::from_bits(f.to_bits() + n)
}

fn prev_n(f: f32, n: u32) -> f32 {
    f32::from_bits(f.to_bits() - n)
}

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
        self.eq_r1st(&other.re, tol) && 0.0.eq_r1st(&other.im, tol)
    }

    fn eq_r2nd(&self, other: &MyComplex32, tol: &Self::Tol) -> bool {
        self.eq_r2nd(&other.re, tol) && 0.0.eq_r2nd(&other.im, tol)
    }

    fn eq_ulps(&self, other: &MyComplex32, tol: &UlpsTol<Self::Tol>) -> bool {
        other.eq_ulps(self, tol)
    }
}

#[test]
fn float_eq_f32() {
    // non-zero im
    let a = next(0.5f32);
    let b = MyComplex32 { re: 0.5, im: 2.0 };

    assert!(!a.eq_abs(&b, &prev(2.0)));
    assert!(!b.eq_abs(&a, &prev(2.0)));
    assert!(a.eq_abs(&b, &2.0));
    assert!(b.eq_abs(&a, &2.0));

    assert!(!a.eq_rmax(&b, &prev(1.0)));
    assert!(!b.eq_rmax(&a, &prev(1.0)));
    assert!(a.eq_rmax(&b, &1.0));
    assert!(b.eq_rmax(&a, &1.0));

    assert!(!a.eq_rmin(&b, &f32::INFINITY));
    assert!(!b.eq_rmin(&a, &f32::INFINITY));

    assert!(!a.eq_r1st(&b, &f32::INFINITY));
    assert!(!b.eq_r1st(&a, &prev(1.0)));
    assert!(b.eq_r1st(&a, &1.0));

    assert!(!a.eq_r2nd(&b, &prev(1.0)));
    assert!(a.eq_r2nd(&b, &1.0));
    assert!(!b.eq_r2nd(&a, &f32::INFINITY));

    let im_bits = 2.0f32.to_bits();
    assert!(!a.eq_ulps(&b, &(im_bits - 1)));
    assert!(!b.eq_ulps(&a, &(im_bits - 1)));
    assert!(a.eq_ulps(&b, &im_bits));
    assert!(b.eq_ulps(&a, &im_bits));

    // zero im
    let c = prev_n(8.0f32, 1);
    let d = MyComplex32 { re: 8.0, im: 0.0 };

    assert!(!c.eq_abs(&d, &prev(4.0 * f32::EPSILON)));
    assert!(!d.eq_abs(&c, &prev(4.0 * f32::EPSILON)));
    assert!(c.eq_abs(&d, &(4.0 * f32::EPSILON)));
    assert!(d.eq_abs(&c, &(4.0 * f32::EPSILON)));

    assert!(!c.eq_rmax(&d, &(0.25 * f32::EPSILON)));
    assert!(!d.eq_rmax(&c, &(0.25 * f32::EPSILON)));
    assert!(c.eq_rmax(&d, &(0.5 * f32::EPSILON)));
    assert!(d.eq_rmax(&c, &(0.5 * f32::EPSILON)));

    assert!(!c.eq_rmin(&d, &(0.5 * f32::EPSILON)));
    assert!(!d.eq_rmin(&c, &(0.5 * f32::EPSILON)));
    assert!(c.eq_rmin(&d, &(1.0 * f32::EPSILON)));
    assert!(d.eq_rmin(&c, &(1.0 * f32::EPSILON)));

    assert!(!c.eq_r1st(&d, &(0.5 * f32::EPSILON)));
    assert!(!d.eq_r1st(&c, &(0.25 * f32::EPSILON)));
    assert!(c.eq_r1st(&d, &(1.0 * f32::EPSILON)));
    assert!(d.eq_r1st(&c, &(0.5 * f32::EPSILON)));

    assert!(!c.eq_r2nd(&d, &(0.25 * f32::EPSILON)));
    assert!(!d.eq_r2nd(&c, &(0.5 * f32::EPSILON)));
    assert!(c.eq_r2nd(&d, &(0.5 * f32::EPSILON)));
    assert!(d.eq_r2nd(&c, &(1.0 * f32::EPSILON)));

    assert!(!c.eq_ulps(&d, &0));
    assert!(!d.eq_ulps(&c, &0));
    assert!(c.eq_ulps(&d, &1));
    assert!(d.eq_ulps(&c, &1));
}

#[test]
fn float_eq_macro_f32() {
    // non-zero im
    let a = next(0.5f32);
    let b = MyComplex32 { re: 0.5, im: 2.0 };

    assert!(float_ne!(a, b, abs <= prev(2.0)));
    assert!(float_ne!(b, a, abs <= prev(2.0)));
    assert!(float_eq!(a, b, abs <= 2.0));
    assert!(float_eq!(b, a, abs <= 2.0));

    assert!(float_ne!(a, b, rmax <= prev(1.0)));
    assert!(float_ne!(b, a, rmax <= prev(1.0)));
    assert!(float_eq!(a, b, rmax <= 1.0));
    assert!(float_eq!(b, a, rmax <= 1.0));

    assert!(float_ne!(a, b, rmin <= f32::INFINITY));
    assert!(float_ne!(b, a, rmin <= f32::INFINITY));

    assert!(float_ne!(a, b, r1st <= f32::INFINITY));
    assert!(float_ne!(b, a, r1st <= prev(1.0)));
    assert!(float_eq!(b, a, r1st <= 1.0));

    assert!(float_ne!(a, b, r2nd <= prev(1.0)));
    assert!(float_eq!(a, b, r2nd <= 1.0));
    assert!(float_ne!(b, a, r2nd <= f32::INFINITY));

    let im_bits = 2.0f32.to_bits();
    assert!(float_ne!(a, b, ulps <= im_bits - 1));
    assert!(float_ne!(b, a, ulps <= im_bits - 1));
    assert!(float_eq!(a, b, ulps <= im_bits));
    assert!(float_eq!(b, a, ulps <= im_bits));

    // zero im
    let c = prev_n(8.0f32, 1);
    let d = MyComplex32 { re: 8.0, im: 0.0 };

    assert!(float_ne!(c, d, abs <= prev(4.0 * f32::EPSILON)));
    assert!(float_ne!(d, c, abs <= prev(4.0 * f32::EPSILON)));
    assert!(float_eq!(c, d, abs <= (4.0 * f32::EPSILON)));
    assert!(float_eq!(d, c, abs <= (4.0 * f32::EPSILON)));

    assert!(float_ne!(c, d, rmax <= (0.25 * f32::EPSILON)));
    assert!(float_ne!(d, c, rmax <= (0.25 * f32::EPSILON)));
    assert!(float_eq!(c, d, rmax <= (0.5 * f32::EPSILON)));
    assert!(float_eq!(d, c, rmax <= (0.5 * f32::EPSILON)));

    assert!(float_ne!(c, d, rmin <= (0.5 * f32::EPSILON)));
    assert!(float_ne!(d, c, rmin <= (0.5 * f32::EPSILON)));
    assert!(float_eq!(c, d, rmin <= (1.0 * f32::EPSILON)));
    assert!(float_eq!(d, c, rmin <= (1.0 * f32::EPSILON)));

    assert!(float_ne!(c, d, r1st <= (0.5 * f32::EPSILON)));
    assert!(float_ne!(d, c, r1st <= (0.25 * f32::EPSILON)));
    assert!(float_eq!(c, d, r1st <= (1.0 * f32::EPSILON)));
    assert!(float_eq!(d, c, r1st <= (0.5 * f32::EPSILON)));

    assert!(float_ne!(c, d, r2nd <= (0.25 * f32::EPSILON)));
    assert!(float_ne!(d, c, r2nd <= (0.5 * f32::EPSILON)));
    assert!(float_eq!(c, d, r2nd <= (0.5 * f32::EPSILON)));
    assert!(float_eq!(d, c, r2nd <= (1.0 * f32::EPSILON)));

    assert!(float_ne!(c, d, ulps <= 0));
    assert!(float_ne!(d, c, ulps <= 0));
    assert!(float_eq!(c, d, ulps <= 1));
    assert!(float_eq!(d, c, ulps <= 1));
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

    fn debug_ulps_tol(&self, other: &f32, tol: &UlpsTol<Self::Tol>) -> UlpsTol<Self::DebugTol> {
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

    fn debug_abs_tol(&self, other: &MyComplex32, tol: &Self::Tol) -> Self::DebugTol {
        other.debug_abs_tol(self, tol)
    }

    fn debug_rmax_tol(&self, other: &MyComplex32, tol: &Self::Tol) -> Self::DebugTol {
        other.debug_rmax_tol(self, tol)
    }

    fn debug_rmin_tol(&self, other: &MyComplex32, tol: &Self::Tol) -> Self::DebugTol {
        other.debug_rmin_tol(self, tol)
    }

    fn debug_r1st_tol(&self, other: &MyComplex32, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.debug_r1st_tol(&other.re, tol),
            im: 0.0.debug_r1st_tol(&other.im, tol),
        }
    }

    fn debug_r2nd_tol(&self, other: &MyComplex32, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.debug_r2nd_tol(&other.re, tol),
            im: 0.0.debug_r2nd_tol(&other.im, tol),
        }
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

    assert_eq!(a.debug_abs_tol(&b, &0.1), MyComplex32 { re: 0.1, im: 0.1 });
    assert_eq!(b.debug_abs_tol(&a, &0.1), MyComplex32 { re: 0.1, im: 0.1 });

    assert_eq!(
        a.debug_rmax_tol(&b, &0.1),
        MyComplex32 { re: 15.0, im: 20.0 }
    );
    assert_eq!(
        b.debug_rmax_tol(&a, &0.1),
        MyComplex32 { re: 15.0, im: 20.0 }
    );

    assert_eq!(a.debug_rmin_tol(&b, &0.1), MyComplex32 { re: 0.1, im: 0.0 });
    assert_eq!(b.debug_rmin_tol(&a, &0.1), MyComplex32 { re: 0.1, im: 0.0 });

    assert_eq!(
        a.debug_r1st_tol(&b, &0.1),
        MyComplex32 { re: 15.0, im: 20.0 }
    );
    assert_eq!(b.debug_r1st_tol(&a, &0.1), MyComplex32 { re: 0.1, im: 0.0 });

    assert_eq!(a.debug_r2nd_tol(&b, &0.1), MyComplex32 { re: 0.1, im: 0.0 });
    assert_eq!(
        b.debug_r2nd_tol(&a, &0.1),
        MyComplex32 { re: 15.0, im: 20.0 }
    );

    assert_eq!(a.debug_ulps_tol(&b, &42), MyComplex32Ulps::new(42, 42));
    assert_eq!(b.debug_ulps_tol(&a, &42), MyComplex32Ulps::new(42, 42));

    let c = 9000.0_f32;

    assert_eq!(a.debug_abs_tol(&c, &0.1), MyComplex32 { re: 0.1, im: 0.1 });
    assert_eq!(c.debug_abs_tol(&a, &0.1), MyComplex32 { re: 0.1, im: 0.1 });

    assert_eq!(
        a.debug_rmax_tol(&c, &0.1),
        MyComplex32 {
            re: 900.0,
            im: 20.0
        }
    );
    assert_eq!(
        c.debug_rmax_tol(&a, &0.1),
        MyComplex32 {
            re: 900.0,
            im: 20.0
        }
    );

    assert_eq!(
        a.debug_rmin_tol(&c, &0.1),
        MyComplex32 { re: 15.0, im: 0.0 }
    );
    assert_eq!(
        c.debug_rmin_tol(&a, &0.1),
        MyComplex32 { re: 15.0, im: 0.0 }
    );

    assert_eq!(
        a.debug_r1st_tol(&c, &0.1),
        MyComplex32 { re: 15.0, im: 20.0 }
    );
    assert_eq!(
        c.debug_r1st_tol(&a, &0.1),
        MyComplex32 { re: 900.0, im: 0.0 }
    );

    assert_eq!(
        a.debug_r2nd_tol(&c, &0.1),
        MyComplex32 { re: 900.0, im: 0.0 }
    );
    assert_eq!(
        c.debug_r2nd_tol(&a, &0.1),
        MyComplex32 { re: 15.0, im: 20.0 }
    );

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
    // non-zero im
    let a = next(0.5f32);
    let b = MyComplex32 { re: 0.5, im: 2.0 };

    assert_float_ne!(a, b, abs <= prev(2.0));
    assert_float_ne!(b, a, abs <= prev(2.0));
    assert_float_eq!(a, b, abs <= 2.0);
    assert_float_eq!(b, a, abs <= 2.0);

    assert_float_ne!(a, b, rmax <= prev(1.0));
    assert_float_ne!(b, a, rmax <= prev(1.0));
    assert_float_eq!(a, b, rmax <= 1.0);
    assert_float_eq!(b, a, rmax <= 1.0);

    assert_float_ne!(a, b, rmin <= f32::INFINITY);
    assert_float_ne!(b, a, rmin <= f32::INFINITY);

    assert_float_ne!(a, b, r1st <= f32::INFINITY);
    assert_float_ne!(b, a, r1st <= prev(1.0));
    assert_float_eq!(b, a, r1st <= 1.0);

    assert_float_ne!(a, b, r2nd <= prev(1.0));
    assert_float_eq!(a, b, r2nd <= 1.0);
    assert_float_ne!(b, a, r2nd <= f32::INFINITY);

    let im_bits = 2.0f32.to_bits();
    assert_float_ne!(a, b, ulps <= im_bits - 1);
    assert_float_ne!(b, a, ulps <= im_bits - 1);
    assert_float_eq!(a, b, ulps <= im_bits);
    assert_float_eq!(b, a, ulps <= im_bits);

    // zero im
    let c = prev_n(8.0f32, 1);
    let d = MyComplex32 { re: 8.0, im: 0.0 };

    assert_float_ne!(c, d, abs <= prev(4.0 * f32::EPSILON));
    assert_float_ne!(d, c, abs <= prev(4.0 * f32::EPSILON));
    assert_float_eq!(c, d, abs <= (4.0 * f32::EPSILON));
    assert_float_eq!(d, c, abs <= (4.0 * f32::EPSILON));

    assert_float_ne!(c, d, rmax <= (0.25 * f32::EPSILON));
    assert_float_ne!(d, c, rmax <= (0.25 * f32::EPSILON));
    assert_float_eq!(c, d, rmax <= (0.5 * f32::EPSILON));
    assert_float_eq!(d, c, rmax <= (0.5 * f32::EPSILON));

    assert_float_ne!(c, d, rmin <= (0.5 * f32::EPSILON));
    assert_float_ne!(d, c, rmin <= (0.5 * f32::EPSILON));
    assert_float_eq!(c, d, rmin <= (1.0 * f32::EPSILON));
    assert_float_eq!(d, c, rmin <= (1.0 * f32::EPSILON));

    assert_float_ne!(c, d, r1st <= (0.5 * f32::EPSILON));
    assert_float_ne!(d, c, r1st <= (0.25 * f32::EPSILON));
    assert_float_eq!(c, d, r1st <= (1.0 * f32::EPSILON));
    assert_float_eq!(d, c, r1st <= (0.5 * f32::EPSILON));

    assert_float_ne!(c, d, r2nd <= (0.25 * f32::EPSILON));
    assert_float_ne!(d, c, r2nd <= (0.5 * f32::EPSILON));
    assert_float_eq!(c, d, r2nd <= (0.5 * f32::EPSILON));
    assert_float_eq!(d, c, r2nd <= (1.0 * f32::EPSILON));

    assert_float_ne!(c, d, ulps <= 0);
    assert_float_ne!(d, c, ulps <= 0);
    assert_float_eq!(c, d, ulps <= 1);
    assert_float_eq!(d, c, ulps <= 1);
}
