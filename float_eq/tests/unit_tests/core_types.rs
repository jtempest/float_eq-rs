use core::cell::{Cell, RefCell};
use float_eq::{assert_float_eq, float_eq, float_ne, FloatDiff, FloatEqAllDebug, FloatEqDebug};

#[allow(clippy::unnecessary_mut_passed)]
mod refs {
    use super::*;

    #[test]
    fn refs_float_diff() {
        let a = &1.0f32;
        let b = &1.5f32;
        let mut ma = &mut 1.0f32;
        let mut mb = &mut 1.5f32;
        assert_eq!(FloatDiff::abs_diff(&a, &b), 0.5);
        assert_eq!(FloatDiff::abs_diff(&mut ma, &b), 0.5);
        assert_eq!(FloatDiff::abs_diff(&a, &mut mb), 0.5);
        assert_eq!(FloatDiff::abs_diff(&mut ma, &mut mb), 0.5);

        let c = &1.000_000_2f32;
        let mut mc = &mut 1.000_000_2f32;
        assert_eq!(FloatDiff::ulps_diff(&a, &c), Some(2));
        assert_eq!(FloatDiff::ulps_diff(&mut ma, &c), Some(2));
        assert_eq!(FloatDiff::ulps_diff(&a, &mut mc), Some(2));
        assert_eq!(FloatDiff::ulps_diff(&mut ma, &mut mc), Some(2));
    }

    #[test]
    fn refs_float_eq() {
        let a = &1.0f32;
        let b = &1.5f32;
        let mut ma = &mut 1.0f32;
        let mut mb = &mut 1.5f32;
        assert_float_eq!(&a, &b, abs <= 1.0);
        assert_float_eq!(&mut ma, &b, abs <= 1.0);
        assert_float_eq!(&a, &mut mb, abs <= 1.0);
        assert_float_eq!(&mut ma, &mut mb, abs <= 1.0);
        assert_float_eq!(&a, &b, abs_all <= 1.0);
        assert_float_eq!(&mut ma, &b, abs_all <= 1.0);
        assert_float_eq!(&a, &mut mb, abs_all <= 1.0);
        assert_float_eq!(&mut ma, &mut mb, abs_all <= 1.0);

        let c = &1.000_000_2f32;
        let mut mc = &mut 1.000_000_2f32;
        let eps = 2.0 * f32::EPSILON;
        assert_float_eq!(&a, &c, rel <= eps);
        assert_float_eq!(&mut ma, &c, rel <= eps);
        assert_float_eq!(&a, &mut mc, rel <= eps);
        assert_float_eq!(&mut ma, &mut mc, rel <= eps);
        assert_float_eq!(&a, &c, rel_all <= eps);
        assert_float_eq!(&mut ma, &c, rel_all <= eps);
        assert_float_eq!(&a, &mut mc, rel_all <= eps);
        assert_float_eq!(&mut ma, &mut mc, rel_all <= eps);

        assert_float_eq!(&a, &c, ulps <= 2);
        assert_float_eq!(&mut ma, &c, ulps <= 2);
        assert_float_eq!(&a, &mut mc, ulps <= 2);
        assert_float_eq!(&mut ma, &mut mc, ulps <= 2);
        assert_float_eq!(&a, &c, ulps_all <= 2);
        assert_float_eq!(&mut ma, &c, ulps_all <= 2);
        assert_float_eq!(&a, &mut mc, ulps_all <= 2);
        assert_float_eq!(&mut ma, &mut mc, ulps_all <= 2);
    }

    #[test]
    fn refs_debug_float_eq() {
        let a = &1.0f32;
        let b = &2.0f32;
        let mut ma = &mut 1.0f32;
        let mut mb = &mut 2.0f32;

        assert_eq!(FloatEqDebug::debug_abs_epsilon(&a, &b, &0.5), 0.5);
        assert_eq!(FloatEqDebug::debug_abs_epsilon(&mut ma, &b, &0.5), 0.5);
        assert_eq!(FloatEqDebug::debug_abs_epsilon(&a, &mut mb, &0.5), 0.5);
        assert_eq!(FloatEqDebug::debug_abs_epsilon(&mut ma, &mut mb, &0.5), 0.5);

        assert_eq!(FloatEqAllDebug::debug_abs_all_epsilon(&a, &b, &0.5), 0.5);
        assert_eq!(
            FloatEqAllDebug::debug_abs_all_epsilon(&mut ma, &b, &0.5),
            0.5
        );
        assert_eq!(
            FloatEqAllDebug::debug_abs_all_epsilon(&a, &mut mb, &0.5),
            0.5
        );
        assert_eq!(
            FloatEqAllDebug::debug_abs_all_epsilon(&mut ma, &mut mb, &0.5),
            0.5
        );

        assert_eq!(FloatEqDebug::debug_rel_epsilon(&a, &b, &0.1), 0.2);
        assert_eq!(FloatEqDebug::debug_rel_epsilon(&mut ma, &b, &0.1), 0.2);
        assert_eq!(FloatEqDebug::debug_rel_epsilon(&a, &mut mb, &0.1), 0.2);
        assert_eq!(FloatEqDebug::debug_rel_epsilon(&mut ma, &mut mb, &0.1), 0.2);

        assert_eq!(FloatEqAllDebug::debug_rel_all_epsilon(&a, &b, &0.1), 0.2);
        assert_eq!(
            FloatEqAllDebug::debug_rel_all_epsilon(&mut ma, &b, &0.1),
            0.2
        );
        assert_eq!(
            FloatEqAllDebug::debug_rel_all_epsilon(&a, &mut mb, &0.1),
            0.2
        );
        assert_eq!(
            FloatEqAllDebug::debug_rel_all_epsilon(&mut ma, &mut mb, &0.1),
            0.2
        );

        assert_eq!(FloatEqDebug::debug_ulps_epsilon(&a, &b, &1), 1);
        assert_eq!(FloatEqDebug::debug_ulps_epsilon(&mut ma, &b, &1), 1);
        assert_eq!(FloatEqDebug::debug_ulps_epsilon(&a, &mut mb, &1), 1);
        assert_eq!(FloatEqDebug::debug_ulps_epsilon(&mut ma, &mut mb, &1), 1);

        assert_eq!(FloatEqAllDebug::debug_ulps_all_epsilon(&a, &b, &1), 1);
        assert_eq!(FloatEqAllDebug::debug_ulps_all_epsilon(&mut ma, &b, &1), 1);
        assert_eq!(FloatEqAllDebug::debug_ulps_all_epsilon(&a, &mut mb, &1), 1);
        assert_eq!(
            FloatEqAllDebug::debug_ulps_all_epsilon(&mut ma, &mut mb, &1),
            1
        );
    }
}

mod option {
    use super::*;

    #[test]
    fn float_diff() {
        let a = Some([1.0f32, 2.0]);
        let b = Some([1.5f32, 2.25]);
        assert_eq!(a.abs_diff(&None), None);
        assert_eq!(None.abs_diff(&a), None);
        assert_eq!(a.abs_diff(&b), Some([0.5, 0.25]));

        let c = Some([1.000_000_1f32, 2.000_000_5]);
        assert_eq!(a.ulps_diff(&None), None);
        assert_eq!(None.ulps_diff(&c), None);
        assert_eq!(a.ulps_diff(&c), Some(Some([1, 2])));
    }

    #[test]
    fn float_eq() {
        let a = Some([1.0f32, 2.0]);
        let b = Some([1.5f32, 2.25]);
        assert!(float_ne!(a, None, abs <= Some([0.5, 0.25])));
        assert!(float_ne!(None, b, abs <= Some([0.5, 0.25])));
        assert!(float_ne!(a, b, abs <= None));
        assert!(float_ne!(a, b, abs <= Some([0.5, 0.24])));
        assert!(float_eq!(a, b, abs <= Some([0.5, 0.25])));

        assert!(float_ne!(a, None, abs_all <= Some(0.5)));
        assert!(float_ne!(None, b, abs_all <= Some(0.5)));
        assert!(float_ne!(a, b, abs_all <= None));
        assert!(float_ne!(a, b, abs_all <= Some(0.4)));
        assert!(float_eq!(a, b, abs_all <= Some(0.5)));

        let c = Some([1.000_000_1f32, 2.000_000_5]);
        assert!(float_ne!(
            a,
            None,
            rel <= Some([f32::EPSILON, 2.0 * f32::EPSILON])
        ));
        assert!(float_ne!(
            None,
            c,
            rel <= Some([f32::EPSILON, 2.0 * f32::EPSILON])
        ));
        assert!(float_ne!(a, c, rel <= None));
        assert!(float_ne!(
            a,
            c,
            rel <= Some([f32::EPSILON, 1.5 * f32::EPSILON])
        ));
        assert!(float_eq!(
            a,
            c,
            rel <= Some([f32::EPSILON, 2.0 * f32::EPSILON])
        ));

        assert!(float_ne!(a, None, rel_all <= Some(2.0 * f32::EPSILON)));
        assert!(float_ne!(None, c, rel_all <= Some(2.0 * f32::EPSILON)));
        assert!(float_ne!(a, c, rel_all <= None));
        assert!(float_ne!(a, c, rel_all <= Some(1.5 * f32::EPSILON)));
        assert!(float_eq!(a, c, rel_all <= Some(2.0 * f32::EPSILON)));

        assert!(float_ne!(a, None, ulps <= Some([1, 2])));
        assert!(float_ne!(None, c, ulps <= Some([1, 2])));
        assert!(float_ne!(a, c, ulps <= None));
        assert!(float_ne!(a, c, ulps <= Some([1, 1])));
        assert!(float_eq!(a, c, ulps <= Some([1, 2])));

        assert!(float_ne!(a, None, ulps_all <= Some(2)));
        assert!(float_ne!(None, c, ulps_all <= Some(2)));
        assert!(float_ne!(a, c, ulps_all <= None));
        assert!(float_ne!(a, c, ulps_all <= Some(1)));
        assert!(float_eq!(a, c, ulps_all <= Some(2)));
    }

    #[test]
    fn float_eq_debug() {
        let a = Some([1.0f32, 2.0]);
        let b = Some([1.5f32, 2.25]);

        assert_eq!(a.debug_abs_epsilon(&None, &Some([0.1, 0.2])), None);
        assert_eq!(None.debug_abs_epsilon(&b, &Some([0.1, 0.2])), None);
        assert_eq!(a.debug_abs_epsilon(&b, &None), None);
        assert_eq!(a.debug_abs_epsilon(&b, &Some([0.1, 0.2])), Some([0.1, 0.2]));

        assert_eq!(a.debug_abs_all_epsilon(&None, &Some(0.2)), None);
        assert_eq!(None.debug_abs_all_epsilon(&b, &Some(0.2)), None);
        assert_eq!(a.debug_abs_all_epsilon(&b, &None), None);
        assert_eq!(a.debug_abs_all_epsilon(&b, &Some(0.2)), Some([0.2, 0.2]));

        assert_eq!(a.debug_rel_epsilon(&None, &Some([0.1, 0.5])), None);
        assert_eq!(None.debug_rel_epsilon(&b, &Some([0.1, 0.5])), None);
        assert_eq!(
            a.debug_rel_epsilon(&b, &Some([0.1, 0.5])),
            Some([0.15, 1.125])
        );

        assert_eq!(a.debug_rel_all_epsilon(&None, &Some(0.5)), None);
        assert_eq!(None.debug_rel_all_epsilon(&b, &Some(0.5)), None);
        assert_eq!(a.debug_rel_all_epsilon(&b, &None), None);
        assert_eq!(a.debug_rel_all_epsilon(&b, &Some(0.5)), Some([0.75, 1.125]));

        assert_eq!(a.debug_ulps_epsilon(&None, &Some([1, 2])), None);
        assert_eq!(None.debug_ulps_epsilon(&b, &Some([1, 2])), None);
        assert_eq!(a.debug_ulps_epsilon(&b, &None), None);
        assert_eq!(a.debug_ulps_epsilon(&b, &Some([1, 2])), Some([1, 2]));

        assert_eq!(a.debug_ulps_all_epsilon(&None, &Some(2)), None);
        assert_eq!(None.debug_ulps_all_epsilon(&b, &Some(2)), None);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &None), None);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &Some(2)), Some([2, 2]));
    }
}

mod cell {
    use super::*;

    #[test]
    fn float_diff() {
        let a = Cell::new([1.0f32, 2.0]);
        let b = Cell::new([1.5f32, 2.25]);
        assert_eq!(a.abs_diff(&b), [0.5, 0.25]);

        let c = Cell::new([1.000_000_1f32, 2.000_000_5]);
        assert_eq!(a.ulps_diff(&c), Some([1, 2]));
    }

    #[test]
    fn float_eq() {
        let a = Cell::new([1.0f32, 2.0]);
        let b = Cell::new([1.5f32, 2.25]);
        assert!(float_eq!(a, b, abs <= [0.5, 0.25]));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = Cell::new([1.000_000_1f32, 2.000_000_5]);
        assert!(float_eq!(a, c, rel <= [f32::EPSILON, 2.0 * f32::EPSILON]));
        assert!(float_eq!(a, c, rel_all <= 2.0 * f32::EPSILON));

        assert!(float_eq!(a, c, ulps <= [1, 2]));
        assert!(float_eq!(a, c, ulps_all <= 2));
    }

    #[test]
    fn float_eq_debug() {
        let a = Cell::new([1.0f32, 2.0]);
        let b = Cell::new([1.5f32, 2.25]);

        assert_eq!(a.debug_abs_epsilon(&b, &[0.1, 0.2]), [0.1, 0.2]);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), [0.2, 0.2]);

        assert_eq!(a.debug_rel_epsilon(&b, &[0.1, 0.5]), [0.15, 1.125]);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), [0.75, 1.125]);

        assert_eq!(a.debug_ulps_epsilon(&b, &[1, 2]), [1, 2]);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), [2, 2]);
    }
}

mod ref_cell {
    use super::*;

    #[test]
    fn float_diff() {
        let a = RefCell::new([1.0f32, 2.0]);
        let b = RefCell::new([1.5f32, 2.25]);
        assert_eq!(a.abs_diff(&b), [0.5, 0.25]);

        let c = RefCell::new([1.000_000_1f32, 2.000_000_5]);
        assert_eq!(a.ulps_diff(&c), Some([1, 2]));
    }

    #[test]
    fn float_eq() {
        let a = RefCell::new([1.0f32, 2.0]);
        let b = RefCell::new([1.5f32, 2.25]);
        assert!(float_eq!(a, b, abs <= [0.5, 0.25]));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = RefCell::new([1.000_000_1f32, 2.000_000_5]);
        assert!(float_eq!(a, c, rel <= [f32::EPSILON, 2.0 * f32::EPSILON]));
        assert!(float_eq!(a, c, rel_all <= 2.0 * f32::EPSILON));

        assert!(float_eq!(a, c, ulps <= [1, 2]));
        assert!(float_eq!(a, c, ulps_all <= 2));
    }

    #[test]
    fn float_eq_debug() {
        let a = RefCell::new([1.0f32, 2.0]);
        let b = RefCell::new([1.5f32, 2.25]);

        assert_eq!(a.debug_abs_epsilon(&b, &[0.1, 0.2]), [0.1, 0.2]);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), [0.2, 0.2]);

        assert_eq!(a.debug_rel_epsilon(&b, &[0.1, 0.5]), [0.15, 1.125]);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), [0.75, 1.125]);

        assert_eq!(a.debug_ulps_epsilon(&b, &[1, 2]), [1, 2]);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), [2, 2]);
    }
}

mod slice {
    use super::*;

    #[test]
    fn float_eq() {
        let a = [1.0f32, 2.0];
        let b = [1.5f32, 2.25];
        assert!(float_ne!(a[..], b[..], abs <= [0.4, 0.25]));
        assert!(float_ne!(a[..], b[..], abs <= [0.5, 0.24]));
        assert!(float_ne!(a[..], b[..], abs <= [f32::INFINITY]));
        assert!(float_ne!(a[..], b[..], abs <= [f32::INFINITY; 3]));
        assert!(float_eq!(a[..], b[..], abs <= [0.5, 0.25]));
        assert!(float_ne!(a[..], b[..], abs_all <= 0.4));
        assert!(float_eq!(a[..], b[..], abs_all <= 0.5));

        let c = vec![1.000_000_1f32, 2.000_000_5];
        let eps = f32::EPSILON;
        assert!(float_ne!(a[..], c[..], rel <= [0.5 * eps, 2.0 * eps]));
        assert!(float_ne!(a[..], c[..], rel <= [eps, 1.0 * eps]));
        assert!(float_ne!(a[..], c[..], rel <= [f32::INFINITY]));
        assert!(float_ne!(a[..], c[..], rel <= [f32::INFINITY; 3]));
        assert!(float_eq!(a[..], c[..], rel <= [eps, 2.0 * eps]));
        assert!(float_ne!(a[..], c[..], rel_all <= eps));
        assert!(float_eq!(a[..], c[..], rel_all <= 2.0 * eps));

        assert!(float_ne!(a[..], c[..], ulps <= [0, 2]));
        assert!(float_ne!(a[..], c[..], ulps <= [1, 1]));
        assert!(float_ne!(a[..], c[..], ulps <= [u32::MAX]));
        assert!(float_ne!(a[..], c[..], ulps <= [u32::MAX; 3]));
        assert!(float_eq!(a[..], c[..], ulps <= [1, 2]));
        assert!(float_ne!(a[..], c[..], ulps_all <= 1));
        assert!(float_eq!(a[..], c[..], ulps_all <= 2));

        let d = &a[..1];
        assert!(float_ne!(a[..], d, abs <= [f32::INFINITY; 2]));
        assert!(float_ne!(d, &a[..], abs_all <= f32::INFINITY));
        assert!(float_ne!(a[..], d, rel <= [f32::INFINITY; 2]));
        assert!(float_ne!(d, &a[..], rel_all <= f32::INFINITY));
        assert!(float_ne!(a[..], d, ulps <= [u32::MAX; 2]));
        assert!(float_ne!(d, &a[..], ulps_all <= u32::MAX));

        let e = &[1.0; 3][..];
        assert!(float_ne!(a[..], e, abs <= [f32::INFINITY; 3]));
        assert!(float_ne!(e, &a[..], abs_all <= f32::INFINITY));
        assert!(float_ne!(a[..], e, rel <= [f32::INFINITY; 3]));
        assert!(float_ne!(e, &a[..], rel_all <= f32::INFINITY));
        assert!(float_ne!(a[..], e, ulps <= [u32::MAX; 3]));
        assert!(float_ne!(e, &a[..], ulps_all <= u32::MAX));
    }
}
