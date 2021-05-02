#![allow(clippy::float_cmp)]

use float_eq::{
    assert_float_eq, assert_float_ne, AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq,
    FloatEqAll, FloatEqDebugUlpsDiff, FloatEqUlpsTol, UlpsTol,
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
impl FloatEq for MyComplex32 {
    type Tol = Self;

    fn eq_abs(&self, other: &Self, tol: &Self::Tol) -> bool {
        self.re.eq_abs(&other.re, &tol.re) && self.im.eq_abs(&other.im, &tol.im)
    }

    fn eq_rmax(&self, other: &Self, tol: &Self::Tol) -> bool {
        self.re.eq_rmax(&other.re, &tol.re) && self.im.eq_rmax(&other.im, &tol.im)
    }

    fn eq_rmin(&self, other: &Self, tol: &Self::Tol) -> bool {
        self.re.eq_rmin(&other.re, &tol.re) && self.im.eq_rmin(&other.im, &tol.im)
    }

    fn eq_r1st(&self, other: &Self, tol: &Self::Tol) -> bool {
        self.re.eq_r1st(&other.re, &tol.re) && self.im.eq_r1st(&other.im, &tol.im)
    }

    fn eq_r2nd(&self, other: &Self, tol: &Self::Tol) -> bool {
        self.re.eq_r2nd(&other.re, &tol.re) && self.im.eq_r2nd(&other.im, &tol.im)
    }

    fn eq_ulps(&self, other: &Self, tol: &UlpsTol<Self::Tol>) -> bool {
        self.re.eq_ulps(&other.re, &tol.re) && self.im.eq_ulps(&other.im, &tol.im)
    }
}

#[test]
fn float_eq() {
    let a = MyComplex32::new(1.0, 3.999_999_5);
    let b = MyComplex32::new(0.999_999_9, 4.0);
    let eps = f32::EPSILON;

    assert!(a.eq_abs(&b, &MyComplex32::new(1.0 * eps, 4.0 * eps)));
    assert!(a.ne_abs(&b, &MyComplex32::new(0.5 * eps, 4.0 * eps)));
    assert!(a.ne_abs(&b, &MyComplex32::new(1.0 * eps, 3.0 * eps)));

    assert!(a.eq_rel(&b, &MyComplex32::new(1.0 * eps, 1.0 * eps)));
    assert!(a.ne_rel(&b, &MyComplex32::new(0.5 * eps, 1.0 * eps)));
    assert!(a.ne_rel(&b, &MyComplex32::new(1.0 * eps, 0.5 * eps)));

    assert!(a.eq_rmax(&b, &MyComplex32::new(1.0 * eps, 1.0 * eps)));
    assert!(a.ne_rmax(&b, &MyComplex32::new(0.5 * eps, 1.0 * eps)));
    assert!(a.ne_rmax(&b, &MyComplex32::new(1.0 * eps, 0.5 * eps)));

    assert!(a.eq_rmin(&b, &MyComplex32::new(2.0 * eps, 2.0 * eps)));
    assert!(a.ne_rmin(&b, &MyComplex32::new(1.0 * eps, 2.0 * eps)));
    assert!(a.ne_rmin(&b, &MyComplex32::new(2.0 * eps, 1.0 * eps)));

    assert!(a.eq_r1st(&b, &MyComplex32::new(1.0 * eps, 2.0 * eps)));
    assert!(a.ne_r1st(&b, &MyComplex32::new(0.5 * eps, 2.0 * eps)));
    assert!(a.ne_r1st(&b, &MyComplex32::new(1.0 * eps, 1.0 * eps)));

    assert!(a.eq_r2nd(&b, &MyComplex32::new(2.0 * eps, 1.0 * eps)));
    assert!(a.ne_r2nd(&b, &MyComplex32::new(1.0 * eps, 1.0 * eps)));
    assert!(a.ne_r2nd(&b, &MyComplex32::new(2.0 * eps, 0.5 * eps)));

    assert!(a.eq_ulps(&b, &MyComplex32Ulps::new(2, 2)));
    assert!(a.ne_ulps(&b, &MyComplex32Ulps::new(1, 2)));
    assert!(a.ne_ulps(&b, &MyComplex32Ulps::new(2, 1)));
}

//------------------------------------------------------------------------------
// FloatEqAll
//------------------------------------------------------------------------------
impl FloatEqAll for MyComplex32 {
    type AllTol = f32;

    fn eq_abs_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
        self.re.eq_abs_all(&other.re, &tol) && self.im.eq_abs_all(&other.im, &tol)
    }

    fn eq_rmax_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
        self.re.eq_rmax_all(&other.re, &tol) && self.im.eq_rmax_all(&other.im, &tol)
    }

    fn eq_rmin_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
        self.re.eq_rmin_all(&other.re, &tol) && self.im.eq_rmin_all(&other.im, &tol)
    }

    fn eq_r1st_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
        self.re.eq_r1st_all(&other.re, &tol) && self.im.eq_r1st_all(&other.im, &tol)
    }

    fn eq_r2nd_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
        self.re.eq_r2nd_all(&other.re, &tol) && self.im.eq_r2nd_all(&other.im, &tol)
    }

    fn eq_ulps_all(&self, other: &Self, tol: &UlpsTol<Self::AllTol>) -> bool {
        self.re.eq_ulps_all(&other.re, &tol) && self.im.eq_ulps_all(&other.im, &tol)
    }
}

#[test]
fn float_eq_all() {
    let a = MyComplex32::new(1.0, 3.999_999_5);
    let b = MyComplex32::new(0.999_999_9, 4.0);
    let eps = f32::EPSILON;

    assert!(a.eq_abs_all(&b, &(4.0 * eps)));
    assert!(a.ne_abs_all(&b, &(3.0 * eps)));

    assert!(a.eq_rel_all(&b, &(1.0 * eps)));
    assert!(a.ne_rel_all(&b, &(0.5 * eps)));

    assert!(a.eq_rmax_all(&b, &(1.0 * eps)));
    assert!(a.ne_rmax_all(&b, &(0.5 * eps)));

    assert!(a.eq_rmin_all(&b, &(2.0 * eps)));
    assert!(a.ne_rmin_all(&b, &(1.0 * eps)));

    assert!(a.eq_r1st_all(&b, &(2.0 * eps)));
    assert!(a.ne_r1st_all(&b, &(1.0 * eps)));

    assert!(a.eq_r2nd_all(&b, &(2.0 * eps)));
    assert!(a.ne_r2nd_all(&b, &(1.0 * eps)));

    assert!(a.eq_ulps_all(&b, &2));
    assert!(a.ne_ulps_all(&b, &1));
}

//------------------------------------------------------------------------------
// AssertFloatEq
//------------------------------------------------------------------------------
impl AssertFloatEq for MyComplex32 {
    type DebugAbsDiff = MyComplex32;
    type DebugTol = MyComplex32;

    fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
        MyComplex32 {
            re: self.re.debug_abs_diff(&other.re),
            im: self.im.debug_abs_diff(&other.im),
        }
    }

    fn debug_ulps_diff(&self, other: &Self) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        MyComplex32DebugUlpsDiff {
            re: self.re.debug_ulps_diff(&other.re),
            im: self.im.debug_ulps_diff(&other.im),
        }
    }

    fn debug_abs_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.re.debug_abs_tol(&other.re, &tol.re),
            im: self.im.debug_abs_tol(&other.im, &tol.im),
        }
    }

    fn debug_rmax_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.re.debug_rmax_tol(&other.re, &tol.re),
            im: self.im.debug_rmax_tol(&other.im, &tol.im),
        }
    }

    fn debug_rmin_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.re.debug_rmin_tol(&other.re, &tol.re),
            im: self.im.debug_rmin_tol(&other.im, &tol.im),
        }
    }

    fn debug_r1st_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.re.debug_r1st_tol(&other.re, &tol.re),
            im: self.im.debug_r1st_tol(&other.im, &tol.im),
        }
    }

    fn debug_r2nd_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex32 {
            re: self.re.debug_r2nd_tol(&other.re, &tol.re),
            im: self.im.debug_r2nd_tol(&other.im, &tol.im),
        }
    }

    fn debug_ulps_tol(
        &self,
        other: &Self,
        tol: &UlpsTol<Self::Tol>,
    ) -> UlpsTol<Self::DebugTol> {
        UlpsTol::<Self::DebugTol> {
            re: self.re.debug_ulps_tol(&other.re, &tol.re),
            im: self.im.debug_ulps_tol(&other.im, &tol.im),
        }
    }
}

#[test]
fn float_diff() {
    let a = MyComplex32::new(1.0, 2.000_003_6);
    assert_eq!(a.debug_abs_diff(&a), MyComplex32::new(0.0, 0.0));
    assert_eq!(
        a.debug_ulps_diff(&a),
        MyComplex32DebugUlpsDiff::new(Some(0), Some(0))
    );

    let b = MyComplex32::new(1.000_000_1, 2.0);
    assert_eq!(
        a.debug_abs_diff(&b),
        MyComplex32::new(0.000_000_119_209_29, 0.000_003_576_278_7)
    );
    assert_eq!(
        a.debug_ulps_diff(&b),
        MyComplex32DebugUlpsDiff::new(Some(1), Some(15))
    );

    let c = MyComplex32::new(1.000_000_1, -2.0);
    assert_eq!(
        a.debug_ulps_diff(&c),
        MyComplex32DebugUlpsDiff::new(Some(1), None)
    );
}

#[test]
fn float_eq_debug() {
    let a = MyComplex32 { re: 1.0, im: 200.0 };
    let b = MyComplex32 { re: 50.0, im: 1.0 };

    assert_eq!(
        a.debug_abs_tol(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(0.1, 0.2)
    );
    assert_eq!(
        a.debug_rel_tol(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(5.0, 40.0)
    );
    assert_eq!(
        a.debug_rmax_tol(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(5.0, 40.0)
    );
    assert_eq!(
        a.debug_rmin_tol(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(0.1, 0.2)
    );
    assert_eq!(
        a.debug_r1st_tol(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(0.1, 40.0)
    );
    assert_eq!(
        a.debug_r2nd_tol(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(5.0, 0.2)
    );
    assert_eq!(
        a.debug_ulps_tol(&b, &MyComplex32Ulps::new(1, 2)),
        MyComplex32Ulps::new(1, 2)
    );
}

//------------------------------------------------------------------------------
// AssertFloatEqAll
//------------------------------------------------------------------------------
impl AssertFloatEqAll for MyComplex32 {
    type AllDebugTol = MyComplex32;

    fn debug_abs_all_tol(
        &self,
        other: &Self,
        tol: &Self::AllTol,
    ) -> Self::AllDebugTol {
        MyComplex32 {
            re: self.re.debug_abs_all_tol(&other.re, tol),
            im: self.im.debug_abs_all_tol(&other.im, tol),
        }
    }

    fn debug_rmax_all_tol(
        &self,
        other: &Self,
        tol: &Self::AllTol,
    ) -> Self::AllDebugTol {
        MyComplex32 {
            re: self.re.debug_rmax_all_tol(&other.re, tol),
            im: self.im.debug_rmax_all_tol(&other.im, tol),
        }
    }

    fn debug_rmin_all_tol(
        &self,
        other: &Self,
        tol: &Self::AllTol,
    ) -> Self::AllDebugTol {
        MyComplex32 {
            re: self.re.debug_rmin_all_tol(&other.re, tol),
            im: self.im.debug_rmin_all_tol(&other.im, tol),
        }
    }

    fn debug_r1st_all_tol(
        &self,
        other: &Self,
        tol: &Self::AllTol,
    ) -> Self::AllDebugTol {
        MyComplex32 {
            re: self.re.debug_r1st_all_tol(&other.re, tol),
            im: self.im.debug_r1st_all_tol(&other.im, tol),
        }
    }

    fn debug_r2nd_all_tol(
        &self,
        other: &Self,
        tol: &Self::AllTol,
    ) -> Self::AllDebugTol {
        MyComplex32 {
            re: self.re.debug_r2nd_all_tol(&other.re, tol),
            im: self.im.debug_r2nd_all_tol(&other.im, tol),
        }
    }

    fn debug_ulps_all_tol(
        &self,
        other: &Self,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol> {
        MyComplex32Ulps {
            re: self.re.debug_ulps_all_tol(&other.re, tol),
            im: self.im.debug_ulps_all_tol(&other.im, tol),
        }
    }
}

#[test]
fn float_eq_all_debug() {
    let a = MyComplex32 { re: 1.0, im: 200.0 };
    let b = MyComplex32 { re: 50.0, im: 1.0 };

    assert_eq!(
        a.debug_abs_all_tol(&b, &0.2),
        MyComplex32::new(0.2, 0.2)
    );
    assert_eq!(
        a.debug_rel_all_tol(&b, &0.2),
        MyComplex32::new(10.0, 40.0)
    );
    assert_eq!(
        a.debug_rmax_all_tol(&b, &0.2),
        MyComplex32::new(10.0, 40.0)
    );
    assert_eq!(
        a.debug_rmin_all_tol(&b, &0.2),
        MyComplex32::new(0.2, 0.2)
    );
    assert_eq!(
        a.debug_r1st_all_tol(&b, &0.2),
        MyComplex32::new(0.2, 40.0)
    );
    assert_eq!(
        a.debug_r2nd_all_tol(&b, &0.2),
        MyComplex32::new(10.0, 0.2)
    );
    assert_eq!(a.debug_ulps_all_tol(&b, &2), MyComplex32Ulps::new(2, 2));
}

//------------------------------------------------------------------------------
// assert_float_eq!/assert_float_ne!
//------------------------------------------------------------------------------
#[test]
fn assert_float_eq() {
    let a = MyComplex32::new(1.0, 3.999_999_5);
    let b = MyComplex32::new(0.999_999_9, 4.0);
    let eps = f32::EPSILON;

    assert_float_eq!(a, b, abs <= MyComplex32::new(1.0 * eps, 4.0 * eps));
    assert_float_ne!(a, b, abs <= MyComplex32::new(0.5 * eps, 4.0 * eps));
    assert_float_ne!(a, b, abs <= MyComplex32::new(1.0 * eps, 3.0 * eps));
    assert_float_eq!(a, b, abs_all <= 4.0 * eps);
    assert_float_ne!(a, b, abs_all <= 3.0 * eps);

    assert_float_eq!(a, b, rel <= MyComplex32::new(1.0 * eps, 1.0 * eps));
    assert_float_ne!(a, b, rel <= MyComplex32::new(0.5 * eps, 1.0 * eps));
    assert_float_ne!(a, b, rel <= MyComplex32::new(1.0 * eps, 0.5 * eps));
    assert_float_eq!(a, b, rel_all <= 1.0 * eps);
    assert_float_ne!(a, b, rel_all <= 0.5 * eps);

    assert_float_eq!(a, b, rmax <= MyComplex32::new(1.0 * eps, 1.0 * eps));
    assert_float_ne!(a, b, rmax <= MyComplex32::new(0.5 * eps, 1.0 * eps));
    assert_float_ne!(a, b, rmax <= MyComplex32::new(1.0 * eps, 0.5 * eps));
    assert_float_eq!(a, b, rmax_all <= 1.0 * eps);
    assert_float_ne!(a, b, rmax_all <= 0.5 * eps);

    assert_float_eq!(a, b, rmin <= MyComplex32::new(2.0 * eps, 2.0 * eps));
    assert_float_ne!(a, b, rmin <= MyComplex32::new(1.0 * eps, 2.0 * eps));
    assert_float_ne!(a, b, rmin <= MyComplex32::new(2.0 * eps, 1.0 * eps));
    assert_float_eq!(a, b, rmin_all <= 2.0 * eps);
    assert_float_ne!(a, b, rmin_all <= 1.0 * eps);

    assert_float_eq!(a, b, r1st <= MyComplex32::new(1.0 * eps, 2.0 * eps));
    assert_float_ne!(a, b, r1st <= MyComplex32::new(0.5 * eps, 2.0 * eps));
    assert_float_ne!(a, b, r1st <= MyComplex32::new(1.0 * eps, 1.0 * eps));
    assert_float_eq!(a, b, r1st_all <= 2.0 * eps);
    assert_float_ne!(a, b, r1st_all <= 1.0 * eps);

    assert_float_eq!(a, b, r2nd <= MyComplex32::new(2.0 * eps, 1.0 * eps));
    assert_float_ne!(a, b, r2nd <= MyComplex32::new(1.0 * eps, 1.0 * eps));
    assert_float_ne!(a, b, r2nd <= MyComplex32::new(2.0 * eps, 0.5 * eps));
    assert_float_eq!(a, b, r2nd_all <= 2.0 * eps);
    assert_float_ne!(a, b, r2nd_all <= 1.0 * eps);

    assert_float_eq!(a, b, ulps <= MyComplex32Ulps::new(2, 2));
    assert_float_ne!(a, b, ulps <= MyComplex32Ulps::new(1, 2));
    assert_float_ne!(a, b, ulps <= MyComplex32Ulps::new(2, 1));
    assert_float_eq!(a, b, ulps_all <= 2);
    assert_float_ne!(a, b, ulps_all <= 1);
}
