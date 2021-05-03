use crate::{AssertFloatEq, DebugUlpsDiff, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsTol, UlpsTol};
use core::fmt;

impl FloatEqUlpsTol for () {
    type UlpsTol = ();
}

impl FloatEqDebugUlpsDiff for () {
    type DebugUlpsDiff = ();
}

impl FloatEq for () {
    type Tol = ();

    #[inline]
    fn eq_abs(&self, _other: &(), _tol: &Self::Tol) -> bool {
        true
    }

    #[inline]
    fn eq_rmax(&self, _other: &(), _tol: &Self::Tol) -> bool {
        true
    }

    #[inline]
    fn eq_rmin(&self, _other: &(), _tol: &Self::Tol) -> bool {
        true
    }

    #[inline]
    fn eq_r1st(&self, _other: &(), _tol: &Self::Tol) -> bool {
        true
    }

    #[inline]
    fn eq_r2nd(&self, _other: &(), _tol: &Self::Tol) -> bool {
        true
    }

    #[inline]
    fn eq_ulps(&self, _other: &(), _tol: &UlpsTol<Self::Tol>) -> bool {
        true
    }
}

impl AssertFloatEq for () {
    type DebugAbsDiff = ();
    type DebugTol = ();

    #[inline]
    fn debug_abs_diff(&self, _other: &()) -> Self::DebugAbsDiff {}

    #[inline]
    fn debug_ulps_diff(&self, _other: &()) -> DebugUlpsDiff<Self::DebugAbsDiff> {}

    #[inline]
    fn debug_abs_tol(&self, _other: &(), _tol: &Self::Tol) -> Self::DebugTol {}

    #[inline]
    fn debug_rmax_tol(&self, _other: &(), _tol: &Self::Tol) -> Self::DebugTol {}

    #[inline]
    fn debug_rmin_tol(&self, _other: &(), _tol: &Self::Tol) -> Self::DebugTol {}

    #[inline]
    fn debug_r1st_tol(&self, _other: &(), _tol: &Self::Tol) -> Self::DebugTol {}

    #[inline]
    fn debug_r2nd_tol(&self, _other: &(), _tol: &Self::Tol) -> Self::DebugTol {}

    #[inline]
    fn debug_ulps_tol(&self, _other: &(), _tol: &UlpsTol<Self::Tol>) -> UlpsTol<Self::DebugTol> {}
}

// Non-unit type tuple impls, as for std PartialEq implementation
macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T:FloatEqUlpsTol),+> FloatEqUlpsTol for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized,
                $(UlpsTol<$T>: Sized,)+
            {
                type UlpsTol = ($(UlpsTol<$T>,)+);
            }

            impl<$($T:FloatEqDebugUlpsDiff),+> FloatEqDebugUlpsDiff for ($($T,)+)
            {
                type DebugUlpsDiff = ($(DebugUlpsDiff<$T>,)+);
            }

            impl<$($T:FloatEq),+> FloatEq for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized,
                $($T::Tol: Sized,)+
                $(UlpsTol<$T::Tol>: Sized,)+
            {
                type Tol = ($($T::Tol,)+);

                #[inline]
                fn eq_abs(&self, other: &Self, tol: &Self::Tol) -> bool {
                    $(self.$idx.eq_abs(&other.$idx, &tol.$idx))&&+
                }

                #[inline]
                fn eq_rmax(&self, other: &Self, tol: &Self::Tol) -> bool {
                    $(self.$idx.eq_rmax(&other.$idx, &tol.$idx))&&+
                }

                #[inline]
                fn eq_rmin(&self, other: &Self, tol: &Self::Tol) -> bool {
                    $(self.$idx.eq_rmin(&other.$idx, &tol.$idx))&&+
                }

                #[inline]
                fn eq_r1st(&self, other: &Self, tol: &Self::Tol) -> bool {
                    $(self.$idx.eq_r1st(&other.$idx, &tol.$idx))&&+
                }

                #[inline]
                fn eq_r2nd(&self, other: &Self, tol: &Self::Tol) -> bool {
                    $(self.$idx.eq_r2nd(&other.$idx, &tol.$idx))&&+
                }

                #[inline]
                fn eq_ulps(&self, other: &Self, tol: &UlpsTol<Self::Tol>) -> bool {
                    $(self.$idx.eq_ulps(&other.$idx, &tol.$idx))&&+
                }
            }

            impl<$($T:AssertFloatEq + fmt::Debug),+> AssertFloatEq for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized,
                $($T::Tol: Sized,)+
                $($T::DebugTol: Sized,)+
                $(UlpsTol<$T::Tol>: Sized,)+
                $(UlpsTol<$T::DebugTol>: Sized,)+
            {
                type DebugAbsDiff = ($($T::DebugAbsDiff,)+);
                type DebugTol = ($($T::DebugTol,)+);

                #[inline]
                fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
                    ($(self.$idx.debug_abs_diff(&other.$idx),)+)
                }

                #[inline]
                fn debug_ulps_diff(&self, other: &Self) -> DebugUlpsDiff<Self::DebugAbsDiff> {
                    ($(self.$idx.debug_ulps_diff(&other.$idx),)+)
                }

                #[inline]
                fn debug_abs_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                    ($(self.$idx.debug_abs_tol(&other.$idx, &tol.$idx),)+)
                }

                #[inline]
                fn debug_rmax_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                    ($(self.$idx.debug_rmax_tol(&other.$idx, &tol.$idx),)+)
                }

                #[inline]
                fn debug_rmin_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                    ($(self.$idx.debug_rmin_tol(&other.$idx, &tol.$idx),)+)
                }

                #[inline]
                fn debug_r1st_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                    ($(self.$idx.debug_r1st_tol(&other.$idx, &tol.$idx),)+)
                }

                #[inline]
                fn debug_r2nd_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                    ($(self.$idx.debug_r2nd_tol(&other.$idx, &tol.$idx),)+)
                }

                #[inline]
                fn debug_ulps_tol(&self, other: &Self, tol: &UlpsTol<Self::Tol>) -> UlpsTol<Self::DebugTol> {
                    ($(self.$idx.debug_ulps_tol(&other.$idx, &tol.$idx),)+)
                }
            }
        )+
    };
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

tuple_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}
