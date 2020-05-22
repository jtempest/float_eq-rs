use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug};
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
            type Epsilon = [T::Epsilon; $n];
            type UlpsEpsilon = [T::UlpsEpsilon; $n];

            #[inline]
            fn eq_abs(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_abs(&other[i], &max_diff[i]) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_rel(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_rel(&other[i], &max_diff[i]) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsEpsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_ulps(&other[i], &max_diff[i]) {
                        return false;
                    }
                }
                true
            }
        }

        #[doc(hidden)]
        impl<T: FloatEqAll> FloatEqAll for [T; $n] {
            type Epsilon = T::Epsilon;
            type UlpsEpsilon = T::UlpsEpsilon;

            #[inline]
            fn eq_abs_all(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.eq_abs_all(b, max_diff))
            }

            #[inline]
            fn eq_rel_all(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.eq_rel_all(b, max_diff))
            }

            #[inline]
            fn eq_ulps_all(&self, other: &Self, max_diff: &Self::UlpsEpsilon) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.eq_ulps_all(b, max_diff))
            }
        }

        #[doc(hidden)]
        impl<T: FloatEqDebug> FloatEqDebug for [T; $n] {
            type DebugEpsilon = [T::DebugEpsilon; $n];
            type DebugUlpsEpsilon = [T::DebugUlpsEpsilon; $n];

            fn debug_abs_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_abs_epsilon(&other[i], &max_diff[i])
                }
                result
            }

            fn debug_rel_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_rel_epsilon(&other[i], &max_diff[i])
                }
                result
            }

            fn debug_ulps_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::UlpsEpsilon,
            ) -> Self::DebugUlpsEpsilon {
                let mut result: Self::DebugUlpsEpsilon =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_ulps_epsilon(&other[i], &max_diff[i])
                }
                result
            }
        }

        #[doc(hidden)]
        impl<T: FloatEqAllDebug> FloatEqAllDebug for [T; $n] {
            type DebugEpsilon = [T::DebugEpsilon; $n];
            type DebugUlpsEpsilon = [T::DebugUlpsEpsilon; $n];

            fn debug_abs_all_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_abs_all_epsilon(&other[i], max_diff)
                }
                result
            }

            fn debug_rel_all_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_rel_all_epsilon(&other[i], max_diff)
                }
                result
            }

            fn debug_ulps_all_epsilon(
                &self,
                other: &Self,
                max_diff: &Self::UlpsEpsilon,
            ) -> Self::DebugUlpsEpsilon {
                let mut result: Self::DebugUlpsEpsilon =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_ulps_all_epsilon(&other[i], max_diff)
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
    type Epsilon = [T::Epsilon; 0];
    type UlpsEpsilon = [T::UlpsEpsilon; 0];

    #[inline]
    fn eq_abs(&self, _other: &Self, _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_rel(&self, _other: &Self, _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_ulps(&self, _other: &Self, _max_diff: &Self::UlpsEpsilon) -> bool {
        true
    }
}

/// This is also implemented on other arrays up to size 32 (inclusive).
impl<T: FloatEqAll> FloatEqAll for [T; 0] {
    type Epsilon = T::Epsilon;
    type UlpsEpsilon = T::UlpsEpsilon;

    #[inline]
    fn eq_abs_all(&self, _other: &Self, _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_rel_all(&self, _other: &Self, _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_ulps_all(&self, _other: &Self, _max_diff: &Self::UlpsEpsilon) -> bool {
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
        _max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        []
    }

    fn debug_rel_epsilon(
        &self,
        _other: &Self,
        _max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        []
    }

    fn debug_ulps_epsilon(
        &self,
        _other: &Self,
        _max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        []
    }
}

/// This is also implemented on other arrays up to size 32 (inclusive).
impl<T: FloatEqAllDebug> FloatEqAllDebug for [T; 0] {
    type DebugEpsilon = [<T as FloatEqAllDebug>::DebugEpsilon; 0];
    type DebugUlpsEpsilon = [<T as FloatEqAllDebug>::DebugUlpsEpsilon; 0];

    fn debug_abs_all_epsilon(
        &self,
        _other: &Self,
        _max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        []
    }

    fn debug_rel_all_epsilon(
        &self,
        _other: &Self,
        _max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        []
    }

    fn debug_ulps_all_epsilon(
        &self,
        _other: &Self,
        _max_diff: &Self::UlpsEpsilon,
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
    #![allow(clippy::float_cmp)]

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
                    //we can infer the checks in between work
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

                        assert_float_eq!(a, a, abs <= [0.0; $n]);
                        assert_float_eq!(a, a, rel <= [0.0; $n]);
                        assert_float_eq!(a, a, ulps <= [0; $n]);

                        for i in 0..$n {
                            let mut b = a;
                            b[i] = a[i] + 0.5;

                            let mut eps = [0.0; $n];
                            assert_float_ne!(a, b, abs <= eps);
                            eps[i] = 0.5;
                            assert_float_eq!(a, b, abs <= eps);

                            let mut b = a;
                            b[i] = $float::from_bits(a[i].to_bits() + 1);

                            let mut eps = [0.0; $n];
                            assert_float_ne!(a, b, rel <= eps);
                            eps[i] = core::$float::EPSILON;
                            assert_float_eq!(a, b, rel <= eps);

                            let mut eps = [0; $n];
                            assert_float_ne!(a, b, ulps <= eps);
                            eps[i] = 1;
                            assert_float_eq!(a, b, ulps <= eps);
                        }
                    }};
                }

                macro_rules! check_float_eq_all {
                    ($n:literal) => {{
                        let mut a: [$float; $n] = [0.; $n];
                        for i in 0..$n {
                            a[i] = (i as $float + 1.);
                        }

                        assert_float_eq!(a, a, abs_all <= 0.0);
                        assert_float_eq!(a, a, rel_all <= 0.0);
                        assert_float_eq!(a, a, ulps_all <= 0);

                        for i in 0..$n {
                            let mut b = a;
                            b[i] = a[i] + 0.5;
                            assert_float_eq!(a, b, abs_all <= 0.5);
                            assert_float_ne!(a, b, abs_all <= 0.0);

                            let mut b = a;
                            b[i] = $float::from_bits(a[i].to_bits() + 1);
                            assert_float_eq!(a, b, rel_all <= core::$float::EPSILON);
                            assert_float_ne!(a, b, rel_all <= 0.0);
                            assert_float_eq!(a, b, ulps_all <= 1);
                            assert_float_ne!(a, b, ulps_all <= 0);
                        }
                    }};
                }

                macro_rules! check_float_eq_macros {
                    ($n:literal) => {
                        check_float_eq!($n);
                        check_float_eq_all!($n);
                    };
                }

                #[test]
                fn float_eq() {
                    //TODO: Use const generics once they're stable
                    check_float_eq_macros!(0);
                    check_float_eq_macros!(1);
                    check_float_eq_macros!(2);
                    //we can infer the checks in between work
                    check_float_eq_macros!(32);

                    // nested
                    let a = [[1_f32, 2.], [1., -2.]];
                    let b = [[1_f32, 3.], [-1., 2.]];
                    assert_float_eq!(a, b, abs <= [[0., 1.], [2., 4.]]);
                    assert_float_eq!(a, b, abs_all <= 4.);
                }
            }
        };
    }

    impl_tests!(f32);
    impl_tests!(f64);
}
