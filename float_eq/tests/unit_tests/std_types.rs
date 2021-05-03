#![allow(clippy::many_single_char_names)]

use float_eq::{assert_float_eq, assert_float_ne, AssertFloatEq, AssertFloatEqAll};
use std::boxed::Box;
use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
use std::rc::Rc;
use std::sync::Arc;

mod rc {
    use super::*;
    wrapper_tests!(Rc);
}

mod arc {
    use super::*;
    wrapper_tests!(Arc);
}

mod r#box {
    use super::*;
    wrapper_tests!(Box);
}

// Note: there are more slice tests in core_types, since only some of the slice
// comparison implementation is no_std.
mod slice {
    use super::*;

    #[test]
    fn debug_diff() {
        let a = [1.0f32, 2.0];
        let b = [1.5f32, 2.25];
        let ulps = Some(vec![Some(4_194_304), Some(1_048_576)]);

        // Same shape
        assert_eq!(a[..].debug_abs_diff(&a[..]), Some(vec![0.0; 2]));
        assert_eq!(a[..].debug_ulps_diff(&a[..]), Some(vec![Some(0); 2]));

        assert_eq!(a[..].debug_abs_diff(&b[..]), Some(vec![0.5, 0.25]));
        assert_eq!(b[..].debug_abs_diff(&a[..]), Some(vec![0.5, 0.25]));

        assert_eq!(a[..].debug_ulps_diff(&b[..]), ulps);
        assert_eq!(b[..].debug_ulps_diff(&a[..]), ulps);

        // Different shape
        assert_eq!(a[1..].debug_abs_diff(&a[..]), None);
        assert_eq!(a[1..].debug_ulps_diff(&a[..]), None);

        assert_eq!(a[..].debug_abs_diff(&a[..1]), None);
        assert_eq!(a[..].debug_ulps_diff(&a[..1]), None);
    }

    #[test]
    fn debug_tol() {
        let a = [2.0f32, 4.25];
        let b = [2.5f32, 4.0];
        let eps = [0.1, 0.2];

        // Same shape a/b/tol
        assert_eq!(a[..].debug_abs_tol(&b[..], &eps), Some(vec![0.1, 0.2]));
        assert_eq!(a[..].debug_rel_tol(&b[..], &eps), Some(vec![0.25, 0.85]));
        assert_eq!(a[..].debug_rmax_tol(&b[..], &eps), Some(vec![0.25, 0.85]));
        assert_eq!(a[..].debug_rmin_tol(&b[..], &eps), Some(vec![0.2, 0.8]));
        assert_eq!(a[..].debug_r1st_tol(&b[..], &eps), Some(vec![0.2, 0.85]));
        assert_eq!(a[..].debug_r2nd_tol(&b[..], &eps), Some(vec![0.25, 0.8]));
        assert_eq!(a[..].debug_ulps_tol(&b[..], &[1, 2]), Some(vec![1, 2]));

        // Different shape a/b
        assert_eq!(a[1..].debug_abs_tol(&a[..], &eps), None);
        assert_eq!(a[1..].debug_rel_tol(&a[..], &eps), None);
        assert_eq!(a[1..].debug_rmax_tol(&a[..], &eps), None);
        assert_eq!(a[1..].debug_rmin_tol(&a[..], &eps), None);
        assert_eq!(a[1..].debug_r1st_tol(&a[..], &eps), None);
        assert_eq!(a[1..].debug_r2nd_tol(&a[..], &eps), None);
        assert_eq!(a[1..].debug_ulps_tol(&a[..], &[u32::MAX; 2]), None);

        assert_eq!(a[..].debug_abs_tol(&a[..1], &eps), None);
        assert_eq!(a[..].debug_rel_tol(&a[..1], &eps), None);
        assert_eq!(a[..].debug_rmax_tol(&a[..1], &eps), None);
        assert_eq!(a[..].debug_rmin_tol(&a[..1], &eps), None);
        assert_eq!(a[..].debug_r1st_tol(&a[..1], &eps), None);
        assert_eq!(a[..].debug_r2nd_tol(&a[..1], &eps), None);
        assert_eq!(a[..].debug_ulps_tol(&a[..1], &[u32::MAX; 2]), None);

        // Different shape tol
        assert_eq!(a[..].debug_abs_tol(&a[..], &[f32::INFINITY]), None);
        assert_eq!(a[..].debug_rel_tol(&a[..], &[f32::INFINITY]), None);
        assert_eq!(a[..].debug_rmax_tol(&a[..], &[f32::INFINITY]), None);
        assert_eq!(a[..].debug_rmin_tol(&a[..], &[f32::INFINITY]), None);
        assert_eq!(a[..].debug_r1st_tol(&a[..], &[f32::INFINITY]), None);
        assert_eq!(a[..].debug_r2nd_tol(&a[..], &[f32::INFINITY]), None);
        assert_eq!(a[..].debug_ulps_tol(&a[..], &[u32::MAX]), None);

        assert_eq!(a[..].debug_abs_tol(&a[..], &[f32::INFINITY; 3]), None);
        assert_eq!(a[..].debug_rel_tol(&a[..], &[f32::INFINITY; 3]), None);
        assert_eq!(a[..].debug_rmax_tol(&a[..], &[f32::INFINITY; 3]), None);
        assert_eq!(a[..].debug_rmin_tol(&a[..], &[f32::INFINITY; 3]), None);
        assert_eq!(a[..].debug_r1st_tol(&a[..], &[f32::INFINITY; 3]), None);
        assert_eq!(a[..].debug_r2nd_tol(&a[..], &[f32::INFINITY; 3]), None);
        assert_eq!(a[..].debug_ulps_tol(&a[..], &[u32::MAX; 3]), None);
    }

    #[test]
    fn debug_all_tol() {
        let a = [2.0f32, 4.25];
        let b = [2.5f32, 4.0];

        // Same shape
        assert_eq!(a[..].debug_abs_all_tol(&b[..], &0.2), Some(vec![0.2, 0.2]));
        assert_eq!(a[..].debug_rel_all_tol(&b[..], &0.2), Some(vec![0.5, 0.85]));
        assert_eq!(
            a[..].debug_rmax_all_tol(&b[..], &0.2),
            Some(vec![0.5, 0.85])
        );
        assert_eq!(a[..].debug_rmin_all_tol(&b[..], &0.2), Some(vec![0.4, 0.8]));
        assert_eq!(
            a[..].debug_r1st_all_tol(&b[..], &0.2),
            Some(vec![0.4, 0.85])
        );
        assert_eq!(a[..].debug_r2nd_all_tol(&b[..], &0.2), Some(vec![0.5, 0.8]));
        assert_eq!(a[..].debug_ulps_all_tol(&b[..], &2), Some(vec![2, 2]));

        // Different shape
        let inf = f32::INFINITY;
        let max = u32::MAX;

        assert_eq!(a[1..].debug_abs_all_tol(&a[..], &inf), None);
        assert_eq!(a[1..].debug_rel_all_tol(&a[..], &inf), None);
        assert_eq!(a[1..].debug_rmax_all_tol(&a[..], &inf), None);
        assert_eq!(a[1..].debug_rmin_all_tol(&a[..], &inf), None);
        assert_eq!(a[1..].debug_r1st_all_tol(&a[..], &inf), None);
        assert_eq!(a[1..].debug_r2nd_all_tol(&a[..], &inf), None);
        assert_eq!(a[1..].debug_ulps_all_tol(&a[..], &max), None);

        assert_eq!(a[..].debug_abs_all_tol(&a[..1], &inf), None);
        assert_eq!(a[..].debug_rel_all_tol(&a[..1], &inf), None);
        assert_eq!(a[..].debug_rmax_all_tol(&a[..1], &inf), None);
        assert_eq!(a[..].debug_rmin_all_tol(&a[..1], &inf), None);
        assert_eq!(a[..].debug_r1st_all_tol(&a[..1], &inf), None);
        assert_eq!(a[..].debug_r2nd_all_tol(&a[..1], &inf), None);
        assert_eq!(a[..].debug_ulps_all_tol(&a[..1], &max), None);
    }
}

macro_rules! impl_linear_collection_tests {
    ($t:ty, $c:ident) => {
        mod $c {
            use super::*;

            #[test]
            fn float_eq() {
                let a = $c![0.999_999_9f32, 4.0];
                let b = $c![1.0f32, 3.999_999_5];
                let c = $c![1.0f32];
                let eps = f32::EPSILON;

                // Same shape a/b/tol
                assert_float_eq!(a, b, abs <= $c![1.0 * eps, 4.0 * eps]);
                assert_float_ne!(a, b, abs <= $c![0.5 * eps, 4.0 * eps]);
                assert_float_ne!(a, b, abs <= $c![1.0 * eps, 2.0 * eps]);

                assert_float_eq!(a, b, rel <= $c![1.0 * eps, 1.0 * eps]);
                assert_float_ne!(a, b, rel <= $c![0.5 * eps, 1.0 * eps]);
                assert_float_ne!(a, b, rel <= $c![1.0 * eps, 0.5 * eps]);

                assert_float_eq!(a, b, rmax <= $c![1.0 * eps, 1.0 * eps]);
                assert_float_ne!(a, b, rmax <= $c![0.5 * eps, 1.0 * eps]);
                assert_float_ne!(a, b, rmax <= $c![1.0 * eps, 0.5 * eps]);

                assert_float_eq!(a, b, rmin <= $c![2.0 * eps, 2.0 * eps]);
                assert_float_ne!(a, b, rmin <= $c![1.0 * eps, 2.0 * eps]);
                assert_float_ne!(a, b, rmin <= $c![2.0 * eps, 1.0 * eps]);

                assert_float_eq!(a, b, r1st <= $c![2.0 * eps, 1.0 * eps]);
                assert_float_ne!(a, b, r1st <= $c![1.0 * eps, 1.0 * eps]);
                assert_float_ne!(a, b, r1st <= $c![2.0 * eps, 0.5 * eps]);

                assert_float_eq!(a, b, r2nd <= $c![1.0 * eps, 2.0 * eps]);
                assert_float_ne!(a, b, r2nd <= $c![0.5 * eps, 2.0 * eps]);
                assert_float_ne!(a, b, r2nd <= $c![1.0 * eps, 1.0 * eps]);

                assert_float_eq!(a, b, ulps <= $c![2, 2]);
                assert_float_ne!(a, b, ulps <= $c![1, 2]);
                assert_float_ne!(a, b, ulps <= $c![2, 1]);

                // Different shape a/b
                assert_float_ne!(a, c, abs <= $c![f32::INFINITY; 2]);
                assert_float_ne!(a, c, rel <= $c![f32::INFINITY; 2]);
                assert_float_ne!(a, c, rmax <= $c![f32::INFINITY; 2]);
                assert_float_ne!(a, c, rmin <= $c![f32::INFINITY; 2]);
                assert_float_ne!(a, c, r1st <= $c![f32::INFINITY; 2]);
                assert_float_ne!(a, c, r2nd <= $c![f32::INFINITY; 2]);
                assert_float_ne!(a, c, ulps <= $c![u32::MAX; 2]);

                assert_float_ne!(c, b, abs <= $c![f32::INFINITY; 2]);
                assert_float_ne!(c, b, rel <= $c![f32::INFINITY; 2]);
                assert_float_ne!(c, b, rmax <= $c![f32::INFINITY; 2]);
                assert_float_ne!(c, b, rmin <= $c![f32::INFINITY; 2]);
                assert_float_ne!(c, b, r1st <= $c![f32::INFINITY; 2]);
                assert_float_ne!(c, b, r2nd <= $c![f32::INFINITY; 2]);
                assert_float_ne!(c, b, ulps <= $c![u32::MAX; 2]);

                // Different shape tol
                assert_float_ne!(a, b, abs <= $c![f32::INFINITY]);
                assert_float_ne!(a, b, rel <= $c![f32::INFINITY]);
                assert_float_ne!(a, b, rmax <= $c![f32::INFINITY]);
                assert_float_ne!(a, b, rmin <= $c![f32::INFINITY]);
                assert_float_ne!(a, b, r1st <= $c![f32::INFINITY]);
                assert_float_ne!(a, b, r2nd <= $c![f32::INFINITY]);
                assert_float_ne!(a, b, ulps <= $c![u32::MAX]);

                assert_float_ne!(a, b, abs <= $c![f32::INFINITY; 3]);
                assert_float_ne!(a, b, rel <= $c![f32::INFINITY; 3]);
                assert_float_ne!(a, b, rmax <= $c![f32::INFINITY; 3]);
                assert_float_ne!(a, b, rmin <= $c![f32::INFINITY; 3]);
                assert_float_ne!(a, b, r1st <= $c![f32::INFINITY; 3]);
                assert_float_ne!(a, b, r2nd <= $c![f32::INFINITY; 3]);
                assert_float_ne!(a, b, ulps <= $c![u32::MAX; 3]);
            }

            #[test]
            fn float_eq_all() {
                let a = $c![0.999_999_9f32, 4.0];
                let b = $c![1.0f32, 3.999_999_5];
                let c = $c![1.0f32];
                let eps = f32::EPSILON;

                // Same shape a/b/tol
                assert_float_eq!(a, b, abs_all <= 4.0 * eps);
                assert_float_ne!(a, b, abs_all <= 2.0 * eps);

                assert_float_eq!(a, b, rel_all <= 1.0 * eps);
                assert_float_ne!(a, b, rel_all <= 0.5 * eps);

                assert_float_eq!(a, b, rmax_all <= 1.0 * eps);
                assert_float_ne!(a, b, rmax_all <= 0.5 * eps);

                assert_float_eq!(a, b, rmin_all <= 2.0 * eps);
                assert_float_ne!(a, b, rmin_all <= 1.0 * eps);

                assert_float_eq!(a, b, r1st_all <= 2.0 * eps);
                assert_float_ne!(a, b, r1st_all <= 1.0 * eps);

                assert_float_eq!(a, b, r2nd_all <= 2.0 * eps);
                assert_float_ne!(a, b, r2nd_all <= 1.0 * eps);

                assert_float_eq!(a, b, ulps_all <= 2);
                assert_float_ne!(a, b, ulps_all <= 1);

                // Different shape a/b
                assert_float_ne!(a, c, abs_all <= f32::INFINITY);
                assert_float_ne!(a, c, rel_all <= f32::INFINITY);
                assert_float_ne!(a, c, rmax_all <= f32::INFINITY);
                assert_float_ne!(a, c, rmin_all <= f32::INFINITY);
                assert_float_ne!(a, c, r1st_all <= f32::INFINITY);
                assert_float_ne!(a, c, r2nd_all <= f32::INFINITY);
                assert_float_ne!(a, c, ulps_all <= u32::MAX);

                assert_float_ne!(c, b, abs_all <= f32::INFINITY);
                assert_float_ne!(c, b, rel_all <= f32::INFINITY);
                assert_float_ne!(c, b, rmax_all <= f32::INFINITY);
                assert_float_ne!(c, b, rmin_all <= f32::INFINITY);
                assert_float_ne!(c, b, r1st_all <= f32::INFINITY);
                assert_float_ne!(c, b, r2nd_all <= f32::INFINITY);
                assert_float_ne!(c, b, ulps_all <= u32::MAX);
            }

            #[test]
            fn debug_diff() {
                let a = $c![1.0f32, 2.0];
                let b = $c![1.5f32, 2.25];
                let c = $c![1.0f32];
                let ulps = Some($c![Some(4_194_304), Some(1_048_576)]);

                // Same shape
                assert_eq!(a.debug_abs_diff(&a), Some($c![0.0; 2]));
                assert_eq!(a.debug_ulps_diff(&a), Some($c![Some(0); 2]));

                assert_eq!(a.debug_abs_diff(&b), Some($c![0.5, 0.25]));
                assert_eq!(b.debug_abs_diff(&a), Some($c![0.5, 0.25]));

                assert_eq!(a.debug_ulps_diff(&b), ulps);
                assert_eq!(b.debug_ulps_diff(&a), ulps);

                // Different shape
                assert_eq!(a.debug_abs_diff(&c), None);
                assert_eq!(a.debug_ulps_diff(&c), None);

                assert_eq!(c.debug_abs_diff(&a), None);
                assert_eq!(c.debug_ulps_diff(&a), None);
            }

            #[test]
            fn debug_tol() {
                let a = $c![2.0f32, 4.25];
                let b = $c![2.5f32, 4.0];
                let c = $c![1.0f32];
                let eps = $c![0.1, 0.2];

                // Same shape a/b/tol
                assert_eq!(a.debug_abs_tol(&b, &eps), Some($c![0.1, 0.2]));
                assert_eq!(a.debug_rel_tol(&b, &eps), Some($c![0.25, 0.85]));
                assert_eq!(a.debug_rmax_tol(&b, &eps), Some($c![0.25, 0.85]));
                assert_eq!(a.debug_rmin_tol(&b, &eps), Some($c![0.2, 0.8]));
                assert_eq!(a.debug_r1st_tol(&b, &eps), Some($c![0.2, 0.85]));
                assert_eq!(a.debug_r2nd_tol(&b, &eps), Some($c![0.25, 0.8]));
                assert_eq!(a.debug_ulps_tol(&b, &$c![1, 2]), Some($c![1, 2]));

                // Different shape a/b
                assert_eq!(a.debug_abs_tol(&c, &eps), None);
                assert_eq!(a.debug_rel_tol(&c, &eps), None);
                assert_eq!(a.debug_rmax_tol(&c, &eps), None);
                assert_eq!(a.debug_rmin_tol(&c, &eps), None);
                assert_eq!(a.debug_r1st_tol(&c, &eps), None);
                assert_eq!(a.debug_r2nd_tol(&c, &eps), None);
                assert_eq!(a.debug_ulps_tol(&c, &$c![u32::MAX; 2]), None);

                assert_eq!(c.debug_abs_tol(&a, &eps), None);
                assert_eq!(c.debug_rel_tol(&a, &eps), None);
                assert_eq!(c.debug_rmax_tol(&a, &eps), None);
                assert_eq!(c.debug_rmin_tol(&a, &eps), None);
                assert_eq!(c.debug_r1st_tol(&a, &eps), None);
                assert_eq!(c.debug_r2nd_tol(&a, &eps), None);
                assert_eq!(c.debug_ulps_tol(&a, &$c![u32::MAX; 2]), None);

                // Different shape tol
                assert_eq!(a.debug_abs_tol(&a, &$c![f32::INFINITY]), None);
                assert_eq!(a.debug_rel_tol(&a, &$c![f32::INFINITY]), None);
                assert_eq!(a.debug_rmax_tol(&a, &$c![f32::INFINITY]), None);
                assert_eq!(a.debug_rmin_tol(&a, &$c![f32::INFINITY]), None);
                assert_eq!(a.debug_r1st_tol(&a, &$c![f32::INFINITY]), None);
                assert_eq!(a.debug_r2nd_tol(&a, &$c![f32::INFINITY]), None);
                assert_eq!(a.debug_ulps_tol(&a, &$c![u32::MAX]), None);

                assert_eq!(a.debug_abs_tol(&a, &$c![f32::INFINITY; 3]), None);
                assert_eq!(a.debug_rel_tol(&a, &$c![f32::INFINITY; 3]), None);
                assert_eq!(a.debug_rmax_tol(&a, &$c![f32::INFINITY; 3]), None);
                assert_eq!(a.debug_rmin_tol(&a, &$c![f32::INFINITY; 3]), None);
                assert_eq!(a.debug_r1st_tol(&a, &$c![f32::INFINITY; 3]), None);
                assert_eq!(a.debug_r2nd_tol(&a, &$c![f32::INFINITY; 3]), None);
                assert_eq!(a.debug_ulps_tol(&a, &$c![u32::MAX; 3]), None);
            }

            #[test]
            fn debug_all_tol() {
                let a = $c![2.0f32, 4.25];
                let b = $c![2.5f32, 4.0];
                let c = $c![1.0f32];

                // Same shape
                assert_eq!(a.debug_abs_all_tol(&b, &0.2), Some($c![0.2, 0.2]));
                assert_eq!(a.debug_rel_all_tol(&b, &0.2), Some($c![0.5, 0.85]));
                assert_eq!(a.debug_rmax_all_tol(&b, &0.2), Some($c![0.5, 0.85]));
                assert_eq!(a.debug_rmin_all_tol(&b, &0.2), Some($c![0.4, 0.8]));
                assert_eq!(a.debug_r1st_all_tol(&b, &0.2), Some($c![0.4, 0.85]));
                assert_eq!(a.debug_r2nd_all_tol(&b, &0.2), Some($c![0.5, 0.8]));
                assert_eq!(a.debug_ulps_all_tol(&b, &2), Some($c![2, 2]));

                // Different shape
                let inf = f32::INFINITY;
                let max = u32::MAX;

                assert_eq!(a.debug_abs_all_tol(&c, &inf), None);
                assert_eq!(a.debug_rel_all_tol(&c, &inf), None);
                assert_eq!(a.debug_rmax_all_tol(&c, &inf), None);
                assert_eq!(a.debug_rmin_all_tol(&c, &inf), None);
                assert_eq!(a.debug_r1st_all_tol(&c, &inf), None);
                assert_eq!(a.debug_r2nd_all_tol(&c, &inf), None);
                assert_eq!(a.debug_ulps_all_tol(&c, &max), None);

                assert_eq!(c.debug_abs_all_tol(&a, &inf), None);
                assert_eq!(c.debug_rel_all_tol(&a, &inf), None);
                assert_eq!(c.debug_rmax_all_tol(&a, &inf), None);
                assert_eq!(c.debug_rmin_all_tol(&a, &inf), None);
                assert_eq!(c.debug_r1st_all_tol(&a, &inf), None);
                assert_eq!(c.debug_r2nd_all_tol(&a, &inf), None);
                assert_eq!(c.debug_ulps_all_tol(&a, &max), None);
            }
        }
    };
}

macro_rules! vec_deque {
    ($($x:expr),*) => {{
        vec![$($x,)*].into_iter().collect::<VecDeque<_>>()
    }};
    ($x:expr; $n:literal) => {{
        vec![$x; $n].into_iter().collect::<VecDeque<_>>()
    }};
}

macro_rules! linked_list {
    ($($x:expr),*) => {{
        vec![$($x,)*].into_iter().collect::<LinkedList<_>>()
    }};
    ($x:expr; $n:literal) => {{
        vec![$x; $n].into_iter().collect::<LinkedList<_>>()
    }};
}

impl_linear_collection_tests!(Vec<f32>, vec);
impl_linear_collection_tests!(VecDeque<f32>, vec_deque);
impl_linear_collection_tests!(LinkedList<f32>, linked_list);

macro_rules! impl_map_tests {
    ($t:ident, $c:ident) => {
        mod $c {
            use super::*;

            fn map1<T>(one: T) -> $t<&'static str, T> {
                $c! { "one" => one }
            }

            fn map12<T>(one: T, two: T) -> $t<&'static str, T> {
                $c! { "one" => one, "two" => two }
            }

            fn map13<T>(one: T, three: T) -> $t<&'static str, T> {
                $c! { "one" => one, "three" => three }
            }

            fn map123<T>(one: T, two: T, three: T) -> $t<&'static str, T> {
                $c! { "one" => one, "two" => two, "three" => three }
            }

            #[test]
            fn float_eq() {
                let a = map12(0.999_999_9f32, 4.0);
                let b = map12(1.0f32, 3.999_999_5);
                let eps = f32::EPSILON;
                let inf = f32::INFINITY;

                // Same shape a/b/tol
                assert_float_eq!(a, b, abs <= map12(1.0 * eps, 4.0 * eps));
                assert_float_ne!(a, b, abs <= map12(0.5 * eps, 4.0 * eps));
                assert_float_ne!(a, b, abs <= map12(1.0 * eps, 2.0 * eps));

                assert_float_eq!(a, b, rel <= map12(1.0 * eps, 1.0 * eps));
                assert_float_ne!(a, b, rel <= map12(0.5 * eps, 1.0 * eps));
                assert_float_ne!(a, b, rel <= map12(1.0 * eps, 0.5 * eps));

                assert_float_eq!(a, b, rmax <= map12(1.0 * eps, 1.0 * eps));
                assert_float_ne!(a, b, rmax <= map12(0.5 * eps, 1.0 * eps));
                assert_float_ne!(a, b, rmax <= map12(1.0 * eps, 0.5 * eps));

                assert_float_eq!(a, b, rmin <= map12(2.0 * eps, 2.0 * eps));
                assert_float_ne!(a, b, rmin <= map12(1.0 * eps, 2.0 * eps));
                assert_float_ne!(a, b, rmin <= map12(2.0 * eps, 1.0 * eps));

                assert_float_eq!(a, b, r1st <= map12(2.0 * eps, 1.0 * eps));
                assert_float_ne!(a, b, r1st <= map12(1.0 * eps, 1.0 * eps));
                assert_float_ne!(a, b, r1st <= map12(2.0 * eps, 0.5 * eps));

                assert_float_eq!(a, b, r2nd <= map12(1.0 * eps, 2.0 * eps));
                assert_float_ne!(a, b, r2nd <= map12(0.5 * eps, 2.0 * eps));
                assert_float_ne!(a, b, r2nd <= map12(1.0 * eps, 1.0 * eps));

                assert_float_eq!(a, b, ulps <= map12(2, 2));
                assert_float_ne!(a, b, ulps <= map12(1, 2));
                assert_float_ne!(a, b, ulps <= map12(2, 1));

                // Different shape a/b: item missing
                let c = map1(1.0f32);

                assert_float_ne!(a, c, abs <= map12(inf, inf));
                assert_float_ne!(a, c, rel <= map12(inf, inf));
                assert_float_ne!(a, c, rmax <= map12(inf, inf));
                assert_float_ne!(a, c, rmin <= map12(inf, inf));
                assert_float_ne!(a, c, r1st <= map12(inf, inf));
                assert_float_ne!(a, c, r2nd <= map12(inf, inf));
                assert_float_ne!(a, c, ulps <= map12(u32::MAX, u32::MAX));

                assert_float_ne!(c, b, abs <= map12(inf, inf));
                assert_float_ne!(c, b, rel <= map12(inf, inf));
                assert_float_ne!(c, b, rmax <= map12(inf, inf));
                assert_float_ne!(c, b, rmin <= map12(inf, inf));
                assert_float_ne!(c, b, r1st <= map12(inf, inf));
                assert_float_ne!(c, b, r2nd <= map12(inf, inf));
                assert_float_ne!(c, b, ulps <= map12(u32::MAX, u32::MAX));

                // Different shape a/b: item extra
                let d = map123(1.0f32, 3.999_999_5, 1337.0);

                assert_float_ne!(a, d, abs <= map12(inf, inf));
                assert_float_ne!(a, d, rel <= map12(inf, inf));
                assert_float_ne!(a, d, rmax <= map12(inf, inf));
                assert_float_ne!(a, d, rmin <= map12(inf, inf));
                assert_float_ne!(a, d, r1st <= map12(inf, inf));
                assert_float_ne!(a, d, r2nd <= map12(inf, inf));
                assert_float_ne!(a, d, ulps <= map12(u32::MAX, u32::MAX));

                assert_float_ne!(d, b, abs <= map12(inf, inf));
                assert_float_ne!(d, b, rel <= map12(inf, inf));
                assert_float_ne!(d, b, rmax <= map12(inf, inf));
                assert_float_ne!(d, b, rmin <= map12(inf, inf));
                assert_float_ne!(d, b, r1st <= map12(inf, inf));
                assert_float_ne!(d, b, r2nd <= map12(inf, inf));
                assert_float_ne!(d, b, ulps <= map12(u32::MAX, u32::MAX));

                // Different shape a/b: different keys
                let e = map13(0.999_999_9f32, 4.0);

                assert_float_ne!(a, e, abs <= map12(inf, inf));
                assert_float_ne!(a, e, rel <= map12(inf, inf));
                assert_float_ne!(a, e, rmax <= map12(inf, inf));
                assert_float_ne!(a, e, rmin <= map12(inf, inf));
                assert_float_ne!(a, e, r1st <= map12(inf, inf));
                assert_float_ne!(a, e, r2nd <= map12(inf, inf));
                assert_float_ne!(a, e, ulps <= map12(u32::MAX, u32::MAX));

                assert_float_ne!(e, b, abs <= map12(inf, inf));
                assert_float_ne!(e, b, rel <= map12(inf, inf));
                assert_float_ne!(e, b, rmax <= map12(inf, inf));
                assert_float_ne!(e, b, rmin <= map12(inf, inf));
                assert_float_ne!(e, b, r1st <= map12(inf, inf));
                assert_float_ne!(e, b, r2nd <= map12(inf, inf));
                assert_float_ne!(e, b, ulps <= map12(u32::MAX, u32::MAX));

                // Different shape tol: item missing
                assert_float_ne!(a, b, abs <= map1(inf));
                assert_float_ne!(a, b, rel <= map1(inf));
                assert_float_ne!(a, b, rmax <= map1(inf));
                assert_float_ne!(a, b, rmin <= map1(inf));
                assert_float_ne!(a, b, r1st <= map1(inf));
                assert_float_ne!(a, b, r2nd <= map1(inf));
                assert_float_ne!(a, b, ulps <= map1(u32::MAX));

                // Different shape tol: item extra
                assert_float_ne!(a, b, abs <= map123(inf, inf, inf));
                assert_float_ne!(a, b, rel <= map123(inf, inf, inf));
                assert_float_ne!(a, b, rmax <= map123(inf, inf, inf));
                assert_float_ne!(a, b, rmin <= map123(inf, inf, inf));
                assert_float_ne!(a, b, r1st <= map123(inf, inf, inf));
                assert_float_ne!(a, b, r2nd <= map123(inf, inf, inf));
                assert_float_ne!(a, b, ulps <= map123(u32::MAX, u32::MAX, u32::MAX));

                // Different shape tol: different keys
                assert_float_ne!(a, b, abs <= map13(inf, inf));
                assert_float_ne!(a, b, rel <= map13(inf, inf));
                assert_float_ne!(a, b, rmax <= map13(inf, inf));
                assert_float_ne!(a, b, rmin <= map13(inf, inf));
                assert_float_ne!(a, b, r1st <= map13(inf, inf));
                assert_float_ne!(a, b, r2nd <= map13(inf, inf));
                assert_float_ne!(a, b, ulps <= map13(u32::MAX, u32::MAX));
            }

            #[test]
            fn float_eq_all() {
                let a = map12(0.999_999_9f32, 4.0);
                let b = map12(1.0f32, 3.999_999_5);
                let eps = f32::EPSILON;

                // Same shape a/b/tol
                assert_float_eq!(a, b, abs_all <= 4.0 * eps);
                assert_float_ne!(a, b, abs_all <= 2.0 * eps);

                assert_float_eq!(a, b, rel_all <= 1.0 * eps);
                assert_float_ne!(a, b, rel_all <= 0.5 * eps);

                assert_float_eq!(a, b, rmax_all <= 1.0 * eps);
                assert_float_ne!(a, b, rmax_all <= 0.5 * eps);

                assert_float_eq!(a, b, rmin_all <= 2.0 * eps);
                assert_float_ne!(a, b, rmin_all <= 1.0 * eps);

                assert_float_eq!(a, b, r1st_all <= 2.0 * eps);
                assert_float_ne!(a, b, r1st_all <= 1.0 * eps);

                assert_float_eq!(a, b, r2nd_all <= 2.0 * eps);
                assert_float_ne!(a, b, r2nd_all <= 1.0 * eps);

                assert_float_eq!(a, b, ulps_all <= 2);
                assert_float_ne!(a, b, ulps_all <= 1);

                // Different shape a/b: item missing
                let c = map1(1.0f32);

                assert_float_ne!(a, c, abs_all <= f32::INFINITY);
                assert_float_ne!(a, c, rel_all <= f32::INFINITY);
                assert_float_ne!(a, c, rmax_all <= f32::INFINITY);
                assert_float_ne!(a, c, rmin_all <= f32::INFINITY);
                assert_float_ne!(a, c, r1st_all <= f32::INFINITY);
                assert_float_ne!(a, c, r2nd_all <= f32::INFINITY);
                assert_float_ne!(a, c, ulps_all <= u32::MAX);

                assert_float_ne!(c, b, abs_all <= f32::INFINITY);
                assert_float_ne!(c, b, rel_all <= f32::INFINITY);
                assert_float_ne!(c, b, rmax_all <= f32::INFINITY);
                assert_float_ne!(c, b, rmin_all <= f32::INFINITY);
                assert_float_ne!(c, b, r1st_all <= f32::INFINITY);
                assert_float_ne!(c, b, r2nd_all <= f32::INFINITY);
                assert_float_ne!(c, b, ulps_all <= u32::MAX);

                // Different shape a/b: item extra
                let d = map123(1.0f32, 3.999_999_5, 1337.0);

                assert_float_ne!(a, d, abs_all <= f32::INFINITY);
                assert_float_ne!(a, d, rel_all <= f32::INFINITY);
                assert_float_ne!(a, d, rmax_all <= f32::INFINITY);
                assert_float_ne!(a, d, rmin_all <= f32::INFINITY);
                assert_float_ne!(a, d, r1st_all <= f32::INFINITY);
                assert_float_ne!(a, d, r2nd_all <= f32::INFINITY);
                assert_float_ne!(a, d, ulps_all <= u32::MAX);

                assert_float_ne!(d, b, abs_all <= f32::INFINITY);
                assert_float_ne!(d, b, rel_all <= f32::INFINITY);
                assert_float_ne!(d, b, rmax_all <= f32::INFINITY);
                assert_float_ne!(d, b, rmin_all <= f32::INFINITY);
                assert_float_ne!(d, b, r1st_all <= f32::INFINITY);
                assert_float_ne!(d, b, r2nd_all <= f32::INFINITY);
                assert_float_ne!(d, b, ulps_all <= u32::MAX);

                // Different shape a/b: different keys
                let e = map13(0.999_999_9f32, 4.0);

                assert_float_ne!(a, e, abs_all <= f32::INFINITY);
                assert_float_ne!(a, e, rel_all <= f32::INFINITY);
                assert_float_ne!(a, e, rmax_all <= f32::INFINITY);
                assert_float_ne!(a, e, rmin_all <= f32::INFINITY);
                assert_float_ne!(a, e, r1st_all <= f32::INFINITY);
                assert_float_ne!(a, e, r2nd_all <= f32::INFINITY);
                assert_float_ne!(a, e, ulps_all <= u32::MAX);

                assert_float_ne!(e, b, abs_all <= f32::INFINITY);
                assert_float_ne!(e, b, rel_all <= f32::INFINITY);
                assert_float_ne!(e, b, rmax_all <= f32::INFINITY);
                assert_float_ne!(e, b, rmin_all <= f32::INFINITY);
                assert_float_ne!(e, b, r1st_all <= f32::INFINITY);
                assert_float_ne!(e, b, r2nd_all <= f32::INFINITY);
                assert_float_ne!(e, b, ulps_all <= u32::MAX);
            }

            #[test]
            fn debug_diff() {
                let a = map12(1.0f32, 2.0);
                let b = map12(1.5f32, 2.25);
                let c = map1(1.0f32);
                let d = map123(1.0f32, 2.0, 4.0);
                let ulps = Some(map12(Some(4_194_304), Some(1_048_576)));

                // Same shape
                assert_eq!(a.debug_abs_diff(&a), Some(map12(0.0, 0.0)));
                assert_eq!(a.debug_ulps_diff(&a), Some(map12(Some(0), Some(0))));

                assert_eq!(a.debug_abs_diff(&b), Some(map12(0.5, 0.25)));
                assert_eq!(b.debug_abs_diff(&a), Some(map12(0.5, 0.25)));

                assert_eq!(a.debug_ulps_diff(&b), ulps);
                assert_eq!(b.debug_ulps_diff(&a), ulps);

                // Different shape: item missing
                assert_eq!(a.debug_abs_diff(&c), None);
                assert_eq!(a.debug_ulps_diff(&c), None);

                assert_eq!(c.debug_abs_diff(&a), None);
                assert_eq!(c.debug_ulps_diff(&a), None);

                // Different shape: item extra
                assert_eq!(a.debug_abs_diff(&d), None);
                assert_eq!(a.debug_ulps_diff(&d), None);

                assert_eq!(d.debug_abs_diff(&a), None);
                assert_eq!(d.debug_ulps_diff(&a), None);
            }

            #[test]
            fn debug_tol() {
                let a = map12(2.0f32, 4.25);
                let b = map12(2.5f32, 4.0);
                let c = map1(1.0f32);
                let d = map123(1.0f32, 2.0, 4.0);
                let inf = f32::INFINITY;
                let eps = map12(0.1, 0.2);

                // Same shape a/b/tol
                assert_eq!(a.debug_abs_tol(&b, &eps), Some(map12(0.1, 0.2)));
                assert_eq!(a.debug_rel_tol(&b, &eps), Some(map12(0.25, 0.85)));
                assert_eq!(a.debug_rmax_tol(&b, &eps), Some(map12(0.25, 0.85)));
                assert_eq!(a.debug_rmin_tol(&b, &eps), Some(map12(0.2, 0.8)));
                assert_eq!(a.debug_r1st_tol(&b, &eps), Some(map12(0.2, 0.85)));
                assert_eq!(a.debug_r2nd_tol(&b, &eps), Some(map12(0.25, 0.8)));
                assert_eq!(a.debug_ulps_tol(&b, &map12(1, 2)), Some(map12(1, 2)));

                // Different shape a/b: item missing
                assert_eq!(a.debug_abs_tol(&c, &eps), None);
                assert_eq!(a.debug_rel_tol(&c, &eps), None);
                assert_eq!(a.debug_rmax_tol(&c, &eps), None);
                assert_eq!(a.debug_rmin_tol(&c, &eps), None);
                assert_eq!(a.debug_r1st_tol(&c, &eps), None);
                assert_eq!(a.debug_r2nd_tol(&c, &eps), None);
                assert_eq!(a.debug_ulps_tol(&c, &map12(1, 2)), None);

                assert_eq!(c.debug_abs_tol(&a, &eps), None);
                assert_eq!(c.debug_rel_tol(&a, &eps), None);
                assert_eq!(c.debug_rmax_tol(&a, &eps), None);
                assert_eq!(c.debug_rmin_tol(&a, &eps), None);
                assert_eq!(c.debug_r1st_tol(&a, &eps), None);
                assert_eq!(c.debug_r2nd_tol(&a, &eps), None);
                assert_eq!(c.debug_ulps_tol(&a, &map12(1, 2)), None);

                // Different shape a/b: item extra
                assert_eq!(a.debug_abs_tol(&d, &eps), None);
                assert_eq!(a.debug_rel_tol(&d, &eps), None);
                assert_eq!(a.debug_rmax_tol(&d, &eps), None);
                assert_eq!(a.debug_rmin_tol(&d, &eps), None);
                assert_eq!(a.debug_r1st_tol(&d, &eps), None);
                assert_eq!(a.debug_r2nd_tol(&d, &eps), None);
                assert_eq!(a.debug_ulps_tol(&d, &map12(u32::MAX, u32::MAX)), None);

                assert_eq!(d.debug_abs_tol(&a, &eps), None);
                assert_eq!(d.debug_rel_tol(&a, &eps), None);
                assert_eq!(d.debug_rmax_tol(&a, &eps), None);
                assert_eq!(d.debug_rmin_tol(&a, &eps), None);
                assert_eq!(d.debug_r1st_tol(&a, &eps), None);
                assert_eq!(d.debug_r2nd_tol(&a, &eps), None);
                assert_eq!(d.debug_ulps_tol(&a, &map12(u32::MAX, u32::MAX)), None);

                // Different shape tol
                assert_eq!(a.debug_abs_tol(&a, &map1(inf)), None);
                assert_eq!(a.debug_rel_tol(&a, &map1(inf)), None);
                assert_eq!(a.debug_rmax_tol(&a, &map1(inf)), None);
                assert_eq!(a.debug_rmin_tol(&a, &map1(inf)), None);
                assert_eq!(a.debug_r1st_tol(&a, &map1(inf)), None);
                assert_eq!(a.debug_r2nd_tol(&a, &map1(inf)), None);
                assert_eq!(a.debug_ulps_tol(&a, &map1(u32::MAX)), None);

                assert_eq!(a.debug_abs_tol(&a, &map123(inf, inf, inf)), None);
                assert_eq!(a.debug_rel_tol(&a, &map123(inf, inf, inf)), None);
                assert_eq!(a.debug_rmax_tol(&a, &map123(inf, inf, inf)), None);
                assert_eq!(a.debug_rmin_tol(&a, &map123(inf, inf, inf)), None);
                assert_eq!(a.debug_r1st_tol(&a, &map123(inf, inf, inf)), None);
                assert_eq!(a.debug_r2nd_tol(&a, &map123(inf, inf, inf)), None);
                assert_eq!(
                    a.debug_ulps_tol(&a, &map123(u32::MAX, u32::MAX, u32::MAX)),
                    None
                );
            }

            #[test]
            fn debug_all_tol() {
                let a = map12(2.0f32, 4.25);
                let b = map12(2.5f32, 4.0);
                let c = map1(1.0f32);
                let d = map123(1.0f32, 2.0, 4.0);
                let inf = f32::INFINITY;
                let max = u32::MAX;

                // Same shape
                assert_eq!(a.debug_abs_all_tol(&b, &0.2), Some(map12(0.2, 0.2)));
                assert_eq!(a.debug_rel_all_tol(&b, &0.2), Some(map12(0.5, 0.85)));
                assert_eq!(a.debug_rmax_all_tol(&b, &0.2), Some(map12(0.5, 0.85)));
                assert_eq!(a.debug_rmin_all_tol(&b, &0.2), Some(map12(0.4, 0.8)));
                assert_eq!(a.debug_r1st_all_tol(&b, &0.2), Some(map12(0.4, 0.85)));
                assert_eq!(a.debug_r2nd_all_tol(&b, &0.2), Some(map12(0.5, 0.8)));
                assert_eq!(a.debug_ulps_all_tol(&b, &2), Some(map12(2, 2)));

                // Different shape - item missing
                assert_eq!(a.debug_abs_all_tol(&c, &inf), None);
                assert_eq!(a.debug_rel_all_tol(&c, &inf), None);
                assert_eq!(a.debug_rmax_all_tol(&c, &inf), None);
                assert_eq!(a.debug_rmin_all_tol(&c, &inf), None);
                assert_eq!(a.debug_r1st_all_tol(&c, &inf), None);
                assert_eq!(a.debug_r2nd_all_tol(&c, &inf), None);
                assert_eq!(a.debug_ulps_all_tol(&c, &max), None);

                assert_eq!(c.debug_abs_all_tol(&a, &inf), None);
                assert_eq!(c.debug_rel_all_tol(&a, &inf), None);
                assert_eq!(c.debug_rmax_all_tol(&a, &inf), None);
                assert_eq!(c.debug_rmin_all_tol(&a, &inf), None);
                assert_eq!(c.debug_r1st_all_tol(&a, &inf), None);
                assert_eq!(c.debug_r2nd_all_tol(&a, &inf), None);
                assert_eq!(c.debug_ulps_all_tol(&a, &max), None);

                // Different shape - item extra
                assert_eq!(a.debug_abs_all_tol(&d, &inf), None);
                assert_eq!(a.debug_rel_all_tol(&d, &inf), None);
                assert_eq!(a.debug_rmax_all_tol(&d, &inf), None);
                assert_eq!(a.debug_rmin_all_tol(&d, &inf), None);
                assert_eq!(a.debug_r1st_all_tol(&d, &inf), None);
                assert_eq!(a.debug_r2nd_all_tol(&d, &inf), None);
                assert_eq!(a.debug_ulps_all_tol(&d, &max), None);

                assert_eq!(d.debug_abs_all_tol(&a, &inf), None);
                assert_eq!(d.debug_rel_all_tol(&a, &inf), None);
                assert_eq!(d.debug_rmax_all_tol(&a, &inf), None);
                assert_eq!(d.debug_rmin_all_tol(&a, &inf), None);
                assert_eq!(d.debug_r1st_all_tol(&a, &inf), None);
                assert_eq!(d.debug_r2nd_all_tol(&a, &inf), None);
                assert_eq!(d.debug_ulps_all_tol(&a, &max), None);
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

impl_map_tests!(BTreeMap, btree_map);
impl_map_tests!(HashMap, hash_map);
