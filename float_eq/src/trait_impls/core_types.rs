use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug, FloatUlps, Ulps};
use core::cell::{Cell, RefCell};

//------------------------------------------------------------------------------
// ref types
//------------------------------------------------------------------------------
macro_rules! impl_for_refs {
    // $a and $b are mutability
    (&$($a:ident)?, &$($b:ident)?) => {
        impl<A: ?Sized, B: ?Sized> FloatDiff<&$($b)? B> for &$($a)? A
        where
            A: FloatDiff<B>,
        {
            type Output = A::Output;

            #[inline]
            fn abs_diff(&self, other: &&$($b)? B) -> Self::Output {
                FloatDiff::abs_diff(*self, *other)
            }

            #[inline]
            fn ulps_diff(&self, other: &&$($b)? B) -> Option<Ulps<Self::Output>>
            where
                Ulps<Self::Output>: Sized
            {
                FloatDiff::ulps_diff(*self, *other)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEq<&$($b)? B> for &$($a)? A
        where
            A: FloatEq<B>,
        {
            type Epsilon = A::Epsilon;

            #[inline]
            fn eq_abs(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_abs(*self, *other, max_diff)
            }

            #[inline]
            fn eq_rel(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_rel(*self, *other, max_diff)
            }

            #[inline]
            fn eq_ulps(&self, other: &&$($b)? B, max_diff: &Ulps<Self::Epsilon>) -> bool {
                FloatEq::eq_ulps(*self, *other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqAll<&$($b)? B> for &$($a)? A
        where
            A: FloatEqAll<B>,
        {
            type AllEpsilon = A::AllEpsilon;

            #[inline]
            fn eq_abs_all(&self, other: &&$($b)? B, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_abs_all(*self, *other, max_diff)
            }

            #[inline]
            fn eq_rel_all(&self, other: &&$($b)? B, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_rel_all(*self, *other, max_diff)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &&$($b)? B, max_diff: &Ulps<Self::AllEpsilon>) -> bool {
                FloatEqAll::eq_ulps_all(*self, *other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqDebug<&$($b)? B> for &$($a)? A
        where
            A: FloatEqDebug<B>,
        {
            type DebugEpsilon = A::DebugEpsilon;

            #[inline]
            fn debug_abs_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::Epsilon
            ) -> Self::DebugEpsilon {
                FloatEqDebug::debug_abs_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_rel_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::Epsilon
            ) -> Self::DebugEpsilon {
                FloatEqDebug::debug_rel_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_ulps_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Ulps<Self::Epsilon>,
            ) -> Ulps<Self::DebugEpsilon>
            where
                Ulps<Self::DebugEpsilon>: Sized
            {
                FloatEqDebug::debug_ulps_epsilon(*self, *other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqAllDebug<&$($b)? B> for &$($a)? A
        where
            A: FloatEqAllDebug<B>,
        {
            type AllDebugEpsilon = A::AllDebugEpsilon;

            #[inline]
            fn debug_abs_all_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::AllEpsilon
            ) -> Self::AllDebugEpsilon {
                FloatEqAllDebug::debug_abs_all_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_rel_all_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::AllEpsilon
            ) -> Self::AllDebugEpsilon {
                FloatEqAllDebug::debug_rel_all_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Ulps<Self::AllEpsilon>,
            ) -> Ulps<Self::AllDebugEpsilon>
            where
                Ulps<Self::AllDebugEpsilon>: Sized
            {
                FloatEqAllDebug::debug_ulps_all_epsilon(*self, *other, max_diff)
            }
        }
    };
}

impl_for_refs!(&, &);
impl_for_refs!(&, &mut);
impl_for_refs!(&mut, &);
impl_for_refs!(&mut, &mut);

//------------------------------------------------------------------------------
// Option
//
// Note: The Option impls are over `impl<T>` and not `impl<A, B>` since that breaks
// type inference and makes it harder to use `None`.
//------------------------------------------------------------------------------
impl<T: FloatUlps> FloatUlps for Option<T>
where
    Ulps<T>: Sized,
{
    type Ulps = Option<Ulps<T>>;
}

impl<T: FloatDiff> FloatDiff for Option<T>
where
    Ulps<T::Output>: Sized,
{
    type Output = Option<T::Output>;

    #[inline]
    fn abs_diff(&self, other: &Option<T>) -> Self::Output {
        Some(FloatDiff::abs_diff(self.as_ref()?, other.as_ref()?))
    }

    #[inline]
    fn ulps_diff(&self, other: &Option<T>) -> Option<Ulps<Self::Output>> {
        Some(FloatDiff::ulps_diff(self.as_ref()?, other.as_ref()?))
    }
}

impl<T: FloatEq> FloatEq for Option<T>
where
    T::Epsilon: Sized,
    Ulps<T::Epsilon>: Sized,
{
    type Epsilon = Option<T::Epsilon>;

    #[inline]
    fn eq_abs(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEq::eq_abs(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_rel(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEq::eq_rel(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_ulps(&self, other: &Option<T>, max_diff: &Ulps<Self::Epsilon>) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEq::eq_ulps(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }
}

impl<T: FloatEqAll> FloatEqAll<Option<T>> for Option<T>
where
    T::AllEpsilon: Sized,
    Ulps<T::AllEpsilon>: Sized,
{
    type AllEpsilon = Option<T::AllEpsilon>;

    #[inline]
    fn eq_abs_all(&self, other: &Option<T>, max_diff: &Self::AllEpsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEqAll::eq_abs_all(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_rel_all(&self, other: &Option<T>, max_diff: &Self::AllEpsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEqAll::eq_rel_all(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Option<T>, max_diff: &Ulps<Self::AllEpsilon>) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEqAll::eq_ulps_all(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }
}

impl<T: FloatEqDebug> FloatEqDebug for Option<T>
where
    T::Epsilon: Sized,
    Ulps<T::Epsilon>: Sized,
    Ulps<T::DebugEpsilon>: Sized,
{
    type DebugEpsilon = Option<T::DebugEpsilon>;

    #[inline]
    fn debug_abs_epsilon(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        Some(FloatEqDebug::debug_abs_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_rel_epsilon(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        Some(FloatEqDebug::debug_rel_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        Some(FloatEqDebug::debug_ulps_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }
}

impl<T: FloatEqAllDebug> FloatEqAllDebug for Option<T>
where
    T::AllEpsilon: Sized,
    Ulps<T::AllEpsilon>: Sized,
    Ulps<T::AllDebugEpsilon>: Sized,
{
    type AllDebugEpsilon = Option<T::AllDebugEpsilon>;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Some(FloatEqAllDebug::debug_abs_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Some(FloatEqAllDebug::debug_rel_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Ulps<Self::AllEpsilon>,
    ) -> Ulps<Self::AllDebugEpsilon>
    where
        Ulps<Self::AllDebugEpsilon>: Sized,
    {
        Some(FloatEqAllDebug::debug_ulps_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }
}

//------------------------------------------------------------------------------
// Cell
//------------------------------------------------------------------------------
impl<A, B> FloatDiff<Cell<B>> for Cell<A>
where
    A: FloatDiff<B> + Copy,
    B: Copy,
{
    type Output = A::Output;

    #[inline]
    fn abs_diff(&self, other: &Cell<B>) -> Self::Output {
        FloatDiff::abs_diff(&self.get(), &other.get())
    }

    #[inline]
    fn ulps_diff(&self, other: &Cell<B>) -> Option<Ulps<Self::Output>>
    where
        Ulps<Self::Output>: Sized,
    {
        FloatDiff::ulps_diff(&self.get(), &other.get())
    }
}

impl<A, B> FloatEq<Cell<B>> for Cell<A>
where
    A: FloatEq<B> + Copy,
    B: Copy,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn eq_abs(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_abs(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_rel(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rel(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &Cell<B>, max_diff: &Ulps<Self::Epsilon>) -> bool {
        FloatEq::eq_ulps(&self.get(), &other.get(), max_diff)
    }
}

impl<A, B> FloatEqAll<Cell<B>> for Cell<A>
where
    A: FloatEqAll<B> + Copy,
    B: Copy,
{
    type AllEpsilon = A::AllEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &Cell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_abs_all(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &Cell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_rel_all(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Cell<B>, max_diff: &Ulps<Self::AllEpsilon>) -> bool {
        FloatEqAll::eq_ulps_all(&self.get(), &other.get(), max_diff)
    }
}

impl<A, B> FloatEqDebug<Cell<B>> for Cell<A>
where
    A: FloatEqDebug<B> + Copy,
    B: Copy,
{
    type DebugEpsilon = A::DebugEpsilon;

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
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon>
    where
        Ulps<Self::DebugEpsilon>: Sized,
    {
        FloatEqDebug::debug_ulps_epsilon(&self.get(), &other.get(), max_diff)
    }
}

impl<A, B> FloatEqAllDebug<Cell<B>> for Cell<A>
where
    A: FloatEqAllDebug<B> + Copy,
    B: Copy,
{
    type AllDebugEpsilon = A::AllDebugEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        FloatEqAllDebug::debug_abs_all_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        FloatEqAllDebug::debug_rel_all_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Ulps<Self::AllEpsilon>,
    ) -> Ulps<Self::AllDebugEpsilon>
    where
        Ulps<Self::AllDebugEpsilon>: Sized,
    {
        FloatEqAllDebug::debug_ulps_all_epsilon(&self.get(), &other.get(), max_diff)
    }
}

//------------------------------------------------------------------------------
// RefCell
//------------------------------------------------------------------------------
impl<A: ?Sized, B: ?Sized> FloatDiff<RefCell<B>> for RefCell<A>
where
    A: FloatDiff<B>,
{
    type Output = A::Output;

    #[inline]
    fn abs_diff(&self, other: &RefCell<B>) -> Self::Output {
        FloatDiff::abs_diff(&*self.borrow(), &*other.borrow())
    }

    #[inline]
    fn ulps_diff(&self, other: &RefCell<B>) -> Option<Ulps<Self::Output>>
    where
        Ulps<Self::Output>: Sized,
    {
        FloatDiff::ulps_diff(&*self.borrow(), &*other.borrow())
    }
}

impl<A: ?Sized, B: ?Sized> FloatEq<RefCell<B>> for RefCell<A>
where
    A: FloatEq<B>,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn eq_abs(&self, other: &RefCell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_abs(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_rel(&self, other: &RefCell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rel(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &RefCell<B>, max_diff: &Ulps<Self::Epsilon>) -> bool {
        FloatEq::eq_ulps(&*self.borrow(), &*other.borrow(), max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAll<RefCell<B>> for RefCell<A>
where
    A: FloatEqAll<B>,
{
    type AllEpsilon = A::AllEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &RefCell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_abs_all(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &RefCell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_rel_all(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &RefCell<B>, max_diff: &Ulps<Self::AllEpsilon>) -> bool {
        FloatEqAll::eq_ulps_all(&*self.borrow(), &*other.borrow(), max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqDebug<RefCell<B>> for RefCell<A>
where
    A: FloatEqDebug<B> + Copy,
    B: Copy,
{
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        FloatEqDebug::debug_abs_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_rel_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        FloatEqDebug::debug_rel_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon>
    where
        Ulps<Self::DebugEpsilon>: Sized,
    {
        FloatEqDebug::debug_ulps_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAllDebug<RefCell<B>> for RefCell<A>
where
    A: FloatEqAllDebug<B> + Copy,
    B: Copy,
{
    type AllDebugEpsilon = A::AllDebugEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        FloatEqAllDebug::debug_abs_all_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        FloatEqAllDebug::debug_rel_all_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Ulps<Self::AllEpsilon>,
    ) -> Ulps<Self::AllDebugEpsilon>
    where
        Ulps<Self::AllDebugEpsilon>: Sized,
    {
        FloatEqAllDebug::debug_ulps_all_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }
}

//------------------------------------------------------------------------------
// Slices
//------------------------------------------------------------------------------
impl<T: FloatUlps> FloatUlps for [T]
where
    Ulps<T>: Sized,
{
    type Ulps = [Ulps<T>];
}

impl<A, B> FloatEq<[B]> for [A]
where
    A: FloatEq<B>,
    A::Epsilon: Sized,
    Ulps<A::Epsilon>: Sized,
{
    type Epsilon = [A::Epsilon];

    #[inline]
    fn eq_abs(&self, other: &[B], max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_diff.iter())
                .all(|((a, b), eps)| a.eq_abs(b, eps))
    }

    #[inline]
    fn eq_rel(&self, other: &[B], max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_diff.iter())
                .all(|((a, b), eps)| a.eq_rel(b, eps))
    }

    #[inline]
    fn eq_ulps(&self, other: &[B], max_diff: &Ulps<Self::Epsilon>) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_diff.iter())
                .all(|((a, b), eps)| a.eq_ulps(b, eps))
    }
}

impl<A, B> FloatEqAll<[B]> for [A]
where
    A: FloatEqAll<B>,
{
    type AllEpsilon = A::AllEpsilon;

    #[inline]
    fn eq_abs_all(&self, other: &[B], max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_abs_all(b, max_diff))
    }

    #[inline]
    fn eq_rel_all(&self, other: &[B], max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_rel_all(b, max_diff))
    }

    #[inline]
    fn eq_ulps_all(&self, other: &[B], max_diff: &Ulps<Self::AllEpsilon>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_ulps_all(b, max_diff))
    }
}
