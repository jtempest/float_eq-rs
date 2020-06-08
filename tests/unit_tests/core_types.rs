use float_eq::{float_eq, float_ne, FloatDiff, FloatEqAllDebug, FloatEqDebug};

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
        assert_eq!(a.ulps_diff(&c), Some([1, 2]));
    }

    #[test]
    fn float_eq() {
        let a = Some([1.0f32, 2.0]);
        let b = Some([1.5f32, 2.25]);
        assert!(float_ne!(a, None, abs <= [0.5, 0.25]));
        assert!(float_ne!(None, b, abs <= [0.5, 0.25]));
        assert!(float_ne!(a, b, abs <= [0.5, 0.24]));
        assert!(float_eq!(a, b, abs <= [0.5, 0.25]));

        assert!(float_ne!(a, None, abs_all <= 0.5));
        assert!(float_ne!(None, b, abs_all <= 0.5));
        assert!(float_ne!(a, b, abs_all <= 0.4));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = Some([1.000_000_1f32, 2.000_000_5]);
        assert!(float_ne!(
            a,
            None,
            rel <= [f32::EPSILON, 2.0 * f32::EPSILON]
        ));
        assert!(float_ne!(
            None,
            c,
            rel <= [f32::EPSILON, 2.0 * f32::EPSILON]
        ));
        assert!(float_ne!(a, c, rel <= [f32::EPSILON, 1.5 * f32::EPSILON]));
        assert!(float_eq!(a, c, rel <= [f32::EPSILON, 2.0 * f32::EPSILON]));

        assert!(float_ne!(a, None, rel_all <= 2.0 * f32::EPSILON));
        assert!(float_ne!(None, c, rel_all <= 2.0 * f32::EPSILON));
        assert!(float_ne!(a, c, rel_all <= 1.5 * f32::EPSILON));
        assert!(float_eq!(a, c, rel_all <= 2.0 * f32::EPSILON));

        assert!(float_ne!(a, None, ulps <= [1, 2]));
        assert!(float_ne!(None, c, ulps <= [1, 2]));
        assert!(float_ne!(a, c, ulps <= [1, 1]));
        assert!(float_eq!(a, c, ulps <= [1, 2]));

        assert!(float_ne!(a, None, ulps_all <= 2));
        assert!(float_ne!(None, c, ulps_all <= 2));
        assert!(float_ne!(a, c, ulps_all <= 1));
        assert!(float_eq!(a, c, ulps_all <= 2));
    }

    #[test]
    fn float_eq_debug() {
        let a = Some([1.0f32, 2.0]);
        let b = Some([1.5f32, 2.25]);

        assert_eq!(a.debug_abs_epsilon(&None, &[0.1, 0.2]), None);
        assert_eq!(None.debug_abs_epsilon(&b, &[0.1, 0.2]), None);
        assert_eq!(a.debug_abs_epsilon(&b, &[0.1, 0.2]), Some([0.1, 0.2]));

        assert_eq!(a.debug_abs_all_epsilon(&None, &0.2), None);
        assert_eq!(None.debug_abs_all_epsilon(&b, &0.2), None);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), Some([0.2, 0.2]));

        assert_eq!(a.debug_rel_epsilon(&None, &[0.1, 0.5]), None);
        assert_eq!(None.debug_rel_epsilon(&b, &[0.1, 0.5]), None);
        assert_eq!(a.debug_rel_epsilon(&b, &[0.1, 0.5]), Some([0.15, 1.125]));

        assert_eq!(a.debug_rel_all_epsilon(&None, &0.5), None);
        assert_eq!(None.debug_rel_all_epsilon(&b, &0.5), None);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), Some([0.75, 1.125]));

        assert_eq!(a.debug_ulps_epsilon(&None, &[1, 2]), None);
        assert_eq!(None.debug_ulps_epsilon(&b, &[1, 2]), None);
        assert_eq!(a.debug_ulps_epsilon(&b, &[1, 2]), Some([1, 2]));

        assert_eq!(a.debug_ulps_all_epsilon(&None, &2), None);
        assert_eq!(None.debug_ulps_all_epsilon(&b, &2), None);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), Some([2, 2]));
    }
}
