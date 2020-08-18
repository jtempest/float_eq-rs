use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsEpsilon, UlpsEpsilon,
};
use core::cell::{Cell, RefCell};

//------------------------------------------------------------------------------
// ref types
//------------------------------------------------------------------------------
macro_rules! impl_for_refs {
    // $a and $b are mutability
    (&$($a:ident)?, &$($b:ident)?) => {
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
            fn eq_rmax(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_rmax(*self, *other, max_diff)
            }

            #[inline]
            fn eq_rmin(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_rmin(*self, *other, max_diff)
            }

            #[inline]
            fn eq_r1st(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_r1st(*self, *other, max_diff)
            }

            #[inline]
            fn eq_r2nd(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_r2nd(*self, *other, max_diff)
            }

            #[inline]
            fn eq_ulps(&self, other: &&$($b)? B, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
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
            fn eq_rmax_all(&self, other: &&$($b)? B, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_rmax_all(*self, *other, max_diff)
            }

            #[inline]
            fn eq_rmin_all(&self, other: &&$($b)? B, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_rmin_all(*self, *other, max_diff)
            }

            #[inline]
            fn eq_r1st_all(&self, other: &&$($b)? B, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_r1st_all(*self, *other, max_diff)
            }

            #[inline]
            fn eq_r2nd_all(&self, other: &&$($b)? B, max_diff: &Self::AllEpsilon) -> bool {
                FloatEqAll::eq_r2nd_all(*self, *other, max_diff)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &&$($b)? B, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
                FloatEqAll::eq_ulps_all(*self, *other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEq<&$($b)? B> for &$($a)? A
        where
            A: AssertFloatEq<B>,
        {
            type DebugAbsDiff = A::DebugAbsDiff;
            type DebugEpsilon = A::DebugEpsilon;

            #[inline]
            fn debug_abs_diff(&self, other: &&$($b)? B) -> Self::DebugAbsDiff {
                AssertFloatEq::debug_abs_diff(*self, *other)
            }

            #[inline]
            fn debug_ulps_diff(&self, other: &&$($b)? B) -> DebugUlpsDiff<Self::DebugAbsDiff> {
                AssertFloatEq::debug_ulps_diff(*self, *other)
            }

            #[inline]
            fn debug_abs_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::Epsilon
            ) -> Self::DebugEpsilon {
                AssertFloatEq::debug_abs_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_rmax_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::Epsilon
            ) -> Self::DebugEpsilon {
                AssertFloatEq::debug_rmax_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_rmin_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::Epsilon
            ) -> Self::DebugEpsilon {
                AssertFloatEq::debug_rmin_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_r1st_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::Epsilon
            ) -> Self::DebugEpsilon {
                AssertFloatEq::debug_r1st_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_r2nd_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::Epsilon
            ) -> Self::DebugEpsilon {
                AssertFloatEq::debug_r2nd_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_ulps_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &UlpsEpsilon<Self::Epsilon>,
            ) -> UlpsEpsilon<Self::DebugEpsilon>
            where
                UlpsEpsilon<Self::DebugEpsilon>: Sized
            {
                AssertFloatEq::debug_ulps_epsilon(*self, *other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEqAll<&$($b)? B> for &$($a)? A
        where
            A: AssertFloatEqAll<B>,
        {
            type AllDebugEpsilon = A::AllDebugEpsilon;

            #[inline]
            fn debug_abs_all_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::AllEpsilon
            ) -> Self::AllDebugEpsilon {
                AssertFloatEqAll::debug_abs_all_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_rmax_all_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::AllEpsilon
            ) -> Self::AllDebugEpsilon {
                AssertFloatEqAll::debug_rmax_all_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_rmin_all_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::AllEpsilon
            ) -> Self::AllDebugEpsilon {
                AssertFloatEqAll::debug_rmin_all_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_r1st_all_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::AllEpsilon
            ) -> Self::AllDebugEpsilon {
                AssertFloatEqAll::debug_r1st_all_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_r2nd_all_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Self::AllEpsilon
            ) -> Self::AllDebugEpsilon {
                AssertFloatEqAll::debug_r2nd_all_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &UlpsEpsilon<Self::AllEpsilon>,
            ) -> UlpsEpsilon<Self::AllDebugEpsilon>
            where
                UlpsEpsilon<Self::AllDebugEpsilon>: Sized
            {
                AssertFloatEqAll::debug_ulps_all_epsilon(*self, *other, max_diff)
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
//
// Open question: should None == None? it currently does not.
//------------------------------------------------------------------------------
impl<T: FloatEqUlpsEpsilon> FloatEqUlpsEpsilon for Option<T>
where
    UlpsEpsilon<T>: Sized,
{
    type UlpsEpsilon = Option<UlpsEpsilon<T>>;
}

impl<T: FloatEqDebugUlpsDiff> FloatEqDebugUlpsDiff for Option<T> {
    type DebugUlpsDiff = Option<DebugUlpsDiff<T>>;
}

impl<T: FloatEq> FloatEq for Option<T>
where
    T::Epsilon: Sized,
    UlpsEpsilon<T::Epsilon>: Sized,
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
    fn eq_rmax(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEq::eq_rmax(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_rmin(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEq::eq_rmin(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_r1st(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEq::eq_r1st(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_r2nd(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEq::eq_r2nd(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_ulps(&self, other: &Option<T>, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
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
    UlpsEpsilon<T::AllEpsilon>: Sized,
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
    fn eq_rmax_all(&self, other: &Option<T>, max_diff: &Self::AllEpsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEqAll::eq_rmax_all(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_rmin_all(&self, other: &Option<T>, max_diff: &Self::AllEpsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEqAll::eq_rmin_all(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_r1st_all(&self, other: &Option<T>, max_diff: &Self::AllEpsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEqAll::eq_r1st_all(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &Option<T>, max_diff: &Self::AllEpsilon) -> bool {
        self.is_some()
            && other.is_some()
            && max_diff.is_some()
            && FloatEqAll::eq_r2nd_all(
                self.as_ref().unwrap(),
                other.as_ref().unwrap(),
                max_diff.as_ref().unwrap(),
            )
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Option<T>, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
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

impl<T: AssertFloatEq> AssertFloatEq for Option<T>
where
    T::Epsilon: Sized,
    UlpsEpsilon<T::Epsilon>: Sized,
    UlpsEpsilon<T::DebugEpsilon>: Sized,
{
    type DebugAbsDiff = Option<T::DebugAbsDiff>;
    type DebugEpsilon = Option<T::DebugEpsilon>;

    #[inline]
    fn debug_abs_diff(&self, other: &Option<T>) -> Self::DebugAbsDiff {
        Some(AssertFloatEq::debug_abs_diff(
            self.as_ref()?,
            other.as_ref()?,
        ))
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Option<T>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        Some(AssertFloatEq::debug_ulps_diff(
            self.as_ref()?,
            other.as_ref()?,
        ))
    }

    #[inline]
    fn debug_abs_epsilon(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        Some(AssertFloatEq::debug_abs_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_rmax_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        Some(AssertFloatEq::debug_rmax_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_rmin_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        Some(AssertFloatEq::debug_rmin_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_r1st_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        Some(AssertFloatEq::debug_r1st_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_r2nd_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        Some(AssertFloatEq::debug_r2nd_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon> {
        Some(AssertFloatEq::debug_ulps_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }
}

impl<T: AssertFloatEqAll> AssertFloatEqAll for Option<T>
where
    T::AllEpsilon: Sized,
    UlpsEpsilon<T::AllEpsilon>: Sized,
    UlpsEpsilon<T::AllDebugEpsilon>: Sized,
{
    type AllDebugEpsilon = Option<T::AllDebugEpsilon>;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Some(AssertFloatEqAll::debug_abs_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_rmax_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Some(AssertFloatEqAll::debug_rmax_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_rmin_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Some(AssertFloatEqAll::debug_rmin_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_r1st_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Some(AssertFloatEqAll::debug_r1st_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_r2nd_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        Some(AssertFloatEqAll::debug_r2nd_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon>
    where
        UlpsEpsilon<Self::AllDebugEpsilon>: Sized,
    {
        Some(AssertFloatEqAll::debug_ulps_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff.as_ref()?,
        ))
    }
}

//------------------------------------------------------------------------------
// Cell
//------------------------------------------------------------------------------
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
    fn eq_rmax(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rmax(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_rmin(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rmin(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_r1st(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_r1st(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_r2nd(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_r2nd(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &Cell<B>, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
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
    fn eq_rmax_all(&self, other: &Cell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_rmax_all(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_rmin_all(&self, other: &Cell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_rmin_all(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_r1st_all(&self, other: &Cell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_r1st_all(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &Cell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_r2nd_all(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Cell<B>, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
        FloatEqAll::eq_ulps_all(&self.get(), &other.get(), max_diff)
    }
}

impl<A, B> AssertFloatEq<Cell<B>> for Cell<A>
where
    A: AssertFloatEq<B> + Copy,
    B: Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_diff(&self, other: &Cell<B>) -> Self::DebugAbsDiff {
        AssertFloatEq::debug_abs_diff(&self.get(), &other.get())
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Cell<B>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        AssertFloatEq::debug_ulps_diff(&self.get(), &other.get())
    }

    #[inline]
    fn debug_abs_epsilon(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        AssertFloatEq::debug_abs_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_rmax_epsilon(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        AssertFloatEq::debug_rmax_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_rmin_epsilon(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        AssertFloatEq::debug_rmin_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_r1st_epsilon(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        AssertFloatEq::debug_r1st_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_r2nd_epsilon(&self, other: &Cell<B>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        AssertFloatEq::debug_r2nd_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon>
    where
        UlpsEpsilon<Self::DebugEpsilon>: Sized,
    {
        AssertFloatEq::debug_ulps_epsilon(&self.get(), &other.get(), max_diff)
    }
}

impl<A, B> AssertFloatEqAll<Cell<B>> for Cell<A>
where
    A: AssertFloatEqAll<B> + Copy,
    B: Copy,
{
    type AllDebugEpsilon = A::AllDebugEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        AssertFloatEqAll::debug_abs_all_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_rmax_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        AssertFloatEqAll::debug_rmax_all_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_rmin_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        AssertFloatEqAll::debug_rmin_all_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_r1st_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        AssertFloatEqAll::debug_r1st_all_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_r2nd_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        AssertFloatEqAll::debug_r2nd_all_epsilon(&self.get(), &other.get(), max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &Cell<B>,
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon>
    where
        UlpsEpsilon<Self::AllDebugEpsilon>: Sized,
    {
        AssertFloatEqAll::debug_ulps_all_epsilon(&self.get(), &other.get(), max_diff)
    }
}

//------------------------------------------------------------------------------
// RefCell
//------------------------------------------------------------------------------
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
    fn eq_rmax(&self, other: &RefCell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rmax(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_rmin(&self, other: &RefCell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_rmin(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_r1st(&self, other: &RefCell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_r1st(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_r2nd(&self, other: &RefCell<B>, max_diff: &Self::Epsilon) -> bool {
        FloatEq::eq_r2nd(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &RefCell<B>, max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
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
    fn eq_rmax_all(&self, other: &RefCell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_rmax_all(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_rmin_all(&self, other: &RefCell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_rmin_all(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_r1st_all(&self, other: &RefCell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_r1st_all(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &RefCell<B>, max_diff: &Self::AllEpsilon) -> bool {
        FloatEqAll::eq_r2nd_all(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &RefCell<B>, max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
        FloatEqAll::eq_ulps_all(&*self.borrow(), &*other.borrow(), max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> AssertFloatEq<RefCell<B>> for RefCell<A>
where
    A: AssertFloatEq<B> + Copy,
    B: Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugEpsilon = A::DebugEpsilon;

    #[inline]
    fn debug_abs_diff(&self, other: &RefCell<B>) -> Self::DebugAbsDiff {
        AssertFloatEq::debug_abs_diff(&*self.borrow(), &*other.borrow())
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &RefCell<B>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        AssertFloatEq::debug_ulps_diff(&*self.borrow(), &*other.borrow())
    }

    #[inline]
    fn debug_abs_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        AssertFloatEq::debug_abs_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_rmax_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        AssertFloatEq::debug_rmax_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_rmin_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        AssertFloatEq::debug_rmin_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_r1st_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        AssertFloatEq::debug_r1st_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_r2nd_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        AssertFloatEq::debug_r2nd_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &UlpsEpsilon<Self::Epsilon>,
    ) -> UlpsEpsilon<Self::DebugEpsilon>
    where
        UlpsEpsilon<Self::DebugEpsilon>: Sized,
    {
        AssertFloatEq::debug_ulps_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }
}

impl<A: ?Sized, B: ?Sized> AssertFloatEqAll<RefCell<B>> for RefCell<A>
where
    A: AssertFloatEqAll<B> + Copy,
    B: Copy,
{
    type AllDebugEpsilon = A::AllDebugEpsilon;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        AssertFloatEqAll::debug_abs_all_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_rmax_all_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        AssertFloatEqAll::debug_rmax_all_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_rmin_all_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        AssertFloatEqAll::debug_rmin_all_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_r1st_all_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        AssertFloatEqAll::debug_r1st_all_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_r2nd_all_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &Self::AllEpsilon,
    ) -> Self::AllDebugEpsilon {
        AssertFloatEqAll::debug_r2nd_all_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &RefCell<B>,
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon>
    where
        UlpsEpsilon<Self::AllDebugEpsilon>: Sized,
    {
        AssertFloatEqAll::debug_ulps_all_epsilon(&*self.borrow(), &*other.borrow(), max_diff)
    }
}

//------------------------------------------------------------------------------
// Slices
//------------------------------------------------------------------------------
impl<T: FloatEqUlpsEpsilon> FloatEqUlpsEpsilon for [T]
where
    UlpsEpsilon<T>: Sized,
{
    type UlpsEpsilon = [UlpsEpsilon<T>];
}

impl<A, B> FloatEq<[B]> for [A]
where
    A: FloatEq<B>,
    A::Epsilon: Sized,
    UlpsEpsilon<A::Epsilon>: Sized,
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
    fn eq_rmax(&self, other: &[B], max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_diff.iter())
                .all(|((a, b), eps)| a.eq_rmax(b, eps))
    }

    #[inline]
    fn eq_rmin(&self, other: &[B], max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_diff.iter())
                .all(|((a, b), eps)| a.eq_rmin(b, eps))
    }

    #[inline]
    fn eq_r1st(&self, other: &[B], max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_diff.iter())
                .all(|((a, b), eps)| a.eq_r1st(b, eps))
    }

    #[inline]
    fn eq_r2nd(&self, other: &[B], max_diff: &Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.len() == max_diff.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_diff.iter())
                .all(|((a, b), eps)| a.eq_r2nd(b, eps))
    }

    #[inline]
    fn eq_ulps(&self, other: &[B], max_diff: &UlpsEpsilon<Self::Epsilon>) -> bool {
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
    fn eq_rmax_all(&self, other: &[B], max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_rmax_all(b, max_diff))
    }

    #[inline]
    fn eq_rmin_all(&self, other: &[B], max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_rmin_all(b, max_diff))
    }

    #[inline]
    fn eq_r1st_all(&self, other: &[B], max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_r1st_all(b, max_diff))
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &[B], max_diff: &Self::AllEpsilon) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_r2nd_all(b, max_diff))
    }

    #[inline]
    fn eq_ulps_all(&self, other: &[B], max_diff: &UlpsEpsilon<Self::AllEpsilon>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_ulps_all(b, max_diff))
    }
}
