use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug};
use std::rc::Rc;
use std::sync::Arc;

//------------------------------------------------------------------------------
// FloatDiff
//------------------------------------------------------------------------------
impl<A: ?Sized, B: ?Sized> FloatDiff<Rc<B>> for Rc<A>
where
    A: FloatDiff<B>,
{
    type AbsDiff = A::AbsDiff;
    type UlpsDiff = A::UlpsDiff;

    #[inline]
    fn abs_diff(&self, other: &Rc<B>) -> Self::AbsDiff {
        FloatDiff::abs_diff(&**self, &**other)
    }

    #[inline]
    fn ulps_diff(&self, other: &Rc<B>) -> Option<Self::UlpsDiff> {
        FloatDiff::ulps_diff(&**self, &**other)
    }
}

impl<A: ?Sized, B: ?Sized> FloatDiff<Arc<B>> for Arc<A>
where
    A: FloatDiff<B>,
{
    type AbsDiff = A::AbsDiff;
    type UlpsDiff = A::UlpsDiff;

    #[inline]
    fn abs_diff(&self, other: &Arc<B>) -> Self::AbsDiff {
        FloatDiff::abs_diff(&**self, &**other)
    }

    #[inline]
    fn ulps_diff(&self, other: &Arc<B>) -> Option<Self::UlpsDiff> {
        FloatDiff::ulps_diff(&**self, &**other)
    }
}

//------------------------------------------------------------------------------
// FloatEq
//------------------------------------------------------------------------------
impl<A: ?Sized, B: ?Sized> FloatEq<Rc<B>> for Rc<A>
where
    A: FloatEq<B>,
{
    type Epsilon = A::Epsilon;
    type UlpsEpsilon = A::UlpsEpsilon;

    #[inline]
    fn eq_abs(&self, other: &Rc<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_abs(&**self, &**other, max_diff)
    }

    #[inline]
    fn eq_rel(&self, other: &Rc<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rel(&**self, &**other, max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &Rc<B>, max_diff: &Self::UlpsEpsilon) -> bool {
        FloatEq::eq_ulps(&**self, &**other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEq<Arc<B>> for Arc<A>
where
    A: FloatEq<B>,
{
    type Epsilon = A::Epsilon;
    type UlpsEpsilon = A::UlpsEpsilon;

    #[inline]
    fn eq_abs(&self, other: &Arc<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_abs(&**self, &**other, max_diff)
    }

    #[inline]
    fn eq_rel(&self, other: &Arc<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rel(&**self, &**other, max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &Arc<B>, max_diff: &Self::UlpsEpsilon) -> bool {
        FloatEq::eq_ulps(&**self, &**other, max_diff)
    }
}

//------------------------------------------------------------------------------
// FloatEqAll
//------------------------------------------------------------------------------
impl<A: ?Sized, B: ?Sized> FloatEqAll<Rc<B>> for Rc<A>
where
    A: FloatEqAll<B>,
{
    type Epsilon = A::Epsilon;
    type UlpsEpsilon = A::UlpsEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &Rc<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_abs_all(&**self, &**other, max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &Rc<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_rel_all(&**self, &**other, max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Rc<B>, max_diff: &Self::UlpsEpsilon) -> bool {
        FloatEqAll::eq_ulps_all(&**self, &**other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAll<Arc<B>> for Arc<A>
where
    A: FloatEqAll<B>,
{
    type Epsilon = A::Epsilon;
    type UlpsEpsilon = A::UlpsEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &Arc<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_abs_all(&**self, &**other, max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &Arc<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_rel_all(&**self, &**other, max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Arc<B>, max_diff: &Self::UlpsEpsilon) -> bool {
        FloatEqAll::eq_ulps_all(&**self, &**other, max_diff)
    }
}

//------------------------------------------------------------------------------
// FloatEqDebug
//------------------------------------------------------------------------------
impl<A: ?Sized, B: ?Sized> FloatEqDebug<Rc<B>> for Rc<A>
where
    A: FloatEqDebug<B> + Copy,
    B: Copy,
{
    type DebugEpsilon = A::DebugEpsilon;
    type DebugUlpsEpsilon = A::DebugUlpsEpsilon;

    #[inline]
    fn debug_abs_epsilon(&self, other: &Rc<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_abs_epsilon(&**self, &**other, max_diff)
    }

    #[inline]
    fn debug_rel_epsilon(&self, other: &Rc<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_rel_epsilon(&**self, &**other, max_diff)
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &Rc<B>,
        max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        FloatEqDebug::debug_ulps_epsilon(&**self, &**other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqDebug<Arc<B>> for Arc<A>
where
    A: FloatEqDebug<B> + Copy,
    B: Copy,
{
    type DebugEpsilon = A::DebugEpsilon;
    type DebugUlpsEpsilon = A::DebugUlpsEpsilon;

    #[inline]
    fn debug_abs_epsilon(&self, other: &Arc<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_abs_epsilon(&**self, &**other, max_diff)
    }

    #[inline]
    fn debug_rel_epsilon(&self, other: &Arc<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_rel_epsilon(&**self, &**other, max_diff)
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &Arc<B>,
        max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        FloatEqDebug::debug_ulps_epsilon(&**self, &**other, max_diff)
    }
}

//------------------------------------------------------------------------------
// FloatEqAllDebug
//------------------------------------------------------------------------------
impl<A: ?Sized, B: ?Sized> FloatEqAllDebug<Rc<B>> for Rc<A>
where
    A: FloatEqAllDebug<B> + Copy,
    B: Copy,
{
    type DebugEpsilon = A::DebugEpsilon;
    type DebugUlpsEpsilon = A::DebugUlpsEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(&self, other: &Rc<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_abs_all_epsilon(&**self, &**other, max_diff)
    }

    #[inline]
    fn debug_rel_all_epsilon(&self, other: &Rc<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_rel_all_epsilon(&**self, &**other, max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &Rc<B>,
        max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        FloatEqAllDebug::debug_ulps_all_epsilon(&**self, &**other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAllDebug<Arc<B>> for Arc<A>
where
    A: FloatEqAllDebug<B> + Copy,
    B: Copy,
{
    type DebugEpsilon = A::DebugEpsilon;
    type DebugUlpsEpsilon = A::DebugUlpsEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &Arc<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_abs_all_epsilon(&**self, &**other, max_diff)
    }

    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &Arc<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_rel_all_epsilon(&**self, &**other, max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &Arc<B>,
        max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        FloatEqAllDebug::debug_ulps_all_epsilon(&**self, &**other, max_diff)
    }
}
