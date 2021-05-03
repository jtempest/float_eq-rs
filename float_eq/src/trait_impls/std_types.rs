use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsTol, UlpsTol,
};
use std::boxed::Box;
use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
use std::fmt;
use std::hash::{BuildHasher, Hash};
use std::rc::Rc;
use std::sync::Arc;

//------------------------------------------------------------------------------
// Simple wrapper types
//------------------------------------------------------------------------------
macro_rules! impl_traits_for_wrapper {
    ($t:ident) => {
        impl<T: ?Sized + FloatEqUlpsTol> FloatEqUlpsTol for $t<T> {
            type UlpsTol = $t<UlpsTol<T>>;
        }

        impl<A: ?Sized, B: ?Sized> FloatEq<$t<B>> for $t<A>
        where
            A: FloatEq<B>,
        {
            type Tol = A::Tol;

            #[inline]
            fn eq_abs(&self, other: &$t<B>, tol: &Self::Tol) -> bool {
                FloatEq::eq_abs(&**self, &**other, tol)
            }

            #[inline]
            fn eq_rmax(&self, other: &$t<B>, tol: &Self::Tol) -> bool {
                FloatEq::eq_rmax(&**self, &**other, tol)
            }

            #[inline]
            fn eq_rmin(&self, other: &$t<B>, tol: &Self::Tol) -> bool {
                FloatEq::eq_rmin(&**self, &**other, tol)
            }

            #[inline]
            fn eq_r1st(&self, other: &$t<B>, tol: &Self::Tol) -> bool {
                FloatEq::eq_r1st(&**self, &**other, tol)
            }

            #[inline]
            fn eq_r2nd(&self, other: &$t<B>, tol: &Self::Tol) -> bool {
                FloatEq::eq_r2nd(&**self, &**other, tol)
            }

            #[inline]
            fn eq_ulps(&self, other: &$t<B>, tol: &UlpsTol<Self::Tol>) -> bool {
                FloatEq::eq_ulps(&**self, &**other, tol)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqAll<$t<B>> for $t<A>
        where
            A: FloatEqAll<B>,
        {
            type AllTol = A::AllTol;

            #[inline]
            fn eq_abs_all(&self, other: &$t<B>, tol: &Self::AllTol) -> bool {
                FloatEqAll::eq_abs_all(&**self, &**other, tol)
            }

            #[inline]
            fn eq_rmax_all(&self, other: &$t<B>, tol: &Self::AllTol) -> bool {
                FloatEqAll::eq_rmax_all(&**self, &**other, tol)
            }

            #[inline]
            fn eq_rmin_all(&self, other: &$t<B>, tol: &Self::AllTol) -> bool {
                FloatEqAll::eq_rmin_all(&**self, &**other, tol)
            }

            #[inline]
            fn eq_r1st_all(&self, other: &$t<B>, tol: &Self::AllTol) -> bool {
                FloatEqAll::eq_r1st_all(&**self, &**other, tol)
            }

            #[inline]
            fn eq_r2nd_all(&self, other: &$t<B>, tol: &Self::AllTol) -> bool {
                FloatEqAll::eq_r2nd_all(&**self, &**other, tol)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &$t<B>, tol: &UlpsTol<Self::AllTol>) -> bool {
                FloatEqAll::eq_ulps_all(&**self, &**other, tol)
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEq<$t<B>> for $t<A>
        where
            A: AssertFloatEq<B> + Copy,
            B: Copy,
        {
            type DebugAbsDiff = A::DebugAbsDiff;
            type DebugTol = A::DebugTol;

            #[inline]
            fn debug_abs_diff(&self, other: &$t<B>) -> Self::DebugAbsDiff {
                AssertFloatEq::debug_abs_diff(&**self, &**other)
            }

            #[inline]
            fn debug_ulps_diff(&self, other: &$t<B>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
                AssertFloatEq::debug_ulps_diff(&**self, &**other)
            }

            #[inline]
            fn debug_abs_tol(&self, other: &$t<B>, tol: &Self::Tol) -> Self::DebugTol {
                AssertFloatEq::debug_abs_tol(&**self, &**other, tol)
            }

            #[inline]
            fn debug_rmax_tol(&self, other: &$t<B>, tol: &Self::Tol) -> Self::DebugTol {
                AssertFloatEq::debug_rmax_tol(&**self, &**other, tol)
            }

            #[inline]
            fn debug_rmin_tol(&self, other: &$t<B>, tol: &Self::Tol) -> Self::DebugTol {
                AssertFloatEq::debug_rmin_tol(&**self, &**other, tol)
            }

            #[inline]
            fn debug_r1st_tol(&self, other: &$t<B>, tol: &Self::Tol) -> Self::DebugTol {
                AssertFloatEq::debug_r1st_tol(&**self, &**other, tol)
            }

            #[inline]
            fn debug_r2nd_tol(&self, other: &$t<B>, tol: &Self::Tol) -> Self::DebugTol {
                AssertFloatEq::debug_r2nd_tol(&**self, &**other, tol)
            }

            #[inline]
            fn debug_ulps_tol(
                &self,
                other: &$t<B>,
                tol: &UlpsTol<Self::Tol>,
            ) -> UlpsTol<Self::DebugTol>
            where
                UlpsTol<Self::DebugTol>: Sized,
            {
                AssertFloatEq::debug_ulps_tol(&**self, &**other, tol)
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEqAll<$t<B>> for $t<A>
        where
            A: AssertFloatEqAll<B> + Copy,
            B: Copy,
        {
            type AllDebugTol = A::AllDebugTol;

            #[inline]
            fn debug_abs_all_tol(&self, other: &$t<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
                AssertFloatEqAll::debug_abs_all_tol(&**self, &**other, tol)
            }

            #[inline]
            fn debug_rmax_all_tol(&self, other: &$t<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
                AssertFloatEqAll::debug_rmax_all_tol(&**self, &**other, tol)
            }

            #[inline]
            fn debug_rmin_all_tol(&self, other: &$t<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
                AssertFloatEqAll::debug_rmin_all_tol(&**self, &**other, tol)
            }

            #[inline]
            fn debug_r1st_all_tol(&self, other: &$t<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
                AssertFloatEqAll::debug_r1st_all_tol(&**self, &**other, tol)
            }

            #[inline]
            fn debug_r2nd_all_tol(&self, other: &$t<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
                AssertFloatEqAll::debug_r2nd_all_tol(&**self, &**other, tol)
            }

            #[inline]
            fn debug_ulps_all_tol(
                &self,
                other: &$t<B>,
                tol: &UlpsTol<Self::AllTol>,
            ) -> UlpsTol<Self::AllDebugTol>
            where
                UlpsTol<Self::AllDebugTol>: Sized,
            {
                AssertFloatEqAll::debug_ulps_all_tol(&**self, &**other, tol)
            }
        }
    };
}

impl_traits_for_wrapper!(Arc);
impl_traits_for_wrapper!(Box);
impl_traits_for_wrapper!(Rc);

//------------------------------------------------------------------------------
// Slices
//------------------------------------------------------------------------------
impl<A, B> AssertFloatEq<[B]> for [A]
where
    A: AssertFloatEq<B>,
    A::Tol: Sized,
    A::DebugTol: Sized,
    UlpsTol<A::Tol>: Sized,
    UlpsTol<A::DebugTol>: Sized,
{
    type DebugAbsDiff = Option<Vec<A::DebugAbsDiff>>;
    type DebugTol = Option<Vec<A::DebugTol>>;

    #[inline]
    fn debug_abs_diff(&self, other: &[B]) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_abs_diff(b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &[B]) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_ulps_diff(b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_tol(&self, other: &[B], tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(tol)
                    .map(|((a, b), eps)| AssertFloatEq::debug_abs_tol(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_tol(&self, other: &[B], tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(tol)
                    .map(|((a, b), eps)| AssertFloatEq::debug_rmax_tol(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_tol(&self, other: &[B], tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(tol)
                    .map(|((a, b), eps)| AssertFloatEq::debug_rmin_tol(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_tol(&self, other: &[B], tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(tol)
                    .map(|((a, b), eps)| AssertFloatEq::debug_r1st_tol(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_tol(&self, other: &[B], tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(tol)
                    .map(|((a, b), eps)| AssertFloatEq::debug_r2nd_tol(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_tol(&self, other: &[B], tol: &UlpsTol<Self::Tol>) -> UlpsTol<Self::DebugTol> {
        if self.len() == other.len() && self.len() == tol.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(tol)
                    .map(|((a, b), eps)| AssertFloatEq::debug_ulps_tol(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertFloatEqAll<[B]> for [A]
where
    A: AssertFloatEqAll<B>,
    A::AllDebugTol: Sized,
    UlpsTol<A::AllDebugTol>: Sized,
{
    type AllDebugTol = Option<Vec<A::AllDebugTol>>;

    #[inline]
    fn debug_abs_all_tol(&self, other: &[B], tol: &Self::AllTol) -> Self::AllDebugTol {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_abs_all_tol(b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_all_tol(&self, other: &[B], tol: &Self::AllTol) -> Self::AllDebugTol {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_rmax_all_tol(b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_all_tol(&self, other: &[B], tol: &Self::AllTol) -> Self::AllDebugTol {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_rmin_all_tol(b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_all_tol(&self, other: &[B], tol: &Self::AllTol) -> Self::AllDebugTol {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_r1st_all_tol(b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_all_tol(&self, other: &[B], tol: &Self::AllTol) -> Self::AllDebugTol {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_r2nd_all_tol(b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_tol(
        &self,
        other: &[B],
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol>
    where
        UlpsTol<Self::AllDebugTol>: Sized,
    {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_ulps_all_tol(b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

//------------------------------------------------------------------------------
// Linear collections
//------------------------------------------------------------------------------
macro_rules! impl_traits_for_linear_collection {
    ($t:ident) => {
        impl<T: FloatEqUlpsTol> FloatEqUlpsTol for $t<T>
        where
            UlpsTol<T>: Sized,
        {
            type UlpsTol = $t<UlpsTol<T>>;
        }

        impl<T: FloatEqDebugUlpsDiff> FloatEqDebugUlpsDiff for $t<T> {
            type DebugUlpsDiff = $t<DebugUlpsDiff<T>>;
        }

        impl<A, B> FloatEq<$t<B>> for $t<A>
        where
            A: FloatEq<B>,
            A::Tol: Sized,
            UlpsTol<A::Tol>: Sized,
        {
            type Tol = $t<A::Tol>;

            #[inline]
            fn eq_abs(&self, other: &$t<B>, tol: &Self::Tol) -> bool {
                self.len() == other.len()
                    && self.len() == tol.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(tol)
                        .all(|((a, b), eps)| FloatEq::eq_abs(a, b, eps))
            }

            #[inline]
            fn eq_rmax(&self, other: &$t<B>, tol: &Self::Tol) -> bool {
                self.len() == other.len()
                    && self.len() == tol.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(tol)
                        .all(|((a, b), eps)| FloatEq::eq_rmax(a, b, eps))
            }

            #[inline]
            fn eq_rmin(&self, other: &$t<B>, tol: &Self::Tol) -> bool {
                self.len() == other.len()
                    && self.len() == tol.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(tol)
                        .all(|((a, b), eps)| FloatEq::eq_rmin(a, b, eps))
            }

            #[inline]
            fn eq_r1st(&self, other: &$t<B>, tol: &Self::Tol) -> bool {
                self.len() == other.len()
                    && self.len() == tol.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(tol)
                        .all(|((a, b), eps)| FloatEq::eq_r1st(a, b, eps))
            }

            #[inline]
            fn eq_r2nd(&self, other: &$t<B>, tol: &Self::Tol) -> bool {
                self.len() == other.len()
                    && self.len() == tol.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(tol)
                        .all(|((a, b), eps)| FloatEq::eq_r2nd(a, b, eps))
            }

            #[inline]
            fn eq_ulps(&self, other: &$t<B>, tol: &UlpsTol<Self::Tol>) -> bool {
                self.len() == other.len()
                    && self.len() == tol.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(tol)
                        .all(|((a, b), eps)| FloatEq::eq_ulps(a, b, eps))
            }
        }

        impl<A, B> FloatEqAll<$t<B>> for $t<A>
        where
            A: FloatEqAll<B>,
        {
            type AllTol = A::AllTol;

            #[inline]
            fn eq_abs_all(&self, other: &$t<B>, tol: &Self::AllTol) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_abs_all(a, b, tol))
            }

            #[inline]
            fn eq_rmax_all(&self, other: &$t<B>, tol: &Self::AllTol) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_rmax_all(a, b, tol))
            }

            #[inline]
            fn eq_rmin_all(&self, other: &$t<B>, tol: &Self::AllTol) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_rmin_all(a, b, tol))
            }

            #[inline]
            fn eq_r1st_all(&self, other: &$t<B>, tol: &Self::AllTol) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_r1st_all(a, b, tol))
            }

            #[inline]
            fn eq_r2nd_all(&self, other: &$t<B>, tol: &Self::AllTol) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_r2nd_all(a, b, tol))
            }

            #[inline]
            fn eq_ulps_all(&self, other: &$t<B>, tol: &UlpsTol<Self::AllTol>) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_ulps_all(a, b, tol))
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEq<$t<B>> for $t<A>
        where
            A: AssertFloatEq<B> + Copy,
            B: Copy,
            A::Tol: Sized,
            A::DebugTol: Sized,
            UlpsTol<A::Tol>: Sized,
            UlpsTol<A::DebugTol>: Sized,
        {
            type DebugAbsDiff = Option<$t<A::DebugAbsDiff>>;
            type DebugTol = Option<$t<A::DebugTol>>;

            #[inline]
            fn debug_abs_diff(&self, other: &$t<B>) -> Self::DebugAbsDiff {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEq::debug_abs_diff(a, b))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_ulps_diff(&self, other: &$t<B>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEq::debug_ulps_diff(a, b))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_abs_tol(&self, other: &$t<B>, tol: &Self::Tol) -> Self::DebugTol {
                if self.len() == other.len() && self.len() == tol.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(tol)
                            .map(|((a, b), eps)| AssertFloatEq::debug_abs_tol(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_rmax_tol(&self, other: &$t<B>, tol: &Self::Tol) -> Self::DebugTol {
                if self.len() == other.len() && self.len() == tol.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(tol)
                            .map(|((a, b), eps)| AssertFloatEq::debug_rmax_tol(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_rmin_tol(&self, other: &$t<B>, tol: &Self::Tol) -> Self::DebugTol {
                if self.len() == other.len() && self.len() == tol.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(tol)
                            .map(|((a, b), eps)| AssertFloatEq::debug_rmin_tol(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_r1st_tol(&self, other: &$t<B>, tol: &Self::Tol) -> Self::DebugTol {
                if self.len() == other.len() && self.len() == tol.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(tol)
                            .map(|((a, b), eps)| AssertFloatEq::debug_r1st_tol(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_r2nd_tol(&self, other: &$t<B>, tol: &Self::Tol) -> Self::DebugTol {
                if self.len() == other.len() && self.len() == tol.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(tol)
                            .map(|((a, b), eps)| AssertFloatEq::debug_r2nd_tol(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_ulps_tol(
                &self,
                other: &$t<B>,
                tol: &UlpsTol<Self::Tol>,
            ) -> UlpsTol<Self::DebugTol> {
                if self.len() == other.len() && self.len() == tol.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(tol)
                            .map(|((a, b), eps)| AssertFloatEq::debug_ulps_tol(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEqAll<$t<B>> for $t<A>
        where
            A: AssertFloatEqAll<B> + Copy,
            B: Copy,
            A::AllDebugTol: Sized,
            UlpsTol<A::AllDebugTol>: Sized,
        {
            type AllDebugTol = Option<$t<A::AllDebugTol>>;

            #[inline]
            fn debug_abs_all_tol(&self, other: &$t<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_abs_all_tol(a, b, tol))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_rmax_all_tol(&self, other: &$t<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_rmax_all_tol(a, b, tol))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_rmin_all_tol(&self, other: &$t<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_rmin_all_tol(a, b, tol))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_r1st_all_tol(&self, other: &$t<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_r1st_all_tol(a, b, tol))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_r2nd_all_tol(&self, other: &$t<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_r2nd_all_tol(a, b, tol))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_ulps_all_tol(
                &self,
                other: &$t<B>,
                tol: &UlpsTol<Self::AllTol>,
            ) -> UlpsTol<Self::AllDebugTol>
            where
                UlpsTol<Self::AllDebugTol>: Sized,
            {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_ulps_all_tol(a, b, tol))
                            .collect(),
                    )
                } else {
                    None
                }
            }
        }
    };
}

impl_traits_for_linear_collection!(Vec);
impl_traits_for_linear_collection!(VecDeque);
impl_traits_for_linear_collection!(LinkedList);

//------------------------------------------------------------------------------
// HashMap
//------------------------------------------------------------------------------
impl<K, V, S> FloatEqUlpsTol for HashMap<K, V, S>
where
    V: FloatEqUlpsTol,
    UlpsTol<V>: Sized,
{
    type UlpsTol = HashMap<K, UlpsTol<V>, S>;
}

impl<K, V, S> FloatEqDebugUlpsDiff for HashMap<K, V, S>
where
    V: FloatEqDebugUlpsDiff,
{
    type DebugUlpsDiff = HashMap<K, DebugUlpsDiff<V>, S>;
}

impl<K, VA, VB, S> FloatEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + Hash,
    S: BuildHasher,
    VA: FloatEq<VB>,
    VA::Tol: Sized,
    UlpsTol<VA::Tol>: Sized,
{
    type Tol = HashMap<K, VA::Tol, S>;

    #[inline]
    fn eq_abs(&self, other: &HashMap<K, VB, S>, tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_abs(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmax(&self, other: &HashMap<K, VB, S>, tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_rmax(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmin(&self, other: &HashMap<K, VB, S>, tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_rmin(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r1st(&self, other: &HashMap<K, VB, S>, tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_r1st(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r2nd(&self, other: &HashMap<K, VB, S>, tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_r2nd(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_ulps(&self, other: &HashMap<K, VB, S>, tol: &UlpsTol<Self::Tol>) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_ulps(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }
}

impl<K, VA, VB, S> FloatEqAll<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + Hash,
    S: BuildHasher,
    VA: FloatEqAll<VB>,
{
    type AllTol = VA::AllTol;

    #[inline]
    fn eq_abs_all(&self, other: &HashMap<K, VB, S>, tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_abs_all(a, b, tol)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmax_all(&self, other: &HashMap<K, VB, S>, tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_rmax_all(a, b, tol)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmin_all(&self, other: &HashMap<K, VB, S>, tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_rmin_all(a, b, tol)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r1st_all(&self, other: &HashMap<K, VB, S>, tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_r1st_all(a, b, tol)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &HashMap<K, VB, S>, tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_r2nd_all(a, b, tol)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_ulps_all(&self, other: &HashMap<K, VB, S>, tol: &UlpsTol<Self::AllTol>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_ulps_all(a, b, tol)
                } else {
                    false
                }
            })
    }
}

impl<K, VA, VB, S> AssertFloatEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + Hash + Clone + fmt::Debug,
    S: BuildHasher + Clone,
    VA: AssertFloatEq<VB>,
    VA::Tol: Sized,
    UlpsTol<VA::Tol>: Sized,
    VA::DebugTol: Sized,
    UlpsTol<VA::DebugTol>: Sized,
{
    type DebugAbsDiff = Option<HashMap<K, VA::DebugAbsDiff, S>>;
    type DebugTol = Option<HashMap<K, VA::DebugTol, S>>;

    #[inline]
    fn debug_abs_diff(&self, other: &HashMap<K, VB, S>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_abs_diff(other.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &HashMap<K, VB, S>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_ulps_diff(other.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_tol(&self, other: &HashMap<K, VB, S>, tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_abs_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_tol(&self, other: &HashMap<K, VB, S>, tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmax_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_tol(&self, other: &HashMap<K, VB, S>, tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmin_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_tol(&self, other: &HashMap<K, VB, S>, tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r1st_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_tol(&self, other: &HashMap<K, VB, S>, tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r2nd_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_tol(
        &self,
        other: &HashMap<K, VB, S>,
        tol: &UlpsTol<Self::Tol>,
    ) -> UlpsTol<Self::DebugTol> {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_ulps_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }
}

impl<K, VA, VB, S> AssertFloatEqAll<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + Hash + Clone + fmt::Debug,
    S: BuildHasher + Clone,
    VA: AssertFloatEqAll<VB>,
    VA::AllDebugTol: Sized,
    UlpsTol<VA::AllDebugTol>: Sized,
{
    type AllDebugTol = Option<HashMap<K, VA::AllDebugTol, S>>;

    #[inline]
    fn debug_abs_all_tol(
        &self,
        other: &HashMap<K, VB, S>,
        tol: &Self::AllTol,
    ) -> Self::AllDebugTol {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_abs_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_all_tol(
        &self,
        other: &HashMap<K, VB, S>,
        tol: &Self::AllTol,
    ) -> Self::AllDebugTol {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmax_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_all_tol(
        &self,
        other: &HashMap<K, VB, S>,
        tol: &Self::AllTol,
    ) -> Self::AllDebugTol {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmin_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_all_tol(
        &self,
        other: &HashMap<K, VB, S>,
        tol: &Self::AllTol,
    ) -> Self::AllDebugTol {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r1st_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_all_tol(
        &self,
        other: &HashMap<K, VB, S>,
        tol: &Self::AllTol,
    ) -> Self::AllDebugTol {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r2nd_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_tol(
        &self,
        other: &HashMap<K, VB, S>,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol>
    where
        UlpsTol<Self::AllDebugTol>: Sized,
    {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_ulps_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }
}

//------------------------------------------------------------------------------
// BTreeMap
//------------------------------------------------------------------------------
impl<K, V> FloatEqUlpsTol for BTreeMap<K, V>
where
    V: FloatEqUlpsTol,
    UlpsTol<V>: Sized,
{
    type UlpsTol = BTreeMap<K, UlpsTol<V>>;
}

impl<K, V> FloatEqDebugUlpsDiff for BTreeMap<K, V>
where
    V: FloatEqDebugUlpsDiff,
{
    type DebugUlpsDiff = BTreeMap<K, DebugUlpsDiff<V>>;
}

impl<K, VA, VB> FloatEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord,
    VA: FloatEq<VB>,
    VA::Tol: Sized,
    UlpsTol<VA::Tol>: Sized,
{
    type Tol = BTreeMap<K, VA::Tol>;

    #[inline]
    fn eq_abs(&self, other: &BTreeMap<K, VB>, tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_abs(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmax(&self, other: &BTreeMap<K, VB>, tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_rmax(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmin(&self, other: &BTreeMap<K, VB>, tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_rmin(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r1st(&self, other: &BTreeMap<K, VB>, tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_r1st(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r2nd(&self, other: &BTreeMap<K, VB>, tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_r2nd(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_ulps(&self, other: &BTreeMap<K, VB>, tol: &UlpsTol<Self::Tol>) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = tol.get(k) {
                        FloatEq::eq_ulps(a, b, eps)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }
}

impl<K, VA, VB> FloatEqAll<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord,
    VA: FloatEqAll<VB>,
{
    type AllTol = VA::AllTol;

    #[inline]
    fn eq_abs_all(&self, other: &BTreeMap<K, VB>, tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_abs_all(a, b, tol)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmax_all(&self, other: &BTreeMap<K, VB>, tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_rmax_all(a, b, tol)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmin_all(&self, other: &BTreeMap<K, VB>, tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_rmin_all(a, b, tol)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r1st_all(&self, other: &BTreeMap<K, VB>, tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_r1st_all(a, b, tol)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &BTreeMap<K, VB>, tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_r2nd_all(a, b, tol)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_ulps_all(&self, other: &BTreeMap<K, VB>, tol: &UlpsTol<Self::AllTol>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_ulps_all(a, b, tol)
                } else {
                    false
                }
            })
    }
}

impl<K, VA, VB> AssertFloatEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord + Clone + fmt::Debug,
    VA: AssertFloatEq<VB>,
    VA::Tol: Sized,
    VA::DebugTol: Sized,
    UlpsTol<VA::Tol>: Sized,
    UlpsTol<VA::DebugTol>: Sized,
{
    type DebugAbsDiff = Option<BTreeMap<K, VA::DebugAbsDiff>>;
    type DebugTol = Option<BTreeMap<K, VA::DebugTol>>;

    #[inline]
    fn debug_abs_diff(&self, other: &BTreeMap<K, VB>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_abs_diff(other.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &BTreeMap<K, VB>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_ulps_diff(other.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_tol(&self, other: &BTreeMap<K, VB>, tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_abs_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_tol(&self, other: &BTreeMap<K, VB>, tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmax_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_tol(&self, other: &BTreeMap<K, VB>, tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmin_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_tol(&self, other: &BTreeMap<K, VB>, tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r1st_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_tol(&self, other: &BTreeMap<K, VB>, tol: &Self::Tol) -> Self::DebugTol {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r2nd_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_tol(
        &self,
        other: &BTreeMap<K, VB>,
        tol: &UlpsTol<Self::Tol>,
    ) -> UlpsTol<Self::DebugTol>
    where
        UlpsTol<Self::DebugTol>: Sized,
    {
        if self.len() == other.len() && self.len() == tol.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_ulps_tol(other.get(k)?, tol.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }
}

impl<K, VA, VB> AssertFloatEqAll<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord + Clone + fmt::Debug,
    VA: AssertFloatEqAll<VB>,
    VA::AllDebugTol: Sized,
    UlpsTol<VA::AllDebugTol>: Sized,
{
    type AllDebugTol = Option<BTreeMap<K, VA::AllDebugTol>>;

    #[inline]
    fn debug_abs_all_tol(&self, other: &BTreeMap<K, VB>, tol: &Self::AllTol) -> Self::AllDebugTol {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_abs_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_all_tol(&self, other: &BTreeMap<K, VB>, tol: &Self::AllTol) -> Self::AllDebugTol {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmax_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_all_tol(&self, other: &BTreeMap<K, VB>, tol: &Self::AllTol) -> Self::AllDebugTol {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmin_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_all_tol(&self, other: &BTreeMap<K, VB>, tol: &Self::AllTol) -> Self::AllDebugTol {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r1st_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_all_tol(&self, other: &BTreeMap<K, VB>, tol: &Self::AllTol) -> Self::AllDebugTol {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r2nd_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_tol(
        &self,
        other: &BTreeMap<K, VB>,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol>
    where
        UlpsTol<Self::AllDebugTol>: Sized,
    {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_ulps_all_tol(other.get(k)?, tol));
            }
            Some(result)
        } else {
            None
        }
    }
}
