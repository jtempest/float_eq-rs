use crate::{FloatDiff, FloatEq, FloatEqDebug, FloatUlps, Ulps};
use core::fmt;

impl FloatUlps for () {
    type Ulps = ();
}

impl FloatDiff for () {
    type Output = ();

    #[inline]
    fn abs_diff(&self, _other: &()) -> Self::Output {}

    #[inline]
    fn ulps_diff(&self, _other: &()) -> Option<Ulps<Self::Output>> {
        Some(())
    }
}

impl FloatEq for () {
    type Epsilon = ();

    #[inline]
    fn eq_abs(&self, _other: &(), _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_rel(&self, _other: &(), _max_diff: &Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn eq_ulps(&self, _other: &(), _max_diff: &Ulps<Self::Epsilon>) -> bool {
        true
    }
}

impl FloatEqDebug for () {
    type DebugEpsilon = ();

    #[inline]
    fn debug_abs_epsilon(&self, _other: &(), _max_diff: &Self::Epsilon) -> Self::DebugEpsilon {}

    #[inline]
    fn debug_rel_epsilon(&self, _other: &(), _max_diff: &Self::Epsilon) -> Self::DebugEpsilon {}

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        _other: &(),
        _max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
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
            impl<$($T:FloatUlps),+> FloatUlps for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized,
                $(Ulps<$T>: Sized,)+
            {
                type Ulps = ($(Ulps<$T>,)+);
            }

            impl<$($T:FloatDiff),+> FloatDiff for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized,
                $($T::Output: Sized,)+
                $(Ulps<$T::Output>: Sized,)+
            {
                type Output = ($($T::Output,)+);

                #[inline]
                fn abs_diff(&self, other: &Self) -> Self::Output {
                    ($(self.$idx.abs_diff(&other.$idx),)+)
                }

                #[inline]
                fn ulps_diff(&self, other: &Self) -> Option<Ulps<Self::Output>>
                where
                    Ulps<Self::Output>: Sized
                {
                    Some(($(self.$idx.ulps_diff(&other.$idx)?,)+))
                }
            }

            impl<$($T:FloatEq),+> FloatEq for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized,
                $($T::Epsilon: Sized,)+
                $(Ulps<$T::Epsilon>: Sized,)+
            {
                type Epsilon = ($($T::Epsilon,)+);

                #[inline]
                fn eq_abs(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                    $(self.$idx.eq_abs(&other.$idx, &max_diff.$idx))&&+
                }

                #[inline]
                fn eq_rel(&self, other: &Self, max_diff: &Self::Epsilon) -> bool {
                    $(self.$idx.eq_rel(&other.$idx, &max_diff.$idx))&&+
                }

                #[inline]
                fn eq_ulps(&self, other: &Self, max_diff: &Ulps<Self::Epsilon>) -> bool {
                    $(self.$idx.eq_ulps(&other.$idx, &max_diff.$idx))&&+
                }
            }

            impl<$($T:FloatEqDebug + fmt::Debug),+> FloatEqDebug for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized,
                $($T::Epsilon: Sized,)+
                $($T::DebugEpsilon: Sized,)+
                $(Ulps<$T::Epsilon>: Sized,)+
                $(Ulps<$T::DebugEpsilon>: Sized,)+
            {
                type DebugEpsilon = ($($T::DebugEpsilon,)+);

                #[inline]
                fn debug_abs_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                    ($(self.$idx.debug_abs_epsilon(&other.$idx, &max_diff.$idx),)+)
                }

                #[inline]
                fn debug_rel_epsilon(&self, other: &Self, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                    ($(self.$idx.debug_rel_epsilon(&other.$idx, &max_diff.$idx),)+)
                }

                #[inline]
                fn debug_ulps_epsilon(&self, other: &Self, max_diff: &Ulps<Self::Epsilon>) -> Ulps<Self::DebugEpsilon> {
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
