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

mod primitives {
    mod eq_abs;
    mod eq_rel;
    mod eq_ulps;
    mod float_diff;
}
