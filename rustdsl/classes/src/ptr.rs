use alloc::rc::{Rc, Weak};
use core::{
    alloc::Layout,
    cell::Cell,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    num::NonZero,
    ops::Deref,
    ptr::NonNull,
};

use crate::class::{
    ClassImpl, ClassVtable, ClassVtableBase, ConcreteClass, HasImpl, HasSuper, MixinHasImpl,
    VtableHasImpl, VtableHasSuper,
};
use crate::{
    class::MixinClassImpl,
    vtable::{MixinData, MixinInstanceType, MixinVtable, MixinVtableHeader, Type, cast_failed},
};

#[repr(transparent)]
pub struct Dyn<D>(D, PhantomData<D>);

#[repr(C)]
pub struct FatPtr<D, V> {
    data: NonNull<D>,
    vtable: NonNull<V>,
}

impl<D, V> Clone for FatPtr<D, V> {
    fn clone(&self) -> Self {
        Self {
            data: self.data,
            vtable: self.vtable,
        }
    }
}
impl<D, V> Copy for FatPtr<D, V> {}

// According to dart, vtables are not considered in the equality and hash code of objects.
impl<D, E, V, W> PartialEq<FatPtr<E, W>> for FatPtr<D, V> {
    fn eq(&self, other: &FatPtr<E, W>) -> bool {
        self.data.addr() == other.data.addr() // && core::ptr::addr_eq(self.vtable, other.vtable)
    }
}

impl<D, V> Eq for FatPtr<D, V> {}

impl<D, V> Hash for FatPtr<D, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        // core::ptr::from_ref(self.vtable).hash(state);
    }
}

impl<D, V: ClassVtable> fmt::Debug for FatPtr<D, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:p} as *const {:?}", self.data, self.vtable().ty())
    }
}

impl<D, V: ClassVtable> fmt::Display for FatPtr<D, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}#{:05x}",
            self.vtable().ty(),
            // Classes are typically aligned to 16, so the lowest 4 bits are useless
            (self.addr().get() >> 4) & 0xfffff
        )
    }
}

pub type PtrDyn<V> = FatPtr<NonNull<<V as ClassVtableBase>::Data>, V>;
pub type MixinPtrDyn<M> = FatPtr<NonNull<MixinData<<M as ClassVtableBase>::Data>>, MixinVtable<M>>;

impl<D, V> FatPtr<D, V> {
    #[inline]
    pub fn addr(self) -> NonZero<usize> {
        self.data.addr()
    }
    pub const fn ptr(self) -> NonNull<D> {
        self.data
    }
    pub const fn vtable(&self) -> &V {
        unsafe { self.vtable.as_ref() }
    }
}

impl<M: ClassVtableBase> MixinPtrDyn<M> {
    #[inline]
    #[track_caller]
    pub fn mixin_cast_impl<I: ClassVtable>(self) -> PtrDyn<I> {
        let vtable = self.vtable();
        debug_assert!(
            I::TYPE
                .as_mixin()
                .is_none_or(|mixin| { vtable.ty().mixin_instance_of(mixin).is_some() })
        );
        let offset = vtable
            .mixin_header()
            .instance()
            .offset_of_impl_with_mixin_header(I::TYPE);
        let mut this = PtrDyn {
            data: self.data.cast(),
            vtable: unsafe { self.vtable.byte_add(offset).cast() },
        };
        if let Some(mixin_instance) = I::TYPE
            .as_mixin()
            .and_then(|mixin| vtable.ty().mixin_instance_of(mixin))
        {
            this.vtable = unsafe { this.vtable.byte_sub(mixin_instance.mixin_offset()) };
        }
        this
    }

    #[inline]
    #[track_caller]
    pub fn mixin_downcast<S: ClassVtable>(self) -> PtrDyn<S> {
        self.mixin_try_downcast()
            .unwrap_or_else(|| cast_failed(S::TYPE, self.vtable().ty()))
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn mixin_try_downcast<S: ClassVtable>(self) -> Option<PtrDyn<S>> {
        S::TYPE
            .is_subclass_of(self.vtable().ty())
            .then(|| unsafe { self.mixin_downcast_unchecked() })
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn mixin_downcast_unchecked<S: ClassVtable>(self) -> PtrDyn<S> {
        debug_assert!(S::TYPE.is_subclass_of(self.vtable().mixin_header().instance().as_type()));
        let vtable = self.vtable().mixin_header().instance().mixin_offset();
        PtrDyn {
            data: self.data.cast(),
            vtable: unsafe { self.vtable.byte_add(vtable).cast() },
        }
    }
}

impl<V: ClassVtable> PtrDyn<V> {
    #[inline]
    pub fn cast_super(self) -> PtrDyn<V::SuperVtable>
    where
        V: VtableHasSuper,
    {
        PtrDyn {
            data: self.data.cast(),
            vtable: self.vtable.cast(),
        }
    }

    #[inline]
    pub fn cast_impl<I: ClassImpl>(self) -> PtrDyn<I::Vtable>
    where
        V: VtableHasImpl<I>,
    {
        PtrDyn {
            data: self.data.cast(),
            vtable: VtableHasImpl::cast_impl(self.vtable),
        }
    }

    /// Cast the pointer to its superclass `A`.
    ///
    /// # Safety
    /// `A` must be a superclass of `C`.
    #[inline]
    #[track_caller]
    pub fn cast_superclass<A: ClassVtable>(self) -> PtrDyn<A> {
        crate::static_assert_subclass!(V, A);
        debug_assert!(self.vtable().is_subclass_of::<A>());
        // `is_subclass_of` is also checked in `#[cfg(not(debug_assertions))]` now.
        unsafe { self.cast_superclass_unchecked() }
        // self.try_cast_superclass()
        //     .unwrap_or_else(|| cast_failed(A::TYPE, self.vtable().ty()))
    }

    /// Cast the pointer to its superclass `A`.
    ///
    /// # Safety
    /// `A` must be a superclass of `V`.
    #[inline]
    // #[cfg_attr(debug_assertions, track_caller)]
    #[track_caller]
    pub unsafe fn cast_superclass_unchecked<A: ClassVtable>(self) -> PtrDyn<A> {
        crate::static_assert_subclass!(V, A);
        debug_assert!(self.vtable().is_subclass_of::<A>());
        PtrDyn {
            data: self.data.cast(),
            vtable: self.vtable.cast(),
        }
    }

    /// Cast the pointer to its superclass `A`.
    ///
    /// # Safety
    /// `A` must be a superclass of `V`.
    #[inline]
    // #[cfg_attr(debug_assertions, track_caller)]
    #[track_caller]
    pub fn try_cast_superclass<A: ClassVtable>(self) -> Option<PtrDyn<A>> {
        crate::static_assert_subclass!(V, A);
        debug_assert!(self.vtable().is_subclass_of::<A>());
        Some(unsafe { self.cast_superclass_unchecked() })
        // self.vtable()
        //     .is_subclass_of::<A>()
        //     .then(|| unsafe { self.cast_superclass_unchecked() })
    }

    /// Cast the pointer to its subclass `D`.
    ///
    /// # Safety
    /// `D` must be a subclass of `V`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn cast_subclass_unchecked<D: ClassVtable>(self) -> PtrDyn<D> {
        crate::static_assert_subclass!(D, V);
        debug_assert!(self.vtable().is_subclass_of::<D>());
        PtrDyn {
            data: self.data.cast(),
            vtable: self.vtable.cast(),
        }
    }

    /// Cast the pointer to its subclass `D`.
    ///
    /// # Safety
    /// `D` must be a subclass of `V`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_cast_subclass<D: ClassVtable>(self) -> Option<PtrDyn<D>> {
        crate::static_assert_subclass!(D, V);
        self.vtable()
            .is_subclass_of::<D>()
            .then(|| unsafe { self.cast_subclass_unchecked() })
    }

    /// Cast the pointer to its subclass `D`.
    ///
    /// # Safety
    /// `D` must be a subclass of `C`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn cast_subclass<D: ClassVtable>(self) -> PtrDyn<D> {
        crate::static_assert_subclass!(D, V);
        self.try_cast_subclass()
            .unwrap_or_else(|| cast_failed(D::TYPE, self.vtable().ty()))
    }

    /// Cast the pointer to its supertype `S`.
    pub fn cast_supertype<S: ClassVtable>(self) -> PtrDyn<S> {
        crate::static_assert_subtype!(V, S);
        self.try_cast_supertype()
            .unwrap_or_else(|| cast_failed(S::TYPE, self.vtable().ty()))
    }

    /// Cast the pointer to its supertype `S`.
    pub fn try_cast_supertype<S: ClassVtable>(self) -> Option<PtrDyn<S>> {
        Some(PtrDyn {
            data: self.data.cast(),
            vtable: NonNull::from(crate::vtable::vtable_upcast(self.vtable())?),
        })
    }

    /// Cast the pointer to its subtype `D`.
    pub fn try_cast_subtype<D: ClassVtable>(self) -> Option<PtrDyn<D>> {
        Some(PtrDyn {
            data: self.data.cast(),
            vtable: NonNull::from(crate::vtable::vtable_cast(self.vtable())?),
        })
    }

    /// Cast the pointer to its subtype `D`.
    #[track_caller]
    pub fn cast_subtype<D: ClassVtable>(self) -> PtrDyn<D> {
        self.try_cast_subtype()
            .unwrap_or_else(|| cast_failed(D::TYPE, self.vtable().ty()))
    }

    /// Upcast this pointer to the supertype `I`, with an offset `OFFSET`.
    ///
    /// # Safety
    ///
    /// `OFFSET` must be the offset of `I` in vtable `V`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn upcast_with_offset_unchecked<I: ClassVtable, const OFFSET: usize>(
        self,
    ) -> PtrDyn<I> {
        crate::static_assert_subtype_with_offset!(V, I, OFFSET);
        PtrDyn {
            data: self.data.cast(),
            vtable: unsafe { self.vtable.byte_add(OFFSET).cast() },
        }
    }

    /// Upcast this pointer to the subtype `I`, with an offset `OFFSET`.
    ///
    /// # Safety
    ///
    /// `OFFSET` must be the offset of `C` in vtable `I`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn downcast_with_offset_unchecked<I: ClassVtable, const OFFSET: usize>(
        self,
    ) -> PtrDyn<I> {
        crate::static_assert_subtype_with_offset!(I, V, OFFSET);
        PtrDyn {
            data: self.data.cast(),
            vtable: unsafe { self.vtable.byte_sub(OFFSET).cast() },
        }
    }

    /// First cast the pointer to its superclass `A`, then cast to
    /// the interface of superclass `I`
    ///
    /// # Safety
    /// `A` must be a superclass of `V`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn upcast_unchecked<A: VtableHasImpl<I>, I: ClassImpl>(self) -> PtrDyn<I::Vtable> {
        crate::static_assert_subclass!(V, A);
        unsafe { self.cast_superclass_unchecked::<A>() }.cast_impl()
    }

    /// First cast the pointer to its superclass `A`, then cast to
    /// the interface of superclass `I`
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_upcast<A: VtableHasImpl<I>, I: ClassImpl>(self) -> Option<PtrDyn<I::Vtable>> {
        crate::static_assert_subclass!(V, A);
        Some(self.try_cast_superclass::<A>()?.cast_impl())
    }

    /// First cast the pointer to its superclass `A`, then cast to
    /// the interface of superclass `I`
    #[inline]
    #[track_caller]
    pub fn upcast<A: VtableHasImpl<I>, I: ClassImpl>(self) -> PtrDyn<I::Vtable> {
        crate::static_assert_subclass!(V, A);
        self.cast_superclass::<A>().cast_impl()
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn cast_mixin_unchecked<M: ClassVtableBase>(
        self,
        instance: MixinInstanceType,
    ) -> MixinPtrDyn<M> {
        let mut vtable = self.vtable;
        // Add the offset of the mixin instance, if it is a mixin pointer.
        if let Some(mixin) = V::TYPE.as_mixin() {
            let header =
                unsafe { &*core::ptr::from_ref(self.vtable()).cast::<MixinVtableHeader>() };
            debug_assert_eq!(header.instance().mixin(), mixin);
            vtable = unsafe { vtable.byte_add(header.instance().mixin_offset()) };
        }
        debug_assert!(M::KIND.is_mixin());
        debug_assert_eq!(self.vtable().mixin_instance_of::<M>(), Some(instance));
        debug_assert_eq!(M::TYPE.as_mixin(), Some(instance.mixin()));
        PtrDyn {
            data: self.data.cast(),
            vtable: unsafe { vtable.byte_sub(instance.mixin_offset()) }.cast(),
        }
    }

    #[inline]
    #[track_caller]
    pub fn cast_mixin<M: ClassVtableBase>(self) -> MixinPtrDyn<M> {
        debug_assert!(M::KIND.is_mixin());
        self.try_cast_mixin::<M>()
            .unwrap_or_else(|| cast_failed(M::TYPE, self.vtable().ty()))
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_cast_mixin<M: ClassVtableBase>(self) -> Option<MixinPtrDyn<M>> {
        debug_assert!(M::KIND.is_mixin());
        unsafe { Some(self.cast_mixin_unchecked(self.vtable().mixin_instance_of::<M>()?)) }
    }

    /// First downcast to a base class `B` that implements this interface,
    /// then downcast to the subclass `S` of `B`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_downcast<C: ClassImpl<Vtable = V>, B: VtableHasImpl<C>, S: ClassVtable>(
        self,
    ) -> Option<PtrDyn<S>> {
        crate::static_assert_subclass!(S, B);
        self.vtable()
            .is_subclass_of::<S>()
            .then(|| unsafe { self.downcast_unchecked::<C, B, S>() })
    }

    /// First downcast to a base class `B` that implements this interface,
    /// then downcast to the subclass `S` of `B`.
    #[inline]
    #[track_caller]
    pub fn downcast<C: ClassImpl<Vtable = V>, B: VtableHasImpl<C>, S: ClassVtable>(
        self,
    ) -> PtrDyn<S> {
        crate::static_assert_subclass!(S, B);
        self.try_downcast::<C, B, S>()
            .unwrap_or_else(|| cast_failed(S::TYPE, self.vtable().ty()))
    }

    /// First downcast to a base class `B` that implements this interface,
    /// then downcast to the subclass `S` of `B` without checking.
    ///
    /// # Safety
    /// The pointer must be a subclass of `B`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn downcast_unchecked<
        C: ClassImpl<Vtable = V>,
        B: VtableHasImpl<C>,
        S: ClassVtable,
    >(
        self,
    ) -> PtrDyn<S> {
        // This validates that `S` can be downcast to `B` without pointer offsets.
        crate::static_assert_subclass!(S, B);
        // This validates that `self.ty` canbe casted to `S`.
        debug_assert!(self.vtable().is_subtype_of::<S>());
        FatPtr {
            data: self.data.cast(),
            vtable: <B as VtableHasImpl<C>>::downcast_impl(self.vtable).cast(),
        }
    }

    #[inline]
    pub fn try_downcast_ty(self, ty: Type) -> Option<PtrDyn<V>> {
        self.vtable().is_subclass_of_ty(ty).then_some(self)
    }
}

impl<C: ClassImpl> RcDyn<C> {
    #[inline]
    pub fn strong_count(this: &Self) -> usize {
        Rc::strong_count(&this.data)
    }
    #[inline]
    pub fn weak_count(this: &Self) -> usize {
        Rc::weak_count(&this.data)
    }
    #[inline]
    pub fn downgrade(this: &Self) -> WeakDyn<C> {
        WeakDyn {
            data: ManuallyDrop::new(Rc::downgrade(&this.data)),
            vtable: this.vtable,
        }
    }
}

impl<C: ClassImpl> WeakDyn<C> {
    #[inline]
    pub fn strong_count(&self) -> usize {
        self.data.strong_count()
    }
    #[inline]
    pub fn weak_count(&self) -> usize {
        self.data.weak_count()
    }
    #[inline]
    pub fn upgrade(&self) -> Option<RcDyn<C>> {
        Some(RcDyn {
            data: ManuallyDrop::new(self.data.upgrade()?),
            vtable: self.vtable,
        })
    }
}

/// A reference-counted fat pointer to a class.
///
/// # Invariants
/// - `vtable` must be a valid `&'static C::CVtable`
#[repr(C)]
pub struct RcDyn<C: ClassImpl> {
    // Use `MannullyDrop` here because `Rc` uses `V::drop_in_place` where
    // change and depends on `V::layout()` (however, `Layout::of::<Dyn<V>>` doesn't
    // equal to `V::layout()`)
    data: ManuallyDrop<Rc<Dyn<C::Data>>>,
    // Note: `vtable` is `NonNull<C::CVtable>` instead of `&'static C::CVtable` in
    // order to pass MIRI's aliasing analysis.
    vtable: NonNull<C::Vtable>,
}

impl<C: ClassImpl> Clone for RcDyn<C> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            vtable: self.vtable,
        }
    }
}

impl<C: ClassImpl> Deref for RcDyn<C> {
    type Target = <C as ClassImpl>::Data;

    fn deref(&self) -> &Self::Target {
        &self.data.0
    }
}

impl<C: ClassImpl> RcDyn<C> {
    pub fn instance_code(&self) -> usize {
        Rc::as_ptr(self.data.deref()).addr()
    }
    #[inline]
    pub const fn vtable(&self) -> &C::Vtable {
        unsafe { self.vtable.as_ref() }
    }

    /// Create a new `RcDyn` from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid.
    #[inline]
    pub unsafe fn from_raw(raw: PtrDyn<C::Vtable>) -> RcDyn<C> {
        RcDyn {
            data: unsafe { ManuallyDrop::new(Rc::from_raw(raw.data.cast().as_ptr())) },
            vtable: raw.vtable,
        }
    }

    #[inline]
    pub fn into_raw(this: RcDyn<C>) -> PtrDyn<C::Vtable> {
        let this = ManuallyDrop::new(this);
        PtrDyn {
            data: unsafe { NonNull::new_unchecked(Rc::as_ptr(&this.data).cast_mut().cast()) },
            vtable: this.vtable,
        }
    }
    #[inline]
    pub fn as_ptr(this: &RcDyn<C>) -> PtrDyn<C::Vtable> {
        PtrDyn {
            data: unsafe { NonNull::new_unchecked(Rc::as_ptr(&this.data).cast_mut().cast()) },
            vtable: this.vtable,
        }
    }
    /// See [`Rc::drop_slow`], it does the same except using `V::drop_in_place` instead.
    #[inline(never)]
    fn drop_slow(&self) {
        let ptr = Rc::as_ptr(&self.data);
        let _weak: WeakDyn<C> = unsafe { WeakDyn::from_raw(RcDyn::as_ptr(self)) };
        unsafe { self.vtable().drop_in_place(ptr.cast_mut().cast()) };
    }
}

impl<C: ConcreteClass> From<Rc<C::Data>> for RcDyn<C> {
    fn from(data: Rc<C::Data>) -> Self {
        Self {
            data: ManuallyDrop::new(unsafe { Rc::from_raw(Rc::into_raw(data).cast()) }),
            vtable: NonNull::from(C::VTABLE),
        }
    }
}

impl<C: HasSuper> RcDyn<C> {
    #[inline]
    pub fn into_super(this: Self) -> RcDyn<C::Super> {
        unsafe { RcDyn::from_raw(RcDyn::into_raw(this).cast_super()) }
    }
}

impl<C: ClassImpl> RcDyn<C> {
    #[inline]
    pub fn into_impl<I: ClassImpl>(this: Self) -> RcDyn<I>
    where
        C: HasImpl<I>,
    {
        unsafe { RcDyn::from_raw(RcDyn::into_raw(this).cast_impl()) }
    }

    #[inline]
    #[track_caller]
    pub fn mixin_into_impl<I: ClassImpl>(this: Self) -> RcDyn<I>
    where
        C: MixinHasImpl<I>,
    {
        unsafe { RcDyn::from_raw(RcDyn::into_raw(this).mixin_cast_impl()) }
    }

    /// Cast the `Rc` to its superclass `A`.
    ///
    /// # Safety
    /// `A` must be a superclass of `C`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn into_superclass_unchecked<A: ClassImpl>(this: Self) -> RcDyn<A> {
        crate::static_assert_subclass!(C::Vtable, A::Vtable);
        let casted = unsafe { RcDyn::into_raw(this).cast_superclass_unchecked() };
        unsafe { RcDyn::from_raw(casted) }
    }

    /// Cast the `Rc` to its superclass `A`.
    #[inline]
    #[track_caller]
    pub fn into_superclass<A: ClassImpl>(this: Self) -> RcDyn<A> {
        crate::static_assert_subclass!(C::Vtable, A::Vtable);
        let casted = RcDyn::into_raw(this).cast_superclass();
        unsafe { RcDyn::from_raw(casted) }
    }

    /// Cast the `Rc` to its superclass `A`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_into_superclass<A: ClassImpl>(this: Self) -> Option<RcDyn<A>> {
        crate::static_assert_subclass!(C::Vtable, A::Vtable);
        let casted = RcDyn::into_raw(this).try_cast_superclass()?;
        unsafe { Some(RcDyn::from_raw(casted)) }
    }

    /// Cast the `Rc` to its superclass `A`.
    ///
    /// # Safety
    /// `A` must be a superclass of `C`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn as_superclass_unchecked<A: ClassImpl>(this: &Self) -> &RcDyn<A> {
        crate::static_assert_subclass!(C::Vtable, A::Vtable);
        unsafe { &*core::ptr::from_ref(this).cast() }
    }

    /// Cast the `Rc` to its superclass `A`.
    #[inline]
    #[track_caller]
    pub fn as_superclass<A: ClassImpl>(this: &Self) -> &RcDyn<A> {
        crate::static_assert_subclass!(C::Vtable, A::Vtable);
        Self::try_as_superclass(this).unwrap_or_else(|| {
            cast_failed(<A::Vtable as ClassVtableBase>::TYPE, this.vtable().ty())
        })
    }

    /// Cast the `Rc` to its superclass `A`.
    ///
    /// # Safety
    /// `A` must be a superclass of `C`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_as_superclass<A: ClassImpl>(this: &Self) -> Option<&RcDyn<A>> {
        crate::static_assert_subclass!(C::Vtable, A::Vtable);
        this.vtable()
            .is_subclass_of::<A::Vtable>()
            .then_some(unsafe { Self::as_superclass_unchecked(this) })
    }

    /// Cast the `Rc` to its subclass `D`.
    ///
    /// # Safety
    /// `D` must be a superclass of `D`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn as_subclass_unchecked<D: ClassImpl>(this: &Self) -> &RcDyn<D> {
        crate::static_assert_subclass!(D::Vtable, C::Vtable);
        unsafe { &*core::ptr::from_ref(this).cast() }
    }

    /// Cast the `Rc` to its subclass `D`.
    #[inline]
    #[track_caller]
    pub fn as_subclass<D: ClassImpl>(this: &Self) -> &RcDyn<D> {
        crate::static_assert_subclass!(D::Vtable, C::Vtable);
        Self::try_as_subclass(this)
            .unwrap_or_else(|| cast_failed(<D::Vtable>::TYPE, this.vtable().ty()))
    }

    /// Cast the `Rc` to its subclass `D`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_as_subclass<D: ClassImpl>(this: &Self) -> Option<&RcDyn<D>> {
        crate::static_assert_subclass!(D::Vtable, C::Vtable);
        this.vtable()
            .is_subclass_of::<D::Vtable>()
            .then_some(unsafe { Self::as_subclass_unchecked(this) })
    }

    /// Cast the `Rc` to its subclass `D`.
    ///
    /// # Safety
    /// `D` must be a superclass of `D`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn into_subclass_unchecked<D: ClassImpl>(this: Self) -> RcDyn<D> {
        crate::static_assert_subclass!(D::Vtable, C::Vtable);
        let casted = unsafe { Self::into_raw(this).cast_subclass_unchecked() };
        unsafe { RcDyn::from_raw(casted) }
    }

    /// Cast the `Rc` to its subclass `D`.
    #[inline]
    #[track_caller]
    pub fn into_subclass<D: ClassImpl>(this: Self) -> RcDyn<D> {
        crate::static_assert_subclass!(D::Vtable, C::Vtable);
        let casted = Self::into_raw(this).cast_subclass();
        unsafe { RcDyn::from_raw(casted) }
    }

    /// Cast the `Rc` to its subclass `D`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_into_subclass<D: ClassImpl>(this: Self) -> Option<RcDyn<D>> {
        crate::static_assert_subclass!(D::Vtable, C::Vtable);
        let casted = Self::into_raw(this).try_cast_subclass()?;
        unsafe { Some(RcDyn::from_raw(casted)) }
    }

    /// Cast the `Rc` to its supertype `A`.
    #[inline]
    pub fn into_supertype<A: ClassImpl>(this: Self) -> RcDyn<A> {
        crate::static_assert_subtype!(C::Vtable, A::Vtable);
        let casted = RcDyn::into_raw(this).cast_supertype();
        unsafe { RcDyn::from_raw(casted) }
    }

    /// Cast the `Rc` to its supertype `A`.
    #[inline]
    pub fn to_supertype<A: ClassImpl>(this: &Self) -> RcDyn<A> {
        crate::static_assert_subtype!(C::Vtable, A::Vtable);
        let casted = RcDyn::into_raw(this.clone()).cast_supertype();
        unsafe { RcDyn::from_raw(casted) }
    }

    /// Cast the `Rc` to its supertype `A`.
    #[inline]
    pub fn try_into_supertype<A: ClassImpl>(this: Self) -> Option<RcDyn<A>> {
        let casted = RcDyn::into_raw(this).try_cast_supertype()?;
        Some(unsafe { RcDyn::from_raw(casted) })
    }

    /// Cast the `Rc` to its supertype `A`.
    #[inline]
    pub fn try_to_supertype<A: ClassImpl>(this: &Self) -> Option<RcDyn<A>> {
        let casted = RcDyn::into_raw(this.clone()).try_cast_supertype()?;
        Some(unsafe { RcDyn::from_raw(casted) })
    }

    /// Cast the `Rc` to its subtype `D`.
    #[inline]
    pub fn try_into_subtype<D: ClassImpl>(this: Self) -> Option<RcDyn<D>> {
        let casted = RcDyn::into_raw(this).try_cast_subtype()?;
        unsafe { Some(RcDyn::from_raw(casted)) }
    }

    /// Cast the `Rc` to its subtype `D`.
    #[inline]
    pub fn into_subtype<D: ClassImpl>(this: Self) -> RcDyn<D> {
        let casted = RcDyn::into_raw(this).cast_subtype();
        unsafe { RcDyn::from_raw(casted) }
    }

    /// Upcast this pointer to the supertype `I`, with an offset `OFFSET`.
    ///
    /// # Safety
    ///
    /// `OFFSET` must be the offset of `I` in vtable `C`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn upcast_with_offset_unchecked<I: ClassImpl, const OFFSET: usize>(
        this: Self,
    ) -> RcDyn<I> {
        crate::static_assert_subtype_with_offset!(C::Vtable, I::Vtable, OFFSET);
        let casted =
            unsafe { RcDyn::into_raw(this).upcast_with_offset_unchecked::<I::Vtable, OFFSET>() };
        unsafe { RcDyn::from_raw(casted) }
    }

    /// Upcast this pointer to the subtype `I`, with an offset `OFFSET`.
    ///
    /// # Safety
    ///
    /// `OFFSET` must be the offset of `C` in vtable `I`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn downcast_with_offset_unchecked<I: ClassImpl, const OFFSET: usize>(
        this: Self,
    ) -> RcDyn<I> {
        crate::static_assert_subtype_with_offset!(I::Vtable, C::Vtable, OFFSET);
        let casted =
            unsafe { RcDyn::into_raw(this).downcast_with_offset_unchecked::<I::Vtable, OFFSET>() };
        unsafe { RcDyn::from_raw(casted) }
    }

    /// First cast the `RcDyn` to its superclass `A`, then cast to
    /// the interface of superclass `I`
    ///
    /// # Safety
    /// `A` must be a superclass of `C`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn upcast_unchecked<A: HasImpl<I>, I: ClassImpl>(this: Self) -> RcDyn<I> {
        crate::static_assert_subclass!(C::Vtable, A::Vtable);
        let casted = unsafe { RcDyn::into_raw(this).upcast_unchecked::<A::Vtable, I>() };
        unsafe { RcDyn::from_raw(casted) }
    }

    /// First cast the `RcDyn` to its superclass `A`, then cast to
    /// the interface of superclass `I`
    ///
    /// # Safety
    /// `A` must be a superclass of `C`.
    #[inline]
    #[track_caller]
    pub fn upcast<A: HasImpl<I>, I: ClassImpl>(this: Self) -> RcDyn<I> {
        crate::static_assert_subclass!(C::Vtable, A::Vtable);
        let casted = RcDyn::into_raw(this).upcast::<A::Vtable, I>();
        unsafe { RcDyn::from_raw(casted) }
    }

    /// First cast the `RcDyn` to its superclass `A`, then cast to
    /// the interface of superclass `I`
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_upcast<A: HasImpl<I>, I: ClassImpl>(this: Self) -> Option<RcDyn<I>> {
        crate::static_assert_subclass!(C::Vtable, A::Vtable);
        let casted = RcDyn::into_raw(this).try_upcast::<A::Vtable, I>()?;
        unsafe { Some(RcDyn::from_raw(casted)) }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn mixin_try_downcast<S: ClassImpl>(this: Self) -> Option<RcDyn<S>>
    where
        C: MixinClassImpl,
    {
        let casted = RcDyn::into_raw(this).mixin_try_downcast::<S::Vtable>()?;
        unsafe { Some(RcDyn::from_raw(casted)) }
    }

    #[inline]
    #[track_caller]
    pub fn mixin_downcast<S: ClassImpl>(this: Self) -> RcDyn<S>
    where
        C: MixinClassImpl,
    {
        let casted = RcDyn::into_raw(this).mixin_downcast::<S::Vtable>();
        unsafe { RcDyn::from_raw(casted) }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn mixin_downcast_unchecked<S: ClassImpl>(this: Self) -> RcDyn<S>
    where
        C: MixinClassImpl,
    {
        let casted = unsafe { RcDyn::into_raw(this).mixin_downcast_unchecked::<S::Vtable>() };
        unsafe { RcDyn::from_raw(casted) }
    }

    /// Try to cast the `RcDyn` to a mixin `RcDyn<M>`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_into_mixin<M: MixinClassImpl>(this: Self) -> Option<RcDyn<M>> {
        let casted = RcDyn::into_raw(this).try_cast_mixin::<M::VtableWithoutSuper>()?;
        unsafe { Some(RcDyn::from_raw(casted)) }
    }

    /// Cast the `RcDyn` to a mixin `RcDyn<M>`.
    #[inline]
    #[track_caller]
    pub fn into_mixin<M: MixinClassImpl>(this: Self) -> RcDyn<M> {
        let casted = RcDyn::into_raw(this).cast_mixin::<M::VtableWithoutSuper>();
        unsafe { RcDyn::from_raw(casted) }
    }

    /// Cast the `RcDyn` to a mixin `RcDyn<M>`.
    ///
    /// # Safety
    /// `C` must be a mixin instance of `M`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn into_mixin_unchecked<M: MixinClassImpl>(
        this: Self,
        instance: MixinInstanceType,
    ) -> RcDyn<M> {
        let casted = unsafe {
            RcDyn::into_raw(this).cast_mixin_unchecked::<M::VtableWithoutSuper>(instance)
        };
        unsafe { RcDyn::from_raw(casted) }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn try_downcast<B: HasImpl<C>, S: ClassImpl>(this: Self) -> Option<RcDyn<S>> {
        crate::static_assert_subclass!(S::Vtable, B::Vtable);
        let casted = RcDyn::into_raw(this).try_downcast::<C, B::Vtable, S::Vtable>()?;
        unsafe { Some(RcDyn::from_raw(casted)) }
    }

    /// First downcast the `RcDyn` to a base class `RcDyn<B>` that
    /// implements this interface, then downcast to the subclass `S` of `B`.
    #[inline]
    #[track_caller]
    pub fn downcast<B: HasImpl<C>, S: ClassImpl>(this: Self) -> RcDyn<S> {
        crate::static_assert_subclass!(S::Vtable, B::Vtable);
        let casted = RcDyn::into_raw(this).downcast::<C, B::Vtable, S::Vtable>();
        unsafe { RcDyn::from_raw(casted) }
    }

    /// First downcast the `RcDyn` to a base class `RcDyn<B>` that
    /// implements this interface, then downcast to the subclass `S` of `B`.
    /// Downcast the `RcDyn` to a `RcDyn<A>` without checking.
    ///
    /// # Safety
    /// The `RcDyn` must be a subclass of `S`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn downcast_unchecked<B: HasImpl<C>, S: ClassImpl>(this: Self) -> RcDyn<S> {
        crate::static_assert_subclass!(S::Vtable, B::Vtable);
        let casted =
            unsafe { RcDyn::into_raw(this).downcast_unchecked::<C, B::Vtable, S::Vtable>() };
        unsafe { RcDyn::from_raw(casted) }
    }

    #[inline]
    pub fn try_downcast_ty(&self, ty: Type) -> Option<&Self> {
        self.vtable().is_subtype_of_ty(ty).then_some(self)
    }

    #[inline]
    #[track_caller]
    pub fn downcast_ty(&self, ty: Type) -> &Self {
        self.try_downcast_ty(ty)
            .unwrap_or_else(|| cast_failed(ty, self.vtable().ty()))
    }
}

/// Since there is no public API to get the real weak count when strong
/// count is zero (`Weak::weak_count` returns `0` for that case),
/// we use a helper struct that has the same header as `std::rc::RcInner`,
/// to get the real weak count of a `Weak`.
#[repr(C)]
struct RcInnerHeader {
    strong: Cell<usize>,
    weak: Cell<usize>,
}

#[inline]
const fn data_offset_align(align: usize) -> usize {
    let layout = Layout::new::<RcInnerHeader>();
    // copied from `layou.padding_needed_for(align)` since it is not stable yet.
    let padding_needed = {
        let Some(align_m1) = align.checked_sub(1) else {
            return usize::MAX;
        };
        let len_rounded_up = (layout.size() + align_m1) & !align_m1;
        len_rounded_up - layout.size()
    };
    layout.size() + padding_needed
}

unsafe fn get_rc_inner<T>(rc_data: *const T, layout: Layout) -> *const RcInnerHeader {
    unsafe { rc_data.byte_sub(data_offset_align(layout.align())) }.cast::<RcInnerHeader>()
}

/// Calculate layout for `RcInner<T>` using the inner value's layout
fn rc_inner_layout_for_value_layout(layout: Layout) -> Layout {
    // Calculate layout using the given value layout.
    // Previously, layout was calculated on the expression
    // `&*(ptr as *const RcInner<T>)`, but this created a misaligned
    // reference (see #54908).
    Layout::new::<RcInnerHeader>()
        .extend(layout)
        .unwrap()
        .0
        .pad_to_align()
}

impl<C: ClassImpl> Drop for RcDyn<C> {
    /// See [`Rc::drop`], it does the same except using `V::drop_in_place` instead.
    #[inline]
    fn drop(&mut self) {
        let inner = unsafe { &*get_rc_inner(Rc::as_ptr(&self.data), self.vtable().layout()) };
        // do not use [`Rc::decrement_strong`] becaues it also decrement the weak reference.
        inner.strong.set(inner.strong.get() - 1);
        if Rc::strong_count(&self.data) == 0 {
            self.drop_slow();
        }
    }
}

/// A reference-counted fat pointer to a class.
///
/// # Invariants
/// - `vtable` must be a valid `&'static C::CVtable`
#[repr(C)]
pub struct WeakDyn<C: ClassImpl> {
    // Use `MannullyDrop` here because `Rc` uses `V::drop_in_place` where
    // change and depends on `V::layout()` (however, `Layout::of::<Dyn<V>>` doesn't
    // equal to `V::layout()`)
    data: ManuallyDrop<Weak<Dyn<C::Data>>>,
    // Note: `vtable` is `NonNull<C::CVtable>` instead of `&'static C::CVtable` in
    // order to pass MIRI's aliasing analysis.
    vtable: NonNull<C::Vtable>,
}

impl<C: ClassImpl> Clone for WeakDyn<C> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            vtable: self.vtable,
        }
    }
}

impl<C: ClassImpl> WeakDyn<C> {
    #[inline]
    pub const fn vtable(&self) -> &C::Vtable {
        unsafe { self.vtable.as_ref() }
    }
    #[inline]
    /// Create a new `WeakDyn` from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid except maybe uninitialized (when the strong count is 0).
    pub unsafe fn from_raw(ptr: PtrDyn<C::Vtable>) -> Self {
        Self {
            data: unsafe { ManuallyDrop::new(Weak::from_raw(ptr.data.cast().as_ptr())) },
            vtable: ptr.vtable,
        }
    }
    #[inline]
    pub fn into_raw(self) -> PtrDyn<C::Vtable> {
        let this = ManuallyDrop::new(self);
        PtrDyn {
            data: unsafe { NonNull::new_unchecked(this.data.as_ptr().cast_mut().cast()) },
            vtable: this.vtable,
        }
    }
    #[inline]
    fn is_dangling(&self) -> bool {
        self.data.as_ptr().addr() == usize::MAX
    }
    #[inline]
    pub fn as_ptr(&self) -> PtrDyn<C::Vtable> {
        PtrDyn {
            data: unsafe { NonNull::new_unchecked(self.data.as_ptr().cast_mut().cast()) },
            vtable: self.vtable,
        }
    }
}

impl<C: HasSuper> WeakDyn<C> {
    #[inline]
    pub fn into_super(self) -> WeakDyn<C::Super> {
        unsafe { WeakDyn::from_raw(self.into_raw().cast_super()) }
    }
}

impl<C: ClassImpl> WeakDyn<C> {
    #[inline]
    pub fn into_impl<I: ClassImpl>(self) -> WeakDyn<I>
    where
        C: HasImpl<I>,
    {
        unsafe { WeakDyn::from_raw(self.into_raw().cast_impl()) }
    }

    #[inline]
    pub fn mixin_into_impl<I: ClassImpl>(this: Self) -> WeakDyn<I>
    where
        C: MixinHasImpl<I>,
    {
        unsafe { WeakDyn::from_raw(WeakDyn::into_raw(this).mixin_cast_impl()) }
    }

    /// Cast the pointer to its superclass `A`.
    ///
    /// # Safety
    /// `A` must be a superclass of `C`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn into_superclass_unchecked<A: ClassImpl>(self) -> WeakDyn<A> {
        unsafe { WeakDyn::from_raw(self.into_raw().cast_superclass_unchecked()) }
    }
}

impl<C: ClassImpl> Drop for WeakDyn<C> {
    /// See [`Weak::drop`], it does the same except using `V::layout` instead for dealloc.
    fn drop(&mut self) {
        if self.is_dangling() {
            return;
        }

        let layout = unsafe { self.vtable().layout() };
        let inner_ptr = unsafe { get_rc_inner(self.data.as_ptr(), layout) };
        let inner = unsafe { &*inner_ptr };

        inner.weak.set(inner.weak.get() - 1);
        // the weak count starts at 1, and will only go to zero if all
        // the strong pointers have disappeared.
        if inner.weak.get() == 0 {
            let layout_with_inner = rc_inner_layout_for_value_layout(layout);
            // note that we have to dealloc the `RcInner` instead of the data only.
            unsafe { alloc::alloc::dealloc(inner_ptr.cast_mut().cast(), layout_with_inner) };
        }
    }
}

/// A reference-counted fat pointer to an uninitialized class.
///
/// # Invariants
/// - `vtable` must be a valid `&'static C::CVtable`
/// - `strong_count` must be 1 and `weak_count` must be 0, i.e. this `Rc` is unique.
///   See [`alloc::rc::UniqueRc`] for more details.
#[repr(C)]
pub struct RcDynUninit<C: ClassImpl> {
    // Use `MannullyDrop` here because `Rc` uses `V::drop_in_place` where
    // change and depends on `V::layout()` (however, `Layout::of::<Dyn<V>>` doesn't
    // equal to `V::layout()`)
    data: ManuallyDrop<Rc<MaybeUninit<Dyn<C::Data>>>>,
    // Note: `vtable` is `NonNull<C::CVtable>` instead of `&'static C::CVtable` in
    // order to pass MIRI's aliasing analysis.
    vtable: NonNull<C::Vtable>,
}

impl<C: ConcreteClass> RcDynUninit<C> {
    #[inline]
    pub fn new_uninit() -> Self {
        Self {
            data: ManuallyDrop::new(Rc::new_uninit()),
            vtable: NonNull::from(C::VTABLE),
        }
    }
}

impl<C: ClassImpl> RcDynUninit<C> {
    #[inline]
    fn into_raw(self) -> (*mut C::Data, NonNull<C::Vtable>) {
        debug_assert_eq!(Rc::strong_count(&self.data), 1);
        debug_assert_eq!(Rc::weak_count(&self.data), 0);
        let mut this = ManuallyDrop::new(self);
        let data = Rc::into_raw(unsafe { ManuallyDrop::take(&mut this.data) })
            .cast_mut()
            .cast();
        (data, this.vtable)
    }

    #[inline]
    unsafe fn from_raw(data: *mut C::Data, vtable: NonNull<C::Vtable>) -> Self {
        RcDynUninit {
            data: ManuallyDrop::new(unsafe { Rc::from_raw(data.cast()) }),
            vtable,
        }
    }

    #[inline]
    pub fn into_super(self) -> RcDynUninit<C::Super>
    where
        C: HasSuper,
    {
        self.into_superclass()
    }
    #[inline]
    pub fn into_superclass<A: ClassImpl>(self) -> RcDynUninit<A> {
        crate::static_assert_subclass!(C::Vtable, A::Vtable);
        let (data, vtable) = self.into_raw();
        unsafe { RcDynUninit::from_raw(data.cast(), vtable.cast()) }
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut C::Data {
        debug_assert_eq!(Rc::strong_count(&self.data), 1);
        debug_assert_eq!(Rc::weak_count(&self.data), 0);
        Rc::as_ptr(&self.data).cast_mut().cast()
    }

    #[inline]
    pub unsafe fn assume_init(self) -> RcDyn<C> {
        let (data, vtable) = self.into_raw();
        RcDyn {
            data: unsafe { ManuallyDrop::new(Rc::from_raw(data.cast())) },
            vtable,
        }
    }
}

impl<C: ClassImpl> Drop for RcDynUninit<C> {
    /// See [`alloc::rc::UniqueRc`]`::drop`, it does the same except the `data` is not dropped
    /// because it is a `MaybeUninit`.
    fn drop(&mut self) {
        let layout = unsafe { self.vtable.as_ref().layout() };
        let inner_ptr = unsafe { get_rc_inner(Rc::as_ptr(&self.data), layout) };
        let inner = unsafe { &*inner_ptr };

        inner.weak.set(inner.weak.get() - 1);
        // the weak count starts at 1, and will only go to zero if all
        // the strong pointers have disappeared.
        if inner.weak.get() == 0 {
            let layout_with_inner = rc_inner_layout_for_value_layout(layout);
            // note that we have to dealloc the `RcInner` instead of the data only.
            unsafe { alloc::alloc::dealloc(inner_ptr.cast_mut().cast(), layout_with_inner) };
        }
    }
}

#[cfg(test)]
mod tests {
    use core::hash::Hash;

    use crate::{
        class::{
            Class, ClassData, ClassDataBase, ClassRcWeak, ClassVtableBase, ClassVtableOpt, IsClass,
        },
        vtable::{TypeInfo, VtableHeader},
    };

    use super::*;

    #[repr(C)]
    struct MyData {}

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    struct MyVtable {
        header: VtableHeader,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct MyVtableOpt {
        header: Option<VtableHeader>,
    }

    struct MyClass;
    static TYPE: TypeInfo<0> = TypeInfo::new_concrete_class::<MyClass>(
        None,
        [],
        // #[cfg(debug_assertions)]
        module_path!(),
        // #[cfg(debug_assertions)]
        "MyClass",
    );

    impl ClassImpl for MyClass {
        type DataBase = MyData;
        type Data = MyData;
        type VtableBase = MyVtable;
        type Vtable = MyVtable;
        type VtableOpt = MyVtableOpt;
    }
    impl ClassImpl for RcDyn<MyClass> {
        type DataBase = MyData;
        type Data = MyData;
        type VtableBase = MyVtable;
        type Vtable = MyVtable;
        type VtableOpt = MyVtableOpt;
    }
    impl ClassImpl for WeakDyn<MyClass> {
        type DataBase = MyData;
        type Data = MyData;
        type VtableBase = MyVtable;
        type Vtable = MyVtable;
        type VtableOpt = MyVtableOpt;
    }
    impl ClassDataBase for MyData {
        type Vtable = MyVtable;
    }
    impl ClassData for MyData {}
    impl ClassVtableBase for MyVtable {
        const TYPE: Type = TYPE.as_type();

        type Data = MyData;
        type Opt = MyVtableOpt;

        type DebugVtableLayout<'a> = &'a MyVtable;

        fn debug_vtable_layout(&self, _offset: usize) -> Self::DebugVtableLayout<'_> {
            self
        }
    }
    unsafe impl ClassVtable for MyVtable {}
    impl ClassVtableOpt for MyVtableOpt {
        type VtableBase = MyVtable;
        type Vtable = MyVtable;
    }
    impl Class for MyClass {
        type Ptr = PtrDyn<MyVtable>;
        // type Ref<'a> = RefDyn<'a, MyClass>;
        type Rc = RcDyn<MyClass>;
        type Weak = WeakDyn<MyClass>;
    }
    impl ClassRcWeak for RcDyn<MyClass> {
        type Upgraded = Self;
        type UpgradedOpt = Self;
        type DowngradeFrom = Self;

        fn as_ptr(this: &Self) -> PtrDyn<MyVtable> {
            RcDyn::as_ptr(this)
        }

        fn vtable(this: &Self) -> &Self::Vtable {
            RcDyn::vtable(this)
        }

        fn upgrade(this: &Self) -> Self::Upgraded {
            this.clone()
        }

        fn upgrade_opt(this: Option<&Self>) -> Option<Self::UpgradedOpt> {
            this.cloned()
        }

        fn downgrade_from(from: &Self::DowngradeFrom) -> Self {
            from.clone()
        }
    }
    impl ClassRcWeak for WeakDyn<MyClass> {
        type Upgraded = Option<RcDyn<MyClass>>;
        type UpgradedOpt = RcDyn<MyClass>;
        type DowngradeFrom = RcDyn<MyClass>;

        fn as_ptr(this: &Self) -> PtrDyn<MyVtable> {
            this.as_ptr()
        }

        fn vtable(this: &Self) -> &Self::Vtable {
            WeakDyn::vtable(this)
        }

        fn upgrade(this: &Self) -> Self::Upgraded {
            WeakDyn::upgrade(this)
        }
        fn upgrade_opt(this: Option<&Self>) -> Option<Self::UpgradedOpt> {
            this.and_then(|this| this.upgrade())
        }
        fn downgrade_from(from: &Self::DowngradeFrom) -> Self {
            RcDyn::downgrade(from)
        }
    }
    impl Hash for MyVtable {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            ::core::ptr::from_ref(self).hash(state);
        }
    }
    impl IsClass for RcDyn<MyClass> {
        type Class = MyClass;
    }
    impl IsClass for WeakDyn<MyClass> {
        type Class = MyClass;
    }
    unsafe impl ConcreteClass for MyClass {
        const VTABLE: NonNull<Self::Vtable> = {
            static VTABLE: MyVtable = MyVtable {
                // header: VtableHeader::from_type::<MyData>(),
                header: VtableHeader::new(MyVtable::TYPE, 0),
            };
            unsafe { NonNull::new_unchecked((&raw const VTABLE).cast_mut()) }
        };
    }

    #[test]
    fn test_rc() {
        let rc_dyn: RcDyn<MyClass> = Rc::new(MyData {}).into();
        assert_eq!(RcDyn::strong_count(&rc_dyn), 1);
        assert_eq!(RcDyn::weak_count(&rc_dyn), 0);
        let cloned = rc_dyn.clone();
        assert_eq!(RcDyn::strong_count(&rc_dyn), 2);
        assert_eq!(RcDyn::weak_count(&rc_dyn), 0);
        drop(cloned);
        assert_eq!(RcDyn::strong_count(&rc_dyn), 1);
        assert_eq!(RcDyn::weak_count(&rc_dyn), 0);
    }

    #[test]
    fn test_weak() {
        let rc_dyn: RcDyn<MyClass> = Rc::new(MyData {}).into();
        assert_eq!(RcDyn::strong_count(&rc_dyn), 1);
        assert_eq!(RcDyn::weak_count(&rc_dyn), 0);
        let weak = RcDyn::downgrade(&rc_dyn);
        assert_eq!(RcDyn::strong_count(&rc_dyn), 1);
        assert_eq!(RcDyn::weak_count(&rc_dyn), 1);
        assert_eq!(weak.strong_count(), 1);
        assert_eq!(weak.weak_count(), 1);
        let _cloned = weak.clone();
        assert_eq!(RcDyn::strong_count(&rc_dyn), 1);
        assert_eq!(RcDyn::weak_count(&rc_dyn), 2);
        assert_eq!(weak.strong_count(), 1);
        assert_eq!(weak.weak_count(), 2);
        drop(_cloned);
        assert_eq!(RcDyn::strong_count(&rc_dyn), 1);
        assert_eq!(RcDyn::weak_count(&rc_dyn), 1);
        assert_eq!(weak.strong_count(), 1);
        assert_eq!(weak.weak_count(), 1);
        let strong = weak.upgrade().unwrap();
        assert_eq!(RcDyn::strong_count(&rc_dyn), 2);
        assert_eq!(RcDyn::weak_count(&rc_dyn), 1);
        assert_eq!(weak.strong_count(), 2);
        assert_eq!(weak.weak_count(), 1);
        drop(strong);
        assert_eq!(RcDyn::strong_count(&rc_dyn), 1);
        assert_eq!(RcDyn::weak_count(&rc_dyn), 1);
        assert_eq!(weak.strong_count(), 1);
        assert_eq!(weak.weak_count(), 1);
        drop(rc_dyn);
        assert_eq!(weak.strong_count(), 0);
        assert_eq!(weak.weak_count(), 0);
    }

    #[test]
    fn test_rc_uninit() {
        let rc_dyn_uninit: RcDynUninit<MyClass> = RcDynUninit::new_uninit();
        let rc_dyn = unsafe { rc_dyn_uninit.assume_init() };
        assert_eq!(RcDyn::strong_count(&rc_dyn), 1);
        assert_eq!(RcDyn::weak_count(&rc_dyn), 0);

        let rc_dyn_uninit: RcDynUninit<MyClass> = RcDynUninit::new_uninit();
        drop(rc_dyn_uninit);
    }
}
