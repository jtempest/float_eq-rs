//! A test of implementing `FloatDiff`, `FloatEq` and `FloatDiff` on a foreign type.
//!
//! This is also a more thorough test of the example `MyComplex32` type used by
//! the implementation examples in documentation.
#![allow(clippy::float_cmp, clippy::cognitive_complexity)]

use float_eq::{
    assert_float_eq, assert_float_ne, float_eq, float_ne, FloatDiff, FloatEq, FloatEqAll,
    FloatEqAllDebug, FloatEqDebug, FloatUlps, Ulps,
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
    re: Ulps<f32>,
    im: Ulps<f32>,
}

impl MyComplex32Ulps {
    fn new(re: Ulps<f32>, im: Ulps<f32>) -> MyComplex32Ulps {
        MyComplex32Ulps { re, im }
    }
}

impl FloatUlps for MyComplex32 {
    type Ulps = MyComplex32Ulps;
}

//------------------------------------------------------------------------------
// FloatDiff
//------------------------------------------------------------------------------
impl FloatDiff for MyComplex32 {
    type Output = Self;

    fn abs_diff(&self, other: &Self) -> Option<Self::Output> {
        Some(MyComplex32 {
            re: self.re.abs_diff(&other.re)?,
            im: self.im.abs_diff(&other.im)?,
        })
    }

    fn ulps_diff(&self, other: &Self) -> Option<Ulps<Self::Output>> {
        Some(Ulps::<Self::Output> {
            re: self.re.ulps_diff(&other.re)?,
            im: self.im.ulps_diff(&other.im)?,
        })
    }
}

#[test]
fn float_diff() {
    let a = MyComplex32::new(1.0, 2.000_003_6);
    assert_eq!(a.abs_diff(&a), Some(MyComplex32::new(0., 0.)));
    assert_eq!(a.ulps_diff(&a), Some(MyComplex32Ulps::new(0, 0)));

    let b = MyComplex32::new(1.000_000_1, 2.0);
    assert_eq!(
        a.abs_diff(&b),
        Some(MyComplex32::new(0.000_000_119_209_29, 0.000_003_576_278_7))
    );
    assert_eq!(a.ulps_diff(&b), Some(MyComplex32Ulps::new(1, 15)));
}

//------------------------------------------------------------------------------
// FloatEq
//------------------------------------------------------------------------------
impl FloatEq for MyComplex32 {
    type Epsilon = Self;

    fn eq_abs(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_abs(&other.re, &max_diff.re) && self.im.eq_abs(&other.im, &max_diff.im)
    }

    fn eq_rel(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_rel(&other.re, &max_diff.re) && self.im.eq_rel(&other.im, &max_diff.im)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &Ulps<Self::Epsilon>) -> bool {
        self.re.eq_ulps(&other.re, &max_diff.re) && self.im.eq_ulps(&other.im, &max_diff.im)
    }
}

#[test]
fn float_eq() {
    let a = MyComplex32::new(2.0, -1_000_000.);
    let b = MyComplex32::new(2.000_000_5, -1_000_000.06);

    assert!(a.eq_abs(&b, &MyComplex32::new(0.000_000_5, 0.07)));
    assert!(a.ne_abs(&b, &MyComplex32::new(0.000_000_4, 0.07)));
    assert!(a.ne_abs(&b, &MyComplex32::new(0.000_000_5, 0.06)));

    assert!(a.eq_rel(&b, &MyComplex32::new(0.000_000_25, 0.000_000_1)));
    assert!(a.ne_rel(&b, &MyComplex32::new(0.000_000_15, 0.000_000_1)));
    assert!(a.ne_rel(&b, &MyComplex32::new(0.000_000_25, 0.000_000_05)));

    assert!(a.eq_ulps(&b, &MyComplex32Ulps::new(2, 1)));
    assert!(a.ne_ulps(&b, &MyComplex32Ulps::new(0, 1)));
    assert!(a.ne_ulps(&b, &MyComplex32Ulps::new(2, 0)));
}

//------------------------------------------------------------------------------
// FloatEqAll
//------------------------------------------------------------------------------
impl FloatEqAll for MyComplex32 {
    type Epsilon = f32;

    fn eq_abs_all(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_abs_all(&other.re, &max_diff) && self.im.eq_abs_all(&other.im, &max_diff)
    }

    fn eq_rel_all(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_rel_all(&other.re, &max_diff) && self.im.eq_rel_all(&other.im, &max_diff)
    }

    fn eq_ulps_all(&self, other: &Self, max_diff: &Ulps<Self::Epsilon>) -> bool {
        self.re.eq_ulps_all(&other.re, &max_diff) && self.im.eq_ulps_all(&other.im, &max_diff)
    }
}

#[test]
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

//------------------------------------------------------------------------------
// float_eq!/float_ne!
//------------------------------------------------------------------------------
#[test]
fn float_eq_macro() {
    let a = MyComplex32::new(2.0, -1_000_000.);
    let b = MyComplex32::new(2.000_000_5, -1_000_000.06);

    assert!(float_eq!(a, b, abs <= MyComplex32::new(0.000_000_5, 0.07)));
    assert!(float_ne!(a, b, abs <= MyComplex32::new(0.000_000_4, 0.07)));
    assert!(float_ne!(a, b, abs <= MyComplex32::new(0.000_000_5, 0.06)));
    assert!(float_eq!(a, b, abs_all <= 0.07));
    assert!(float_ne!(a, b, abs_all <= 0.06));

    assert!(float_eq!(
        a,
        b,
        rel <= MyComplex32::new(0.000_000_25, 0.000_000_1)
    ));
    assert!(float_ne!(
        a,
        b,
        rel <= MyComplex32::new(0.000_000_15, 0.000_000_1)
    ));
    assert!(float_ne!(
        a,
        b,
        rel <= MyComplex32::new(0.000_000_25, 0.000_000_05)
    ));
    assert!(float_eq!(a, b, rel_all <= 0.000_000_25));
    assert!(float_ne!(a, b, rel_all <= 0.000_000_15));

    assert!(float_eq!(a, b, ulps <= MyComplex32Ulps::new(2, 1)));
    assert!(float_ne!(a, b, ulps <= MyComplex32Ulps::new(0, 1)));
    assert!(float_ne!(a, b, ulps <= MyComplex32Ulps::new(2, 0)));
    assert!(float_eq!(a, b, ulps_all <= 2));
    assert!(float_ne!(a, b, ulps_all <= 1));
}

//------------------------------------------------------------------------------
// FloatEqDebug
//------------------------------------------------------------------------------
impl FloatEqDebug for MyComplex32 {
    type DebugEpsilon = MyComplex32;

    fn debug_abs_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        MyComplex32 {
            re: self.re.debug_abs_epsilon(&other.re, &max_diff.re),
            im: self.im.debug_abs_epsilon(&other.im, &max_diff.im),
        }
    }

    fn debug_rel_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        MyComplex32 {
            re: self.re.debug_rel_epsilon(&other.re, &max_diff.re),
            im: self.im.debug_rel_epsilon(&other.im, &max_diff.im),
        }
    }

    fn debug_ulps_epsilon(
        &self,
        other: &Self,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        Ulps::<Self::DebugEpsilon> {
            re: self.re.debug_ulps_epsilon(&other.re, &max_diff.re),
            im: self.im.debug_ulps_epsilon(&other.im, &max_diff.im),
        }
    }
}

#[test]
fn float_eq_debug() {
    let a = MyComplex32 { re: 1.0, im: 200.0 };
    let b = MyComplex32 { re: 50.0, im: 1.0 };

    assert_eq!(
        a.debug_abs_epsilon(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(0.1, 0.2)
    );
    assert_eq!(
        a.debug_rel_epsilon(&b, &MyComplex32::new(0.1, 0.2)),
        MyComplex32::new(5.0, 40.0)
    );
    assert_eq!(
        a.debug_ulps_epsilon(&b, &MyComplex32Ulps::new(1, 2)),
        MyComplex32Ulps::new(1, 2)
    );
}

//------------------------------------------------------------------------------
// FloatEqDebug
//------------------------------------------------------------------------------
impl FloatEqAllDebug for MyComplex32 {
    type DebugEpsilon = MyComplex32;

    fn debug_abs_all_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        MyComplex32 {
            re: self.re.debug_abs_all_epsilon(&other.re, max_diff),
            im: self.im.debug_abs_all_epsilon(&other.im, max_diff),
        }
    }

    fn debug_rel_all_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        MyComplex32 {
            re: self.re.debug_rel_epsilon(&other.re, max_diff),
            im: self.im.debug_rel_epsilon(&other.im, max_diff),
        }
    }

    fn debug_ulps_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        MyComplex32Ulps {
            re: self.re.debug_ulps_all_epsilon(&other.re, max_diff),
            im: self.im.debug_ulps_all_epsilon(&other.im, max_diff),
        }
    }
}

#[test]
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

//------------------------------------------------------------------------------
// assert_float_eq!/assert_float_ne!
//------------------------------------------------------------------------------
#[test]
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
