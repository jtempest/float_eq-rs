use crate::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsTol, UlpsTol,
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
            type Tol = A::Tol;

            #[inline]
            fn eq_abs(&self, other: &&$($b)? B, tol: &Self::Tol) -> bool {
                FloatEq::eq_abs(*self, *other, tol)
            }

            #[inline]
            fn eq_rmax(&self, other: &&$($b)? B, tol: &Self::Tol) -> bool {
                FloatEq::eq_rmax(*self, *other, tol)
            }

            #[inline]
            fn eq_rmin(&self, other: &&$($b)? B, tol: &Self::Tol) -> bool {
                FloatEq::eq_rmin(*self, *other, tol)
            }

            #[inline]
            fn eq_r1st(&self, other: &&$($b)? B, tol: &Self::Tol) -> bool {
                FloatEq::eq_r1st(*self, *other, tol)
            }

            #[inline]
            fn eq_r2nd(&self, other: &&$($b)? B, tol: &Self::Tol) -> bool {
                FloatEq::eq_r2nd(*self, *other, tol)
            }

            #[inline]
            fn eq_ulps(&self, other: &&$($b)? B, tol: &UlpsTol<Self::Tol>) -> bool {
                FloatEq::eq_ulps(*self, *other, tol)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqAll<&$($b)? B> for &$($a)? A
        where
            A: FloatEqAll<B>,
        {
            type AllTol = A::AllTol;

            #[inline]
            fn eq_abs_all(&self, other: &&$($b)? B, tol: &Self::AllTol) -> bool {
                FloatEqAll::eq_abs_all(*self, *other, tol)
            }

            #[inline]
            fn eq_rmax_all(&self, other: &&$($b)? B, tol: &Self::AllTol) -> bool {
                FloatEqAll::eq_rmax_all(*self, *other, tol)
            }

            #[inline]
            fn eq_rmin_all(&self, other: &&$($b)? B, tol: &Self::AllTol) -> bool {
                FloatEqAll::eq_rmin_all(*self, *other, tol)
            }

            #[inline]
            fn eq_r1st_all(&self, other: &&$($b)? B, tol: &Self::AllTol) -> bool {
                FloatEqAll::eq_r1st_all(*self, *other, tol)
            }

            #[inline]
            fn eq_r2nd_all(&self, other: &&$($b)? B, tol: &Self::AllTol) -> bool {
                FloatEqAll::eq_r2nd_all(*self, *other, tol)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &&$($b)? B, tol: &UlpsTol<Self::AllTol>) -> bool {
                FloatEqAll::eq_ulps_all(*self, *other, tol)
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEq<&$($b)? B> for &$($a)? A
        where
            A: AssertFloatEq<B>,
        {
            type DebugAbsDiff = A::DebugAbsDiff;
            type DebugTol = A::DebugTol;

            #[inline]
            fn debug_abs_diff(&self, other: &&$($b)? B) -> Self::DebugAbsDiff {
                AssertFloatEq::debug_abs_diff(*self, *other)
            }

            #[inline]
            fn debug_ulps_diff(&self, other: &&$($b)? B) -> DebugUlpsDiff<Self::DebugAbsDiff> {
                AssertFloatEq::debug_ulps_diff(*self, *other)
            }

            #[inline]
            fn debug_abs_tol(
                &self,
                other: &&$($b)? B,
                tol: &Self::Tol
            ) -> Self::DebugTol {
                AssertFloatEq::debug_abs_tol(*self, *other, tol)
            }

            #[inline]
            fn debug_rmax_tol(
                &self,
                other: &&$($b)? B,
                tol: &Self::Tol
            ) -> Self::DebugTol {
                AssertFloatEq::debug_rmax_tol(*self, *other, tol)
            }

            #[inline]
            fn debug_rmin_tol(
                &self,
                other: &&$($b)? B,
                tol: &Self::Tol
            ) -> Self::DebugTol {
                AssertFloatEq::debug_rmin_tol(*self, *other, tol)
            }

            #[inline]
            fn debug_r1st_tol(
                &self,
                other: &&$($b)? B,
                tol: &Self::Tol
            ) -> Self::DebugTol {
                AssertFloatEq::debug_r1st_tol(*self, *other, tol)
            }

            #[inline]
            fn debug_r2nd_tol(
                &self,
                other: &&$($b)? B,
                tol: &Self::Tol
            ) -> Self::DebugTol {
                AssertFloatEq::debug_r2nd_tol(*self, *other, tol)
            }

            #[inline]
            fn debug_ulps_tol(
                &self,
                other: &&$($b)? B,
                tol: &UlpsTol<Self::Tol>,
            ) -> UlpsTol<Self::DebugTol>
            where
                UlpsTol<Self::DebugTol>: Sized
            {
                AssertFloatEq::debug_ulps_tol(*self, *other, tol)
            }
        }

        impl<A: ?Sized, B: ?Sized> AssertFloatEqAll<&$($b)? B> for &$($a)? A
        where
            A: AssertFloatEqAll<B>,
        {
            type AllDebugTol = A::AllDebugTol;

            #[inline]
            fn debug_abs_all_tol(
                &self,
                other: &&$($b)? B,
                tol: &Self::AllTol
            ) -> Self::AllDebugTol {
                AssertFloatEqAll::debug_abs_all_tol(*self, *other, tol)
            }

            #[inline]
            fn debug_rmax_all_tol(
                &self,
                other: &&$($b)? B,
                tol: &Self::AllTol
            ) -> Self::AllDebugTol {
                AssertFloatEqAll::debug_rmax_all_tol(*self, *other, tol)
            }

            #[inline]
            fn debug_rmin_all_tol(
                &self,
                other: &&$($b)? B,
                tol: &Self::AllTol
            ) -> Self::AllDebugTol {
                AssertFloatEqAll::debug_rmin_all_tol(*self, *other, tol)
            }

            #[inline]
            fn debug_r1st_all_tol(
                &self,
                other: &&$($b)? B,
                tol: &Self::AllTol
            ) -> Self::AllDebugTol {
                AssertFloatEqAll::debug_r1st_all_tol(*self, *other, tol)
            }

            #[inline]
            fn debug_r2nd_all_tol(
                &self,
                other: &&$($b)? B,
                tol: &Self::AllTol
            ) -> Self::AllDebugTol {
                AssertFloatEqAll::debug_r2nd_all_tol(*self, *other, tol)
            }

            #[inline]
            fn debug_ulps_all_tol(
                &self,
                other: &&$($b)? B,
                tol: &UlpsTol<Self::AllTol>,
            ) -> UlpsTol<Self::AllDebugTol>
            where
                UlpsTol<Self::AllDebugTol>: Sized
            {
                AssertFloatEqAll::debug_ulps_all_tol(*self, *other, tol)
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
impl<T: FloatEqUlpsTol> FloatEqUlpsTol for Option<T>
where
    UlpsTol<T>: Sized,
{
    type UlpsTol = Option<UlpsTol<T>>;
}

impl<T: FloatEqDebugUlpsDiff> FloatEqDebugUlpsDiff for Option<T> {
    type DebugUlpsDiff = Option<DebugUlpsDiff<T>>;
}

impl<T: FloatEq> FloatEq for Option<T>
where
    T::Tol: Sized,
    UlpsTol<T::Tol>: Sized,
{
    type Tol = Option<T::Tol>;

    #[inline]
    fn eq_abs(&self, other: &Option<T>, tol: &Self::Tol) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_abs(o, t)
        } else {
            false
        }
    }

    #[inline]
    fn eq_rmax(&self, other: &Option<T>, tol: &Self::Tol) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_rmax(o, t)
        } else {
            false
        }
    }

    #[inline]
    fn eq_rmin(&self, other: &Option<T>, tol: &Self::Tol) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_rmin(o, t)
        } else {
            false
        }
    }

    #[inline]
    fn eq_r1st(&self, other: &Option<T>, tol: &Self::Tol) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_r1st(o, t)
        } else {
            false
        }
    }

    #[inline]
    fn eq_r2nd(&self, other: &Option<T>, tol: &Self::Tol) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_r2nd(o, t)
        } else {
            false
        }
    }

    #[inline]
    fn eq_ulps(&self, other: &Option<T>, tol: &UlpsTol<Self::Tol>) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_ulps(o, t)
        } else {
            false
        }
    }
}

impl<T: FloatEqAll> FloatEqAll<Option<T>> for Option<T>
where
    T::AllTol: Sized,
    UlpsTol<T::AllTol>: Sized,
{
    type AllTol = Option<T::AllTol>;

    #[inline]
    fn eq_abs_all(&self, other: &Option<T>, tol: &Self::AllTol) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_abs_all(o, t)
        } else {
            false
        }
    }

    #[inline]
    fn eq_rmax_all(&self, other: &Option<T>, tol: &Self::AllTol) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_rmax_all(o, t)
        } else {
            false
        }
    }

    #[inline]
    fn eq_rmin_all(&self, other: &Option<T>, tol: &Self::AllTol) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_rmin_all(o, t)
        } else {
            false
        }
    }

    #[inline]
    fn eq_r1st_all(&self, other: &Option<T>, tol: &Self::AllTol) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_r1st_all(o, t)
        } else {
            false
        }
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &Option<T>, tol: &Self::AllTol) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_r2nd_all(o, t)
        } else {
            false
        }
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Option<T>, tol: &UlpsTol<Self::AllTol>) -> bool {
        if let (Some(s), Some(o), Some(t)) = (self, other, tol) {
            s.eq_ulps_all(o, t)
        } else {
            false
        }
    }
}

impl<T: AssertFloatEq> AssertFloatEq for Option<T>
where
    T::Tol: Sized,
    UlpsTol<T::Tol>: Sized,
    UlpsTol<T::DebugTol>: Sized,
{
    type DebugAbsDiff = Option<T::DebugAbsDiff>;
    type DebugTol = Option<T::DebugTol>;

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
    fn debug_abs_tol(&self, other: &Option<T>, tol: &Self::Tol) -> Self::DebugTol {
        Some(AssertFloatEq::debug_abs_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }

    #[inline]
    fn debug_rmax_tol(&self, other: &Option<T>, tol: &Self::Tol) -> Self::DebugTol {
        Some(AssertFloatEq::debug_rmax_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }

    #[inline]
    fn debug_rmin_tol(&self, other: &Option<T>, tol: &Self::Tol) -> Self::DebugTol {
        Some(AssertFloatEq::debug_rmin_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }

    #[inline]
    fn debug_r1st_tol(&self, other: &Option<T>, tol: &Self::Tol) -> Self::DebugTol {
        Some(AssertFloatEq::debug_r1st_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }

    #[inline]
    fn debug_r2nd_tol(&self, other: &Option<T>, tol: &Self::Tol) -> Self::DebugTol {
        Some(AssertFloatEq::debug_r2nd_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }

    #[inline]
    fn debug_ulps_tol(
        &self,
        other: &Option<T>,
        tol: &UlpsTol<Self::Tol>,
    ) -> UlpsTol<Self::DebugTol> {
        Some(AssertFloatEq::debug_ulps_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }
}

impl<T: AssertFloatEqAll> AssertFloatEqAll for Option<T>
where
    T::AllTol: Sized,
    UlpsTol<T::AllTol>: Sized,
    UlpsTol<T::AllDebugTol>: Sized,
{
    type AllDebugTol = Option<T::AllDebugTol>;

    #[inline]
    fn debug_abs_all_tol(&self, other: &Option<T>, tol: &Self::AllTol) -> Self::AllDebugTol {
        Some(AssertFloatEqAll::debug_abs_all_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }

    #[inline]
    fn debug_rmax_all_tol(&self, other: &Option<T>, tol: &Self::AllTol) -> Self::AllDebugTol {
        Some(AssertFloatEqAll::debug_rmax_all_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }

    #[inline]
    fn debug_rmin_all_tol(&self, other: &Option<T>, tol: &Self::AllTol) -> Self::AllDebugTol {
        Some(AssertFloatEqAll::debug_rmin_all_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }

    #[inline]
    fn debug_r1st_all_tol(&self, other: &Option<T>, tol: &Self::AllTol) -> Self::AllDebugTol {
        Some(AssertFloatEqAll::debug_r1st_all_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }

    #[inline]
    fn debug_r2nd_all_tol(&self, other: &Option<T>, tol: &Self::AllTol) -> Self::AllDebugTol {
        Some(AssertFloatEqAll::debug_r2nd_all_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
        ))
    }

    #[inline]
    fn debug_ulps_all_tol(
        &self,
        other: &Option<T>,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol>
    where
        UlpsTol<Self::AllDebugTol>: Sized,
    {
        Some(AssertFloatEqAll::debug_ulps_all_tol(
            &self.as_ref()?,
            &other.as_ref()?,
            tol.as_ref()?,
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
    type Tol = A::Tol;

    #[inline]
    fn eq_abs(&self, other: &Cell<B>, tol: &Self::Tol) -> bool {
        FloatEq::eq_abs(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn eq_rmax(&self, other: &Cell<B>, tol: &Self::Tol) -> bool {
        FloatEq::eq_rmax(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn eq_rmin(&self, other: &Cell<B>, tol: &Self::Tol) -> bool {
        FloatEq::eq_rmin(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn eq_r1st(&self, other: &Cell<B>, tol: &Self::Tol) -> bool {
        FloatEq::eq_r1st(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn eq_r2nd(&self, other: &Cell<B>, tol: &Self::Tol) -> bool {
        FloatEq::eq_r2nd(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn eq_ulps(&self, other: &Cell<B>, tol: &UlpsTol<Self::Tol>) -> bool {
        FloatEq::eq_ulps(&self.get(), &other.get(), tol)
    }
}

impl<A, B> FloatEqAll<Cell<B>> for Cell<A>
where
    A: FloatEqAll<B> + Copy,
    B: Copy,
{
    type AllTol = A::AllTol;

    #[inline]
    fn eq_abs_all(&self, other: &Cell<B>, tol: &Self::AllTol) -> bool {
        FloatEqAll::eq_abs_all(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn eq_rmax_all(&self, other: &Cell<B>, tol: &Self::AllTol) -> bool {
        FloatEqAll::eq_rmax_all(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn eq_rmin_all(&self, other: &Cell<B>, tol: &Self::AllTol) -> bool {
        FloatEqAll::eq_rmin_all(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn eq_r1st_all(&self, other: &Cell<B>, tol: &Self::AllTol) -> bool {
        FloatEqAll::eq_r1st_all(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &Cell<B>, tol: &Self::AllTol) -> bool {
        FloatEqAll::eq_r2nd_all(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Cell<B>, tol: &UlpsTol<Self::AllTol>) -> bool {
        FloatEqAll::eq_ulps_all(&self.get(), &other.get(), tol)
    }
}

impl<A, B> AssertFloatEq<Cell<B>> for Cell<A>
where
    A: AssertFloatEq<B> + Copy,
    B: Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTol = A::DebugTol;

    #[inline]
    fn debug_abs_diff(&self, other: &Cell<B>) -> Self::DebugAbsDiff {
        AssertFloatEq::debug_abs_diff(&self.get(), &other.get())
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Cell<B>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        AssertFloatEq::debug_ulps_diff(&self.get(), &other.get())
    }

    #[inline]
    fn debug_abs_tol(&self, other: &Cell<B>, tol: &Self::Tol) -> Self::DebugTol {
        AssertFloatEq::debug_abs_tol(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn debug_rmax_tol(&self, other: &Cell<B>, tol: &Self::Tol) -> Self::DebugTol {
        AssertFloatEq::debug_rmax_tol(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn debug_rmin_tol(&self, other: &Cell<B>, tol: &Self::Tol) -> Self::DebugTol {
        AssertFloatEq::debug_rmin_tol(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn debug_r1st_tol(&self, other: &Cell<B>, tol: &Self::Tol) -> Self::DebugTol {
        AssertFloatEq::debug_r1st_tol(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn debug_r2nd_tol(&self, other: &Cell<B>, tol: &Self::Tol) -> Self::DebugTol {
        AssertFloatEq::debug_r2nd_tol(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn debug_ulps_tol(&self, other: &Cell<B>, tol: &UlpsTol<Self::Tol>) -> UlpsTol<Self::DebugTol>
    where
        UlpsTol<Self::DebugTol>: Sized,
    {
        AssertFloatEq::debug_ulps_tol(&self.get(), &other.get(), tol)
    }
}

impl<A, B> AssertFloatEqAll<Cell<B>> for Cell<A>
where
    A: AssertFloatEqAll<B> + Copy,
    B: Copy,
{
    type AllDebugTol = A::AllDebugTol;

    #[inline]
    fn debug_abs_all_tol(&self, other: &Cell<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
        AssertFloatEqAll::debug_abs_all_tol(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn debug_rmax_all_tol(&self, other: &Cell<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
        AssertFloatEqAll::debug_rmax_all_tol(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn debug_rmin_all_tol(&self, other: &Cell<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
        AssertFloatEqAll::debug_rmin_all_tol(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn debug_r1st_all_tol(&self, other: &Cell<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
        AssertFloatEqAll::debug_r1st_all_tol(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn debug_r2nd_all_tol(&self, other: &Cell<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
        AssertFloatEqAll::debug_r2nd_all_tol(&self.get(), &other.get(), tol)
    }

    #[inline]
    fn debug_ulps_all_tol(
        &self,
        other: &Cell<B>,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol>
    where
        UlpsTol<Self::AllDebugTol>: Sized,
    {
        AssertFloatEqAll::debug_ulps_all_tol(&self.get(), &other.get(), tol)
    }
}

//------------------------------------------------------------------------------
// RefCell
//------------------------------------------------------------------------------
impl<A: ?Sized, B: ?Sized> FloatEq<RefCell<B>> for RefCell<A>
where
    A: FloatEq<B>,
{
    type Tol = A::Tol;

    #[inline]
    fn eq_abs(&self, other: &RefCell<B>, tol: &Self::Tol) -> bool {
        FloatEq::eq_abs(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn eq_rmax(&self, other: &RefCell<B>, tol: &Self::Tol) -> bool {
        FloatEq::eq_rmax(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn eq_rmin(&self, other: &RefCell<B>, tol: &Self::Tol) -> bool {
        FloatEq::eq_rmin(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn eq_r1st(&self, other: &RefCell<B>, tol: &Self::Tol) -> bool {
        FloatEq::eq_r1st(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn eq_r2nd(&self, other: &RefCell<B>, tol: &Self::Tol) -> bool {
        FloatEq::eq_r2nd(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn eq_ulps(&self, other: &RefCell<B>, tol: &UlpsTol<Self::Tol>) -> bool {
        FloatEq::eq_ulps(&*self.borrow(), &*other.borrow(), tol)
    }
}

impl<A: ?Sized, B: ?Sized> FloatEqAll<RefCell<B>> for RefCell<A>
where
    A: FloatEqAll<B>,
{
    type AllTol = A::AllTol;

    #[inline]
    fn eq_abs_all(&self, other: &RefCell<B>, tol: &Self::AllTol) -> bool {
        FloatEqAll::eq_abs_all(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn eq_rmax_all(&self, other: &RefCell<B>, tol: &Self::AllTol) -> bool {
        FloatEqAll::eq_rmax_all(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn eq_rmin_all(&self, other: &RefCell<B>, tol: &Self::AllTol) -> bool {
        FloatEqAll::eq_rmin_all(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn eq_r1st_all(&self, other: &RefCell<B>, tol: &Self::AllTol) -> bool {
        FloatEqAll::eq_r1st_all(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &RefCell<B>, tol: &Self::AllTol) -> bool {
        FloatEqAll::eq_r2nd_all(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &RefCell<B>, tol: &UlpsTol<Self::AllTol>) -> bool {
        FloatEqAll::eq_ulps_all(&*self.borrow(), &*other.borrow(), tol)
    }
}

impl<A: ?Sized, B: ?Sized> AssertFloatEq<RefCell<B>> for RefCell<A>
where
    A: AssertFloatEq<B> + Copy,
    B: Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTol = A::DebugTol;

    #[inline]
    fn debug_abs_diff(&self, other: &RefCell<B>) -> Self::DebugAbsDiff {
        AssertFloatEq::debug_abs_diff(&*self.borrow(), &*other.borrow())
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &RefCell<B>) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        AssertFloatEq::debug_ulps_diff(&*self.borrow(), &*other.borrow())
    }

    #[inline]
    fn debug_abs_tol(&self, other: &RefCell<B>, tol: &Self::Tol) -> Self::DebugTol {
        AssertFloatEq::debug_abs_tol(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn debug_rmax_tol(&self, other: &RefCell<B>, tol: &Self::Tol) -> Self::DebugTol {
        AssertFloatEq::debug_rmax_tol(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn debug_rmin_tol(&self, other: &RefCell<B>, tol: &Self::Tol) -> Self::DebugTol {
        AssertFloatEq::debug_rmin_tol(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn debug_r1st_tol(&self, other: &RefCell<B>, tol: &Self::Tol) -> Self::DebugTol {
        AssertFloatEq::debug_r1st_tol(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn debug_r2nd_tol(&self, other: &RefCell<B>, tol: &Self::Tol) -> Self::DebugTol {
        AssertFloatEq::debug_r2nd_tol(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn debug_ulps_tol(
        &self,
        other: &RefCell<B>,
        tol: &UlpsTol<Self::Tol>,
    ) -> UlpsTol<Self::DebugTol>
    where
        UlpsTol<Self::DebugTol>: Sized,
    {
        AssertFloatEq::debug_ulps_tol(&*self.borrow(), &*other.borrow(), tol)
    }
}

impl<A: ?Sized, B: ?Sized> AssertFloatEqAll<RefCell<B>> for RefCell<A>
where
    A: AssertFloatEqAll<B> + Copy,
    B: Copy,
{
    type AllDebugTol = A::AllDebugTol;

    #[inline]
    fn debug_abs_all_tol(&self, other: &RefCell<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
        AssertFloatEqAll::debug_abs_all_tol(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn debug_rmax_all_tol(&self, other: &RefCell<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
        AssertFloatEqAll::debug_rmax_all_tol(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn debug_rmin_all_tol(&self, other: &RefCell<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
        AssertFloatEqAll::debug_rmin_all_tol(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn debug_r1st_all_tol(&self, other: &RefCell<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
        AssertFloatEqAll::debug_r1st_all_tol(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn debug_r2nd_all_tol(&self, other: &RefCell<B>, tol: &Self::AllTol) -> Self::AllDebugTol {
        AssertFloatEqAll::debug_r2nd_all_tol(&*self.borrow(), &*other.borrow(), tol)
    }

    #[inline]
    fn debug_ulps_all_tol(
        &self,
        other: &RefCell<B>,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol>
    where
        UlpsTol<Self::AllDebugTol>: Sized,
    {
        AssertFloatEqAll::debug_ulps_all_tol(&*self.borrow(), &*other.borrow(), tol)
    }
}

//------------------------------------------------------------------------------
// Slices
//------------------------------------------------------------------------------
impl<T: FloatEqUlpsTol> FloatEqUlpsTol for [T]
where
    UlpsTol<T>: Sized,
{
    type UlpsTol = [UlpsTol<T>];
}

impl<A, B> FloatEq<[B]> for [A]
where
    A: FloatEq<B>,
    A::Tol: Sized,
    UlpsTol<A::Tol>: Sized,
{
    type Tol = [A::Tol];

    #[inline]
    fn eq_abs(&self, other: &[B], tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(tol.iter())
                .all(|((a, b), eps)| a.eq_abs(b, eps))
    }

    #[inline]
    fn eq_rmax(&self, other: &[B], tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(tol.iter())
                .all(|((a, b), eps)| a.eq_rmax(b, eps))
    }

    #[inline]
    fn eq_rmin(&self, other: &[B], tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(tol.iter())
                .all(|((a, b), eps)| a.eq_rmin(b, eps))
    }

    #[inline]
    fn eq_r1st(&self, other: &[B], tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(tol.iter())
                .all(|((a, b), eps)| a.eq_r1st(b, eps))
    }

    #[inline]
    fn eq_r2nd(&self, other: &[B], tol: &Self::Tol) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(tol.iter())
                .all(|((a, b), eps)| a.eq_r2nd(b, eps))
    }

    #[inline]
    fn eq_ulps(&self, other: &[B], tol: &UlpsTol<Self::Tol>) -> bool {
        self.len() == other.len()
            && self.len() == tol.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(tol.iter())
                .all(|((a, b), eps)| a.eq_ulps(b, eps))
    }
}

impl<A, B> FloatEqAll<[B]> for [A]
where
    A: FloatEqAll<B>,
{
    type AllTol = A::AllTol;

    #[inline]
    fn eq_abs_all(&self, other: &[B], tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_abs_all(b, tol))
    }

    #[inline]
    fn eq_rmax_all(&self, other: &[B], tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_rmax_all(b, tol))
    }

    #[inline]
    fn eq_rmin_all(&self, other: &[B], tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_rmin_all(b, tol))
    }

    #[inline]
    fn eq_r1st_all(&self, other: &[B], tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_r1st_all(b, tol))
    }

    #[inline]
    fn eq_r2nd_all(&self, other: &[B], tol: &Self::AllTol) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_r2nd_all(b, tol))
    }

    #[inline]
    fn eq_ulps_all(&self, other: &[B], tol: &UlpsTol<Self::AllTol>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.eq_ulps_all(b, tol))
    }
}
