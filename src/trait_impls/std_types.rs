use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug, Ulps};
use std::boxed::Box;
use std::rc::Rc;
use std::sync::Arc;

macro_rules! impl_traits_for_wrapper {
    ($t:ident) => {
        impl<A: ?Sized, B: ?Sized> FloatDiff<$t<B>> for $t<A>
        where
            A: FloatDiff<B>,
        {
            type Output = A::Output;

            #[inline]
            fn abs_diff(&self, other: &$t<B>) -> Option<Self::Output> {
                FloatDiff::abs_diff(&**self, &**other)
            }

            #[inline]
            fn ulps_diff(&self, other: &$t<B>) -> Option<Ulps<Self::Output>> {
                FloatDiff::ulps_diff(&**self, &**other)
            }
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
            fn eq_rel(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_rel(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_ulps(&self, other: &$t<B>, max_diff: &Ulps<Self::Epsilon>) -> bool {
                FloatEq::eq_ulps(&**self, &**other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqAll<$t<B>> for $t<A>
        where
            A: FloatEqAll<B>,
        {
            type Epsilon = A::Epsilon;

            #[inline]
            fn eq_abs_all(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                FloatEqAll::eq_abs_all(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_rel_all(&self, other: &$t<B>, max_diff: &Self::Epsilon) -> bool {
                FloatEqAll::eq_rel_all(&**self, &**other, max_diff)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &$t<B>, max_diff: &Ulps<Self::Epsilon>) -> bool {
                FloatEqAll::eq_ulps_all(&**self, &**other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqDebug<$t<B>> for $t<A>
        where
            A: FloatEqDebug<B> + Copy,
            B: Copy,
        {
            type DebugEpsilon = A::DebugEpsilon;

            #[inline]
            fn debug_abs_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                FloatEqDebug::debug_abs_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_rel_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                FloatEqDebug::debug_rel_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_ulps_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Ulps<Self::Epsilon>,
            ) -> Ulps<Self::DebugEpsilon> {
                FloatEqDebug::debug_ulps_epsilon(&**self, &**other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqAllDebug<$t<B>> for $t<A>
        where
            A: FloatEqAllDebug<B> + Copy,
            B: Copy,
        {
            type DebugEpsilon = A::DebugEpsilon;

            #[inline]
            fn debug_abs_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                FloatEqAllDebug::debug_abs_all_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_rel_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Self::Epsilon,
            ) -> Self::DebugEpsilon {
                FloatEqAllDebug::debug_rel_all_epsilon(&**self, &**other, max_diff)
            }

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &$t<B>,
                max_diff: &Ulps<Self::Epsilon>,
            ) -> Ulps<Self::DebugEpsilon> {
                FloatEqAllDebug::debug_ulps_all_epsilon(&**self, &**other, max_diff)
            }
        }
    };
}

impl_traits_for_wrapper!(Arc);
impl_traits_for_wrapper!(Box);
impl_traits_for_wrapper!(Rc);
