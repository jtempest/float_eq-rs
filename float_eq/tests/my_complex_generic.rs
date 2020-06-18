#![allow(clippy::float_cmp, clippy::cognitive_complexity)]

use core::fmt;
use float_eq::{
    assert_float_eq, assert_float_ne, float_eq, float_ne, FloatDiff, FloatEq, FloatEqAll,
    FloatEqAllDebug, FloatEqDebug, FloatUlps, Ulps,
};

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
    T: FloatUlps + PartialEq + fmt::Debug,
    Ulps<T>: PartialEq + fmt::Debug + Sized,
{
    re: Ulps<T>,
    im: Ulps<T>,
}

impl<T> MyComplexUlps<T>
where
    T: FloatUlps + PartialEq + fmt::Debug,
    Ulps<T>: PartialEq + fmt::Debug + Sized,
{
    fn new(re: Ulps<T>, im: Ulps<T>) -> Self {
        Self { re, im }
    }
}

impl<T> FloatUlps for MyComplex<T>
where
    T: FloatUlps + PartialEq + fmt::Debug,
    Ulps<T>: PartialEq + fmt::Debug + Sized,
{
    type Ulps = MyComplexUlps<T>;
}

//------------------------------------------------------------------------------
// FloatDiff
//------------------------------------------------------------------------------
impl<T> FloatDiff for MyComplex<T>
where
    T: PartialEq + fmt::Debug + FloatUlps + FloatDiff,
    T::Output: PartialEq + fmt::Debug + Sized,
    Ulps<T>: PartialEq + fmt::Debug,
    Ulps<T::Output>: PartialEq + fmt::Debug + Sized,
{
    type Output = MyComplex<T::Output>;

    fn abs_diff(&self, other: &Self) -> Self::Output {
        Self::Output {
            re: self.re.abs_diff(&other.re),
            im: self.im.abs_diff(&other.im),
        }
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
    let a = MyComplex::<f32>::new(1.0, 2.000_003_6);
    assert_eq!(a.abs_diff(&a), MyComplex::<f32>::new(0., 0.));
    assert_eq!(a.ulps_diff(&a), Some(MyComplexUlps::<f32>::new(0, 0)));

    let b = MyComplex::<f32>::new(1.000_000_1, 2.0);
    assert_eq!(
        a.abs_diff(&b),
        MyComplex::new(0.000_000_119_209_29, 0.000_003_576_278_7)
    );
    assert_eq!(a.ulps_diff(&b), Some(MyComplexUlps::<f32>::new(1, 15)));
}

//------------------------------------------------------------------------------
// FloatEq
//------------------------------------------------------------------------------
impl<T> FloatEq for MyComplex<T>
where
    T: PartialEq + fmt::Debug + FloatUlps + FloatEq,
    T::Epsilon: PartialEq + fmt::Debug + Sized,
    Ulps<T>: PartialEq + fmt::Debug,
    Ulps<T::Epsilon>: PartialEq + fmt::Debug + Sized,
{
    type Epsilon = MyComplex<T::Epsilon>;

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
    let a = MyComplex::<f32>::new(2.0, -1_000_000.);
    let b = MyComplex::<f32>::new(2.000_000_5, -1_000_000.06);

    assert!(a.eq_abs(&b, &MyComplex::new(0.000_000_5, 0.07)));
    assert!(a.ne_abs(&b, &MyComplex::new(0.000_000_4, 0.07)));
    assert!(a.ne_abs(&b, &MyComplex::new(0.000_000_5, 0.06)));

    assert!(a.eq_rel(&b, &MyComplex::new(0.000_000_25, 0.000_000_1)));
    assert!(a.ne_rel(&b, &MyComplex::new(0.000_000_15, 0.000_000_1)));
    assert!(a.ne_rel(&b, &MyComplex::new(0.000_000_25, 0.000_000_05)));

    assert!(a.eq_ulps(&b, &MyComplexUlps::new(2, 1)));
    assert!(a.ne_ulps(&b, &MyComplexUlps::new(0, 1)));
    assert!(a.ne_ulps(&b, &MyComplexUlps::new(2, 0)));
}

//------------------------------------------------------------------------------
// FloatEqAll
//------------------------------------------------------------------------------
impl<T> FloatEqAll for MyComplex<T>
where
    T: FloatUlps + FloatEqAll,
{
    type AllEpsilon = T::AllEpsilon;

    fn eq_abs_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
        self.re.eq_abs_all(&other.re, &max_diff) && self.im.eq_abs_all(&other.im, &max_diff)
    }

    fn eq_rel_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
        self.re.eq_rel_all(&other.re, &max_diff) && self.im.eq_rel_all(&other.im, &max_diff)
    }

    fn eq_ulps_all(&self, other: &Self, max_diff: &Ulps<Self::AllEpsilon>) -> bool {
        self.re.eq_ulps_all(&other.re, &max_diff) && self.im.eq_ulps_all(&other.im, &max_diff)
    }
}

#[test]
fn float_eq_all() {
    let a = MyComplex::<f32>::new(2.0, -1_000_000.);
    let b = MyComplex::<f32>::new(2.000_000_5, -1_000_000.06);

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
    let a = MyComplex::<f32>::new(2.0, -1_000_000.);
    let b = MyComplex::<f32>::new(2.000_000_5, -1_000_000.06);

    assert!(float_eq!(a, b, abs <= MyComplex::new(0.000_000_5, 0.07)));
    assert!(float_ne!(a, b, abs <= MyComplex::new(0.000_000_4, 0.07)));
    assert!(float_ne!(a, b, abs <= MyComplex::new(0.000_000_5, 0.06)));
    assert!(float_eq!(a, b, abs_all <= 0.07));
    assert!(float_ne!(a, b, abs_all <= 0.06));

    assert!(float_eq!(
        a,
        b,
        rel <= MyComplex::new(0.000_000_25, 0.000_000_1)
    ));
    assert!(float_ne!(
        a,
        b,
        rel <= MyComplex::new(0.000_000_15, 0.000_000_1)
    ));
    assert!(float_ne!(
        a,
        b,
        rel <= MyComplex::new(0.000_000_25, 0.000_000_05)
    ));
    assert!(float_eq!(a, b, rel_all <= 0.000_000_25));
    assert!(float_ne!(a, b, rel_all <= 0.000_000_15));

    assert!(float_eq!(a, b, ulps <= MyComplexUlps::new(2, 1)));
    assert!(float_ne!(a, b, ulps <= MyComplexUlps::new(0, 1)));
    assert!(float_ne!(a, b, ulps <= MyComplexUlps::new(2, 0)));
    assert!(float_eq!(a, b, ulps_all <= 2));
    assert!(float_ne!(a, b, ulps_all <= 1));
}

//------------------------------------------------------------------------------
// FloatEqDebug
//------------------------------------------------------------------------------
impl<T> FloatEqDebug for MyComplex<T>
where
    T: PartialEq + fmt::Debug + FloatUlps + FloatEqDebug,
    T::Epsilon: PartialEq + fmt::Debug + Sized,
    T::DebugEpsilon: PartialEq + fmt::Debug + Sized,
    Ulps<T>: PartialEq + fmt::Debug,
    Ulps<T::Epsilon>: PartialEq + fmt::Debug + Sized,
    Ulps<T::DebugEpsilon>: PartialEq + fmt::Debug + Sized,
{
    type DebugEpsilon = MyComplex<T::DebugEpsilon>;

    fn debug_abs_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        MyComplex {
            re: self.re.debug_abs_epsilon(&other.re, &max_diff.re),
            im: self.im.debug_abs_epsilon(&other.im, &max_diff.im),
        }
    }

    fn debug_rel_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        MyComplex {
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
    let a = MyComplex::<f32> { re: 1.0, im: 200.0 };
    let b = MyComplex::<f32> { re: 50.0, im: 1.0 };

    assert_eq!(
        a.debug_abs_epsilon(&b, &MyComplex::new(0.1, 0.2)),
        MyComplex::new(0.1, 0.2)
    );
    assert_eq!(
        a.debug_rel_epsilon(&b, &MyComplex::new(0.1, 0.2)),
        MyComplex::new(5.0, 40.0)
    );
    assert_eq!(
        a.debug_ulps_epsilon(&b, &MyComplexUlps::new(1, 2)),
        MyComplexUlps::new(1, 2)
    );
}

//------------------------------------------------------------------------------
// FloatEqDebug
//------------------------------------------------------------------------------
impl<T> FloatEqAllDebug for MyComplex<T>
where
    T: PartialEq + fmt::Debug + FloatUlps + FloatEqAllDebug,
    T::AllEpsilon: PartialEq + fmt::Debug,
    T::AllDebugEpsilon: PartialEq + fmt::Debug + Sized,
    Ulps<T>: PartialEq + fmt::Debug,
    Ulps<T::AllEpsilon>: PartialEq + fmt::Debug,
    Ulps<T::AllDebugEpsilon>: PartialEq + fmt::Debug + Sized,
{
    type AllDebugEpsilon = MyComplex<T::AllDebugEpsilon>;

    fn debug_abs_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Self::AllDebugEpsilon {
            re: self.re.debug_abs_all_epsilon(&other.re, max_diff),
            im: self.im.debug_abs_all_epsilon(&other.im, max_diff),
        }
    }

    fn debug_rel_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Self::AllDebugEpsilon {
            re: self.re.debug_rel_all_epsilon(&other.re, max_diff),
            im: self.im.debug_rel_all_epsilon(&other.im, max_diff),
        }
    }

    fn debug_ulps_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Ulps<Self::AllEpsilon>,
    ) -> Ulps<Self::AllDebugEpsilon> {
        MyComplexUlps {
            re: self.re.debug_ulps_all_epsilon(&other.re, max_diff),
            im: self.im.debug_ulps_all_epsilon(&other.im, max_diff),
        }
    }
}

#[test]
fn float_eq_all_debug() {
    let a = MyComplex::<f32> { re: 1.0, im: 200.0 };
    let b = MyComplex::<f32> { re: 50.0, im: 1.0 };

    assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), MyComplex::new(0.2, 0.2));
    assert_eq!(
        a.debug_rel_all_epsilon(&b, &0.2),
        MyComplex::new(10.0, 40.0)
    );
    assert_eq!(a.debug_ulps_all_epsilon(&b, &2), MyComplexUlps::new(2, 2));
}

//------------------------------------------------------------------------------
// assert_float_eq!/assert_float_ne!
//------------------------------------------------------------------------------
#[test]
fn assert_float_eq() {
    let a = MyComplex::<f32>::new(2.0, -1_000_000.);
    let b = MyComplex::<f32>::new(2.000_000_5, -1_000_000.06);

    assert_float_eq!(a, b, abs <= MyComplex::new(0.000_000_5, 0.07));
    assert_float_ne!(a, b, abs <= MyComplex::new(0.000_000_4, 0.07));
    assert_float_ne!(a, b, abs <= MyComplex::new(0.000_000_5, 0.06));
    assert_float_eq!(a, b, abs_all <= 0.07);
    assert_float_ne!(a, b, abs_all <= 0.06);

    assert_float_eq!(a, b, rel <= MyComplex::new(0.000_000_25, 0.000_000_1));
    assert_float_ne!(a, b, rel <= MyComplex::new(0.000_000_15, 0.000_000_1));
    assert_float_ne!(a, b, rel <= MyComplex::new(0.000_000_25, 0.000_000_05));
    assert_float_eq!(a, b, rel_all <= 0.000_000_25);
    assert_float_ne!(a, b, rel_all <= 0.000_000_15);

    assert_float_eq!(a, b, ulps <= MyComplexUlps::new(2, 1));
    assert_float_ne!(a, b, ulps <= MyComplexUlps::new(0, 1));
    assert_float_ne!(a, b, ulps <= MyComplexUlps::new(2, 0));
    assert_float_eq!(a, b, ulps_all <= 2);
    assert_float_ne!(a, b, ulps_all <= 1);
}
