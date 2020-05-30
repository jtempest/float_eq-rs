use crate::{FloatDiff, FloatEq, FloatEqDebug};
use core::fmt;

impl FloatDiff for () {
    type AbsDiff = ();
    type UlpsDiff = ();

    #[inline]
    fn abs_diff(&self, _other: &()) -> Self::AbsDiff {}

    #[inline]
    fn ulps_diff(&self, _other: &()) -> Option<Self::UlpsDiff> {
        Some(())
    }
}

impl FloatEq for () {
    type Epsilon = ();
    type UlpsEpsilon = ();

    #[inline]
    fn eq_abs(&self, _other: &(), _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_rel(&self, _other: &(), _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_ulps(&self, _other: &(), _max_diff: &Self::UlpsEpsilon) -> bool {
        true
    }
}

impl FloatEqDebug for () {
    type DebugEpsilon = ();
    type DebugUlpsEpsilon = ();

    #[inline]
    fn debug_abs_epsilon(&self, _other: &(), _max_diff: &Self::Epsilon) -> Self::DebugEpsilon {}

    #[inline]
    fn debug_rel_epsilon(&self, _other: &(), _max_diff: &Self::Epsilon) -> Self::DebugEpsilon {}

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        _other: &(),
        _max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon {
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
            impl<$($T:FloatDiff),+> FloatDiff for ($($T,)+) where last_type!($($T,)+): ?Sized {
                type AbsDiff = ($($T::AbsDiff,)+);
                type UlpsDiff = ($($T::UlpsDiff,)+);

                #[inline]
                fn abs_diff(&self, other: &Self) -> Self::AbsDiff {
                    ($(self.$idx.abs_diff(&other.$idx),)+)
                }

                #[inline]
                fn ulps_diff(&self, other: &Self) -> Option<Self::UlpsDiff> {
                    Some(($(self.$idx.ulps_diff(&other.$idx)?,)+))
                }
            }

            impl<$($T:FloatEq),+> FloatEq for ($($T,)+) where last_type!($($T,)+): ?Sized {
                type Epsilon = ($($T::Epsilon,)+);
                type UlpsEpsilon = ($($T::UlpsEpsilon,)+);

                #[inline]
                fn eq_abs(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                    $(self.$idx.eq_abs(&other.$idx, &max_diff.$idx))&&+
                }

                #[inline]
                fn eq_rel(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                    $(self.$idx.eq_rel(&other.$idx, &max_diff.$idx))&&+
                }

                #[inline]
                fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsEpsilon) -> bool {
                    $(self.$idx.eq_ulps(&other.$idx, &max_diff.$idx))&&+
                }
            }

            impl<$($T:FloatEqDebug + fmt::Debug),+> FloatEqDebug for ($($T,)+) where last_type!($($T,)+): ?Sized {
                type DebugEpsilon = ($($T::DebugEpsilon,)+);
                type DebugUlpsEpsilon = ($($T::DebugUlpsEpsilon,)+);

                #[inline]
                fn debug_abs_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                    ($(self.$idx.debug_abs_epsilon(&other.$idx, &max_diff.$idx),)+)
                }

                #[inline]
                fn debug_rel_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                    ($(self.$idx.debug_rel_epsilon(&other.$idx, &max_diff.$idx),)+)
                }

                #[inline]
                fn debug_ulps_epsilon(&self, other: &Self, max_diff: &Self::UlpsEpsilon) -> Self::DebugUlpsEpsilon {
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
