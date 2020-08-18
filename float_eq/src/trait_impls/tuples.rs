use crate::{
    AssertFloatEq, DebugUlpsDiff, FloatEq, FloatEqDebugUlpsDiff, FloatEqUlpsEpsilon, UlpsEpsilon,
};
use core::fmt;

impl FloatEqUlpsEpsilon for () {
    type UlpsEpsilon = ();
}

impl FloatEqDebugUlpsDiff for () {
    type DebugUlpsDiff = ();
}

impl FloatEq for () {
    type Epsilon = ();

    #[inline]
    fn eq_abs(&self, _other: &(), _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_rmax(&self, _other: &(), _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_rmin(&self, _other: &(), _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_r1st(&self, _other: &(), _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_r2nd(&self, _other: &(), _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_ulps(&self, _other: &(), _max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
        true
    }
}

impl AssertFloatEq for () {
    type DebugAbsDiff = ();
    type DebugEpsilon = ();

    #[inline]
    fn debug_abs_diff(&self, _other: &()) -> Self::DebugAbsDiff {}

    #[inline]
    fn debug_ulps_diff(&self, _other: &()) -> DebugUlpsDiff<Self::DebugAbsDiff> {}

    #[inline]
    fn debug_abs_epsilon(&self, _other: &(), _max_diff: &Self::Epsilon) -> Self::DebugEpsilon {}

    #[inline]
    fn debug_rmax_epsilon(&self, _other: &(), _max_diff: &Self::Epsilon) -> Self::DebugEpsilon {}

    #[inline]
    fn debug_rmin_epsilon(&self, _other: &(), _max_diff: &Self::Epsilon) -> Self::DebugEpsilon {}

    #[inline]
    fn debug_r1st_epsilon(&self, _other: &(), _max_diff: &Self::Epsilon) -> Self::DebugEpsilon {}

    #[inline]
    fn debug_r2nd_epsilon(&self, _other: &(), _max_diff: &Self::Epsilon) -> Self::DebugEpsilon {}

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        _other: &(),
        _max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon> {
    }
}

// Non-unit type tuple impls, as for std PartialEq implementation
macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T:FloatEqUlpsEpsilon),+> FloatEqUlpsEpsilon for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized,
                $(UlpsEpsilon<$T>: Sized,)+
            {
                type UlpsEpsilon = ($(UlpsEpsilon<$T>,)+);
            }

            impl<$($T:FloatEqDebugUlpsDiff),+> FloatEqDebugUlpsDiff for ($($T,)+)
            {
                type DebugUlpsDiff = ($(DebugUlpsDiff<$T>,)+);
            }

            impl<$($T:FloatEq),+> FloatEq for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized,
                $($T::Epsilon: Sized,)+
                $(UlpsEpsilon<$T::Epsilon>: Sized,)+
            {
                type Epsilon = ($($T::Epsilon,)+);

                #[inline]
                fn eq_abs(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                    $(self.$idx.eq_abs(&other.$idx, &max_diff.$idx))&&+
                }

                #[inline]
                fn eq_rmax(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                    $(self.$idx.eq_rmax(&other.$idx, &max_diff.$idx))&&+
                }

                #[inline]
                fn eq_rmin(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                    $(self.$idx.eq_rmin(&other.$idx, &max_diff.$idx))&&+
                }

                #[inline]
                fn eq_r1st(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                    $(self.$idx.eq_r1st(&other.$idx, &max_diff.$idx))&&+
                }

                #[inline]
                fn eq_r2nd(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                    $(self.$idx.eq_r2nd(&other.$idx, &max_diff.$idx))&&+
                }

                #[inline]
                fn eq_ulps(&self, other: &Self, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
                    $(self.$idx.eq_ulps(&other.$idx, &max_diff.$idx))&&+
                }
            }

            impl<$($T:AssertFloatEq + fmt::Debug),+> AssertFloatEq for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized,
                $($T::Epsilon: Sized,)+
                $($T::DebugEpsilon: Sized,)+
                $(UlpsEpsilon<$T::Epsilon>: Sized,)+
                $(UlpsEpsilon<$T::DebugEpsilon>: Sized,)+
            {
                type DebugAbsDiff = ($($T::DebugAbsDiff,)+);
                type DebugEpsilon = ($($T::DebugEpsilon,)+);

                #[inline]
                fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
                    ($(self.$idx.debug_abs_diff(&other.$idx),)+)
                }

                #[inline]
                fn debug_ulps_diff(&self, other: &Self) -> DebugUlpsDiff<Self::DebugAbsDiff> {
                    ($(self.$idx.debug_ulps_diff(&other.$idx),)+)
                }

                #[inline]
                fn debug_abs_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                    ($(self.$idx.debug_abs_epsilon(&other.$idx, &max_diff.$idx),)+)
                }

                #[inline]
                fn debug_rmax_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                    ($(self.$idx.debug_rmax_epsilon(&other.$idx, &max_diff.$idx),)+)
                }

                #[inline]
                fn debug_rmin_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                    ($(self.$idx.debug_rmin_epsilon(&other.$idx, &max_diff.$idx),)+)
                }

                #[inline]
                fn debug_r1st_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                    ($(self.$idx.debug_r1st_epsilon(&other.$idx, &max_diff.$idx),)+)
                }

                #[inline]
                fn debug_r2nd_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                    ($(self.$idx.debug_r2nd_epsilon(&other.$idx, &max_diff.$idx),)+)
                }

                #[inline]
                fn debug_ulps_epsilon(&self, other: &Self, max_diff: &UlpsEpsilon<Self::Epsilon>) -> UlpsEpsilon<Self::DebugEpsilon> {
                    ($(self.$idx.debug_ulps_epsilon(&other.$idx, &max_diff.$idx),)+)
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
