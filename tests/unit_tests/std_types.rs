use float_eq::{float_eq, float_ne, FloatDiff, FloatEqAllDebug, FloatEqDebug};
use std::boxed::Box;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::rc::Rc;
use std::sync::Arc;

mod rc {
    use super::*;

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

mod slice {
    use super::*;

    #[test]
    fn float_diff() {
        let a = [1.0f32, 2.0];
        let b = vec![1.5f32, 2.25];
        assert_eq!(a[..].abs_diff(&b[..]), Some(vec![0.5, 0.25]));

        let c = &[1.000_000_1f32, 2.000_000_5];
        assert_eq!(a[..].ulps_diff(&c[..]), Some(Some(vec![1, 2])));

        let d = &[][..];
        assert_eq!(a[..].abs_diff(&d), None);
        assert_eq!(d.abs_diff(&a), None);
        assert_eq!(a[..].ulps_diff(&d), None);
        assert_eq!(d.ulps_diff(&a), None);

        let e = [1.0; 3];
        assert_eq!(a[..].abs_diff(&e[..]), None);
        assert_eq!(e[..].abs_diff(&a[..]), None);
        assert_eq!(a[..].ulps_diff(&e[..]), None);
        assert_eq!(e[..].ulps_diff(&a[..]), None);
    }

    #[test]
    fn float_eq_debug() {
        let a = [1.0f32, 2.0];
        let b = vec![1.5f32, 2.25];

        assert_eq!(
            a[..].debug_abs_all_epsilon(&b[..], &0.2),
            Some(vec![0.2, 0.2])
        );
        assert_eq!(
            a[..].debug_rel_all_epsilon(&b[..], &0.5),
            Some(vec![0.75, 1.125])
        );
        assert_eq!(a[..].debug_ulps_all_epsilon(&b[..], &2), Some(vec![2, 2]));

        let d = &[][..];
        assert_eq!(a[..].debug_abs_all_epsilon(d, &0.2), None);
        assert_eq!(a[..].debug_rel_all_epsilon(d, &0.5), None);
        assert_eq!(a[..].debug_ulps_all_epsilon(d, &2), None);

        let e = [1.0; 3];
        assert_eq!(a[..].debug_abs_all_epsilon(&e[..], &0.2), None);
        assert_eq!(a[..].debug_rel_all_epsilon(&e[..], &0.5), None);
        assert_eq!(a[..].debug_ulps_all_epsilon(&e[..], &2), None);
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
        assert!(float_ne!(a, d, abs <= vec![f32::INFINITY; 2]));
        assert!(float_ne!(d, a, abs_all <= f32::INFINITY));
        assert!(float_ne!(a, d, rel <= vec![f32::INFINITY; 2]));
        assert!(float_ne!(d, a, rel_all <= f32::INFINITY));
        assert!(float_ne!(a, d, ulps <= vec![u32::MAX; 2]));
        assert!(float_ne!(d, a, ulps_all <= u32::MAX));

        let e = vec![1.0; 3];
        assert!(float_ne!(a, e, abs <= vec![f32::INFINITY; 3]));
        assert!(float_ne!(e, a, abs_all <= f32::INFINITY));
        assert!(float_ne!(a, e, rel <= vec![f32::INFINITY; 3]));
        assert!(float_ne!(e, a, rel_all <= f32::INFINITY));
        assert!(float_ne!(a, e, ulps <= vec![u32::MAX; 3]));
        assert!(float_ne!(e, a, ulps_all <= u32::MAX));
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

        assert_eq!(a.debug_ulps_epsilon(&b, &vec![1]), None);
        assert_eq!(a.debug_ulps_epsilon(&b, &vec![1, 2]), Some(vec![1, 2]));
        assert_eq!(a.debug_ulps_epsilon(&b, &vec![1, 2, 3]), None);
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
        assert!(float_ne!(a, d, abs <= vecd![f32::INFINITY; 2]));
        assert!(float_ne!(d, a, abs_all <= f32::INFINITY));
        assert!(float_ne!(a, d, rel <= vecd![f32::INFINITY; 2]));
        assert!(float_ne!(d, a, rel_all <= f32::INFINITY));
        assert!(float_ne!(a, d, ulps <= vecd![u32::MAX; 2]));
        assert!(float_ne!(d, a, ulps_all <= u32::MAX));

        let e = vecd![1.0; 3];
        assert!(float_ne!(a, e, abs <= vecd![f32::INFINITY; 3]));
        assert!(float_ne!(e, a, abs_all <= f32::INFINITY));
        assert!(float_ne!(a, e, rel <= vecd![f32::INFINITY; 3]));
        assert!(float_ne!(e, a, rel_all <= f32::INFINITY));
        assert!(float_ne!(a, e, ulps <= vecd![u32::MAX; 3]));
        assert!(float_ne!(e, a, ulps_all <= u32::MAX));
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

        assert_eq!(a.debug_ulps_epsilon(&b, &vecd![1]), None);
        assert_eq!(a.debug_ulps_epsilon(&b, &vecd![1, 2]), Some(vecd![1, 2]));
        assert_eq!(a.debug_ulps_epsilon(&b, &vecd![1, 2, 3]), None);
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
    }
}

mod linked_list {
    use super::*;
    use std::collections::LinkedList;

    macro_rules! list {
        ($($x:expr),+) => {{
            vec![$($x,)+].into_iter().collect::<LinkedList<_>>()
        }};
        ($x:expr; $n:literal) => {{
            vec![$x; $n].into_iter().collect::<LinkedList<_>>()
        }};
    }

    #[test]
    fn float_diff() {
        let a = list![1.0f32, 2.0];
        let b = list![1.5f32, 2.25];
        assert_eq!(a.abs_diff(&b), Some(list![0.5, 0.25]));

        let c = list![1.000_000_1f32, 2.000_000_5];
        assert_eq!(a.ulps_diff(&c), Some(Some(list![1, 2])));

        let d = LinkedList::new();
        assert_eq!(a.abs_diff(&d), None);
        assert_eq!(d.abs_diff(&a), None);
        assert_eq!(a.ulps_diff(&d), None);
        assert_eq!(d.ulps_diff(&a), None);

        let e = list![1.0; 3];
        assert_eq!(a.abs_diff(&e), None);
        assert_eq!(e.abs_diff(&a), None);
        assert_eq!(a.ulps_diff(&e), None);
        assert_eq!(e.ulps_diff(&a), None);
    }

    #[test]
    fn float_eq() {
        let a = list![1.0f32, 2.0];
        let b = list![1.5f32, 2.25];
        assert!(float_ne!(a, b, abs <= list![0.4, 0.25]));
        assert!(float_ne!(a, b, abs <= list![0.5, 0.24]));
        assert!(float_ne!(a, b, abs <= list![f32::INFINITY]));
        assert!(float_ne!(a, b, abs <= list![f32::INFINITY; 3]));
        assert!(float_eq!(a, b, abs <= list![0.5, 0.25]));
        assert!(float_ne!(a, b, abs_all <= 0.4));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = list![1.000_000_1f32, 2.000_000_5];
        let eps = f32::EPSILON;
        assert!(float_ne!(a, c, rel <= list![0.5 * eps, 2.0 * eps]));
        assert!(float_ne!(a, c, rel <= list![eps, 1.0 * eps]));
        assert!(float_ne!(a, c, rel <= list![f32::INFINITY]));
        assert!(float_ne!(a, c, rel <= list![f32::INFINITY; 3]));
        assert!(float_eq!(a, c, rel <= list![eps, 2.0 * eps]));
        assert!(float_ne!(a, c, rel_all <= eps));
        assert!(float_eq!(a, c, rel_all <= 2.0 * eps));

        assert!(float_ne!(a, c, ulps <= list![0, 2]));
        assert!(float_ne!(a, c, ulps <= list![1, 1]));
        assert!(float_ne!(a, c, ulps <= list![u32::MAX]));
        assert!(float_ne!(a, c, ulps <= list![u32::MAX; 3]));
        assert!(float_eq!(a, c, ulps <= list![1, 2]));
        assert!(float_ne!(a, c, ulps_all <= 1));
        assert!(float_eq!(a, c, ulps_all <= 2));

        let d = LinkedList::new();
        assert!(float_ne!(a, d, abs <= list![f32::INFINITY; 2]));
        assert!(float_ne!(d, a, abs_all <= f32::INFINITY));
        assert!(float_ne!(a, d, rel <= list![f32::INFINITY; 2]));
        assert!(float_ne!(d, a, rel_all <= f32::INFINITY));
        assert!(float_ne!(a, d, ulps <= list![u32::MAX; 2]));
        assert!(float_ne!(d, a, ulps_all <= u32::MAX));

        let e = list![1.0; 3];
        assert!(float_ne!(a, e, abs <= list![f32::INFINITY; 3]));
        assert!(float_ne!(e, a, abs_all <= f32::INFINITY));
        assert!(float_ne!(a, e, rel <= list![f32::INFINITY; 3]));
        assert!(float_ne!(e, a, rel_all <= f32::INFINITY));
        assert!(float_ne!(a, e, ulps <= list![u32::MAX; 3]));
        assert!(float_ne!(e, a, ulps_all <= u32::MAX));
    }

    #[test]
    fn float_eq_debug() {
        let a = list![1.0f32, 2.0];
        let b = list![1.5f32, 2.25];

        assert_eq!(
            a.debug_abs_epsilon(&b, &list![0.1, 0.2]),
            Some(list![0.1, 0.2])
        );
        assert_eq!(a.debug_abs_epsilon(&b, &list![0.1]), None);
        assert_eq!(a.debug_abs_epsilon(&b, &list![0.1; 3]), None);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), Some(list![0.2, 0.2]));

        assert_eq!(
            a.debug_rel_epsilon(&b, &list![0.1, 0.5]),
            Some(list![0.15, 1.125])
        );
        assert_eq!(a.debug_rel_epsilon(&b, &list![0.1]), None);
        assert_eq!(a.debug_rel_epsilon(&b, &list![0.1; 3]), None);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), Some(list![0.75, 1.125]));

        assert_eq!(a.debug_ulps_epsilon(&b, &list![1]), None);
        assert_eq!(a.debug_ulps_epsilon(&b, &list![1, 2, 3]), None);
        assert_eq!(a.debug_ulps_epsilon(&b, &list![1, 2]), Some(list![1, 2]));
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), Some(list![2, 2]));

        let d = LinkedList::new();
        assert_eq!(a.debug_abs_epsilon(&d, &list![0.1, 0.2]), None);
        assert_eq!(a.debug_abs_all_epsilon(&d, &0.2), None);
        assert_eq!(a.debug_rel_epsilon(&d, &list![0.1, 0.5]), None);
        assert_eq!(a.debug_rel_all_epsilon(&d, &0.5), None);
        assert_eq!(a.debug_ulps_epsilon(&d, &list![1, 2]), None);
        assert_eq!(a.debug_ulps_all_epsilon(&d, &2), None);

        let e = list![1.0; 3];
        assert_eq!(a.debug_abs_epsilon(&e, &list![0.1, 0.2]), None);
        assert_eq!(a.debug_abs_all_epsilon(&e, &0.2), None);
        assert_eq!(a.debug_rel_epsilon(&e, &list![0.1, 0.5]), None);
        assert_eq!(a.debug_rel_all_epsilon(&e, &0.5), None);
        assert_eq!(a.debug_ulps_epsilon(&e, &list![1, 2]), None);
        assert_eq!(a.debug_ulps_all_epsilon(&e, &2), None);
    }
}

macro_rules! impl_map_tests {
    ($map:ident) => {
        mod $map {
            use super::*;

            #[test]
            fn float_diff() {
                let a = $map! {"one" => 1.0f32, "two" => 2.0};
                let b = $map! {"one" => 1.5f32, "two" => 2.25};
                assert_eq!(a.abs_diff(&b), Some($map! {"one" => 0.5, "two" => 0.25}));

                let c = $map! {"one" => 1.000_000_1f32, "two" => 2.000_000_5};
                assert_eq!(a.ulps_diff(&c), Some(Some($map! {"one" => 1, "two" => 2})));

                let d = $map! {};
                assert_eq!(a.abs_diff(&d), None);
                assert_eq!(d.abs_diff(&a), None);
                assert_eq!(a.ulps_diff(&d), None);
                assert_eq!(d.ulps_diff(&a), None);

                let e = $map! {"one" => 1.000_000_1f32, "two" => 2.000_000_5, "three" => 3.0};
                assert_eq!(a.abs_diff(&e), None);
                assert_eq!(e.abs_diff(&a), None);
                assert_eq!(a.ulps_diff(&e), None);
                assert_eq!(e.ulps_diff(&a), None);
            }

            #[test]
            fn float_eq() {
                const INF: f32 = f32::INFINITY;

                let a = $map! {"one" => 1.0f32, "two" => 2.0};
                let b = $map! {"one" => 1.5f32, "two" => 2.25};
                assert!(float_ne!(a, b, abs <= $map! { "one" => 0.4, "two" => 0.25}));
                assert!(float_ne!(a, b, abs <= $map! { "one" => 0.5, "two" => 0.24}));
                assert!(float_ne!(a, b, abs <= $map! { "one" => INF}));
                assert!(float_ne!(
                    a,
                    b,
                    abs <= $map! { "one" => INF, "three" => INF}
                ));
                assert!(float_ne!(
                    a,
                    b,
                    abs <= $map! { "one" => INF, "two" => INF, "three" => INF}
                ));
                assert!(float_eq!(a, b, abs <= $map! { "one" => 0.5, "two" => 0.25}));
                assert!(float_ne!(a, b, abs_all <= 0.4));
                assert!(float_eq!(a, b, abs_all <= 0.5));

                let c = $map! { "one" => 1.000_000_1f32, "two" => 2.000_000_5 };
                let eps = f32::EPSILON;
                assert!(float_ne!(
                    a,
                    c,
                    rel <= $map! { "one" => 0.5 * eps, "two" => 2.0 * eps }
                ));
                assert!(float_ne!(a, c, rel <= $map! { "one" => eps, "two" => eps }));
                assert!(float_ne!(a, c, rel <= $map! { "one" => INF }));
                assert!(float_ne!(
                    a,
                    c,
                    rel <= $map! { "one" => INF, "three" => INF }
                ));
                assert!(float_ne!(
                    a,
                    c,
                    rel <= $map! { "one" => INF, "two" => INF, "three" => INF }
                ));
                assert!(float_eq!(
                    a,
                    c,
                    rel <= $map! { "one" => eps, "two" => 2.0 * eps }
                ));
                assert!(float_ne!(a, c, rel_all <= eps));
                assert!(float_eq!(a, c, rel_all <= 2.0 * eps));

                assert!(float_ne!(a, c, ulps <= $map! { "one" => 0, "two" => 2 }));
                assert!(float_ne!(a, c, ulps <= $map! { "one" => 1, "two" => 1 }));
                assert!(float_ne!(a, c, ulps <= $map! { "two" => u32::MAX }));
                assert!(float_ne!(
                    a,
                    c,
                    ulps <= $map! { "two" => u32::MAX, "three" => u32::MAX }
                ));
                assert!(float_ne!(
                    a,
                    c,
                    ulps <= $map! { "one" => u32::MAX, "two" => u32::MAX, "three" => u32::MAX }
                ));
                assert!(float_eq!(a, c, ulps <= $map! { "one" => 1, "two" => 2 }));
                assert!(float_ne!(a, c, ulps_all <= 1));
                assert!(float_eq!(a, c, ulps_all <= 2));

                let d = $map! {};
                assert!(float_ne!(a, d, abs <= $map! { "one" => INF, "two" => INF}));
                assert!(float_ne!(a, d, abs_all <= INF));
                assert!(float_ne!(a, d, rel <= $map! { "one" => INF, "two" => INF}));
                assert!(float_ne!(a, d, rel_all <= INF));
                assert!(float_ne!(
                    a,
                    d,
                    ulps <= $map! { "one" => u32::MAX, "two" => u32::MAX }
                ));
                assert!(float_ne!(a, d, ulps_all <= u32::MAX));

                let e = $map! {"one" => 1.0f32, "two" => 2.0, "three" => 3.0};
                assert!(float_ne!(
                    a,
                    e,
                    abs <= $map! {"one" => INF, "two" => INF, "three" => INF }
                ));
                assert!(float_ne!(e, a, abs_all <= INF));
                assert!(float_ne!(
                    a,
                    e,
                    rel <= $map! {"one" => INF, "two" => INF, "three" => INF }
                ));
                assert!(float_ne!(e, a, rel_all <= INF));
                assert!(float_ne!(
                    a,
                    e,
                    ulps <= $map! {"one" => u32::MAX, "two" => u32::MAX, "three" => u32::MAX }
                ));
                assert!(float_ne!(e, a, ulps_all <= u32::MAX));

                let f = $map! { "one" => 1.0, "three" => 3.0 };
                assert!(float_ne!(a, f, abs <= $map! {"one" => INF, "two" => INF }));
                assert!(float_ne!(a, f, abs_all <= INF));
                assert!(float_ne!(a, f, rel <= $map! {"one" => INF, "two" => INF }));
                assert!(float_ne!(a, f, rel_all <= INF));
                assert!(float_ne!(
                    a,
                    f,
                    ulps <= $map! {"one" => u32::MAX, "two" => u32::MAX }
                ));
                assert!(float_ne!(a, f, ulps_all <= u32::MAX));
            }

            #[test]
            fn float_eq_debug() {
                let a = $map! {"one" => 1.0f32, "two" => 2.0};
                let b = $map! {"one" => 1.5f32, "two" => 2.25};

                assert_eq!(
                    a.debug_abs_epsilon(&b, &$map! { "one" => 0.1, "two" => 0.2 }),
                    Some($map! { "one" => 0.1, "two" => 0.2 })
                );
                assert_eq!(a.debug_abs_epsilon(&b, &$map! { "one" => 0.1 }), None);
                assert_eq!(
                    a.debug_abs_epsilon(&b, &$map! { "one" => 0.1, "two" => 0.2, "three" => 0.3 }),
                    None
                );
                assert_eq!(
                    a.debug_abs_all_epsilon(&b, &0.2),
                    Some($map! { "one" => 0.2, "two" => 0.2 })
                );

                assert_eq!(
                    a.debug_rel_epsilon(&b, &$map! { "one" => 0.1, "two" => 0.5 }),
                    Some($map! { "one" => 0.15, "two" => 1.125 })
                );
                assert_eq!(a.debug_rel_epsilon(&b, &$map! { "one" => 0.1 }), None);
                assert_eq!(
                    a.debug_rel_epsilon(&b, &$map! { "one" => 0.1, "two" => 0.2, "three" => 0.3 }),
                    None
                );
                assert_eq!(
                    a.debug_rel_all_epsilon(&b, &0.5),
                    Some($map! { "one" => 0.75, "two" => 1.125 })
                );

                assert_eq!(
                    a.debug_ulps_epsilon(&b, &$map! { "one" => 1, "two" => 2 }),
                    Some($map! { "one" => 1, "two" => 2 })
                );
                assert_eq!(
                    a.debug_ulps_all_epsilon(&b, &2),
                    Some($map! { "one" => 2, "two" => 2 })
                );

                let d = $map! {};
                assert_eq!(
                    a.debug_abs_epsilon(&d, &$map! { "one" => 0.1, "two" => 0.2 }),
                    None
                );
                assert_eq!(a.debug_abs_all_epsilon(&d, &0.2), None);
                assert_eq!(
                    a.debug_rel_epsilon(&d, &$map! { "one" => 0.1, "two" => 0.5 }),
                    None
                );
                assert_eq!(a.debug_rel_all_epsilon(&d, &0.5), None);
                assert_eq!(
                    a.debug_ulps_epsilon(&d, &$map! { "one" => 1, "two" => 2 }),
                    None
                );
                assert_eq!(a.debug_ulps_all_epsilon(&d, &2), None);

                let e = $map! { "one" => 1.0, "two" => 2.0, "three" => 3.0 };
                assert_eq!(
                    a.debug_abs_epsilon(&e, &$map! { "one" => 0.1, "two" => 0.2 }),
                    None
                );
                assert_eq!(a.debug_abs_all_epsilon(&e, &0.2), None);
                assert_eq!(
                    a.debug_rel_epsilon(&e, &$map! { "one" => 0.1, "two" => 0.2 }),
                    None
                );
                assert_eq!(a.debug_rel_all_epsilon(&e, &0.5), None);
                assert_eq!(
                    a.debug_ulps_epsilon(&e, &$map! { "one" => 1, "two" => 2 }),
                    None
                );
                assert_eq!(a.debug_ulps_all_epsilon(&e, &2), None);
            }
        }
    };
}

macro_rules! btree_map {
    () => {{
        BTreeMap::new()
    }};
    ($($k:expr => $v:expr),+) => {{
        let mut m = BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
}

macro_rules! hash_map {
    () => {{
        HashMap::new()
    }};
    ($($k:expr => $v:expr),+) => {{
        let mut m = HashMap::new();
        $(m.insert($k, $v);)+
        m
    }};
}

impl_map_tests!(btree_map);
impl_map_tests!(hash_map);
