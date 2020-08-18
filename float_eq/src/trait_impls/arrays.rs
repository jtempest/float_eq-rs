#![allow(clippy::reversed_empty_ranges)]

use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsEpsilon, UlpsEpsilon,
};
use core::mem::MaybeUninit;

// arrays
//TODO: Should this be publically available for users to conditionally implement
// support if they need it?
macro_rules! impl_float_eq_traits_for_array {
    ($n:literal) => {
        impl<T: FloatEqUlpsEpsilon> FloatEqUlpsEpsilon for [T; $n]
        where
            UlpsEpsilon<T>: Sized,
        {
            type UlpsEpsilon = [UlpsEpsilon<T>; $n];
        }

        impl<T: FloatEqDebugUlpsDiff> FloatEqDebugUlpsDiff for [T; $n] {
            type DebugUlpsDiff = [DebugUlpsDiff<T>; $n];
        }

        impl<A, B> FloatEq<[B; $n]> for [A; $n]
        where
            A: FloatEq<B>,
            A::Epsilon: Sized,
            UlpsEpsilon<A::Epsilon>: Sized,
        {
            type Epsilon = [A::Epsilon; $n];

            #[inline]
            fn eq_abs(&self, other: &[B; $n], max_diff: &Self::Epsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_abs(&other[i], &max_diff[i]) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_rmax(&self, other: &[B; $n], max_diff: &Self::Epsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_rmax(&other[i], &max_diff[i]) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_rmin(&self, other: &[B; $n], max_diff: &Self::Epsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_rmin(&other[i], &max_diff[i]) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_r1st(&self, other: &[B; $n], max_diff: &Self::Epsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_r1st(&other[i], &max_diff[i]) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_r2nd(&self, other: &[B; $n], max_diff: &Self::Epsilon) -> bool {
                for i in 0..$n {
                    if !self[i].eq_r2nd(&other[i], &max_diff[i]) {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn eq_ulps(&self, other: &[B; $n], max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
                for i in 0..$n {
                    if !self[i].eq_ulps(&other[i], &max_diff[i]) {
                        return false;
                    }
                }
                true
            }
        }

        impl<A, B> FloatEqAll<[B; $n]> for [A; $n]
        where
            A: FloatEqAll<B>,
        {
            type AllEpsilon = A::AllEpsilon;

            #[inline]
            fn eq_abs_all(&self, other: &[B; $n], max_diff: &Self::AllEpsilon) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.eq_abs_all(b, max_diff))
            }

            #[inline]
            fn eq_rmax_all(&self, other: &[B; $n], max_diff: &Self::AllEpsilon) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.eq_rmax_all(b, max_diff))
            }

            #[inline]
            fn eq_rmin_all(&self, other: &[B; $n], max_diff: &Self::AllEpsilon) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.eq_rmin_all(b, max_diff))
            }

            #[inline]
            fn eq_r1st_all(&self, other: &[B; $n], max_diff: &Self::AllEpsilon) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.eq_r1st_all(b, max_diff))
            }

            #[inline]
            fn eq_r2nd_all(&self, other: &[B; $n], max_diff: &Self::AllEpsilon) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.eq_r2nd_all(b, max_diff))
            }

            #[inline]
            fn eq_ulps_all(
                &self,
                other: &[B; $n],
                max_diff: &UlpsEpsilon<Self::AllEpsilon>,
            ) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.eq_ulps_all(b, max_diff))
            }
        }

        impl<A, B> AssertFloatEq<[B; $n]> for [A; $n]
        where
            A: AssertFloatEq<B>,
            A::Epsilon: Sized,
            A::DebugEpsilon: Sized,
            UlpsEpsilon<A::Epsilon>: Sized,
            UlpsEpsilon<A::DebugEpsilon>: Sized,
        {
            type DebugAbsDiff = [A::DebugAbsDiff; $n];
            type DebugEpsilon = [A::DebugEpsilon; $n];

            #[inline]
            fn debug_abs_diff(&self, other: &[B; $n]) -> Self::DebugAbsDiff {
                let mut result: Self::DebugAbsDiff = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_abs_diff(&other[i])
                }
                result
            }

            #[inline]
            fn debug_ulps_diff(&self, other: &[B; $n]) -> DebugUlpsDiff<Self::DebugAbsDiff> {
                let mut result: DebugUlpsDiff<Self::DebugAbsDiff> =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_ulps_diff(&other[i])
                }
                result
            }

            #[inline]
            fn debug_abs_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_abs_epsilon(&other[i], &max_diff[i])
                }
                result
            }

            #[inline]
            fn debug_rmax_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_rmax_epsilon(&other[i], &max_diff[i])
                }
                result
            }

            #[inline]
            fn debug_rmin_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_rmin_epsilon(&other[i], &max_diff[i])
                }
                result
            }

            #[inline]
            fn debug_r1st_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_r1st_epsilon(&other[i], &max_diff[i])
                }
                result
            }

            #[inline]
            fn debug_r2nd_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_r2nd_epsilon(&other[i], &max_diff[i])
                }
                result
            }

            #[inline]
            fn debug_ulps_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &UlpsEpsilon<Self::Epsilon>,
            ) -> UlpsEpsilon<Self::DebugEpsilon> {
                let mut result: UlpsEpsilon<Self::DebugEpsilon> =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_ulps_epsilon(&other[i], &max_diff[i])
                }
                result
            }
        }

        impl<A, B> AssertFloatEqAll<[B; $n]> for [A; $n]
        where
            A: AssertFloatEqAll<B>,
            UlpsEpsilon<A::AllDebugEpsilon>: Sized,
        {
            type AllDebugEpsilon = [A::AllDebugEpsilon; $n];

            #[inline]
            fn debug_abs_all_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                let mut result: Self::AllDebugEpsilon =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_abs_all_epsilon(&other[i], max_diff)
                }
                result
            }

            #[inline]
            fn debug_rmax_all_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                let mut result: Self::AllDebugEpsilon =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_rmax_all_epsilon(&other[i], max_diff)
                }
                result
            }

            #[inline]
            fn debug_rmin_all_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                let mut result: Self::AllDebugEpsilon =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_rmin_all_epsilon(&other[i], max_diff)
                }
                result
            }

            #[inline]
            fn debug_r1st_all_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                let mut result: Self::AllDebugEpsilon =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_r1st_all_epsilon(&other[i], max_diff)
                }
                result
            }

            #[inline]
            fn debug_r2nd_all_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                let mut result: Self::AllDebugEpsilon =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..$n {
                    result[i] = self[i].debug_r2nd_all_epsilon(&other[i], max_diff)
                }
                result
            }

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &[B; $n],
                max_diff: &UlpsEpsilon<Self::AllEpsilon>,
            ) -> UlpsEpsilon<Self::AllDebugEpsilon> {
                let mut result: UlpsEpsilon<Self::AllDebugEpsilon> =
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
