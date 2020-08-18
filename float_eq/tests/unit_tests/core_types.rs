#![allow(clippy::many_single_char_names)]

use core::cell::{Cell, RefCell};
use float_eq::{assert_float_eq, assert_float_ne, AssertFloatEq, AssertFloatEqAll};

#[allow(clippy::unnecessary_mut_passed)]
mod refs {
    use super::*;

    #[test]
    fn float_eq() {
        let a = &1.0f32;
        let b = &1.5f32;
        let mut ma = &mut 1.0f32;
        let mut mb = &mut 1.5f32;

        assert_float_eq!(&a, &b, abs <= 1.0);
        assert_float_eq!(&mut ma, &b, abs <= 1.0);
        assert_float_eq!(&a, &mut mb, abs <= 1.0);
        assert_float_eq!(&mut ma, &mut mb, abs <= 1.0);

        let c = &0.999_999_9f32;
        let mut mc = &mut 0.999_999_9f32;
        let eps = 1.0 * f32::EPSILON;
        let eps2 = 2.0 * f32::EPSILON;

        assert_float_eq!(&a, &c, rel <= eps);
        assert_float_eq!(&mut ma, &c, rel <= eps);
        assert_float_eq!(&a, &mut mc, rel <= eps);
        assert_float_eq!(&mut ma, &mut mc, rel <= eps);

        assert_float_eq!(&a, &c, rmax <= eps);
        assert_float_eq!(&mut ma, &c, rmax <= eps);
        assert_float_eq!(&a, &mut mc, rmax <= eps);
        assert_float_eq!(&mut ma, &mut mc, rmax <= eps);

        assert_float_eq!(&a, &c, rmin <= eps2);
        assert_float_eq!(&mut ma, &c, rmin <= eps2);
        assert_float_eq!(&a, &mut mc, rmin <= eps2);
        assert_float_eq!(&mut ma, &mut mc, rmin <= eps2);

        assert_float_eq!(&a, &c, r1st <= eps);
        assert_float_eq!(&mut ma, &c, r1st <= eps);
        assert_float_eq!(&a, &mut mc, r1st <= eps);
        assert_float_eq!(&mut ma, &mut mc, r1st <= eps);

        assert_float_eq!(&a, &c, r2nd <= eps2);
        assert_float_eq!(&mut ma, &c, r2nd <= eps2);
        assert_float_eq!(&a, &mut mc, r2nd <= eps2);
        assert_float_eq!(&mut ma, &mut mc, r2nd <= eps2);

        assert_float_eq!(&a, &c, ulps <= 2);
        assert_float_eq!(&mut ma, &c, ulps <= 2);
        assert_float_eq!(&a, &mut mc, ulps <= 2);
        assert_float_eq!(&mut ma, &mut mc, ulps <= 2);
    }

    #[test]
    fn float_eq_all() {
        let a = &1.0f32;
        let b = &1.5f32;
        let mut ma = &mut 1.0f32;
        let mut mb = &mut 1.5f32;

        assert_float_eq!(&a, &b, abs_all <= 1.0);
        assert_float_eq!(&mut ma, &b, abs_all <= 1.0);
        assert_float_eq!(&a, &mut mb, abs_all <= 1.0);
        assert_float_eq!(&mut ma, &mut mb, abs_all <= 1.0);

        let c = &0.999_999_9f32;
        let mut mc = &mut 0.999_999_9f32;
        let eps = 1.0 * f32::EPSILON;
        let eps2 = 2.0 * f32::EPSILON;

        assert_float_eq!(&a, &c, rel_all <= eps);
        assert_float_eq!(&mut ma, &c, rel_all <= eps);
        assert_float_eq!(&a, &mut mc, rel_all <= eps);
        assert_float_eq!(&mut ma, &mut mc, rel_all <= eps);

        assert_float_eq!(&a, &c, rmax_all <= eps);
        assert_float_eq!(&mut ma, &c, rmax_all <= eps);
        assert_float_eq!(&a, &mut mc, rmax_all <= eps);
        assert_float_eq!(&mut ma, &mut mc, rmax_all <= eps);

        assert_float_eq!(&a, &c, rmin_all <= eps2);
        assert_float_eq!(&mut ma, &c, rmin_all <= eps2);
        assert_float_eq!(&a, &mut mc, rmin_all <= eps2);
        assert_float_eq!(&mut ma, &mut mc, rmin_all <= eps2);

        assert_float_eq!(&a, &c, r1st_all <= eps);
        assert_float_eq!(&mut ma, &c, r1st_all <= eps);
        assert_float_eq!(&a, &mut mc, r1st_all <= eps);
        assert_float_eq!(&mut ma, &mut mc, r1st_all <= eps);

        assert_float_eq!(&a, &c, r2nd_all <= eps2);
        assert_float_eq!(&mut ma, &c, r2nd_all <= eps2);
        assert_float_eq!(&a, &mut mc, r2nd_all <= eps2);
        assert_float_eq!(&mut ma, &mut mc, r2nd_all <= eps2);

        assert_float_eq!(&a, &c, ulps_all <= 2);
        assert_float_eq!(&mut ma, &c, ulps_all <= 2);
        assert_float_eq!(&a, &mut mc, ulps_all <= 2);
        assert_float_eq!(&mut ma, &mut mc, ulps_all <= 2);
    }

    #[test]
    fn debug_diff() {
        let a = &1.0f32;
        let b = &1.5f32;
        let mut ma = &mut 1.0f32;
        let mut mb = &mut 1.5f32;

        assert_eq!(AssertFloatEq::debug_abs_diff(&a, &b), 0.5);
        assert_eq!(AssertFloatEq::debug_abs_diff(&mut ma, &b), 0.5);
        assert_eq!(AssertFloatEq::debug_abs_diff(&a, &mut mb), 0.5);
        assert_eq!(AssertFloatEq::debug_abs_diff(&mut ma, &mut mb), 0.5);

        let c = &1.000_000_2f32;
        let mut mc = &mut 1.000_000_2f32;

        assert_eq!(AssertFloatEq::debug_ulps_diff(&a, &c), Some(2));
        assert_eq!(AssertFloatEq::debug_ulps_diff(&mut ma, &c), Some(2));
        assert_eq!(AssertFloatEq::debug_ulps_diff(&a, &mut mc), Some(2));
        assert_eq!(AssertFloatEq::debug_ulps_diff(&mut ma, &mut mc), Some(2));
    }

    #[test]
    fn debug_epsilon() {
        use AssertFloatEq as AFE;

        let a = &1.0f32;
        let b = &2.0f32;
        let mut ma = &mut 1.0f32;
        let mut mb = &mut 2.0f32;

        assert_eq!(AFE::debug_abs_epsilon(&a, &b, &0.5), 0.5);
        assert_eq!(AFE::debug_abs_epsilon(&mut ma, &b, &0.5), 0.5);
        assert_eq!(AFE::debug_abs_epsilon(&a, &mut mb, &0.5), 0.5);
        assert_eq!(AFE::debug_abs_epsilon(&mut ma, &mut mb, &0.5), 0.5);

        assert_eq!(AFE::debug_rel_epsilon(&a, &b, &0.1), 0.2);
        assert_eq!(AFE::debug_rel_epsilon(&mut ma, &b, &0.1), 0.2);
        assert_eq!(AFE::debug_rel_epsilon(&a, &mut mb, &0.1), 0.2);
        assert_eq!(AFE::debug_rel_epsilon(&mut ma, &mut mb, &0.1), 0.2);

        assert_eq!(AFE::debug_rmax_epsilon(&a, &b, &0.1), 0.2);
        assert_eq!(AFE::debug_rmax_epsilon(&mut ma, &b, &0.1), 0.2);
        assert_eq!(AFE::debug_rmax_epsilon(&a, &mut mb, &0.1), 0.2);
        assert_eq!(AFE::debug_rmax_epsilon(&mut ma, &mut mb, &0.1), 0.2);

        assert_eq!(AFE::debug_rmin_epsilon(&a, &b, &0.1), 0.1);
        assert_eq!(AFE::debug_rmin_epsilon(&mut ma, &b, &0.1), 0.1);
        assert_eq!(AFE::debug_rmin_epsilon(&a, &mut mb, &0.1), 0.1);
        assert_eq!(AFE::debug_rmin_epsilon(&mut ma, &mut mb, &0.1), 0.1);

        assert_eq!(AFE::debug_r1st_epsilon(&a, &b, &0.1), 0.1);
        assert_eq!(AFE::debug_r1st_epsilon(&mut ma, &b, &0.1), 0.1);
        assert_eq!(AFE::debug_r1st_epsilon(&a, &mut mb, &0.1), 0.1);
        assert_eq!(AFE::debug_r1st_epsilon(&mut ma, &mut mb, &0.1), 0.1);

        assert_eq!(AFE::debug_r2nd_epsilon(&a, &b, &0.1), 0.2);
        assert_eq!(AFE::debug_r2nd_epsilon(&mut ma, &b, &0.1), 0.2);
        assert_eq!(AFE::debug_r2nd_epsilon(&a, &mut mb, &0.1), 0.2);
        assert_eq!(AFE::debug_r2nd_epsilon(&mut ma, &mut mb, &0.1), 0.2);

        assert_eq!(AFE::debug_ulps_epsilon(&a, &b, &1), 1);
        assert_eq!(AFE::debug_ulps_epsilon(&mut ma, &b, &1), 1);
        assert_eq!(AFE::debug_ulps_epsilon(&a, &mut mb, &1), 1);
        assert_eq!(AFE::debug_ulps_epsilon(&mut ma, &mut mb, &1), 1);
    }

    #[test]
    fn debug_all_epsilon() {
        use AssertFloatEqAll as AFEA;

        let a = &1.0f32;
        let b = &2.0f32;
        let mut ma = &mut 1.0f32;
        let mut mb = &mut 2.0f32;

        assert_eq!(AFEA::debug_abs_all_epsilon(&a, &b, &0.5), 0.5);
        assert_eq!(AFEA::debug_abs_all_epsilon(&mut ma, &b, &0.5), 0.5);
        assert_eq!(AFEA::debug_abs_all_epsilon(&a, &mut mb, &0.5), 0.5);
        assert_eq!(AFEA::debug_abs_all_epsilon(&mut ma, &mut mb, &0.5), 0.5);

        assert_eq!(AFEA::debug_rel_all_epsilon(&a, &b, &0.1), 0.2);
        assert_eq!(AFEA::debug_rel_all_epsilon(&mut ma, &b, &0.1), 0.2);
        assert_eq!(AFEA::debug_rel_all_epsilon(&a, &mut mb, &0.1), 0.2);
        assert_eq!(AFEA::debug_rel_all_epsilon(&mut ma, &mut mb, &0.1), 0.2);

        assert_eq!(AFEA::debug_rmax_all_epsilon(&a, &b, &0.1), 0.2);
        assert_eq!(AFEA::debug_rmax_all_epsilon(&mut ma, &b, &0.1), 0.2);
        assert_eq!(AFEA::debug_rmax_all_epsilon(&a, &mut mb, &0.1), 0.2);
        assert_eq!(AFEA::debug_rmax_all_epsilon(&mut ma, &mut mb, &0.1), 0.2);

        assert_eq!(AFEA::debug_rmin_all_epsilon(&a, &b, &0.1), 0.1);
        assert_eq!(AFEA::debug_rmin_all_epsilon(&mut ma, &b, &0.1), 0.1);
        assert_eq!(AFEA::debug_rmin_all_epsilon(&a, &mut mb, &0.1), 0.1);
        assert_eq!(AFEA::debug_rmin_all_epsilon(&mut ma, &mut mb, &0.1), 0.1);

        assert_eq!(AFEA::debug_r1st_all_epsilon(&a, &b, &0.1), 0.1);
        assert_eq!(AFEA::debug_r1st_all_epsilon(&mut ma, &b, &0.1), 0.1);
        assert_eq!(AFEA::debug_r1st_all_epsilon(&a, &mut mb, &0.1), 0.1);
        assert_eq!(AFEA::debug_r1st_all_epsilon(&mut ma, &mut mb, &0.1), 0.1);

        assert_eq!(AFEA::debug_r2nd_all_epsilon(&a, &b, &0.1), 0.2);
        assert_eq!(AFEA::debug_r2nd_all_epsilon(&mut ma, &b, &0.1), 0.2);
        assert_eq!(AFEA::debug_r2nd_all_epsilon(&a, &mut mb, &0.1), 0.2);
        assert_eq!(AFEA::debug_r2nd_all_epsilon(&mut ma, &mut mb, &0.1), 0.2);

        assert_eq!(AFEA::debug_ulps_all_epsilon(&a, &b, &1), 1);
        assert_eq!(AFEA::debug_ulps_all_epsilon(&mut ma, &b, &1), 1);
        assert_eq!(AFEA::debug_ulps_all_epsilon(&a, &mut mb, &1), 1);
        assert_eq!(AFEA::debug_ulps_all_epsilon(&mut ma, &mut mb, &1), 1);
    }
}

mod option {
    use super::*;

    #[test]
    fn float_eq() {
        let a = Some([0.999_999_9f32, 4.0]);
        let b = Some([1.0f32, 3.999_999_5]);
        let eps = f32::EPSILON;

        // Same shape Some(a/b/epsilon)
        assert_float_eq!(a, b, abs <= Some([1.0 * eps, 4.0 * eps]));
        assert_float_ne!(a, b, abs <= Some([0.5 * eps, 4.0 * eps]));
        assert_float_ne!(a, b, abs <= Some([1.0 * eps, 2.0 * eps]));

        assert_float_eq!(a, b, rel <= Some([1.0 * eps, 1.0 * eps]));
        assert_float_ne!(a, b, rel <= Some([0.5 * eps, 1.0 * eps]));
        assert_float_ne!(a, b, rel <= Some([1.0 * eps, 0.5 * eps]));

        assert_float_eq!(a, b, rmax <= Some([1.0 * eps, 1.0 * eps]));
        assert_float_ne!(a, b, rmax <= Some([0.5 * eps, 1.0 * eps]));
        assert_float_ne!(a, b, rmax <= Some([1.0 * eps, 0.5 * eps]));

        assert_float_eq!(a, b, rmin <= Some([2.0 * eps, 2.0 * eps]));
        assert_float_ne!(a, b, rmin <= Some([1.0 * eps, 2.0 * eps]));
        assert_float_ne!(a, b, rmin <= Some([2.0 * eps, 1.0 * eps]));

        assert_float_eq!(a, b, r1st <= Some([2.0 * eps, 1.0 * eps]));
        assert_float_ne!(a, b, r1st <= Some([1.0 * eps, 1.0 * eps]));
        assert_float_ne!(a, b, r1st <= Some([2.0 * eps, 0.5 * eps]));

        assert_float_eq!(a, b, r2nd <= Some([1.0 * eps, 2.0 * eps]));
        assert_float_ne!(a, b, r2nd <= Some([0.5 * eps, 2.0 * eps]));
        assert_float_ne!(a, b, r2nd <= Some([1.0 * eps, 1.0 * eps]));

        assert_float_eq!(a, b, ulps <= Some([2, 2]));
        assert_float_ne!(a, b, ulps <= Some([1, 2]));
        assert_float_ne!(a, b, ulps <= Some([2, 1]));

        // Same shape None
        let none = Option::<[f32; 2]>::None;
        assert_float_ne!(none, None, abs <= None);
        assert_float_ne!(none, None, rel <= None);
        assert_float_ne!(none, None, rmax <= None);
        assert_float_ne!(none, None, rmin <= None);
        assert_float_ne!(none, None, r1st <= None);
        assert_float_ne!(none, None, r2nd <= None);
        assert_float_ne!(none, None, ulps <= None);

        // Differing a/b shape
        assert_float_ne!(a, None, abs <= Some([f32::INFINITY; 2]));
        assert_float_ne!(a, None, rel <= Some([f32::INFINITY; 2]));
        assert_float_ne!(a, None, rmax <= Some([f32::INFINITY; 2]));
        assert_float_ne!(a, None, rmin <= Some([f32::INFINITY; 2]));
        assert_float_ne!(a, None, r1st <= Some([f32::INFINITY; 2]));
        assert_float_ne!(a, None, r2nd <= Some([f32::INFINITY; 2]));
        assert_float_ne!(a, None, ulps <= Some([u32::MAX; 2]));

        assert_float_ne!(None, a, abs <= Some([f32::INFINITY; 2]));
        assert_float_ne!(None, a, rel <= Some([f32::INFINITY; 2]));
        assert_float_ne!(None, a, rmax <= Some([f32::INFINITY; 2]));
        assert_float_ne!(None, a, rmin <= Some([f32::INFINITY; 2]));
        assert_float_ne!(None, a, r1st <= Some([f32::INFINITY; 2]));
        assert_float_ne!(None, a, r2nd <= Some([f32::INFINITY; 2]));
        assert_float_ne!(None, a, ulps <= Some([u32::MAX; 2]));

        // Differing epsilon shape
        assert_float_ne!(a, a, abs <= None);
        assert_float_ne!(a, a, rel <= None);
        assert_float_ne!(a, a, rmax <= None);
        assert_float_ne!(a, a, rmin <= None);
        assert_float_ne!(a, a, r1st <= None);
        assert_float_ne!(a, a, r2nd <= None);
        assert_float_ne!(a, a, ulps <= None);
    }

    #[test]
    fn float_eq_all() {
        let a = Some([0.999_999_9f32, 4.0]);
        let b = Some([1.0f32, 3.999_999_5]);
        let eps = f32::EPSILON;

        // Same shape Some(a/b/epsilon)
        assert_float_eq!(a, b, abs_all <= Some(4.0 * eps));
        assert_float_ne!(a, b, abs_all <= Some(2.0 * eps));

        assert_float_eq!(a, b, rel_all <= Some(1.0 * eps));
        assert_float_ne!(a, b, rel_all <= Some(0.5 * eps));

        assert_float_eq!(a, b, rmax_all <= Some(1.0 * eps));
        assert_float_ne!(a, b, rmax_all <= Some(0.5 * eps));

        assert_float_eq!(a, b, rmin_all <= Some(2.0 * eps));
        assert_float_ne!(a, b, rmin_all <= Some(1.0 * eps));

        assert_float_eq!(a, b, r1st_all <= Some(2.0 * eps));
        assert_float_ne!(a, b, r1st_all <= Some(1.0 * eps));

        assert_float_eq!(a, b, r2nd_all <= Some(2.0 * eps));
        assert_float_ne!(a, b, r2nd_all <= Some(1.0 * eps));

        assert_float_eq!(a, b, ulps_all <= Some(2));
        assert_float_ne!(a, b, ulps_all <= Some(1));

        // Same shape None
        let none = Option::<[f32; 2]>::None;
        assert_float_ne!(none, None, abs_all <= None);
        assert_float_ne!(none, None, rel_all <= None);
        assert_float_ne!(none, None, rmax_all <= None);
        assert_float_ne!(none, None, rmin_all <= None);
        assert_float_ne!(none, None, r1st_all <= None);
        assert_float_ne!(none, None, r2nd_all <= None);
        assert_float_ne!(none, None, ulps_all <= None);

        // Differing a/b shape
        assert_float_ne!(a, None, abs_all <= Some(f32::INFINITY));
        assert_float_ne!(a, None, rel_all <= Some(f32::INFINITY));
        assert_float_ne!(a, None, rmax_all <= Some(f32::INFINITY));
        assert_float_ne!(a, None, rmin_all <= Some(f32::INFINITY));
        assert_float_ne!(a, None, r1st_all <= Some(f32::INFINITY));
        assert_float_ne!(a, None, r2nd_all <= Some(f32::INFINITY));
        assert_float_ne!(a, None, ulps_all <= Some(u32::MAX));

        assert_float_ne!(None, a, abs_all <= Some(f32::INFINITY));
        assert_float_ne!(None, a, rel_all <= Some(f32::INFINITY));
        assert_float_ne!(None, a, rmax_all <= Some(f32::INFINITY));
        assert_float_ne!(None, a, rmin_all <= Some(f32::INFINITY));
        assert_float_ne!(None, a, r1st_all <= Some(f32::INFINITY));
        assert_float_ne!(None, a, r2nd_all <= Some(f32::INFINITY));
        assert_float_ne!(None, a, ulps_all <= Some(u32::MAX));

        // Differing epsilon shape
        assert_float_ne!(a, a, abs_all <= None);
        assert_float_ne!(a, a, rel_all <= None);
        assert_float_ne!(a, a, rmax_all <= None);
        assert_float_ne!(a, a, rmin_all <= None);
        assert_float_ne!(a, a, r1st_all <= None);
        assert_float_ne!(a, a, r2nd_all <= None);
        assert_float_ne!(a, a, ulps_all <= None);
    }

    #[test]
    fn debug_diff() {
        let a = Some([1.0f32, 2.0]);
        let b = Some([1.5f32, 2.25]);
        let ulps = Some([Some(4_194_304), Some(1_048_576)]);

        // Same shape Some(a/b/epsilon)
        assert_eq!(a.debug_abs_diff(&a), Some([0.0; 2]));
        assert_eq!(a.debug_ulps_diff(&a), Some([Some(0); 2]));

        assert_eq!(a.debug_abs_diff(&b), Some([0.5, 0.25]));
        assert_eq!(b.debug_abs_diff(&a), Some([0.5, 0.25]));

        assert_eq!(a.debug_ulps_diff(&b), ulps);
        assert_eq!(b.debug_ulps_diff(&a), ulps);

        // Same shape None
        let none = Option::<[f32; 2]>::None;
        assert_eq!(none.debug_abs_diff(&None), None);
        assert_eq!(none.debug_ulps_diff(&None), None);

        // Different shape
        assert_eq!(a.debug_abs_diff(&None), None);
        assert_eq!(None.debug_abs_diff(&a), None);

        assert_eq!(a.debug_ulps_diff(&None), None);
        assert_eq!(None.debug_ulps_diff(&a), None);
    }

    #[test]
    fn debug_epsilon() {
        let a = Some([2.0f32, 4.25]);
        let b = Some([2.5f32, 4.0]);
        let eps = Some([0.1, 0.2]);

        // Same shape Some(a/b/epsilon)
        assert_eq!(a.debug_abs_epsilon(&b, &eps), Some([0.1, 0.2]));
        assert_eq!(a.debug_rel_epsilon(&b, &eps), Some([0.25, 0.85]));
        assert_eq!(a.debug_rmax_epsilon(&b, &eps), Some([0.25, 0.85]));
        assert_eq!(a.debug_rmin_epsilon(&b, &eps), Some([0.2, 0.8]));
        assert_eq!(a.debug_r1st_epsilon(&b, &eps), Some([0.2, 0.85]));
        assert_eq!(a.debug_r2nd_epsilon(&b, &eps), Some([0.25, 0.8]));
        assert_eq!(a.debug_ulps_epsilon(&b, &Some([1, 2])), Some([1, 2]));

        // Same shape None
        let none = Option::<[f32; 2]>::None;
        assert_eq!(none.debug_abs_epsilon(&a, &None), None);
        assert_eq!(none.debug_rel_epsilon(&a, &None), None);
        assert_eq!(none.debug_rmax_epsilon(&a, &None), None);
        assert_eq!(none.debug_rmin_epsilon(&a, &None), None);
        assert_eq!(none.debug_r1st_epsilon(&a, &None), None);
        assert_eq!(none.debug_r2nd_epsilon(&a, &None), None);
        assert_eq!(none.debug_ulps_epsilon(&a, &None), None);

        // Different shape a/b
        assert_eq!(a.debug_abs_epsilon(&None, &eps), None);
        assert_eq!(a.debug_rel_epsilon(&None, &eps), None);
        assert_eq!(a.debug_rmax_epsilon(&None, &eps), None);
        assert_eq!(a.debug_rmin_epsilon(&None, &eps), None);
        assert_eq!(a.debug_r1st_epsilon(&None, &eps), None);
        assert_eq!(a.debug_r2nd_epsilon(&None, &eps), None);
        assert_eq!(a.debug_ulps_epsilon(&None, &Some([1, 2])), None);

        assert_eq!(none.debug_abs_epsilon(&a, &eps), None);
        assert_eq!(none.debug_rel_epsilon(&a, &eps), None);
        assert_eq!(none.debug_rmax_epsilon(&a, &eps), None);
        assert_eq!(none.debug_rmin_epsilon(&a, &eps), None);
        assert_eq!(none.debug_r1st_epsilon(&a, &eps), None);
        assert_eq!(none.debug_r2nd_epsilon(&a, &eps), None);
        assert_eq!(none.debug_ulps_epsilon(&a, &Some([1, 2])), None);

        // Different shape epsilon
        assert_eq!(a.debug_abs_epsilon(&b, &None), None);
        assert_eq!(a.debug_rel_epsilon(&b, &None), None);
        assert_eq!(a.debug_rmax_epsilon(&b, &None), None);
        assert_eq!(a.debug_rmin_epsilon(&b, &None), None);
        assert_eq!(a.debug_r1st_epsilon(&b, &None), None);
        assert_eq!(a.debug_r2nd_epsilon(&b, &None), None);
        assert_eq!(a.debug_ulps_epsilon(&b, &None), None);
    }

    #[test]
    fn debug_all_epsilon() {
        let a = Some([2.0f32, 4.25]);
        let b = Some([2.5f32, 4.0]);

        // Same shape Some(a/b/epsilon)
        assert_eq!(a.debug_abs_all_epsilon(&b, &Some(0.2)), Some([0.2, 0.2]));
        assert_eq!(a.debug_rel_all_epsilon(&b, &Some(0.2)), Some([0.5, 0.85]));
        assert_eq!(a.debug_rmax_all_epsilon(&b, &Some(0.2)), Some([0.5, 0.85]));
        assert_eq!(a.debug_rmin_all_epsilon(&b, &Some(0.2)), Some([0.4, 0.8]));
        assert_eq!(a.debug_r1st_all_epsilon(&b, &Some(0.2)), Some([0.4, 0.85]));
        assert_eq!(a.debug_r2nd_all_epsilon(&b, &Some(0.2)), Some([0.5, 0.8]));
        assert_eq!(a.debug_ulps_all_epsilon(&b, &Some(2)), Some([2, 2]));

        // Same shape None
        let none = Option::<[f32; 2]>::None;
        assert_eq!(none.debug_abs_all_epsilon(&a, &None), None);
        assert_eq!(none.debug_rel_all_epsilon(&a, &None), None);
        assert_eq!(none.debug_rmax_all_epsilon(&a, &None), None);
        assert_eq!(none.debug_rmin_all_epsilon(&a, &None), None);
        assert_eq!(none.debug_r1st_all_epsilon(&a, &None), None);
        assert_eq!(none.debug_r2nd_all_epsilon(&a, &None), None);
        assert_eq!(none.debug_ulps_all_epsilon(&a, &None), None);

        // Different shape a/b
        assert_eq!(a.debug_abs_all_epsilon(&None, &Some(0.2)), None);
        assert_eq!(a.debug_rel_all_epsilon(&None, &Some(0.2)), None);
        assert_eq!(a.debug_rmax_all_epsilon(&None, &Some(0.2)), None);
        assert_eq!(a.debug_rmin_all_epsilon(&None, &Some(0.2)), None);
        assert_eq!(a.debug_r1st_all_epsilon(&None, &Some(0.2)), None);
        assert_eq!(a.debug_r2nd_all_epsilon(&None, &Some(0.2)), None);
        assert_eq!(a.debug_ulps_all_epsilon(&None, &Some(2)), None);

        assert_eq!(none.debug_abs_all_epsilon(&a, &Some(0.2)), None);
        assert_eq!(none.debug_rel_all_epsilon(&a, &Some(0.2)), None);
        assert_eq!(none.debug_rmax_all_epsilon(&a, &Some(0.2)), None);
        assert_eq!(none.debug_rmin_all_epsilon(&a, &Some(0.2)), None);
        assert_eq!(none.debug_r1st_all_epsilon(&a, &Some(0.2)), None);
        assert_eq!(none.debug_r2nd_all_epsilon(&a, &Some(0.2)), None);
        assert_eq!(none.debug_ulps_all_epsilon(&a, &Some(2)), None);

        // Different shape epsilon
        assert_eq!(a.debug_abs_all_epsilon(&b, &None), None);
        assert_eq!(a.debug_rel_all_epsilon(&b, &None), None);
        assert_eq!(a.debug_rmax_all_epsilon(&b, &None), None);
        assert_eq!(a.debug_rmin_all_epsilon(&b, &None), None);
        assert_eq!(a.debug_r1st_all_epsilon(&b, &None), None);
        assert_eq!(a.debug_r2nd_all_epsilon(&b, &None), None);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &None), None);
    }
}

mod cell {
    use super::*;
    wrapper_tests!(Cell);
}

mod ref_cell {
    use super::*;
    wrapper_tests!(RefCell);
}

// Note: there are more slice tests in std_types, since only some of the slice
// comparison implementation is no_std.
mod slice {
    use super::*;

    #[test]
    fn float_eq() {
        let a = [0.999_999_9f32, 4.0];
        let b = [1.0f32, 3.999_999_5];
        let eps = f32::EPSILON;

        // Same shape a/b/epsilon
        assert_float_eq!(a[..], b[..], abs <= [1.0 * eps, 4.0 * eps]);
        assert_float_ne!(a[..], b[..], abs <= [0.5 * eps, 4.0 * eps]);
        assert_float_ne!(a[..], b[..], abs <= [1.0 * eps, 2.0 * eps]);

        assert_float_eq!(a[..], b[..], rel <= [1.0 * eps, 1.0 * eps]);
        assert_float_ne!(a[..], b[..], rel <= [0.5 * eps, 1.0 * eps]);
        assert_float_ne!(a[..], b[..], rel <= [1.0 * eps, 0.5 * eps]);

        assert_float_eq!(a[..], b[..], rmax <= [1.0 * eps, 1.0 * eps]);
        assert_float_ne!(a[..], b[..], rmax <= [0.5 * eps, 1.0 * eps]);
        assert_float_ne!(a[..], b[..], rmax <= [1.0 * eps, 0.5 * eps]);

        assert_float_eq!(a[..], b[..], rmin <= [2.0 * eps, 2.0 * eps]);
        assert_float_ne!(a[..], b[..], rmin <= [1.0 * eps, 2.0 * eps]);
        assert_float_ne!(a[..], b[..], rmin <= [2.0 * eps, 1.0 * eps]);

        assert_float_eq!(a[..], b[..], r1st <= [2.0 * eps, 1.0 * eps]);
        assert_float_ne!(a[..], b[..], r1st <= [1.0 * eps, 1.0 * eps]);
        assert_float_ne!(a[..], b[..], r1st <= [2.0 * eps, 0.5 * eps]);

        assert_float_eq!(a[..], b[..], r2nd <= [1.0 * eps, 2.0 * eps]);
        assert_float_ne!(a[..], b[..], r2nd <= [0.5 * eps, 2.0 * eps]);
        assert_float_ne!(a[..], b[..], r2nd <= [1.0 * eps, 1.0 * eps]);

        assert_float_eq!(a[..], b[..], ulps <= [2, 2]);
        assert_float_ne!(a[..], b[..], ulps <= [1, 2]);
        assert_float_ne!(a[..], b[..], ulps <= [2, 1]);

        // Different shape a/b
        assert_float_ne!(a[1..], b[..], abs <= [f32::INFINITY; 2]);
        assert_float_ne!(a[1..], b[..], rel <= [f32::INFINITY; 2]);
        assert_float_ne!(a[1..], b[..], rmax <= [f32::INFINITY; 2]);
        assert_float_ne!(a[1..], b[..], rmin <= [f32::INFINITY; 2]);
        assert_float_ne!(a[1..], b[..], r1st <= [f32::INFINITY; 2]);
        assert_float_ne!(a[1..], b[..], r2nd <= [f32::INFINITY; 2]);
        assert_float_ne!(a[1..], b[..], ulps <= [u32::MAX; 2]);

        assert_float_ne!(a[..], b[..1], abs <= [f32::INFINITY; 2]);
        assert_float_ne!(a[..], b[..1], rel <= [f32::INFINITY; 2]);
        assert_float_ne!(a[..], b[..1], rmax <= [f32::INFINITY; 2]);
        assert_float_ne!(a[..], b[..1], rmin <= [f32::INFINITY; 2]);
        assert_float_ne!(a[..], b[..1], r1st <= [f32::INFINITY; 2]);
        assert_float_ne!(a[..], b[..1], r2nd <= [f32::INFINITY; 2]);
        assert_float_ne!(a[..], b[..1], ulps <= [u32::MAX; 2]);

        // Different shape epsilon
        assert_float_ne!(a[..], b[..], abs <= [f32::INFINITY]);
        assert_float_ne!(a[..], b[..], rel <= [f32::INFINITY]);
        assert_float_ne!(a[..], b[..], rmax <= [f32::INFINITY]);
        assert_float_ne!(a[..], b[..], rmin <= [f32::INFINITY]);
        assert_float_ne!(a[..], b[..], r1st <= [f32::INFINITY]);
        assert_float_ne!(a[..], b[..], r2nd <= [f32::INFINITY]);
        assert_float_ne!(a[..], b[..], ulps <= [u32::MAX]);

        assert_float_ne!(a[..], b[..], abs <= [f32::INFINITY; 3]);
        assert_float_ne!(a[..], b[..], rel <= [f32::INFINITY; 3]);
        assert_float_ne!(a[..], b[..], rmax <= [f32::INFINITY; 3]);
        assert_float_ne!(a[..], b[..], rmin <= [f32::INFINITY; 3]);
        assert_float_ne!(a[..], b[..], r1st <= [f32::INFINITY; 3]);
        assert_float_ne!(a[..], b[..], r2nd <= [f32::INFINITY; 3]);
        assert_float_ne!(a[..], b[..], ulps <= [u32::MAX; 3]);
    }

    #[test]
    fn float_eq_all() {
        let a = [0.999_999_9f32, 4.0];
        let b = [1.0f32, 3.999_999_5];
        let eps = f32::EPSILON;

        // Same shape a/b/epsilon
        assert_float_eq!(a[..], b[..], abs_all <= 4.0 * eps);
        assert_float_ne!(a[..], b[..], abs_all <= 2.0 * eps);

        assert_float_eq!(a[..], b[..], rel_all <= 1.0 * eps);
        assert_float_ne!(a[..], b[..], rel_all <= 0.5 * eps);

        assert_float_eq!(a[..], b[..], rmax_all <= 1.0 * eps);
        assert_float_ne!(a[..], b[..], rmax_all <= 0.5 * eps);

        assert_float_eq!(a[..], b[..], rmin_all <= 2.0 * eps);
        assert_float_ne!(a[..], b[..], rmin_all <= 1.0 * eps);

        assert_float_eq!(a[..], b[..], r1st_all <= 2.0 * eps);
        assert_float_ne!(a[..], b[..], r1st_all <= 1.0 * eps);

        assert_float_eq!(a[..], b[..], r2nd_all <= 2.0 * eps);
        assert_float_ne!(a[..], b[..], r2nd_all <= 1.0 * eps);

        assert_float_eq!(a[..], b[..], ulps_all <= 2);
        assert_float_ne!(a[..], b[..], ulps_all <= 1);

        // Different shape a/b
        assert_float_ne!(a[1..], b[..], abs_all <= f32::INFINITY);
        assert_float_ne!(a[1..], b[..], rel_all <= f32::INFINITY);
        assert_float_ne!(a[1..], b[..], rmax_all <= f32::INFINITY);
        assert_float_ne!(a[1..], b[..], rmin_all <= f32::INFINITY);
        assert_float_ne!(a[1..], b[..], r1st_all <= f32::INFINITY);
        assert_float_ne!(a[1..], b[..], r2nd_all <= f32::INFINITY);
        assert_float_ne!(a[1..], b[..], ulps_all <= u32::MAX);

        assert_float_ne!(a[..], b[..1], abs_all <= f32::INFINITY);
        assert_float_ne!(a[..], b[..1], rel_all <= f32::INFINITY);
        assert_float_ne!(a[..], b[..1], rmax_all <= f32::INFINITY);
        assert_float_ne!(a[..], b[..1], rmin_all <= f32::INFINITY);
        assert_float_ne!(a[..], b[..1], r1st_all <= f32::INFINITY);
        assert_float_ne!(a[..], b[..1], r2nd_all <= f32::INFINITY);
        assert_float_ne!(a[..], b[..1], ulps_all <= u32::MAX);
    }
}
