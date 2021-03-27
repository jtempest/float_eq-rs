#![allow(clippy::reversed_empty_ranges)]

use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsEpsilon, UlpsEpsilon,
};
use core::mem::MaybeUninit;

impl<T: FloatEqUlpsEpsilon, const N: usize> FloatEqUlpsEpsilon for [T; N]
where
    UlpsEpsilon<T>: Sized,
{
    type UlpsEpsilon = [UlpsEpsilon<T>; N];
}

impl<T: FloatEqDebugUlpsDiff, const N: usize> FloatEqDebugUlpsDiff for [T; N] {
    type DebugUlpsDiff = [DebugUlpsDiff<T>; N];
}

impl<A, B, const N: usize> FloatEq<[B; N]> for [A; N]
where
    A: FloatEq<B>,
    A::Epsilon: Sized,
    UlpsEpsilon<A::Epsilon>: Sized,
{
    type Epsilon = [A::Epsilon; N];

    #[inline]
    fn eq_abs(&self, other: &[B; N], max_diff: &Self::Epsilon) -> bool {
        for i in 0..N {
            if !self[i].eq_abs(&other[i], &max_diff[i]) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn eq_rmax(&self, other: &[B; N], max_diff: &Self::Epsilon) -> bool {
        for i in 0..N {
            if !self[i].eq_rmax(&other[i], &max_diff[i]) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn eq_rmin(&self, other: &[B; N], max_diff: &Self::Epsilon) -> bool {
        for i in 0..N {
            if !self[i].eq_rmin(&other[i], &max_diff[i]) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn eq_r1st(&self, other: &[B; N], max_diff: &Self::Epsilon) -> bool {
        for i in 0..N {
            if !self[i].eq_r1st(&other[i], &max_diff[i]) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn eq_r2nd(&self, other: &[B; N], max_diff: &Self::Epsilon) -> bool {
        for i in 0..N {
            if !self[i].eq_r2nd(&other[i], &max_diff[i]) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn eq_ulps(&self, other: &[B; N], max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
        for i in 0..N {
            if !self[i].eq_ulps(&other[i], &max_diff[i]) {
                return false;
            }
        }
        true
    }
}

impl<A, B, const N: usize> FloatEqAll<[B; N]> for [A; N]
where
    A: FloatEqAll<B>,
{
    type AllEpsilon = A::AllEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &[B; N], max_diff: &Self::AllEpsilon) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_abs_all(b, max_diff))
    }

    #[inline]
    fn eq_rmax_all(&self, other: &[B; N], max_diff: &Self::AllEpsilon) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_rmax_all(b, max_diff))
    }

    #[inline]
    fn eq_rmin_all(&self, other: &[B; N], max_diff: &Self::AllEpsilon) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_rmin_all(b, max_diff))
    }

    #[inline]
    fn eq_r1st_all(&self, other: &[B; N], max_diff: &Self::AllEpsilon) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_r1st_all(b, max_diff))
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &[B; N], max_diff: &Self::AllEpsilon) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_r2nd_all(b, max_diff))
    }

    #[inline]
    fn eq_ulps_all(&self, other: &[B; N], max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_ulps_all(b, max_diff))
    }
}

impl<A, B, const N: usize> AssertFloatEq<[B; N]> for [A; N]
where
    A: AssertFloatEq<B>,
    A::Epsilon: Sized,
    A::DebugEpsilon: Sized,
    UlpsEpsilon<A::Epsilon>: Sized,
    UlpsEpsilon<A::DebugEpsilon>: Sized,
{
    type DebugAbsDiff = [A::DebugAbsDiff; N];
    type DebugEpsilon = [A::DebugEpsilon; N];

    #[inline]
    fn debug_abs_diff(&self, other: &[B; N]) -> Self::DebugAbsDiff {
        let mut result: Self::DebugAbsDiff = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_abs_diff(&other[i])
        }
        result
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &[B; N]) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        let mut result: DebugUlpsDiff<Self::DebugAbsDiff> =
            unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_ulps_diff(&other[i])
        }
        result
    }

    #[inline]
    fn debug_abs_epsilon(&self, other: &[B; N], max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_abs_epsilon(&other[i], &max_diff[i])
        }
        result
    }

    #[inline]
    fn debug_rmax_epsilon(&self, other: &[B; N], max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_rmax_epsilon(&other[i], &max_diff[i])
        }
        result
    }

    #[inline]
    fn debug_rmin_epsilon(&self, other: &[B; N], max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_rmin_epsilon(&other[i], &max_diff[i])
        }
        result
    }

    #[inline]
    fn debug_r1st_epsilon(&self, other: &[B; N], max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_r1st_epsilon(&other[i], &max_diff[i])
        }
        result
    }

    #[inline]
    fn debug_r2nd_epsilon(&self, other: &[B; N], max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        let mut result: Self::DebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_r2nd_epsilon(&other[i], &max_diff[i])
        }
        result
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &[B; N],
        max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon> {
        let mut result: UlpsEpsilon<Self::DebugEpsilon> =
            unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_ulps_epsilon(&other[i], &max_diff[i])
        }
        result
    }
}

impl<A, B, const N: usize> AssertFloatEqAll<[B; N]> for [A; N]
where
    A: AssertFloatEqAll<B>,
    UlpsEpsilon<A::AllDebugEpsilon>: Sized,
{
    type AllDebugEpsilon = [A::AllDebugEpsilon; N];

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &[B; N],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        let mut result: Self::AllDebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_abs_all_epsilon(&other[i], max_diff)
        }
        result
    }

    #[inline]
    fn debug_rmax_all_epsilon(
        &self,
        other: &[B; N],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        let mut result: Self::AllDebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_rmax_all_epsilon(&other[i], max_diff)
        }
        result
    }

    #[inline]
    fn debug_rmin_all_epsilon(
        &self,
        other: &[B; N],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        let mut result: Self::AllDebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_rmin_all_epsilon(&other[i], max_diff)
        }
        result
    }

    #[inline]
    fn debug_r1st_all_epsilon(
        &self,
        other: &[B; N],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        let mut result: Self::AllDebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_r1st_all_epsilon(&other[i], max_diff)
        }
        result
    }

    #[inline]
    fn debug_r2nd_all_epsilon(
        &self,
        other: &[B; N],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        let mut result: Self::AllDebugEpsilon = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_r2nd_all_epsilon(&other[i], max_diff)
        }
        result
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &[B; N],
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon> {
        let mut result: UlpsEpsilon<Self::AllDebugEpsilon> =
            unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = self[i].debug_ulps_all_epsilon(&other[i], max_diff)
        }
        result
    }
}
