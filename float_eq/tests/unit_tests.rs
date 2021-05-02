#![allow(clippy::float_cmp, clippy::unit_cmp)]

// A selection of NaN values from the edges of the ranges of negative and
// positive NaN values and their payloads. Testing every single NaN value
// is viable in reasonable time for f32, but there are just too many f64
// values so this is the compromise.
#[derive(Clone, Copy)]
struct NaNBits<T> {
    pos_min: T,
    pos_max: T,
    neg_min: T,
    neg_max: T,
}

const F32_NAN_BITS: NaNBits<u32> = NaNBits {
    pos_min: 0x7F_80_00_01,
    pos_max: 0x7F_FF_FF_FF,
    neg_min: 0xFF_80_00_01,
    neg_max: 0xFF_FF_FF_FF,
};

const F64_NAN_BITS: NaNBits<u64> = NaNBits {
    pos_min: 0x7F_F0_00_00_00_00_00_01,
    pos_max: 0x7F_FF_FF_FF_FF_FF_FF_FF,
    neg_min: 0xFF_F0_00_00_00_00_00_01,
    neg_max: 0xFF_FF_FF_FF_FF_FF_FF_FF,
};

macro_rules! impl_test_helpers {
    ($float:ident, $uint:ident, $nan_bits:path) => {
        mod $float {
            pub(crate) const EPSILON: $float = $float::EPSILON;
            pub(crate) const INFINITY: $float = $float::INFINITY;
            pub(crate) const MIN_NORMAL: $float = $float::MIN_POSITIVE;
            pub(crate) const MAX_NORMAL: $float = $float::MAX;
            pub(crate) const MAX_ULPS: $uint = $uint::MAX;

            pub(crate) fn nan_test_values() -> [$float; 4] {
                [
                    $float::from_bits($nan_bits.pos_min),
                    $float::from_bits($nan_bits.pos_max),
                    $float::from_bits($nan_bits.neg_min),
                    $float::from_bits($nan_bits.neg_max),
                ]
            }

            // next representable float
            pub(crate) fn next(f: $float) -> $float {
                next_n(f, 1)
            }

            // previous representable float
            pub(crate) fn prev(f: $float) -> $float {
                prev_n(f, 1)
            }

            pub(crate) fn next_n(f: $float, n: $uint) -> $float {
                $float::from_bits(f.to_bits() + n)
            }

            pub(crate) fn prev_n(f: $float, n: $uint) -> $float {
                $float::from_bits(f.to_bits() - n)
            }
        }
    };
}

impl_test_helpers!(f32, u32, crate::F32_NAN_BITS);
impl_test_helpers!(f64, u64, crate::F64_NAN_BITS);

macro_rules! wrapper_tests {
    ($t:ident) => {
        #[test]
        fn float_eq() {
            let a = $t::new([0.999_999_9f32, 4.0]);
            let b = $t::new([1.0f32, 3.999_999_5]);
            let eps = f32::EPSILON;

            assert_float_eq!(a, b, abs <= [1.0 * eps, 4.0 * eps]);
            assert_float_ne!(a, b, abs <= [0.5 * eps, 4.0 * eps]);
            assert_float_ne!(a, b, abs <= [1.0 * eps, 2.0 * eps]);

            assert_float_eq!(a, b, rel <= [1.0 * eps, 1.0 * eps]);
            assert_float_ne!(a, b, rel <= [0.5 * eps, 1.0 * eps]);
            assert_float_ne!(a, b, rel <= [1.0 * eps, 0.5 * eps]);

            assert_float_eq!(a, b, rmax <= [1.0 * eps, 1.0 * eps]);
            assert_float_ne!(a, b, rmax <= [0.5 * eps, 1.0 * eps]);
            assert_float_ne!(a, b, rmax <= [1.0 * eps, 0.5 * eps]);

            assert_float_eq!(a, b, rmin <= [2.0 * eps, 2.0 * eps]);
            assert_float_ne!(a, b, rmin <= [1.0 * eps, 2.0 * eps]);
            assert_float_ne!(a, b, rmin <= [2.0 * eps, 1.0 * eps]);

            assert_float_eq!(a, b, r1st <= [2.0 * eps, 1.0 * eps]);
            assert_float_ne!(a, b, r1st <= [1.0 * eps, 1.0 * eps]);
            assert_float_ne!(a, b, r1st <= [2.0 * eps, 0.5 * eps]);

            assert_float_eq!(a, b, r2nd <= [1.0 * eps, 2.0 * eps]);
            assert_float_ne!(a, b, r2nd <= [0.5 * eps, 2.0 * eps]);
            assert_float_ne!(a, b, r2nd <= [1.0 * eps, 1.0 * eps]);

            assert_float_eq!(a, b, ulps <= [2, 2]);
            assert_float_ne!(a, b, ulps <= [1, 2]);
            assert_float_ne!(a, b, ulps <= [2, 1]);
        }

        #[test]
        fn float_eq_all() {
            let a = $t::new([0.999_999_9f32, 4.0]);
            let b = $t::new([1.0f32, 3.999_999_5]);
            let eps = f32::EPSILON;

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
        }

        #[test]
        fn debug_diff() {
            let a = $t::new([1.0f32, 2.0]);
            let b = $t::new([1.5f32, 2.25]);
            let ulps = [Some(4_194_304), Some(1_048_576)];

            assert_eq!(a.debug_abs_diff(&a), [0.0; 2]);
            assert_eq!(a.debug_ulps_diff(&a), [Some(0); 2]);

            assert_eq!(a.debug_abs_diff(&b), [0.5, 0.25]);
            assert_eq!(b.debug_abs_diff(&a), [0.5, 0.25]);

            assert_eq!(a.debug_ulps_diff(&b), ulps);
            assert_eq!(b.debug_ulps_diff(&a), ulps);
        }

        #[test]
        fn debug_tol() {
            let a = $t::new([2.0f32, 4.25]);
            let b = $t::new([2.5f32, 4.0]);
            let eps = [0.1, 0.2];

            assert_eq!(a.debug_abs_tol(&b, &eps), [0.1, 0.2]);
            assert_eq!(a.debug_rel_tol(&b, &eps), [0.25, 0.85]);
            assert_eq!(a.debug_rmax_tol(&b, &eps), [0.25, 0.85]);
            assert_eq!(a.debug_rmin_tol(&b, &eps), [0.2, 0.8]);
            assert_eq!(a.debug_r1st_tol(&b, &eps), [0.2, 0.85]);
            assert_eq!(a.debug_r2nd_tol(&b, &eps), [0.25, 0.8]);
            assert_eq!(a.debug_ulps_tol(&b, &[1, 2]), [1, 2]);
        }

        #[test]
        fn debug_all_tol() {
            let a = $t::new([2.0f32, 4.25]);
            let b = $t::new([2.5f32, 4.0]);

            assert_eq!(a.debug_abs_all_tol(&b, &0.2), [0.2, 0.2]);
            assert_eq!(a.debug_rel_all_tol(&b, &0.2), [0.5, 0.85]);
            assert_eq!(a.debug_rmax_all_tol(&b, &0.2), [0.5, 0.85]);
            assert_eq!(a.debug_rmin_all_tol(&b, &0.2), [0.4, 0.8]);
            assert_eq!(a.debug_r1st_all_tol(&b, &0.2), [0.4, 0.85]);
            assert_eq!(a.debug_r2nd_all_tol(&b, &0.2), [0.5, 0.8]);
            assert_eq!(a.debug_ulps_all_tol(&b, &2), [2, 2]);
        }
    };
}

mod unit_tests {
    mod arrays;
    mod core_types;
    mod macros;
    mod primitives;
    mod tuples;

    #[cfg(feature = "std")]
    mod std_types;

    #[cfg(feature = "num")]
    mod num_complex;
}

struct Foo(f32, f64);

#[test]
fn check() {
    let f = Foo { 0: 0.0, 1: 1.0 };
    assert_eq!(f.0, 0.0);
    assert_eq!(f.1, 1.0);
}
