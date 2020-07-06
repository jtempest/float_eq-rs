#![allow(clippy::float_cmp, clippy::cognitive_complexity)]

use core::fmt;
use float_eq::{
    assert_float_eq, assert_float_ne, float_eq, float_ne, AssertFloatEq, AssertFloatEqAll,
    DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff, FloatEqUlpsEpsilon, UlpsEpsilon,
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
    T: FloatEqUlpsEpsilon + PartialEq + fmt::Debug,
    UlpsEpsilon<T>: PartialEq + fmt::Debug + Sized,
{
    re: UlpsEpsilon<T>,
    im: UlpsEpsilon<T>,
}

impl<T> MyComplexUlps<T>
where
    T: FloatEqUlpsEpsilon + PartialEq + fmt::Debug,
    UlpsEpsilon<T>: PartialEq + fmt::Debug + Sized,
{
    fn new(re: UlpsEpsilon<T>, im: UlpsEpsilon<T>) -> Self {
        Self { re, im }
    }
}

impl<T> FloatEqUlpsEpsilon for MyComplex<T>
where
    T: FloatEqUlpsEpsilon + PartialEq + fmt::Debug,
    UlpsEpsilon<T>: PartialEq + fmt::Debug + Sized,
{
    type UlpsEpsilon = MyComplexUlps<T>;
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
    T: PartialEq + fmt::Debug + FloatEqUlpsEpsilon + FloatEq,
    T::Epsilon: PartialEq + fmt::Debug + Sized,
    UlpsEpsilon<T>: PartialEq + fmt::Debug,
    UlpsEpsilon<T::Epsilon>: PartialEq + fmt::Debug + Sized,
{
    type Epsilon = MyComplex<T::Epsilon>;

    fn eq_abs(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_abs(&other.re, &max_diff.re) && self.im.eq_abs(&other.im, &max_diff.im)
    }

    fn eq_rel(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
        self.re.eq_rel(&other.re, &max_diff.re) && self.im.eq_rel(&other.im, &max_diff.im)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
        self.re.eq_ulps(&other.re, &max_diff.re) && self.im.eq_ulps(&other.im, &max_diff.im)
    }
}

#[test]
fn float_eq() {
    let a = MyComplex::<f32>::new(2.0, -1_000_000.0);
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
    T: FloatEqUlpsEpsilon + FloatEqAll,
{
    type AllEpsilon = T::AllEpsilon;

    fn eq_abs_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
        self.re.eq_abs_all(&other.re, &max_diff) && self.im.eq_abs_all(&other.im, &max_diff)
    }

    fn eq_rel_all(&self, other: &Self, max_diff: &Self::AllEpsilon) -> bool {
        self.re.eq_rel_all(&other.re, &max_diff) && self.im.eq_rel_all(&other.im, &max_diff)
    }

    fn eq_ulps_all(&self, other: &Self, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
        self.re.eq_ulps_all(&other.re, &max_diff) && self.im.eq_ulps_all(&other.im, &max_diff)
    }
}

#[test]
fn float_eq_all() {
    let a = MyComplex::<f32>::new(2.0, -1_000_000.0);
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
    let a = MyComplex::<f32>::new(2.0, -1_000_000.0);
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
// AssertFloatEq
//------------------------------------------------------------------------------
impl<T> AssertFloatEq for MyComplex<T>
where
    T: PartialEq + fmt::Debug + FloatEqUlpsEpsilon + AssertFloatEq + FloatEqDebugUlpsDiff,
    T::Epsilon: PartialEq + fmt::Debug + Sized,
    T::DebugEpsilon: PartialEq + fmt::Debug + Sized,
    T::DebugAbsDiff: PartialEq + fmt::Debug,
    UlpsEpsilon<T>: PartialEq + fmt::Debug,
    UlpsEpsilon<T::Epsilon>: PartialEq + fmt::Debug + Sized,
    UlpsEpsilon<T::DebugEpsilon>: PartialEq + fmt::Debug + Sized,
    DebugUlpsDiff<T::DebugAbsDiff>: PartialEq + fmt::Debug,
{
    type DebugAbsDiff = MyComplex<T::DebugAbsDiff>;
    type DebugEpsilon = MyComplex<T::DebugEpsilon>;

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
        max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon> {
        UlpsEpsilon::<Self::DebugEpsilon> {
            re: self.re.debug_ulps_epsilon(&other.re, &max_diff.re),
            im: self.im.debug_ulps_epsilon(&other.im, &max_diff.im),
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
// AssertFloatEq
//------------------------------------------------------------------------------
impl<T> AssertFloatEqAll for MyComplex<T>
where
    T: PartialEq + fmt::Debug + FloatEqUlpsEpsilon + AssertFloatEqAll,
    T::AllEpsilon: PartialEq + fmt::Debug,
    T::AllDebugEpsilon: PartialEq + fmt::Debug + Sized,
    UlpsEpsilon<T>: PartialEq + fmt::Debug,
    UlpsEpsilon<T::AllEpsilon>: PartialEq + fmt::Debug,
    UlpsEpsilon<T::AllDebugEpsilon>: PartialEq + fmt::Debug + Sized,
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
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon> {
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
    let a = MyComplex::<f32>::new(2.0, -1_000_000.0);
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
