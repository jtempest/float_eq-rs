use crate::{FloatDiff, FloatEq, FloatEqDebug};
use core::mem::MaybeUninit;

// arrays
//TODO: Should this be publically available for users to conditionally implement
// support if they need it?
macro_rules! impl_float_eq_traits_for_array {
    ($n:literal) => {
        #[doc(hidden)]
        impl<T: FloatDiff> FloatDiff for [T; $n] {
            type AbsDiff = [<T as $crate::FloatDiff>::AbsDiff; $n];
            type UlpsDiff = [<T as $crate::FloatDiff>::UlpsDiff; $n];

            #[inline]
            fn abs_diff(&self, other: &Self) -> Self::AbsDiff {
                let mut result: Self::AbsDiff = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].abs_diff(&other[i])
                }
                result
            }

            #[inline]
            fn ulps_diff(&self, other: &Self) -> Self::UlpsDiff {
                let mut result: Self::UlpsDiff = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].ulps_diff(&other[i])
                }
                result
            }
        }

        #[doc(hidden)]
        impl<T: FloatEq> FloatEq for [T; $n] {
            type DiffEpsilon = T::DiffEpsilon;
            type UlpsDiffEpsilon = T::UlpsDiffEpsilon;

            #[inline]
            fn eq_abs(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_abs(&other[i], max_diff) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_rel(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_rel(&other[i], max_diff) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_ulps(&other[i], max_diff) {
                        return false;
                    }
                }
                true
            }
        }

        #[doc(hidden)]
        impl<T: FloatEqDebug> FloatEqDebug for [T; $n] {
            type DebugEpsilon = [<T as FloatEqDebug>::DebugEpsilon; $n];
            type DebugUlpsEpsilon = [<T as FloatEqDebug>::DebugUlpsEpsilon; $n];

            fn debug_abs_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEq>::DiffEpsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_abs_epsilon(&other[i], max_diff)
                }
                result
            }

            fn debug_rel_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEq>::DiffEpsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_rel_epsilon(&other[i], max_diff)
                }
                result
            }

            fn debug_ulps_epsilon(
                &self,
                other: &Self,
                max_diff: &<Self as FloatEq>::UlpsDiffEpsilon,
            ) -> Self::DebugUlpsEpsilon {
                let mut result: Self::DebugUlpsEpsilon =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_ulps_epsilon(&other[i], max_diff)
                }
                result
            }
        }
    };
}

// 0 to 32 as per primitive array traits
//TODO: Use const generics once they're stable
/// This is also implemented on other arrays up to size 32 (inclusive).
impl<T: FloatDiff> FloatDiff for [T; 0] {
    type AbsDiff = [<T as crate::FloatDiff>::AbsDiff; 0];
    type UlpsDiff = [<T as crate::FloatDiff>::UlpsDiff; 0];

    #[inline]
    fn abs_diff(&self, _other: &Self) -> Self::AbsDiff {
        []
    }

    #[inline]
    fn ulps_diff(&self, _other: &Self) -> Self::UlpsDiff {
        []
    }
}

/// This is also implemented on other arrays up to size 32 (inclusive).
impl<T: FloatEq> FloatEq for [T; 0] {
    type DiffEpsilon = T::DiffEpsilon;
    type UlpsDiffEpsilon = T::UlpsDiffEpsilon;

    #[inline]
    fn eq_abs(&self, _other: &Self, _max_diff: &Self::DiffEpsilon) -> bool {
        true
    }

    #[inline]
    fn eq_rel(&self, _other: &Self, _max_diff: &Self::DiffEpsilon) -> bool {
        true
    }

    #[inline]
    fn eq_ulps(&self, _other: &Self, _max_diff: &Self::UlpsDiffEpsilon) -> bool {
        true
    }
}

/// This is also implemented on other arrays up to size 32 (inclusive).
impl<T: FloatEqDebug> FloatEqDebug for [T; 0] {
    type DebugEpsilon = [<T as FloatEqDebug>::DebugEpsilon; 0];
    type DebugUlpsEpsilon = [<T as FloatEqDebug>::DebugUlpsEpsilon; 0];

    fn debug_abs_epsilon(
        &self,
        _other: &Self,
        _max_diff: &<Self as FloatEq>::DiffEpsilon,
    ) -> Self::DebugEpsilon {
        []
    }

    fn debug_rel_epsilon(
        &self,
        _other: &Self,
        _max_diff: &<Self as FloatEq>::DiffEpsilon,
    ) -> Self::DebugEpsilon {
        []
    }

    fn debug_ulps_epsilon(
        &self,
        _other: &Self,
        _max_diff: &<Self as FloatEq>::UlpsDiffEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        []
    }
}

impl_float_eq_traits_for_array!(1);
impl_float_eq_traits_for_array!(2);
impl_float_eq_traits_for_array!(3);
impl_float_eq_traits_for_array!(4);
impl_float_eq_traits_for_array!(5);
impl_float_eq_traits_for_array!(6);
impl_float_eq_traits_for_array!(7);
impl_float_eq_traits_for_array!(8);
impl_float_eq_traits_for_array!(9);
impl_float_eq_traits_for_array!(10);
impl_float_eq_traits_for_array!(11);
impl_float_eq_traits_for_array!(12);
impl_float_eq_traits_for_array!(13);
impl_float_eq_traits_for_array!(14);
impl_float_eq_traits_for_array!(15);
impl_float_eq_traits_for_array!(16);
impl_float_eq_traits_for_array!(17);
impl_float_eq_traits_for_array!(18);
impl_float_eq_traits_for_array!(19);
impl_float_eq_traits_for_array!(20);
impl_float_eq_traits_for_array!(21);
impl_float_eq_traits_for_array!(22);
impl_float_eq_traits_for_array!(23);
impl_float_eq_traits_for_array!(24);
impl_float_eq_traits_for_array!(25);
impl_float_eq_traits_for_array!(26);
impl_float_eq_traits_for_array!(27);
impl_float_eq_traits_for_array!(28);
impl_float_eq_traits_for_array!(29);
impl_float_eq_traits_for_array!(30);
impl_float_eq_traits_for_array!(31);
impl_float_eq_traits_for_array!(32);

#[cfg(test)]
mod tests {
    macro_rules! impl_tests {
        ($float:ident) => {
            mod $float {
                use crate::FloatDiff;

                macro_rules! check_float_diff {
                    ($n:literal) => {{
                        let mut a: [$float; $n] = [0.; $n];
                        for i in 0..$n {
                            a[i] = (i as $float + 1.);
                        }

                        let mut b = [0.; $n];
                        for i in 0..$n {
                            b[i] = -a[i];
                        }

                        let abs_diff = a.abs_diff(&b);
                        let ulps_diff = a.ulps_diff(&b);
                        for i in 0..$n {
                            assert_eq!(abs_diff[i], a[i].abs_diff(&b[i]));
                            assert_eq!(ulps_diff[i], a[i].ulps_diff(&b[i]));
                        }
                    }};
                }

                #[test]
                fn float_diff() {
                    //TODO: Use const generics once they're stable
                    check_float_diff!(0);
                    check_float_diff!(1);
                    check_float_diff!(2);
                    check_float_diff!(3);
                    check_float_diff!(4);
                    check_float_diff!(5);
                    check_float_diff!(6);
                    check_float_diff!(7);
                    check_float_diff!(8);
                    check_float_diff!(9);
                    check_float_diff!(10);
                    check_float_diff!(11);
                    check_float_diff!(12);
                    check_float_diff!(13);
                    check_float_diff!(14);
                    check_float_diff!(15);
                    check_float_diff!(16);
                    check_float_diff!(17);
                    check_float_diff!(18);
                    check_float_diff!(19);
                    check_float_diff!(20);
                    check_float_diff!(21);
                    check_float_diff!(22);
                    check_float_diff!(23);
                    check_float_diff!(24);
                    check_float_diff!(25);
                    check_float_diff!(26);
                    check_float_diff!(27);
                    check_float_diff!(28);
                    check_float_diff!(29);
                    check_float_diff!(30);
                    check_float_diff!(31);
                    check_float_diff!(32);

                    // nested
                    let a = [[1_f32, 2.], [1., 2.]];
                    let b = [[1_f32, 2.], [-1., -2.]];
                    assert_eq!(a.abs_diff(&b), [[0., 0.], [2., 4.]]);
                }

                macro_rules! check_float_eq {
                    ($n:literal) => {{
                        let mut a: [$float; $n] = [0.; $n];
                        for i in 0..$n {
                            a[i] = (i as $float + 1.);
                        }

                        assert_float_eq!(a, a, abs <= 0.0);
                        assert_float_eq!(a, a, rel <= 0.0);
                        assert_float_eq!(a, a, ulps <= 0);

                        for i in 0..$n {
                            let mut b = a;
                            b[i] = a[i] + 0.5;
                            assert_float_eq!(a, b, abs <= 0.5);
                            assert_float_ne!(a, b, abs <= 0.0);

                            let mut b = a;
                            b[i] = $float::from_bits(a[i].to_bits() + 1);
                            assert_float_eq!(a, b, rel <= core::$float::EPSILON);
                            assert_float_ne!(a, b, rel <= 0.0);
                            assert_float_eq!(a, b, ulps <= 1);
                            assert_float_ne!(a, b, ulps <= 0);
                        }
                    }};
                }

                #[test]
                fn float_eq() {
                    //TODO: Use const generics once they're stable
                    check_float_eq!(0);
                    check_float_eq!(1);
                    check_float_eq!(2);
                    check_float_eq!(3);
                    check_float_eq!(4);
                    check_float_eq!(5);
                    check_float_eq!(6);
                    check_float_eq!(7);
                    check_float_eq!(8);
                    check_float_eq!(9);
                    check_float_eq!(10);
                    check_float_eq!(11);
                    check_float_eq!(12);
                    check_float_eq!(13);
                    check_float_eq!(14);
                    check_float_eq!(15);
                    check_float_eq!(16);
                    check_float_eq!(17);
                    check_float_eq!(18);
                    check_float_eq!(19);
                    check_float_eq!(20);
                    check_float_eq!(21);
                    check_float_eq!(22);
                    check_float_eq!(23);
                    check_float_eq!(24);
                    check_float_eq!(25);
                    check_float_eq!(26);
                    check_float_eq!(27);
                    check_float_eq!(28);
                    check_float_eq!(29);
                    check_float_eq!(30);
                    check_float_eq!(31);
                    check_float_eq!(32);

                    // nested
                    let a = [[1_f32, 2.], [1., 2.]];
                    let b = [[1_f32, 2.], [-1., -2.]];
                    assert_float_eq!(a, b, abs <= 5.);
                }
            }
        };
    }

    impl_tests!(f32);
    impl_tests!(f64);
}
