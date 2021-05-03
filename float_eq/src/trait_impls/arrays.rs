use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsTol, UlpsTol,
};
use core::mem::MaybeUninit;

// Uses the same technique as MaybeUninit::uninit_array.
#[inline(always)]
fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
    unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
}

// Uses the same technique as MaybeUninit::array_assume_init.
#[inline(always)]
unsafe fn array_assume_init<T, const N: usize>(array: [MaybeUninit<T>; N]) -> [T; N] {
    (&array as *const _ as *const [T; N]).read()
}

impl<T: FloatEqUlpsTol, const N: usize> FloatEqUlpsTol for [T; N]
where
    UlpsTol<T>: Sized,
{
    type UlpsTol = [UlpsTol<T>; N];
}

impl<T: FloatEqDebugUlpsDiff, const N: usize> FloatEqDebugUlpsDiff for [T; N] {
    type DebugUlpsDiff = [DebugUlpsDiff<T>; N];
}

impl<A, B, const N: usize> FloatEq<[B; N]> for [A; N]
where
    A: FloatEq<B>,
    A::Tol: Sized,
    UlpsTol<A::Tol>: Sized,
{
    type Tol = [A::Tol; N];

    #[inline]
    fn eq_abs(&self, other: &[B; N], tol: &Self::Tol) -> bool {
        for i in 0..N {
            if !self[i].eq_abs(&other[i], &tol[i]) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn eq_rmax(&self, other: &[B; N], tol: &Self::Tol) -> bool {
        for i in 0..N {
            if !self[i].eq_rmax(&other[i], &tol[i]) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn eq_rmin(&self, other: &[B; N], tol: &Self::Tol) -> bool {
        for i in 0..N {
            if !self[i].eq_rmin(&other[i], &tol[i]) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn eq_r1st(&self, other: &[B; N], tol: &Self::Tol) -> bool {
        for i in 0..N {
            if !self[i].eq_r1st(&other[i], &tol[i]) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn eq_r2nd(&self, other: &[B; N], tol: &Self::Tol) -> bool {
        for i in 0..N {
            if !self[i].eq_r2nd(&other[i], &tol[i]) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn eq_ulps(&self, other: &[B; N], tol: &UlpsTol<Self::Tol>) -> bool {
        for i in 0..N {
            if !self[i].eq_ulps(&other[i], &tol[i]) {
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
    type AllTol = A::AllTol;

    #[inline]
    fn eq_abs_all(&self, other: &[B; N], tol: &Self::AllTol) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_abs_all(b, tol))
    }

    #[inline]
    fn eq_rmax_all(&self, other: &[B; N], tol: &Self::AllTol) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_rmax_all(b, tol))
    }

    #[inline]
    fn eq_rmin_all(&self, other: &[B; N], tol: &Self::AllTol) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_rmin_all(b, tol))
    }

    #[inline]
    fn eq_r1st_all(&self, other: &[B; N], tol: &Self::AllTol) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_r1st_all(b, tol))
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &[B; N], tol: &Self::AllTol) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_r2nd_all(b, tol))
    }

    #[inline]
    fn eq_ulps_all(&self, other: &[B; N], tol: &UlpsTol<Self::AllTol>) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.eq_ulps_all(b, tol))
    }
}

impl<A, B, const N: usize> AssertFloatEq<[B; N]> for [A; N]
where
    A: AssertFloatEq<B>,
    A::Tol: Sized,
    A::DebugTol: Sized,
    UlpsTol<A::Tol>: Sized,
    UlpsTol<A::DebugTol>: Sized,
{
    type DebugAbsDiff = [A::DebugAbsDiff; N];
    type DebugTol = [A::DebugTol; N];

    #[inline]
    fn debug_abs_diff(&self, other: &[B; N]) -> Self::DebugAbsDiff {
        let mut result: [MaybeUninit<A::DebugAbsDiff>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_abs_diff(&other[i]));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &[B; N]) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        let mut result: [MaybeUninit<DebugUlpsDiff<A::DebugAbsDiff>>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_ulps_diff(&other[i]));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_abs_tol(&self, other: &[B; N], tol: &Self::Tol) -> Self::DebugTol {
        let mut result: [MaybeUninit<A::DebugTol>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_abs_tol(&other[i], &tol[i]));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_rmax_tol(&self, other: &[B; N], tol: &Self::Tol) -> Self::DebugTol {
        let mut result: [MaybeUninit<A::DebugTol>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_rmax_tol(&other[i], &tol[i]));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_rmin_tol(&self, other: &[B; N], tol: &Self::Tol) -> Self::DebugTol {
        let mut result: [MaybeUninit<A::DebugTol>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_rmin_tol(&other[i], &tol[i]));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_r1st_tol(&self, other: &[B; N], tol: &Self::Tol) -> Self::DebugTol {
        let mut result: [MaybeUninit<A::DebugTol>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_r1st_tol(&other[i], &tol[i]));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_r2nd_tol(&self, other: &[B; N], tol: &Self::Tol) -> Self::DebugTol {
        let mut result: [MaybeUninit<A::DebugTol>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_r2nd_tol(&other[i], &tol[i]));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_ulps_tol(&self, other: &[B; N], tol: &UlpsTol<Self::Tol>) -> UlpsTol<Self::DebugTol> {
        let mut result: [MaybeUninit<UlpsTol<A::DebugTol>>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_ulps_tol(&other[i], &tol[i]));
        }
        unsafe { array_assume_init(result) }
    }
}

impl<A, B, const N: usize> AssertFloatEqAll<[B; N]> for [A; N]
where
    A: AssertFloatEqAll<B>,
    UlpsTol<A::AllDebugTol>: Sized,
{
    type AllDebugTol = [A::AllDebugTol; N];

    #[inline]
    fn debug_abs_all_tol(&self, other: &[B; N], tol: &Self::AllTol) -> Self::AllDebugTol {
        let mut result: [MaybeUninit<A::AllDebugTol>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_abs_all_tol(&other[i], tol));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_rmax_all_tol(&self, other: &[B; N], tol: &Self::AllTol) -> Self::AllDebugTol {
        let mut result: [MaybeUninit<A::AllDebugTol>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_rmax_all_tol(&other[i], tol));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_rmin_all_tol(&self, other: &[B; N], tol: &Self::AllTol) -> Self::AllDebugTol {
        let mut result: [MaybeUninit<A::AllDebugTol>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_rmin_all_tol(&other[i], tol));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_r1st_all_tol(&self, other: &[B; N], tol: &Self::AllTol) -> Self::AllDebugTol {
        let mut result: [MaybeUninit<A::AllDebugTol>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_r1st_all_tol(&other[i], tol));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_r2nd_all_tol(&self, other: &[B; N], tol: &Self::AllTol) -> Self::AllDebugTol {
        let mut result: [MaybeUninit<A::AllDebugTol>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_r2nd_all_tol(&other[i], tol));
        }
        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_ulps_all_tol(
        &self,
        other: &[B; N],
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol> {
        let mut result: [MaybeUninit<UlpsTol<A::AllDebugTol>>; N] = uninit_array();
        for i in 0..N {
            result[i] = MaybeUninit::new(self[i].debug_ulps_all_tol(&other[i], tol));
        }
        unsafe { array_assume_init(result) }
    }
}
