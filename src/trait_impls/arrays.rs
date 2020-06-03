use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug, FloatUlps, Ulps};
use core::mem::MaybeUninit;

// arrays
//TODO: Should this be publically available for users to conditionally implement
// support if they need it?
macro_rules! impl_float_eq_traits_for_array {
    ($n:literal) => {
        impl<T: FloatUlps> FloatUlps for [T; $n] {
            type Ulps = [Ulps<T>; $n];
        }

        impl<T: FloatDiff> FloatDiff for [T; $n] {
            type Output = [<T as $crate::FloatDiff>::Output; $n];

            #[inline]
            fn abs_diff(&self, other: &Self) -> Self::Output {
                let mut result: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].abs_diff(&other[i])
                }
                result
            }

            #[inline]
            fn ulps_diff(&self, other: &Self) -> Option<Ulps<Self::Output>> {
                let mut result: Ulps<Self::Output> = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].ulps_diff(&other[i])?
                }
                Some(result)
            }
        }

        impl<T: FloatEq> FloatEq for [T; $n] {
            type Epsilon = [T::Epsilon; $n];

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
            fn eq_ulps(&self, other: &Self, max_diff: &Ulps<Self::Epsilon>) -> bool {
                for i in 0..$n {
                    if !self[i].eq_ulps(&other[i], &max_diff[i]) {
                        return false;
                    }
                }
                true
            }
        }

        impl<T: FloatEqAll> FloatEqAll for [T; $n] {
            type Epsilon = T::Epsilon;

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
            fn eq_ulps_all(&self, other: &Self, max_diff: &Ulps<Self::Epsilon>) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.eq_ulps_all(b, max_diff))
            }
        }

        impl<T: FloatEqDebug> FloatEqDebug for [T; $n] {
            type DebugEpsilon = [T::DebugEpsilon; $n];

            #[inline]
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

            #[inline]
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

            #[inline]
            fn debug_ulps_epsilon(
                &self,
                other: &Self,
                max_diff: &Ulps<Self::Epsilon>,
            ) -> Ulps<Self::DebugEpsilon> {
                let mut result: Ulps<Self::DebugEpsilon> =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_ulps_epsilon(&other[i], &max_diff[i])
                }
                result
            }
        }

        impl<T: FloatEqAllDebug> FloatEqAllDebug for [T; $n] {
            type DebugEpsilon = [T::DebugEpsilon; $n];

            #[inline]
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

            #[inline]
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

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &Self,
                max_diff: &Ulps<Self::Epsilon>,
            ) -> Ulps<Self::DebugEpsilon> {
                let mut result: Ulps<Self::DebugEpsilon> =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_ulps_all_epsilon(&other[i], max_diff)
                }
                result
            }
        }
    };
}

impl_float_eq_traits_for_array!(0);
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
