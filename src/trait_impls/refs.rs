use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug, Ulps};

//------------------------------------------------------------------------------
// FloatDiff
//------------------------------------------------------------------------------
impl<A: ?Sized, B: ?Sized> FloatDiff<&B> for &A
where
    A: FloatDiff<B>,
{
    type Output = A::Output;

    #[inline]
    fn abs_diff(&self, other: &&B) -> Self::Output {
        FloatDiff::abs_diff(*self, *other)
    }

    #[inline]
    fn ulps_diff(&self, other: &&B) -> Option<Ulps<Self::Output>> {
        FloatDiff::ulps_diff(*self, *other)
    }
}

impl<A: ?Sized, B: ?Sized> FloatDiff<&B> for &mut A
where
    A: FloatDiff<B>,
{
    type Output = A::Output;

    #[inline]
    fn abs_diff(&self, other: &&B) -> Self::Output {
        FloatDiff::abs_diff(*self, *other)
    }

    #[inline]
    fn ulps_diff(&self, other: &&B) -> Option<Ulps<Self::Output>> {
        FloatDiff::ulps_diff(*self, *other)
    }
}

impl<A: ?Sized, B: ?Sized> FloatDiff<&mut B> for &A
where
    A: FloatDiff<B>,
{
    type Output = A::Output;

    #[inline]
    fn abs_diff(&self, other: &&mut B) -> Self::Output {
        FloatDiff::abs_diff(*self, *other)
    }

    #[inline]
    fn ulps_diff(&self, other: &&mut B) -> Option<Ulps<Self::Output>> {
        FloatDiff::ulps_diff(*self, *other)
    }
}

impl<A: ?Sized, B: ?Sized> FloatDiff<&mut B> for &mut A
where
    A: FloatDiff<B>,
{
    type Output = A::Output;

    #[inline]
    fn abs_diff(&self, other: &&mut B) -> Self::Output {
        FloatDiff::abs_diff(*self, *other)
    }

    #[inline]
    fn ulps_diff(&self, other: &&mut B) -> Option<Ulps<Self::Output>> {
        FloatDiff::ulps_diff(*self, *other)
    }
}

//------------------------------------------------------------------------------
// FloatEq
//------------------------------------------------------------------------------

impl<A: ?Sized, B: ?Sized> FloatEq<&B> for &A
where
    A: FloatEq<B>,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn eq_abs(&self, other: &&B, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_abs(*self, *other, max_diff)
    }

    #[inline]
    fn eq_rel(&self, other: &&B, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rel(*self, *other, max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &&B, max_diff: &Ulps<Self::Epsilon>) -> bool {
        FloatEq::eq_ulps(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEq<&B> for &mut A
where
    A: FloatEq<B>,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn eq_abs(&self, other: &&B, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_abs(*self, *other, max_diff)
    }

    #[inline]
    fn eq_rel(&self, other: &&B, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rel(*self, *other, max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &&B, max_diff: &Ulps<Self::Epsilon>) -> bool {
        FloatEq::eq_ulps(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEq<&mut B> for &A
where
    A: FloatEq<B>,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn eq_abs(&self, other: &&mut B, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_abs(*self, *other, max_diff)
    }

    #[inline]
    fn eq_rel(&self, other: &&mut B, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rel(*self, *other, max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &&mut B, max_diff: &Ulps<Self::Epsilon>) -> bool {
        FloatEq::eq_ulps(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEq<&mut B> for &mut A
where
    A: FloatEq<B>,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn eq_abs(&self, other: &&mut B, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_abs(*self, *other, max_diff)
    }

    #[inline]
    fn eq_rel(&self, other: &&mut B, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rel(*self, *other, max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &&mut B, max_diff: &Ulps<Self::Epsilon>) -> bool {
        FloatEq::eq_ulps(*self, *other, max_diff)
    }
}

//------------------------------------------------------------------------------
// FloatEqAll
//------------------------------------------------------------------------------

impl<A: ?Sized, B: ?Sized> FloatEqAll<&B> for &A
where
    A: FloatEqAll<B>,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn eq_abs_all(&self, other: &&B, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_abs_all(*self, *other, max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &&B, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_rel_all(*self, *other, max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &&B, max_diff: &Ulps<Self::Epsilon>) -> bool {
        FloatEqAll::eq_ulps_all(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAll<&B> for &mut A
where
    A: FloatEqAll<B>,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn eq_abs_all(&self, other: &&B, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_abs_all(*self, *other, max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &&B, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_rel_all(*self, *other, max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &&B, max_diff: &Ulps<Self::Epsilon>) -> bool {
        FloatEqAll::eq_ulps_all(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAll<&mut B> for &A
where
    A: FloatEqAll<B>,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn eq_abs_all(&self, other: &&mut B, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_abs_all(*self, *other, max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &&mut B, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_rel_all(*self, *other, max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &&mut B, max_diff: &Ulps<Self::Epsilon>) -> bool {
        FloatEqAll::eq_ulps_all(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAll<&mut B> for &mut A
where
    A: FloatEqAll<B>,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn eq_abs_all(&self, other: &&mut B, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_abs_all(*self, *other, max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &&mut B, max_diff: &Self::Epsilon) -> bool {
        FloatEqAll::eq_rel_all(*self, *other, max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &&mut B, max_diff: &Ulps<Self::Epsilon>) -> bool {
        FloatEqAll::eq_ulps_all(*self, *other, max_diff)
    }
}

//------------------------------------------------------------------------------
// FloatEqDebug
//------------------------------------------------------------------------------

impl<A: ?Sized, B: ?Sized> FloatEqDebug<&B> for &A
where
    A: FloatEqDebug<B>,
{
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_epsilon(&self, other: &&B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_abs_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_rel_epsilon(&self, other: &&B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_rel_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &&B,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        FloatEqDebug::debug_ulps_epsilon(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqDebug<&B> for &mut A
where
    A: FloatEqDebug<B>,
{
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_epsilon(&self, other: &&B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_abs_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_rel_epsilon(&self, other: &&B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_rel_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &&B,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        FloatEqDebug::debug_ulps_epsilon(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqDebug<&mut B> for &A
where
    A: FloatEqDebug<B>,
{
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_epsilon(&self, other: &&mut B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_abs_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_rel_epsilon(&self, other: &&mut B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_rel_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &&mut B,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        FloatEqDebug::debug_ulps_epsilon(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqDebug<&mut B> for &mut A
where
    A: FloatEqDebug<B>,
{
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_epsilon(&self, other: &&mut B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_abs_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_rel_epsilon(&self, other: &&mut B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqDebug::debug_rel_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &&mut B,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        FloatEqDebug::debug_ulps_epsilon(*self, *other, max_diff)
    }
}

//------------------------------------------------------------------------------
// FloatEqAllDebug
//------------------------------------------------------------------------------

impl<A: ?Sized, B: ?Sized> FloatEqAllDebug<&B> for &A
where
    A: FloatEqAllDebug<B>,
{
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(&self, other: &&B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_abs_all_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_rel_all_epsilon(&self, other: &&B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_rel_all_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &&B,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        FloatEqAllDebug::debug_ulps_all_epsilon(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAllDebug<&B> for &mut A
where
    A: FloatEqAllDebug<B>,
{
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(&self, other: &&B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_abs_all_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_rel_all_epsilon(&self, other: &&B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_rel_all_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &&B,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        FloatEqAllDebug::debug_ulps_all_epsilon(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAllDebug<&mut B> for &A
where
    A: FloatEqAllDebug<B>,
{
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &&mut B,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_abs_all_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &&mut B,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_rel_all_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &&mut B,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        FloatEqAllDebug::debug_ulps_all_epsilon(*self, *other, max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAllDebug<&mut B> for &mut A
where
    A: FloatEqAllDebug<B>,
{
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &&mut B,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_abs_all_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &&mut B,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        FloatEqAllDebug::debug_rel_all_epsilon(*self, *other, max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &&mut B,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        FloatEqAllDebug::debug_ulps_all_epsilon(*self, *other, max_diff)
    }
}
