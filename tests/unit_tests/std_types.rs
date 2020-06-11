use float_eq::{float_eq, float_ne, FloatDiff, FloatEqAllDebug, FloatEqDebug};

mod rc {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn float_diff() {
        let a = Rc::new([1.0f32, 2.0]);
        let b = Rc::new([1.5f32, 2.25]);
        assert_eq!(a.abs_diff(&b), [0.5, 0.25]);

        let c = Rc::new([1.000_000_1f32, 2.000_000_5]);
        assert_eq!(a.ulps_diff(&c), Some([1, 2]));
    }

    #[test]
    fn float_eq() {
        let a = Rc::new([1.0f32, 2.0]);
        let b = Rc::new([1.5f32, 2.25]);
        assert!(float_eq!(a, b, abs <= [0.5, 0.25]));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = Rc::new([1.000_000_1f32, 2.000_000_5]);
        assert!(float_eq!(a, c, rel <= [f32::EPSILON, 2.0 * f32::EPSILON]));
        assert!(float_eq!(a, c, rel_all <= 2.0 * f32::EPSILON));

        assert!(float_eq!(a, c, ulps <= [1, 2]));
        assert!(float_eq!(a, c, ulps_all <= 2));
    }

    #[test]
    fn float_eq_debug() {
        let a = Rc::new([1.0f32, 2.0]);
        let b = Rc::new([1.5f32, 2.25]);

        assert_eq!(a.debug_abs_epsilon(&b, &[0.1, 0.2]), [0.1, 0.2]);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), [0.2, 0.2]);

        assert_eq!(a.debug_rel_epsilon(&b, &[0.1, 0.5]), [0.15, 1.125]);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), [0.75, 1.125]);

        assert_eq!(a.debug_ulps_epsilon(&b, &[1, 2]), [1, 2]);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), [2, 2]);
    }
}

mod arc {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn float_diff() {
        let a = Arc::new([1.0f32, 2.0]);
        let b = Arc::new([1.5f32, 2.25]);
        assert_eq!(a.abs_diff(&b), [0.5, 0.25]);

        let c = Arc::new([1.000_000_1f32, 2.000_000_5]);
        assert_eq!(a.ulps_diff(&c), Some([1, 2]));
    }

    #[test]
    fn float_eq() {
        let a = Arc::new([1.0f32, 2.0]);
        let b = Arc::new([1.5f32, 2.25]);
        assert!(float_eq!(a, b, abs <= [0.5, 0.25]));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = Arc::new([1.000_000_1f32, 2.000_000_5]);
        assert!(float_eq!(a, c, rel <= [f32::EPSILON, 2.0 * f32::EPSILON]));
        assert!(float_eq!(a, c, rel_all <= 2.0 * f32::EPSILON));

        assert!(float_eq!(a, c, ulps <= [1, 2]));
        assert!(float_eq!(a, c, ulps_all <= 2));
    }

    #[test]
    fn float_eq_debug() {
        let a = Arc::new([1.0f32, 2.0]);
        let b = Arc::new([1.5f32, 2.25]);

        assert_eq!(a.debug_abs_epsilon(&b, &[0.1, 0.2]), [0.1, 0.2]);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), [0.2, 0.2]);

        assert_eq!(a.debug_rel_epsilon(&b, &[0.1, 0.5]), [0.15, 1.125]);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), [0.75, 1.125]);

        assert_eq!(a.debug_ulps_epsilon(&b, &[1, 2]), [1, 2]);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), [2, 2]);
    }
}

mod r#box {
    use super::*;
    use std::boxed::Box;

    #[test]
    fn float_diff() {
        let a = Box::new([1.0f32, 2.0]);
        let b = Box::new([1.5f32, 2.25]);
        assert_eq!(a.abs_diff(&b), [0.5, 0.25]);

        let c = Box::new([1.000_000_1f32, 2.000_000_5]);
        assert_eq!(a.ulps_diff(&c), Some([1, 2]));
    }

    #[test]
    fn float_eq() {
        let a = Box::new([1.0f32, 2.0]);
        let b = Box::new([1.5f32, 2.25]);
        assert!(float_eq!(a, b, abs <= [0.5, 0.25]));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = Box::new([1.000_000_1f32, 2.000_000_5]);
        assert!(float_eq!(a, c, rel <= [f32::EPSILON, 2.0 * f32::EPSILON]));
        assert!(float_eq!(a, c, rel_all <= 2.0 * f32::EPSILON));

        assert!(float_eq!(a, c, ulps <= [1, 2]));
        assert!(float_eq!(a, c, ulps_all <= 2));
    }

    #[test]
    fn float_eq_debug() {
        let a = Box::new([1.0f32, 2.0]);
        let b = Box::new([1.5f32, 2.25]);

        assert_eq!(a.debug_abs_epsilon(&b, &[0.1, 0.2]), [0.1, 0.2]);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), [0.2, 0.2]);

        assert_eq!(a.debug_rel_epsilon(&b, &[0.1, 0.5]), [0.15, 1.125]);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), [0.75, 1.125]);

        assert_eq!(a.debug_ulps_epsilon(&b, &[1, 2]), [1, 2]);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), [2, 2]);
    }
}

mod vec {
    use super::*;

    #[test]
    fn float_diff() {
        let a = vec![1.0f32, 2.0];
        let b = vec![1.5f32, 2.25];
        assert_eq!(a.abs_diff(&b), Some(vec![0.5, 0.25]));

        let c = vec![1.000_000_1f32, 2.000_000_5];
        assert_eq!(a.ulps_diff(&c), Some(Some(vec![1, 2])));

        let d = Vec::new();
        assert_eq!(a.abs_diff(&d), None);
        assert_eq!(d.abs_diff(&a), None);
        assert_eq!(a.ulps_diff(&d), None);
        assert_eq!(d.ulps_diff(&a), None);

        let e = vec![1.0; 3];
        assert_eq!(a.abs_diff(&e), None);
        assert_eq!(e.abs_diff(&a), None);
        assert_eq!(a.ulps_diff(&e), None);
        assert_eq!(e.ulps_diff(&a), None);
    }

    #[test]
    fn float_eq() {
        let a = vec![1.0f32, 2.0];
        let b = vec![1.5f32, 2.25];
        assert!(float_ne!(a, b, abs <= vec![0.4, 0.25]));
        assert!(float_ne!(a, b, abs <= vec![0.5, 0.24]));
        assert!(float_ne!(a, b, abs <= vec![f32::INFINITY]));
        assert!(float_ne!(a, b, abs <= vec![f32::INFINITY; 3]));
        assert!(float_eq!(a, b, abs <= vec![0.5, 0.25]));
        assert!(float_ne!(a, b, abs_all <= 0.4));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = vec![1.000_000_1f32, 2.000_000_5];
        let eps = f32::EPSILON;
        assert!(float_ne!(a, c, rel <= vec![0.5 * eps, 2.0 * eps]));
        assert!(float_ne!(a, c, rel <= vec![eps, 1.0 * eps]));
        assert!(float_ne!(a, c, rel <= vec![f32::INFINITY]));
        assert!(float_ne!(a, c, rel <= vec![f32::INFINITY; 3]));
        assert!(float_eq!(a, c, rel <= vec![eps, 2.0 * eps]));
        assert!(float_ne!(a, c, rel_all <= eps));
        assert!(float_eq!(a, c, rel_all <= 2.0 * eps));

        assert!(float_ne!(a, c, ulps <= vec![0, 2]));
        assert!(float_ne!(a, c, ulps <= vec![1, 1]));
        assert!(float_ne!(a, c, ulps <= vec![u32::MAX]));
        assert!(float_ne!(a, c, ulps <= vec![u32::MAX; 3]));
        assert!(float_eq!(a, c, ulps <= vec![1, 2]));
        assert!(float_ne!(a, c, ulps_all <= 1));
        assert!(float_eq!(a, c, ulps_all <= 2));

        let d = Vec::new();
        assert!(!float_eq!(a, d, abs <= vec![f32::INFINITY; 3]));
        assert!(!float_eq!(d, a, abs_all <= f32::INFINITY));
        assert!(!float_eq!(a, d, rel <= vec![f32::INFINITY; 3]));
        assert!(!float_eq!(d, a, rel_all <= f32::INFINITY));
        assert!(!float_eq!(a, d, ulps <= vec![u32::MAX; 3]));
        assert!(!float_eq!(d, a, ulps_all <= u32::MAX));

        let e = vec![1.0; 3];
        assert!(!float_eq!(a, e, abs <= vec![f32::INFINITY; 3]));
        assert!(!float_eq!(e, a, abs_all <= f32::INFINITY));
        assert!(!float_eq!(a, e, rel <= vec![f32::INFINITY; 3]));
        assert!(!float_eq!(e, a, rel_all <= f32::INFINITY));
        assert!(!float_eq!(a, e, ulps <= vec![u32::MAX; 3]));
        assert!(!float_eq!(e, a, ulps_all <= u32::MAX));
    }

    #[test]
    fn float_eq_debug() {
        let a = vec![1.0f32, 2.0];
        let b = vec![1.5f32, 2.25];

        assert_eq!(
            a.debug_abs_epsilon(&b, &vec![0.1, 0.2]),
            Some(vec![0.1, 0.2])
        );
        assert_eq!(a.debug_abs_epsilon(&b, &vec![0.1]), None);
        assert_eq!(a.debug_abs_epsilon(&b, &vec![0.1; 3]), None);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), Some(vec![0.2, 0.2]));

        assert_eq!(
            a.debug_rel_epsilon(&b, &vec![0.1, 0.5]),
            Some(vec![0.15, 1.125])
        );
        assert_eq!(a.debug_rel_epsilon(&b, &vec![0.1]), None);
        assert_eq!(a.debug_rel_epsilon(&b, &vec![0.1; 3]), None);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), Some(vec![0.75, 1.125]));

        assert_eq!(a.debug_ulps_epsilon(&b, &vec![1, 2]), Some(vec![1, 2]));
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), Some(vec![2, 2]));

        let d = Vec::new();
        assert_eq!(a.debug_abs_epsilon(&d, &vec![0.1, 0.2]), None);
        assert_eq!(a.debug_abs_all_epsilon(&d, &0.2), None);
        assert_eq!(a.debug_rel_epsilon(&d, &vec![0.1, 0.5]), None);
        assert_eq!(a.debug_rel_all_epsilon(&d, &0.5), None);
        assert_eq!(a.debug_ulps_epsilon(&d, &vec![1, 2]), None);
        assert_eq!(a.debug_ulps_all_epsilon(&d, &2), None);

        let e = vec![1.0; 3];
        assert_eq!(a.debug_abs_epsilon(&e, &vec![0.1, 0.2]), None);
        assert_eq!(a.debug_abs_all_epsilon(&e, &0.2), None);
        assert_eq!(a.debug_rel_epsilon(&e, &vec![0.1, 0.5]), None);
        assert_eq!(a.debug_rel_all_epsilon(&e, &0.5), None);
        assert_eq!(a.debug_ulps_epsilon(&e, &vec![1, 2]), None);
        assert_eq!(a.debug_ulps_all_epsilon(&e, &2), None);
    }
}

mod vec_deque {
    use super::*;
    use std::collections::VecDeque;

    macro_rules! vecd {
        ($($x:expr),+) => {{
            vec![$($x,)+].into_iter().collect::<VecDeque<_>>()
        }};
        ($x:expr; $n:literal) => {{
            vec![$x; $n].into_iter().collect::<VecDeque<_>>()
        }};
    }

    #[test]
    fn float_diff() {
        let a = vecd![1.0f32, 2.0];
        let b = vecd![1.5f32, 2.25];
        assert_eq!(a.abs_diff(&b), Some(vecd![0.5, 0.25]));

        let c = vecd![1.000_000_1f32, 2.000_000_5];
        assert_eq!(a.ulps_diff(&c), Some(Some(vecd![1, 2])));

        let d = VecDeque::new();
        assert_eq!(a.abs_diff(&d), None);
        assert_eq!(d.abs_diff(&a), None);
        assert_eq!(a.ulps_diff(&d), None);
        assert_eq!(d.ulps_diff(&a), None);

        let e = vecd![1.0; 3];
        assert_eq!(a.abs_diff(&e), None);
        assert_eq!(e.abs_diff(&a), None);
        assert_eq!(a.ulps_diff(&e), None);
        assert_eq!(e.ulps_diff(&a), None);
    }

    #[test]
    fn float_eq() {
        let a = vecd![1.0f32, 2.0];
        let b = vecd![1.5f32, 2.25];
        assert!(float_ne!(a, b, abs <= vecd![0.4, 0.25]));
        assert!(float_ne!(a, b, abs <= vecd![0.5, 0.24]));
        assert!(float_ne!(a, b, abs <= vecd![f32::INFINITY]));
        assert!(float_ne!(a, b, abs <= vecd![f32::INFINITY; 3]));
        assert!(float_eq!(a, b, abs <= vecd![0.5, 0.25]));
        assert!(float_ne!(a, b, abs_all <= 0.4));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = vecd![1.000_000_1f32, 2.000_000_5];
        let eps = f32::EPSILON;
        assert!(float_ne!(a, c, rel <= vecd![0.5 * eps, 2.0 * eps]));
        assert!(float_ne!(a, c, rel <= vecd![eps, 1.0 * eps]));
        assert!(float_ne!(a, c, rel <= vecd![f32::INFINITY]));
        assert!(float_ne!(a, c, rel <= vecd![f32::INFINITY; 3]));
        assert!(float_eq!(a, c, rel <= vecd![eps, 2.0 * eps]));
        assert!(float_ne!(a, c, rel_all <= eps));
        assert!(float_eq!(a, c, rel_all <= 2.0 * eps));

        assert!(float_ne!(a, c, ulps <= vecd![0, 2]));
        assert!(float_ne!(a, c, ulps <= vecd![1, 1]));
        assert!(float_ne!(a, c, ulps <= vecd![u32::MAX]));
        assert!(float_ne!(a, c, ulps <= vecd![u32::MAX; 3]));
        assert!(float_eq!(a, c, ulps <= vecd![1, 2]));
        assert!(float_ne!(a, c, ulps_all <= 1));
        assert!(float_eq!(a, c, ulps_all <= 2));

        let d = VecDeque::new();
        assert!(!float_eq!(a, d, abs <= vecd![f32::INFINITY; 3]));
        assert!(!float_eq!(d, a, abs_all <= f32::INFINITY));
        assert!(!float_eq!(a, d, rel <= vecd![f32::INFINITY; 3]));
        assert!(!float_eq!(d, a, rel_all <= f32::INFINITY));
        assert!(!float_eq!(a, d, ulps <= vecd![u32::MAX; 3]));
        assert!(!float_eq!(d, a, ulps_all <= u32::MAX));

        let e = vecd![1.0; 3];
        assert!(!float_eq!(a, e, abs <= vecd![f32::INFINITY; 3]));
        assert!(!float_eq!(e, a, abs_all <= f32::INFINITY));
        assert!(!float_eq!(a, e, rel <= vecd![f32::INFINITY; 3]));
        assert!(!float_eq!(e, a, rel_all <= f32::INFINITY));
        assert!(!float_eq!(a, e, ulps <= vecd![u32::MAX; 3]));
        assert!(!float_eq!(e, a, ulps_all <= u32::MAX));
    }

    #[test]
    fn float_eq_debug() {
        let a = vecd![1.0f32, 2.0];
        let b = vecd![1.5f32, 2.25];

        assert_eq!(
            a.debug_abs_epsilon(&b, &vecd![0.1, 0.2]),
            Some(vecd![0.1, 0.2])
        );
        assert_eq!(a.debug_abs_epsilon(&b, &vecd![0.1]), None);
        assert_eq!(a.debug_abs_epsilon(&b, &vecd![0.1; 3]), None);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), Some(vecd![0.2, 0.2]));

        assert_eq!(
            a.debug_rel_epsilon(&b, &vecd![0.1, 0.5]),
            Some(vecd![0.15, 1.125])
        );
        assert_eq!(a.debug_rel_epsilon(&b, &vecd![0.1]), None);
        assert_eq!(a.debug_rel_epsilon(&b, &vecd![0.1; 3]), None);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), Some(vecd![0.75, 1.125]));

        assert_eq!(a.debug_ulps_epsilon(&b, &vecd![1, 2]), Some(vecd![1, 2]));
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), Some(vecd![2, 2]));

        let d = VecDeque::new();
        assert_eq!(a.debug_abs_epsilon(&d, &vecd![0.1, 0.2]), None);
        assert_eq!(a.debug_abs_all_epsilon(&d, &0.2), None);
        assert_eq!(a.debug_rel_epsilon(&d, &vecd![0.1, 0.5]), None);
        assert_eq!(a.debug_rel_all_epsilon(&d, &0.5), None);
        assert_eq!(a.debug_ulps_epsilon(&d, &vecd![1, 2]), None);
        assert_eq!(a.debug_ulps_all_epsilon(&d, &2), None);

        let e = vecd![1.0; 3];
        assert_eq!(a.debug_abs_epsilon(&e, &vecd![0.1, 0.2]), None);
        assert_eq!(a.debug_abs_all_epsilon(&e, &0.2), None);
        assert_eq!(a.debug_rel_epsilon(&e, &vecd![0.1, 0.5]), None);
        assert_eq!(a.debug_rel_all_epsilon(&e, &0.5), None);
        assert_eq!(a.debug_ulps_epsilon(&e, &vecd![1, 2]), None);
        assert_eq!(a.debug_ulps_all_epsilon(&e, &2), None);

        let mut a = VecDeque::new();
        a.push_back(0.1);
        a.push_front(0.2);
        let mut b = VecDeque::new();
        b.push_front(0.1);
        b.push_front(0.4);
        float_eq::assert_float_eq!(a, b, abs_all <= 0.1);
    }
}
