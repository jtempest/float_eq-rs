use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsEpsilon, UlpsEpsilon,
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
        impl<T: ?Sized + FloatEqUlpsEpsilon> FloatEqUlpsEpsilon for $t<T> {
            type UlpsEpsilon = $t<UlpsEpsilon<T>>;
        }

        impl<A: ?Sized, B: ?Sized> FloatEq<$t<B>> for $t<A>
        where
            A: FloatEq<B>,
        {
            type Epsilon = A::Epsilon;

            #[inline]
            fn eq_abs(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_abs(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_rmax(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_rmax(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_rmin(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_rmin(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_r1st(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_r1st(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_r2nd(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_r2nd(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_ulps(&self, other: &$t<B>, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
                FloatEq::eq_ulps(&**self, &**other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqAll<$t<B>> for $t<A>
        where
            A: FloatEqAll<B>,
        {
            type AllEpsilon = A::AllEpsilon;

            #[inline]
            fn eq_abs_all(&self, other: &$t<B>, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_abs_all(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_rmax_all(&self, other: &$t<B>, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_rmax_all(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_rmin_all(&self, other: &$t<B>, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_rmin_all(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_r1st_all(&self, other: &$t<B>, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_r1st_all(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_r2nd_all(&self, other: &$t<B>, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_r2nd_all(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &$t<B>, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
                FloatEqAll::eq_ulps_all(&**self, &**other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEq<$t<B>> for $t<A>
        where
            A: AssertFloatEq<B> + Copy,
            B: Copy,
        {
            type DebugAbsDiff = A::DebugAbsDiff;
            type DebugEpsilon = A::DebugEpsilon;

            #[inline]
            fn debug_abs_diff(&self, other: &$t<B>) -> Self::DebugAbsDiff {
                AssertFloatEq::debug_abs_diff(&**self, &**other)
            }

            #[inline]
            fn debug_ulps_diff(&self, other: &$t<B>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
                AssertFloatEq::debug_ulps_diff(&**self, &**other)
            }

            #[inline]
            fn debug_abs_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                AssertFloatEq::debug_abs_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_rmax_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                AssertFloatEq::debug_rmax_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_rmin_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                AssertFloatEq::debug_rmin_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_r1st_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                AssertFloatEq::debug_r1st_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_r2nd_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                AssertFloatEq::debug_r2nd_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_ulps_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &UlpsEpsilon<Self::Epsilon>,
            ) -> UlpsEpsilon<Self::DebugEpsilon>
            where
                UlpsEpsilon<Self::DebugEpsilon>: Sized,
            {
                AssertFloatEq::debug_ulps_epsilon(&**self, &**other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEqAll<$t<B>> for $t<A>
        where
            A: AssertFloatEqAll<B> + Copy,
            B: Copy,
        {
            type AllDebugEpsilon = A::AllDebugEpsilon;

            #[inline]
            fn debug_abs_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                AssertFloatEqAll::debug_abs_all_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_rmax_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                AssertFloatEqAll::debug_rmax_all_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_rmin_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                AssertFloatEqAll::debug_rmin_all_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_r1st_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                AssertFloatEqAll::debug_r1st_all_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_r2nd_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                AssertFloatEqAll::debug_r2nd_all_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &UlpsEpsilon<Self::AllEpsilon>,
            ) -> UlpsEpsilon<Self::AllDebugEpsilon>
            where
                UlpsEpsilon<Self::AllDebugEpsilon>: Sized,
            {
                AssertFloatEqAll::debug_ulps_all_epsilon(&**self, &**other, max_diff)
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
    A::Epsilon: Sized,
    A::DebugEpsilon: Sized,
    UlpsEpsilon<A::Epsilon>: Sized,
    UlpsEpsilon<A::DebugEpsilon>: Sized,
{
    type DebugAbsDiff = Option<Vec<A::DebugAbsDiff>>;
    type DebugEpsilon = Option<Vec<A::DebugEpsilon>>;

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
    fn debug_abs_epsilon(&self, other: &[B], max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_diff)
                    .map(|((a, b), eps)| AssertFloatEq::debug_abs_epsilon(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_epsilon(&self, other: &[B], max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_diff)
                    .map(|((a, b), eps)| AssertFloatEq::debug_rmax_epsilon(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_epsilon(&self, other: &[B], max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_diff)
                    .map(|((a, b), eps)| AssertFloatEq::debug_rmin_epsilon(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_epsilon(&self, other: &[B], max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_diff)
                    .map(|((a, b), eps)| AssertFloatEq::debug_r1st_epsilon(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_epsilon(&self, other: &[B], max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_diff)
                    .map(|((a, b), eps)| AssertFloatEq::debug_r2nd_epsilon(a, b, eps))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &[B],
        max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon> {
        if self.len() == other.len() && self.len() == max_diff.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_diff)
                    .map(|((a, b), eps)| AssertFloatEq::debug_ulps_epsilon(a, b, eps))
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
    A::AllDebugEpsilon: Sized,
    UlpsEpsilon<A::AllDebugEpsilon>: Sized,
{
    type AllDebugEpsilon = Option<Vec<A::AllDebugEpsilon>>;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &[B],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_abs_all_epsilon(b, max_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &[B],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_rel_all_epsilon(b, max_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_all_epsilon(
        &self,
        other: &[B],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_rmax_all_epsilon(b, max_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_all_epsilon(
        &self,
        other: &[B],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_rmin_all_epsilon(b, max_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_all_epsilon(
        &self,
        other: &[B],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_r1st_all_epsilon(b, max_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_all_epsilon(
        &self,
        other: &[B],
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_r2nd_all_epsilon(b, max_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &[B],
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon>
    where
        UlpsEpsilon<Self::AllDebugEpsilon>: Sized,
    {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_ulps_all_epsilon(b, max_diff))
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
        impl<T: FloatEqUlpsEpsilon> FloatEqUlpsEpsilon for $t<T>
        where
            UlpsEpsilon<T>: Sized,
        {
            type UlpsEpsilon = $t<UlpsEpsilon<T>>;
        }

        impl<T: FloatEqDebugUlpsDiff> FloatEqDebugUlpsDiff for $t<T> {
            type DebugUlpsDiff = $t<DebugUlpsDiff<T>>;
        }

        impl<A, B> FloatEq<$t<B>> for $t<A>
        where
            A: FloatEq<B>,
            A::Epsilon: Sized,
            UlpsEpsilon<A::Epsilon>: Sized,
        {
            type Epsilon = $t<A::Epsilon>;

            #[inline]
            fn eq_abs(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                self.len() == other.len()
                    && self.len() == max_diff.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(max_diff)
                        .all(|((a, b), eps)| FloatEq::eq_abs(a, b, eps))
            }

            #[inline]
            fn eq_rmax(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                self.len() == other.len()
                    && self.len() == max_diff.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(max_diff)
                        .all(|((a, b), eps)| FloatEq::eq_rmax(a, b, eps))
            }

            #[inline]
            fn eq_rmin(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                self.len() == other.len()
                    && self.len() == max_diff.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(max_diff)
                        .all(|((a, b), eps)| FloatEq::eq_rmin(a, b, eps))
            }

            #[inline]
            fn eq_r1st(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                self.len() == other.len()
                    && self.len() == max_diff.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(max_diff)
                        .all(|((a, b), eps)| FloatEq::eq_r1st(a, b, eps))
            }

            #[inline]
            fn eq_r2nd(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                self.len() == other.len()
                    && self.len() == max_diff.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(max_diff)
                        .all(|((a, b), eps)| FloatEq::eq_r2nd(a, b, eps))
            }

            #[inline]
            fn eq_ulps(&self, other: &$t<B>, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
                self.len() == other.len()
                    && self.len() == max_diff.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .zip(max_diff)
                        .all(|((a, b), eps)| FloatEq::eq_ulps(a, b, eps))
            }
        }

        impl<A, B> FloatEqAll<$t<B>> for $t<A>
        where
            A: FloatEqAll<B>,
        {
            type AllEpsilon = A::AllEpsilon;

            #[inline]
            fn eq_abs_all(&self, other: &$t<B>, max_diff: &Self::AllEpsilon) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_abs_all(a, b, max_diff))
            }

            #[inline]
            fn eq_rmax_all(&self, other: &$t<B>, max_diff: &Self::AllEpsilon) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_rmax_all(a, b, max_diff))
            }

            #[inline]
            fn eq_rmin_all(&self, other: &$t<B>, max_diff: &Self::AllEpsilon) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_rmin_all(a, b, max_diff))
            }

            #[inline]
            fn eq_r1st_all(&self, other: &$t<B>, max_diff: &Self::AllEpsilon) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_r1st_all(a, b, max_diff))
            }

            #[inline]
            fn eq_r2nd_all(&self, other: &$t<B>, max_diff: &Self::AllEpsilon) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_r2nd_all(a, b, max_diff))
            }

            #[inline]
            fn eq_ulps_all(&self, other: &$t<B>, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| FloatEqAll::eq_ulps_all(a, b, max_diff))
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEq<$t<B>> for $t<A>
        where
            A: AssertFloatEq<B> + Copy,
            B: Copy,
            A::Epsilon: Sized,
            A::DebugEpsilon: Sized,
            UlpsEpsilon<A::Epsilon>: Sized,
            UlpsEpsilon<A::DebugEpsilon>: Sized,
        {
            type DebugAbsDiff = Option<$t<A::DebugAbsDiff>>;
            type DebugEpsilon = Option<$t<A::DebugEpsilon>>;

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
            fn debug_abs_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                if self.len() == other.len() && self.len() == max_diff.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(max_diff)
                            .map(|((a, b), eps)| AssertFloatEq::debug_abs_epsilon(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_rmax_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                if self.len() == other.len() && self.len() == max_diff.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(max_diff)
                            .map(|((a, b), eps)| AssertFloatEq::debug_rmax_epsilon(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_rmin_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                if self.len() == other.len() && self.len() == max_diff.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(max_diff)
                            .map(|((a, b), eps)| AssertFloatEq::debug_rmin_epsilon(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_r1st_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                if self.len() == other.len() && self.len() == max_diff.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(max_diff)
                            .map(|((a, b), eps)| AssertFloatEq::debug_r1st_epsilon(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_r2nd_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                if self.len() == other.len() && self.len() == max_diff.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(max_diff)
                            .map(|((a, b), eps)| AssertFloatEq::debug_r2nd_epsilon(a, b, eps))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_ulps_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &UlpsEpsilon<Self::Epsilon>,
            ) -> UlpsEpsilon<Self::DebugEpsilon> {
                if self.len() == other.len() && self.len() == max_diff.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .zip(max_diff)
                            .map(|((a, b), eps)| AssertFloatEq::debug_ulps_epsilon(a, b, eps))
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
            A::AllDebugEpsilon: Sized,
            UlpsEpsilon<A::AllDebugEpsilon>: Sized,
        {
            type AllDebugEpsilon = Option<$t<A::AllDebugEpsilon>>;

            #[inline]
            fn debug_abs_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_abs_all_epsilon(a, b, max_diff))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_rmax_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_rmax_all_epsilon(a, b, max_diff))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_rmin_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_rmin_all_epsilon(a, b, max_diff))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_r1st_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_r1st_all_epsilon(a, b, max_diff))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_r2nd_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::AllEpsilon,
            ) -> Self::AllDebugEpsilon {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_r2nd_all_epsilon(a, b, max_diff))
                            .collect(),
                    )
                } else {
                    None
                }
            }

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &UlpsEpsilon<Self::AllEpsilon>,
            ) -> UlpsEpsilon<Self::AllDebugEpsilon>
            where
                UlpsEpsilon<Self::AllDebugEpsilon>: Sized,
            {
                if self.len() == other.len() {
                    Some(
                        self.iter()
                            .zip(other.iter())
                            .map(|(a, b)| AssertFloatEqAll::debug_ulps_all_epsilon(a, b, max_diff))
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
impl<K, V, S> FloatEqUlpsEpsilon for HashMap<K, V, S>
where
    V: FloatEqUlpsEpsilon,
    UlpsEpsilon<V>: Sized,
{
    type UlpsEpsilon = HashMap<K, UlpsEpsilon<V>, S>;
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
    VA::Epsilon: Sized,
    UlpsEpsilon<VA::Epsilon>: Sized,
{
    type Epsilon = HashMap<K, VA::Epsilon, S>;

    #[inline]
    fn eq_abs(&self, other: &HashMap<K, VB, S>, max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    fn eq_rmax(&self, other: &HashMap<K, VB, S>, max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    fn eq_rmin(&self, other: &HashMap<K, VB, S>, max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    fn eq_r1st(&self, other: &HashMap<K, VB, S>, max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    fn eq_r2nd(&self, other: &HashMap<K, VB, S>, max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    fn eq_ulps(&self, other: &HashMap<K, VB, S>, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    type AllEpsilon = VA::AllEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &HashMap<K, VB, S>, max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_abs_all(a, b, max_diff)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmax_all(&self, other: &HashMap<K, VB, S>, max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_rmax_all(a, b, max_diff)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmin_all(&self, other: &HashMap<K, VB, S>, max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_rmin_all(a, b, max_diff)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r1st_all(&self, other: &HashMap<K, VB, S>, max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_r1st_all(a, b, max_diff)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &HashMap<K, VB, S>, max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_r2nd_all(a, b, max_diff)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_ulps_all(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_ulps_all(a, b, max_diff)
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
    VA::Epsilon: Sized,
    UlpsEpsilon<VA::Epsilon>: Sized,
    VA::DebugEpsilon: Sized,
    UlpsEpsilon<VA::DebugEpsilon>: Sized,
{
    type DebugAbsDiff = Option<HashMap<K, VA::DebugAbsDiff, S>>;
    type DebugEpsilon = Option<HashMap<K, VA::DebugEpsilon, S>>;

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
    fn debug_abs_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_abs_epsilon(other.get(k)?, max_diff.get(k)?),
                );
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_rmax_epsilon(other.get(k)?, max_diff.get(k)?),
                );
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_rmin_epsilon(other.get(k)?, max_diff.get(k)?),
                );
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_r1st_epsilon(other.get(k)?, max_diff.get(k)?),
                );
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_r2nd_epsilon(other.get(k)?, max_diff.get(k)?),
                );
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon> {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_ulps_epsilon(other.get(k)?, max_diff.get(k)?),
                );
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
    VA::AllDebugEpsilon: Sized,
    UlpsEpsilon<VA::AllDebugEpsilon>: Sized,
{
    type AllDebugEpsilon = Option<HashMap<K, VA::AllDebugEpsilon, S>>;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_abs_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_all_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmax_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_all_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmin_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_all_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r1st_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_all_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r2nd_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &HashMap<K, VB, S>,
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon>
    where
        UlpsEpsilon<Self::AllDebugEpsilon>: Sized,
    {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (k, v) in self {
                result.insert(k.clone(), v.debug_ulps_all_epsilon(other.get(k)?, max_diff));
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
impl<K, V> FloatEqUlpsEpsilon for BTreeMap<K, V>
where
    V: FloatEqUlpsEpsilon,
    UlpsEpsilon<V>: Sized,
{
    type UlpsEpsilon = BTreeMap<K, UlpsEpsilon<V>>;
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
    VA::Epsilon: Sized,
    UlpsEpsilon<VA::Epsilon>: Sized,
{
    type Epsilon = BTreeMap<K, VA::Epsilon>;

    #[inline]
    fn eq_abs(&self, other: &BTreeMap<K, VB>, max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    fn eq_rmax(&self, other: &BTreeMap<K, VB>, max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    fn eq_rmin(&self, other: &BTreeMap<K, VB>, max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    fn eq_r1st(&self, other: &BTreeMap<K, VB>, max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    fn eq_r2nd(&self, other: &BTreeMap<K, VB>, max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    fn eq_ulps(&self, other: &BTreeMap<K, VB>, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    if let Some(eps) = max_diff.get(k) {
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
    type AllEpsilon = VA::AllEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &BTreeMap<K, VB>, max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_abs_all(a, b, max_diff)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmax_all(&self, other: &BTreeMap<K, VB>, max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_rmax_all(a, b, max_diff)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_rmin_all(&self, other: &BTreeMap<K, VB>, max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_rmin_all(a, b, max_diff)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r1st_all(&self, other: &BTreeMap<K, VB>, max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_r1st_all(a, b, max_diff)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &BTreeMap<K, VB>, max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_r2nd_all(a, b, max_diff)
                } else {
                    false
                }
            })
    }

    #[inline]
    fn eq_ulps_all(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> bool {
        self.len() == other.len()
            && self.iter().all(|(k, a)| {
                if let Some(b) = other.get(k) {
                    FloatEqAll::eq_ulps_all(a, b, max_diff)
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
    VA::Epsilon: Sized,
    VA::DebugEpsilon: Sized,
    UlpsEpsilon<VA::Epsilon>: Sized,
    UlpsEpsilon<VA::DebugEpsilon>: Sized,
{
    type DebugAbsDiff = Option<BTreeMap<K, VA::DebugAbsDiff>>;
    type DebugEpsilon = Option<BTreeMap<K, VA::DebugEpsilon>>;

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
    fn debug_abs_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_abs_epsilon(other.get(k)?, max_diff.get(k)?),
                );
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_rmax_epsilon(other.get(k)?, max_diff.get(k)?),
                );
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_rmin_epsilon(other.get(k)?, max_diff.get(k)?),
                );
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_r1st_epsilon(other.get(k)?, max_diff.get(k)?),
                );
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_r2nd_epsilon(other.get(k)?, max_diff.get(k)?),
                );
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon>
    where
        UlpsEpsilon<Self::DebugEpsilon>: Sized,
    {
        if self.len() == other.len() && self.len() == max_diff.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(
                    k.clone(),
                    v.debug_ulps_epsilon(other.get(k)?, max_diff.get(k)?),
                );
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
    VA::AllDebugEpsilon: Sized,
    UlpsEpsilon<VA::AllDebugEpsilon>: Sized,
{
    type AllDebugEpsilon = Option<BTreeMap<K, VA::AllDebugEpsilon>>;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_abs_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmax_all_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmax_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_rmin_all_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_rmin_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r1st_all_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r1st_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_r2nd_all_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_r2nd_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &BTreeMap<K, VB>,
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon>
    where
        UlpsEpsilon<Self::AllDebugEpsilon>: Sized,
    {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_ulps_all_epsilon(other.get(k)?, max_diff));
            }
            Some(result)
        } else {
            None
        }
    }
}
