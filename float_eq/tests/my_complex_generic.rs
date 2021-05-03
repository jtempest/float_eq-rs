#![allow(clippy::float_cmp)]

use core::fmt;
use float_eq::{
    assert_float_eq, assert_float_ne, float_eq, float_ne, AssertFloatEq, AssertFloatEqAll,
    DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff, FloatEqUlpsTol, UlpsTol,
};

//------------------------------------------------------------------------------
// Helpers
//------------------------------------------------------------------------------

fn prev_n(f: f32, n: u32) -> f32 {
    f32::from_bits(f.to_bits() - n)
}

//------------------------------------------------------------------------------
// MyComplex
//------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct MyComplex<T> {
    re: T,
    im: T,
}

impl<T> MyComplex<T> {
    fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

//------------------------------------------------------------------------------
// MyComplexUlps
//------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct MyComplexUlps<T>
where
    T: FloatEqUlpsTol + PartialEq + fmt::Debug,
    UlpsTol<T>: PartialEq + fmt::Debug + Sized,
{
    re: UlpsTol<T>,
    im: UlpsTol<T>,
}

impl<T> MyComplexUlps<T>
where
    T: FloatEqUlpsTol + PartialEq + fmt::Debug,
    UlpsTol<T>: PartialEq + fmt::Debug + Sized,
{
    fn new(re: UlpsTol<T>, im: UlpsTol<T>) -> Self {
        Self { re, im }
    }
}

impl<T> FloatEqUlpsTol for MyComplex<T>
where
    T: FloatEqUlpsTol + PartialEq + fmt::Debug,
    UlpsTol<T>: PartialEq + fmt::Debug + Sized,
{
    type UlpsTol = MyComplexUlps<T>;
}

//------------------------------------------------------------------------------
// MyComplexUlps
//------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct MyComplexDebugUlpsDiff<T>
where
    T: FloatEqDebugUlpsDiff + PartialEq + fmt::Debug,
    DebugUlpsDiff<T>: PartialEq + fmt::Debug + Sized,
{
    re: DebugUlpsDiff<T>,
    im: DebugUlpsDiff<T>,
}

impl<T> MyComplexDebugUlpsDiff<T>
where
    T: FloatEqDebugUlpsDiff + PartialEq + fmt::Debug,
    DebugUlpsDiff<T>: PartialEq + fmt::Debug + Sized,
{
    fn new(re: DebugUlpsDiff<T>, im: DebugUlpsDiff<T>) -> Self {
        Self { re, im }
    }
}

impl<T> FloatEqDebugUlpsDiff for MyComplex<T>
where
    T: FloatEqDebugUlpsDiff + PartialEq + fmt::Debug,
    DebugUlpsDiff<T>: PartialEq + fmt::Debug + Sized,
{
    type DebugUlpsDiff = MyComplexDebugUlpsDiff<T>;
}

//------------------------------------------------------------------------------
// FloatEq
//------------------------------------------------------------------------------
impl<T> FloatEq for MyComplex<T>
where
    T: PartialEq + fmt::Debug + FloatEqUlpsTol + FloatEq,
    T::Tol: PartialEq + fmt::Debug + Sized,
    UlpsTol<T>: PartialEq + fmt::Debug,
    UlpsTol<T::Tol>: PartialEq + fmt::Debug + Sized,
{
    type Tol = MyComplex<T::Tol>;

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
    let a = MyComplex::<f32>::new(4.0, prev_n(-16.0, 4));
    let b = MyComplex::<f32>::new(prev_n(4.0, 2), -16.0);
    let eps = f32::EPSILON;

    assert!(a.eq_abs(&b, &MyComplex::new(0.000_000_5, 0.000_004)));
    assert!(a.ne_abs(&b, &MyComplex::new(0.000_000_4, 0.000_004)));
    assert!(a.ne_abs(&b, &MyComplex::new(0.000_000_5, 0.000_003)));

    assert!(a.eq_rmax(&b, &MyComplex::new(1.0 * eps, 2.0 * eps)));
    assert!(a.ne_rmax(&b, &MyComplex::new(0.5 * eps, 2.0 * eps)));
    assert!(a.ne_rmax(&b, &MyComplex::new(1.0 * eps, 1.0 * eps)));

    assert!(a.eq_rmin(&b, &MyComplex::new(2.0 * eps, 4.0 * eps)));
    assert!(a.ne_rmin(&b, &MyComplex::new(1.0 * eps, 4.0 * eps)));
    assert!(a.ne_rmin(&b, &MyComplex::new(2.0 * eps, 2.0 * eps)));

    assert!(a.eq_r1st(&b, &MyComplex::new(1.0 * eps, 4.0 * eps)));
    assert!(a.ne_r1st(&b, &MyComplex::new(0.5 * eps, 4.0 * eps)));
    assert!(a.ne_r1st(&b, &MyComplex::new(1.0 * eps, 2.0 * eps)));

    assert!(a.eq_r2nd(&b, &MyComplex::new(2.0 * eps, 2.0 * eps)));
    assert!(a.ne_r2nd(&b, &MyComplex::new(1.0 * eps, 2.0 * eps)));
    assert!(a.ne_r2nd(&b, &MyComplex::new(2.0 * eps, 1.0 * eps)));

    assert!(a.eq_ulps(&b, &MyComplexUlps::new(2, 4)));
    assert!(a.ne_ulps(&b, &MyComplexUlps::new(1, 4)));
    assert!(a.ne_ulps(&b, &MyComplexUlps::new(2, 3)));
}

//------------------------------------------------------------------------------
// FloatEqAll
//------------------------------------------------------------------------------
impl<T> FloatEqAll for MyComplex<T>
where
    T: FloatEqUlpsTol + FloatEqAll,
{
    type AllTol = T::AllTol;

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
    let a = MyComplex::<f32>::new(4.0, prev_n(-16.0, 4));
    let b = MyComplex::<f32>::new(prev_n(4.0, 2), -16.0);
    let eps = f32::EPSILON;

    assert!(a.eq_abs_all(&b, &0.000_004));
    assert!(a.ne_abs_all(&b, &0.000_003));

    assert!(a.eq_rmax_all(&b, &(2.0 * eps)));
    assert!(a.ne_rmax_all(&b, &(1.0 * eps)));

    assert!(a.eq_rmin_all(&b, &(4.0 * eps)));
    assert!(a.ne_rmin_all(&b, &(2.0 * eps)));

    assert!(a.eq_r1st_all(&b, &(4.0 * eps)));
    assert!(a.ne_r1st_all(&b, &(2.0 * eps)));

    assert!(a.eq_r2nd_all(&b, &(2.0 * eps)));
    assert!(a.ne_r2nd_all(&b, &(1.0 * eps)));

    assert!(a.eq_ulps_all(&b, &4));
    assert!(a.ne_ulps_all(&b, &3));
}

//------------------------------------------------------------------------------
// float_eq!/float_ne!
//------------------------------------------------------------------------------
#[test]
fn float_eq_macro() {
    let a = MyComplex::<f32>::new(4.0, prev_n(-16.0, 4));
    let b = MyComplex::<f32>::new(prev_n(4.0, 2), -16.0);
    let eps = f32::EPSILON;

    assert!(float_eq!(
        a,
        b,
        abs <= MyComplex::new(0.000_000_5, 0.000_004)
    ));
    assert!(float_ne!(
        a,
        b,
        abs <= MyComplex::new(0.000_000_4, 0.000_004)
    ));
    assert!(float_ne!(
        a,
        b,
        abs <= MyComplex::new(0.000_000_5, 0.000_003)
    ));

    assert!(float_eq!(
        a,
        b,
        rmax <= MyComplex::new(1.0 * eps, 2.0 * eps)
    ));
    assert!(float_ne!(
        a,
        b,
        rmax <= MyComplex::new(0.5 * eps, 2.0 * eps)
    ));
    assert!(float_ne!(
        a,
        b,
        rmax <= MyComplex::new(1.0 * eps, 1.0 * eps)
    ));

    assert!(float_eq!(
        a,
        b,
        rmin <= MyComplex::new(2.0 * eps, 4.0 * eps)
    ));
    assert!(float_ne!(
        a,
        b,
        rmin <= MyComplex::new(1.0 * eps, 4.0 * eps)
    ));
    assert!(float_ne!(
        a,
        b,
        rmin <= MyComplex::new(2.0 * eps, 2.0 * eps)
    ));

    assert!(float_eq!(
        a,
        b,
        r1st <= MyComplex::new(1.0 * eps, 4.0 * eps)
    ));
    assert!(float_ne!(
        a,
        b,
        r1st <= MyComplex::new(0.5 * eps, 4.0 * eps)
    ));
    assert!(float_ne!(
        a,
        b,
        r1st <= MyComplex::new(1.0 * eps, 2.0 * eps)
    ));

    assert!(float_eq!(
        a,
        b,
        r2nd <= MyComplex::new(2.0 * eps, 2.0 * eps)
    ));
    assert!(float_ne!(
        a,
        b,
        r2nd <= MyComplex::new(1.0 * eps, 2.0 * eps)
    ));
    assert!(float_ne!(
        a,
        b,
        r2nd <= MyComplex::new(2.0 * eps, 1.0 * eps)
    ));

    assert!(float_eq!(a, b, ulps <= MyComplexUlps::new(2, 4)));
    assert!(float_ne!(a, b, ulps <= MyComplexUlps::new(1, 4)));
    assert!(float_ne!(a, b, ulps <= MyComplexUlps::new(2, 3)));
}

#[test]
fn float_eq_macro_all() {
    let a = MyComplex::<f32>::new(4.0, prev_n(-16.0, 4));
    let b = MyComplex::<f32>::new(prev_n(4.0, 2), -16.0);
    let eps = f32::EPSILON;

    assert!(float_eq!(a, b, abs_all <= 0.000_004));
    assert!(float_ne!(a, b, abs_all <= 0.000_003));

    assert!(float_eq!(a, b, rmax_all <= 2.0 * eps));
    assert!(float_ne!(a, b, rmax_all <= 1.0 * eps));

    assert!(float_eq!(a, b, rmin_all <= 4.0 * eps));
    assert!(float_ne!(a, b, rmin_all <= 2.0 * eps));

    assert!(float_eq!(a, b, r1st_all <= 4.0 * eps));
    assert!(float_ne!(a, b, r1st_all <= 2.0 * eps));

    assert!(float_eq!(a, b, r2nd_all <= 2.0 * eps));
    assert!(float_ne!(a, b, r2nd_all <= 1.0 * eps));

    assert!(float_eq!(a, b, ulps_all <= 4));
    assert!(float_ne!(a, b, ulps_all <= 3));
}

//------------------------------------------------------------------------------
// AssertFloatEq
//------------------------------------------------------------------------------
impl<T> AssertFloatEq for MyComplex<T>
where
    T: PartialEq + fmt::Debug + FloatEqUlpsTol + AssertFloatEq + FloatEqDebugUlpsDiff,
    T::Tol: PartialEq + fmt::Debug + Sized,
    T::DebugTol: PartialEq + fmt::Debug + Sized,
    T::DebugAbsDiff: PartialEq + fmt::Debug,
    UlpsTol<T>: PartialEq + fmt::Debug,
    UlpsTol<T::Tol>: PartialEq + fmt::Debug + Sized,
    UlpsTol<T::DebugTol>: PartialEq + fmt::Debug + Sized,
    DebugUlpsDiff<T::DebugAbsDiff>: PartialEq + fmt::Debug,
{
    type DebugAbsDiff = MyComplex<T::DebugAbsDiff>;
    type DebugTol = MyComplex<T::DebugTol>;

    fn debug_abs_diff(&self, other: &MyComplex<T>) -> Self::DebugAbsDiff {
        MyComplex {
            re: self.re.debug_abs_diff(&other.re),
            im: self.im.debug_abs_diff(&other.im),
        }
    }

    fn debug_ulps_diff(&self, other: &MyComplex<T>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        MyComplexDebugUlpsDiff {
            re: self.re.debug_ulps_diff(&other.re),
            im: self.im.debug_ulps_diff(&other.im),
        }
    }

    fn debug_abs_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex {
            re: self.re.debug_abs_tol(&other.re, &tol.re),
            im: self.im.debug_abs_tol(&other.im, &tol.im),
        }
    }

    fn debug_rmax_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex {
            re: self.re.debug_rmax_tol(&other.re, &tol.re),
            im: self.im.debug_rmax_tol(&other.im, &tol.im),
        }
    }

    fn debug_rmin_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex {
            re: self.re.debug_rmin_tol(&other.re, &tol.re),
            im: self.im.debug_rmin_tol(&other.im, &tol.im),
        }
    }

    fn debug_r1st_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex {
            re: self.re.debug_r1st_tol(&other.re, &tol.re),
            im: self.im.debug_r1st_tol(&other.im, &tol.im),
        }
    }

    fn debug_r2nd_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
        MyComplex {
            re: self.re.debug_r2nd_tol(&other.re, &tol.re),
            im: self.im.debug_r2nd_tol(&other.im, &tol.im),
        }
    }

    fn debug_ulps_tol(&self, other: &Self, tol: &UlpsTol<Self::Tol>) -> UlpsTol<Self::DebugTol> {
        UlpsTol::<Self::DebugTol> {
            re: self.re.debug_ulps_tol(&other.re, &tol.re),
            im: self.im.debug_ulps_tol(&other.im, &tol.im),
        }
    }
}

#[test]
fn float_diff() {
    let a = MyComplex::<f32>::new(1.0, 2.000_003_6);
    assert_eq!(a.debug_abs_diff(&a), MyComplex::<f32>::new(0.0, 0.0));
    assert_eq!(
        a.debug_ulps_diff(&a),
        MyComplexDebugUlpsDiff::<f32>::new(Some(0), Some(0))
    );

    let b = MyComplex::<f32>::new(1.000_000_1, 2.0);
    assert_eq!(
        a.debug_abs_diff(&b),
        MyComplex::new(0.000_000_119_209_29, 0.000_003_576_278_7)
    );
    assert_eq!(
        a.debug_ulps_diff(&b),
        MyComplexDebugUlpsDiff::<f32>::new(Some(1), Some(15))
    );

    let c = MyComplex::<f32>::new(1.000_000_1, -2.0);
    assert_eq!(
        a.debug_ulps_diff(&c),
        MyComplexDebugUlpsDiff::<f32>::new(Some(1), None)
    );
}

#[test]
fn float_eq_debug() {
    let a = MyComplex::<f32> { re: 1.0, im: 200.0 };
    let b = MyComplex::<f32> { re: 50.0, im: 2.0 };

    assert_eq!(
        a.debug_abs_tol(&b, &MyComplex::new(0.1, 0.2)),
        MyComplex::new(0.1, 0.2)
    );
    assert_eq!(
        a.debug_rmax_tol(&b, &MyComplex::new(0.1, 0.2)),
        MyComplex::new(5.0, 40.0)
    );
    assert_eq!(
        a.debug_rmin_tol(&b, &MyComplex::new(0.1, 0.2)),
        MyComplex::new(0.1, 0.4)
    );
    assert_eq!(
        a.debug_r1st_tol(&b, &MyComplex::new(0.1, 0.2)),
        MyComplex::new(0.1, 40.0)
    );
    assert_eq!(
        a.debug_r2nd_tol(&b, &MyComplex::new(0.1, 0.2)),
        MyComplex::new(5.0, 0.4)
    );
    assert_eq!(
        a.debug_ulps_tol(&b, &MyComplexUlps::new(1, 2)),
        MyComplexUlps::new(1, 2)
    );
}

//------------------------------------------------------------------------------
// AssertFloatEq
//------------------------------------------------------------------------------
impl<T> AssertFloatEqAll for MyComplex<T>
where
    T: PartialEq + fmt::Debug + FloatEqUlpsTol + AssertFloatEqAll,
    T::AllTol: PartialEq + fmt::Debug,
    T::AllDebugTol: PartialEq + fmt::Debug + Sized,
    UlpsTol<T>: PartialEq + fmt::Debug,
    UlpsTol<T::AllTol>: PartialEq + fmt::Debug,
    UlpsTol<T::AllDebugTol>: PartialEq + fmt::Debug + Sized,
{
    type AllDebugTol = MyComplex<T::AllDebugTol>;

    fn debug_abs_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Self::AllDebugTol {
            re: self.re.debug_abs_all_tol(&other.re, tol),
            im: self.im.debug_abs_all_tol(&other.im, tol),
        }
    }

    fn debug_rmax_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Self::AllDebugTol {
            re: self.re.debug_rmax_all_tol(&other.re, tol),
            im: self.im.debug_rmax_all_tol(&other.im, tol),
        }
    }

    fn debug_rmin_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Self::AllDebugTol {
            re: self.re.debug_rmin_all_tol(&other.re, tol),
            im: self.im.debug_rmin_all_tol(&other.im, tol),
        }
    }

    fn debug_r1st_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Self::AllDebugTol {
            re: self.re.debug_r1st_all_tol(&other.re, tol),
            im: self.im.debug_r1st_all_tol(&other.im, tol),
        }
    }

    fn debug_r2nd_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Self::AllDebugTol {
            re: self.re.debug_r2nd_all_tol(&other.re, tol),
            im: self.im.debug_r2nd_all_tol(&other.im, tol),
        }
    }

    fn debug_ulps_all_tol(
        &self,
        other: &Self,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol> {
        MyComplexUlps {
            re: self.re.debug_ulps_all_tol(&other.re, tol),
            im: self.im.debug_ulps_all_tol(&other.im, tol),
        }
    }
}

#[test]
fn float_eq_all_debug() {
    let a = MyComplex::<f32> { re: 1.0, im: 200.0 };
    let b = MyComplex::<f32> { re: 50.0, im: 2.0 };

    assert_eq!(a.debug_abs_all_tol(&b, &0.2), MyComplex::new(0.2, 0.2));
    assert_eq!(a.debug_rmax_all_tol(&b, &0.2), MyComplex::new(10.0, 40.0));
    assert_eq!(a.debug_rmin_all_tol(&b, &0.2), MyComplex::new(0.2, 0.4));
    assert_eq!(a.debug_r1st_all_tol(&b, &0.2), MyComplex::new(0.2, 40.0));
    assert_eq!(a.debug_r2nd_all_tol(&b, &0.2), MyComplex::new(10.0, 0.4));
    assert_eq!(a.debug_ulps_all_tol(&b, &2), MyComplexUlps::new(2, 2));
}

//------------------------------------------------------------------------------
// assert_float_eq!/assert_float_ne!
//------------------------------------------------------------------------------
#[test]
fn assert_float_eq() {
    let a = MyComplex::<f32>::new(4.0, prev_n(-16.0, 4));
    let b = MyComplex::<f32>::new(prev_n(4.0, 2), -16.0);
    let eps = f32::EPSILON;

    assert_float_eq!(a, b, abs <= MyComplex::new(0.000_000_5, 0.000_004));
    assert_float_ne!(a, b, abs <= MyComplex::new(0.000_000_4, 0.000_004));
    assert_float_ne!(a, b, abs <= MyComplex::new(0.000_000_5, 0.000_003));

    assert_float_eq!(a, b, rmax <= MyComplex::new(1.0 * eps, 2.0 * eps));
    assert_float_ne!(a, b, rmax <= MyComplex::new(0.5 * eps, 2.0 * eps));
    assert_float_ne!(a, b, rmax <= MyComplex::new(1.0 * eps, 1.0 * eps));

    assert_float_eq!(a, b, rmin <= MyComplex::new(2.0 * eps, 4.0 * eps));
    assert_float_ne!(a, b, rmin <= MyComplex::new(1.0 * eps, 4.0 * eps));
    assert_float_ne!(a, b, rmin <= MyComplex::new(2.0 * eps, 2.0 * eps));

    assert_float_eq!(a, b, r1st <= MyComplex::new(1.0 * eps, 4.0 * eps));
    assert_float_ne!(a, b, r1st <= MyComplex::new(0.5 * eps, 4.0 * eps));
    assert_float_ne!(a, b, r1st <= MyComplex::new(1.0 * eps, 2.0 * eps));

    assert_float_eq!(a, b, r2nd <= MyComplex::new(2.0 * eps, 2.0 * eps));
    assert_float_ne!(a, b, r2nd <= MyComplex::new(1.0 * eps, 2.0 * eps));
    assert_float_ne!(a, b, r2nd <= MyComplex::new(2.0 * eps, 1.0 * eps));

    assert_float_eq!(a, b, ulps <= MyComplexUlps::new(2, 4));
    assert_float_ne!(a, b, ulps <= MyComplexUlps::new(1, 4));
    assert_float_ne!(a, b, ulps <= MyComplexUlps::new(2, 3));
}

#[test]
fn assert_float_eq_all() {
    let a = MyComplex::<f32>::new(4.0, prev_n(-16.0, 4));
    let b = MyComplex::<f32>::new(prev_n(4.0, 2), -16.0);
    let eps = f32::EPSILON;

    assert_float_eq!(a, b, abs_all <= 0.000_004);
    assert_float_ne!(a, b, abs_all <= 0.000_003);

    assert_float_eq!(a, b, rmax_all <= 2.0 * eps);
    assert_float_ne!(a, b, rmax_all <= 1.0 * eps);

    assert_float_eq!(a, b, rmin_all <= 4.0 * eps);
    assert_float_ne!(a, b, rmin_all <= 2.0 * eps);

    assert_float_eq!(a, b, r1st_all <= 4.0 * eps);
    assert_float_ne!(a, b, r1st_all <= 2.0 * eps);

    assert_float_eq!(a, b, r2nd_all <= 2.0 * eps);
    assert_float_ne!(a, b, r2nd_all <= 1.0 * eps);

    assert_float_eq!(a, b, ulps_all <= 4);
    assert_float_ne!(a, b, ulps_all <= 3);
}
