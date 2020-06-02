use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug};
use core::cell::Cell;

impl<A, B> FloatDiff<Cell<B>> for Cell<A>
where
    A: FloatDiff<B> + Copy,
    B: Copy,
{
    type AbsDiff = A::AbsDiff;
    type UlpsDiff = A::UlpsDiff;

    #[inline]
    fn abs_diff(&self, other: &Cell<B>) -> Self::AbsDiff {
        FloatDiff::abs_diff(&self.get(), &other.get())
    }

    #[inline]
    fn ulps_diff(&self, other: &Cell<B>) -> Option<Self::UlpsDiff> {
        FloatDiff::ulps_diff(&self.get(), &other.get())
    }
}

impl<A, B> FloatEq<Cell<B>> for Cell<A>
where
    A: FloatEq<B> + Copy,
    B: Copy,
{
    type Epsilon = A::Epsilon;
    type UlpsEpsilon = A::UlpsEpsilon;

    #[inline]
    fn eq_abs(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_abs(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_rel(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rel(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &Cell<B>, max_diff: &Self::UlpsEpsilon) -> bool {
        FloatEq::eq_ulps(&self.get(), &other.get(), max_diff)
    }
}

impl<A, B> FloatEqAll<Cell<B>> for Cell<A>
where
    A: FloatEqAll<B> + Copy,
    B: Copy,
{
    type Epsilon = A::Epsilon;
    type UlpsEpsilon = A::UlpsEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_abs_all(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_rel_all(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Cell<B>, max_diff: &Self::UlpsEpsilon) -> bool {
        FloatEqAll::eq_ulps_all(&self.get(), &other.get(), max_diff)
    }
}

impl<A, B> FloatEqDebug<Cell<B>> for Cell<A>
where
    A: FloatEqDebug<B> + Copy,
    B: Copy,
{
    type DebugEpsilon = A::DebugEpsilon;
    type DebugUlpsEpsilon = A::DebugUlpsEpsilon;

    #[inline]
    fn debug_abs_epsilon(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_abs_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_rel_epsilon(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_rel_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        FloatEqDebug::debug_ulps_epsilon(&self.get(), &other.get(), max_diff)
    }
}

impl<A, B> FloatEqAllDebug<Cell<B>> for Cell<A>
where
    A: FloatEqAllDebug<B> + Copy,
    B: Copy,
{
    type DebugEpsilon = A::DebugEpsilon;
    type DebugUlpsEpsilon = A::DebugUlpsEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_abs_all_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_rel_all_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::UlpsEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        FloatEqAllDebug::debug_ulps_all_epsilon(&self.get(), &other.get(), max_diff)
    }
}
