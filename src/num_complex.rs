use crate::{FloatDiff, FloatEq, FloatEqDebug};
use num_complex::Complex;

/// The absolute difference between two floating point [`Complex<T>`] instances
/// in ULPs.
///
/// The `T` in [`Complex<T>`] is constrained by `Clone` and `PartialEq`, so this
/// implements those too.
///
/// [`Complex<T>`]: https://rust-num.github.io/num/num/struct.Complex.html
#[derive(Clone, Debug, PartialEq)]
pub struct ComplexUlpsDiff<T> {
    re: T,
    im: T,
}

impl<T> FloatDiff for Complex<T>
where
    T: FloatDiff,
{
    type AbsDiff = Complex<<T as FloatDiff>::AbsDiff>;
    type UlpsDiff = ComplexUlpsDiff<<T as FloatDiff>::UlpsDiff>;

    fn abs_diff(&self, other: &Self) -> Self::AbsDiff {
        Self::AbsDiff {
            re: self.re.abs_diff(&other.re),
            im: self.im.abs_diff(&other.im),
        }
    }

    fn ulps_diff(&self, other: &Self) -> Self::UlpsDiff {
        Self::UlpsDiff {
            re: self.re.ulps_diff(&other.re),
            im: self.im.ulps_diff(&other.im),
        }
    }
}

impl<T: FloatEq> FloatEq for Complex<T> {
    type DiffEpsilon = <T as FloatEq>::DiffEpsilon;
    type UlpsDiffEpsilon = <T as FloatEq>::UlpsDiffEpsilon;

    fn eq_abs(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
        self.re.eq_abs(&other.re, max_diff) && self.im.eq_abs(&other.im, max_diff)
    }

    fn eq_rel(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
        self.re.eq_rel(&other.re, max_diff) && self.im.eq_rel(&other.im, max_diff)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool {
        self.re.eq_ulps(&other.re, max_diff) && self.im.eq_ulps(&other.im, max_diff)
    }
}

impl<T> FloatEqDebug for Complex<T>
where
    T: FloatEqDebug,
{
    type DebugEpsilon = Complex<<T as FloatEqDebug>::DebugEpsilon>;
    type DebugUlpsEpsilon = ComplexUlpsDiff<<T as FloatEqDebug>::DebugUlpsEpsilon>;

    fn debug_abs_epsilon(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> Self::DebugEpsilon {
        Self::DebugEpsilon {
            re: self.re.debug_abs_epsilon(&other.re, max_diff),
            im: self.im.debug_abs_epsilon(&other.im, max_diff),
        }
    }

    fn debug_rel_epsilon(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> Self::DebugEpsilon {
        Self::DebugEpsilon {
            re: self.re.debug_rel_epsilon(&other.re, max_diff),
            im: self.im.debug_rel_epsilon(&other.im, max_diff),
        }
    }

    fn debug_ulps_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::UlpsDiffEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        Self::DebugUlpsEpsilon {
            re: self.re.debug_ulps_epsilon(&other.re, max_diff),
            im: self.im.debug_ulps_epsilon(&other.im, max_diff),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32;

    #[test]
    fn float_diff() {
        let a = Complex::<f32>::new(1., 2.);
        let b = Complex::<f32>::new(1.0000001, 2.0000004);

        let abs_diff = a.abs_diff(&b);
        assert_eq!(abs_diff.re, 0.00000011920929);
        assert_eq!(abs_diff.im, 0.00000047683716);

        let ulps_diff = a.ulps_diff(&b);
        assert_eq!(ulps_diff.re, 1);
        assert_eq!(ulps_diff.im, 2);
    }

    #[test]
    fn float_eq_methods() {
        let a = Complex::<f32>::new(1., 2.);
        let b = Complex::<f32>::new(1.0000002, 2.);
        let c = Complex::<f32>::new(1.0, 2.0000004);

        assert!(a.eq_abs(&b, &(2. * f32::EPSILON)));
        assert!(a.eq_abs(&c, &(4. * f32::EPSILON)));
        assert!(a.ne_abs(&b, &f32::EPSILON));
        assert!(a.ne_abs(&c, &(2. * f32::EPSILON)));

        assert!(a.eq_rel(&b, &(2. * f32::EPSILON)));
        assert!(a.eq_rel(&c, &(2. * f32::EPSILON)));
        assert!(a.ne_rel(&b, &f32::EPSILON));
        assert!(a.ne_rel(&c, &f32::EPSILON));

        assert!(a.eq_ulps(&b, &2));
        assert!(a.eq_ulps(&c, &2));
        assert!(a.ne_ulps(&b, &1));
        assert!(a.ne_ulps(&c, &1));
    }

    #[test]
    fn float_eq_macros() {
        let a = Complex::<f32>::new(1., 2.);
        let b = Complex::<f32>::new(1.0000002, 2.);
        let c = Complex::<f32>::new(1.0, 2.0000004);

        assert!(float_eq!(a, b, abs <= (2. * f32::EPSILON)));
        assert!(float_eq!(a, c, abs <= (4. * f32::EPSILON)));
        assert!(float_ne!(a, b, abs <= f32::EPSILON));
        assert!(float_ne!(a, c, abs <= (2. * f32::EPSILON)));

        assert!(float_eq!(a, b, rel <= (2. * f32::EPSILON)));
        assert!(float_eq!(a, c, rel <= (2. * f32::EPSILON)));
        assert!(float_ne!(a, b, rel <= f32::EPSILON));
        assert!(float_ne!(a, c, rel <= f32::EPSILON));

        assert!(float_eq!(a, b, ulps <= 2));
        assert!(float_eq!(a, c, ulps <= 2));
        assert!(float_ne!(a, b, ulps <= 1));
        assert!(float_ne!(a, c, ulps <= 1));
    }

    #[test]
    fn float_eq_debug() {
        let a = Complex::<f32>::new(1., 2.);
        let b = Complex::<f32>::new(1.0000001, 2.0000004);

        let abs_epsilon = a.debug_abs_epsilon(&b, &0.0000001);
        assert_eq!(abs_epsilon.re, 0.0000001);
        assert_eq!(abs_epsilon.im, 0.0000001);

        let rel_epsilon = a.debug_rel_epsilon(&b, &0.0000001);
        assert_eq!(rel_epsilon.re, 1.0000001 * 0.0000001);
        assert_eq!(rel_epsilon.im, 2.0000004 * 0.0000001);

        let ulps_epsilon = a.debug_ulps_epsilon(&b, &2);
        assert_eq!(ulps_epsilon.re, 2);
        assert_eq!(ulps_epsilon.im, 2);
    }

    #[test]
    fn assert_float_eq() {
        let a = Complex::<f32>::new(1., 2.);
        let b = Complex::<f32>::new(1.0000002, 2.);
        let c = Complex::<f32>::new(1.0, 2.0000004);

        assert_float_eq!(a, b, abs <= (2. * f32::EPSILON));
        assert_float_eq!(a, c, abs <= (4. * f32::EPSILON));
        assert_float_ne!(a, b, abs <= f32::EPSILON);
        assert_float_ne!(a, c, abs <= (2. * f32::EPSILON));

        assert_float_eq!(a, b, rel <= (2. * f32::EPSILON));
        assert_float_eq!(a, c, rel <= (2. * f32::EPSILON));
        assert_float_ne!(a, b, rel <= f32::EPSILON);
        assert_float_ne!(a, c, rel <= f32::EPSILON);

        assert_float_eq!(a, b, ulps <= 2);
        assert_float_eq!(a, c, ulps <= 2);
        assert_float_ne!(a, b, ulps <= 1);
        assert_float_ne!(a, c, ulps <= 1);
    }
}
