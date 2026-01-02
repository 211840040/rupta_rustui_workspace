extern crate classes;
use classes::prelude::*;
#[allow(unused_imports)]
use _classes::BuildContext;
#[allow(unused_imports)]
use _classes::Element;
#[allow(unused_imports)]
use _classes::Widget;
use ::classes::prelude::*;
const MODULE_PATH: &str = "temp_expand";
#[allow(unused_macros)]
mod _classes {
    use super::*;
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _BuildContext::BuildContext;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _BuildContext {
        use super::*;
        use ::core::ptr::NonNull;
        use ::classes::class::{ConcreteClass, NonVirtual, Virtual};
        use ::classes::get_set::{GetSet, GetSetCopy};
        use ::classes::prelude::*;
        use ::classes::ptr::RcDyn;
        use ::classes::vtable::{
            MaybeUninitVtableWithMixinHeader, VtableHeader, VtableWithMixinHeader,
        };
        #[repr(transparent)]
        pub struct BuildContext<
            T = ::classes::class::ClassMarker,
            V = ::classes::class::Virtual,
        >(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone
        for self::BuildContext<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy
        for self::BuildContext<T, V> {}
        impl<T, V> self::BuildContext<T, V> {
            #[doc(hidden)]
            #[inline]
            pub fn _into_inner(self) -> T {
                self.0
            }
            #[doc(hidden)]
            #[inline]
            pub fn _as_inner(&self) -> &T {
                &self.0
            }
            #[doc(hidden)]
            #[inline]
            pub fn _from_inner(inner: T) -> Self {
                Self(inner, ::core::marker::PhantomData)
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::BuildContext>>
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            fn from(inner: ::classes::ptr::RcDyn<self::BuildContext>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<
            self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V>,
        > for ::classes::ptr::RcDyn<self::BuildContext> {
            fn from(
                this: self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V>,
            ) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::BuildContext>>
        for self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
            fn from(inner: ::classes::ptr::WeakDyn<self::BuildContext>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<
            self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V>,
        > for ::classes::ptr::WeakDyn<self::BuildContext> {
            fn from(
                this: self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V>,
            ) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::BuildContext<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::BuildContext<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            type Upgraded = Self;
            type UpgradedOpt = Self;
            type DowngradeFrom = Self;
            fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<Self> {
                ::classes::ptr::RcDyn::as_ptr(this._as_inner())
            }
            fn vtable(this: &Self) -> &Self::Vtable {
                this._as_inner().vtable()
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
        impl<V> ::classes::class::ClassRcWeak
        for self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
            type Upgraded = Option<
                self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V>,
            >;
            type UpgradedOpt = self::BuildContext<
                ::classes::ptr::RcDyn<self::BuildContext>,
                V,
            >;
            type DowngradeFrom = self::BuildContext<
                ::classes::ptr::RcDyn<self::BuildContext>,
                V,
            >;
            fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<Self> {
                this._as_inner().as_ptr()
            }
            fn vtable(this: &Self) -> &Self::Vtable {
                this._as_inner().vtable()
            }
            fn upgrade(this: &Self) -> Self::Upgraded {
                this.upgrade()
            }
            fn upgrade_opt(this: Option<&Self>) -> Option<Self::UpgradedOpt> {
                this.and_then(|this| this.upgrade())
            }
            fn downgrade_from(from: &Self::DowngradeFrom) -> Self {
                self::BuildContext::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V>
        where
            for<'a> &'a C: ::core::convert::From<&'a ::classes::ptr::RcDyn<C::Class>>,
        {
            fn eq(&self, other: &C) -> bool {
                type CRcEqHash = ::classes::prelude::CRc<::classes::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    let other = ::classes::class::ClassRc::to_supertype::<
                        ::classes::prelude::CRc<::classes::object::Object>,
                    >(other);
                    CRcEqHash::eq(&this, &other)
                } else {
                    ::classes::class::ClassRcWeak::as_ptr(self)
                        == ::classes::class::ClassRcWeak::as_ptr(other)
                }
            }
        }
        impl<V> ::core::cmp::Eq
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V>
        where
            for<'a> &'a Self: ::core::convert::From<
                &'a ::classes::ptr::RcDyn<self::BuildContext>,
            >,
        {}
        impl<V> ::core::hash::Hash
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                type CRcEqHash = ::classes::prelude::CRc<::classes::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    CRcEqHash::hash(&this, state);
                } else {
                    ::core::hash::Hash::hash(
                        &::classes::class::ClassRcWeak::as_ptr(self),
                        state,
                    );
                }
            }
        }
        impl<V, C: ::classes::class::ClassRcWeak> ::core::cmp::PartialEq<C>
        for self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq
        for self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {}
        impl<V> ::core::hash::Hash
        for self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(
                    &::classes::class::ClassRcWeak::as_ptr(self),
                    state,
                );
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                type CRcFormat = ::classes::prelude::CRc<::classes::fmt::Format>;
                if let Some(this) = self.try_to_supertype::<CRcFormat>() {
                    CRcFormat::fmt_debug(&this, f)
                } else {
                    ::core::fmt::Display::fmt(
                        &::classes::class::ClassRcWeak::as_ptr(self),
                        f,
                    )
                }
            }
        }
        impl<V> ::core::fmt::Debug
        for self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if let Some(this) = self.upgrade() {
                    ::core::fmt::Debug::fmt(&this, f)
                } else {
                    ::core::fmt::Display::fmt(
                        &::classes::class::ClassRcWeak::as_ptr(self),
                        f,
                    )
                }
            }
        }
        impl<T, V> ::classes::class::IsClass for self::BuildContext<T, V> {
            type Class = self::BuildContext;
        }
        impl ::classes::class::IsClass for data::BuildContext {
            type Class = self::BuildContext;
        }
        impl self::BuildContext {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::BuildContext {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::BuildContext {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize = <vtable::BuildContext as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::BuildContext {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::BuildContext {
            type Class = self::BuildContext;
        }
        impl ::classes::class::IsClass for vtable::opt::BuildContext {
            type Class = self::BuildContext;
        }
        impl ::classes::class::ClassDataBase for data::BuildContext {
            type Vtable = vtable::BuildContext;
        }
        impl ::classes::class::ClassVtableBase for vtable::BuildContext {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::BuildContext;
            type Opt = vtable::opt::BuildContext;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::BuildContext<T, V> {
            type DataBase = data::BuildContext;
            type Data = data::BuildContext;
            type VtableBase = vtable::BuildContext;
            type Vtable = vtable::BuildContext;
            type VtableOpt = vtable::opt::BuildContext;
        }
        impl ::classes::class::ClassData for data::BuildContext {}
        unsafe impl ::classes::class::ClassVtable for vtable::BuildContext {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::BuildContext {
            type VtableBase = vtable::BuildContext;
            type Vtable = vtable::BuildContext;
        }
        impl<V> ::classes::class::Class
        for self::BuildContext<::classes::class::ClassMarker, V> {
            type Rc = self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V>;
            type Weak = self::BuildContext<
                ::classes::ptr::WeakDyn<self::BuildContext>,
                V,
            >;
            type Ptr = ::classes::ptr::PtrDyn<vtable::BuildContext>;
        }
        impl<V> self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            pub fn downgrade(
                this: &Self,
            ) -> self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
                self::BuildContext::_from_inner(
                    ::classes::ptr::RcDyn::downgrade(this._as_inner()),
                )
            }
        }
        impl vtable::BuildContext {
            #[inline]
            const fn cast_header(
                this: *const Self,
            ) -> *const ::classes::vtable::VtableHeader {
                this.cast()
            }
            pub const fn header(&self) -> &::classes::vtable::VtableHeader {
                unsafe { &*Self::cast_header(self) }
            }
            #[track_caller]
            pub const fn ty(&self) -> ::classes::vtable::Type {
                self.object_ty().as_type()
            }
            #[track_caller]
            pub const fn object_ty(&self) -> ::classes::vtable::ConcreteClassType {
                let offset = self.header().offset_of_object_header();
                unsafe { &*Self::cast_header(self).byte_offset(offset) }
                    .object_ty()
                    .expect("expect object type")
            }
        }
        impl<V> self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::BuildContext<
                ::classes::ptr::RcDyn<self::BuildContext>,
                ::classes::class::Virtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::BuildContext<
                ::classes::ptr::RcDyn<self::BuildContext>,
                ::classes::class::NonVirtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {}
        impl<V> self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            #[inline]
            #[track_caller]
            pub fn try_into_superclass<A>(self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<
                    A::Class,
                >(self._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn into_superclass<A>(self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn into_superclass_unchecked<A>(self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<
                        A::Class,
                    >(self._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_to_superclass<A>(&self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<
                    A::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn to_superclass<A>(&self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<
                    A::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn to_superclass_unchecked<A>(&self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<
                        A::Class,
                    >(self.clone()._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn as_superclass_unchecked<A>(&self) -> &A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_superclass_unchecked::<
                        A::Class,
                    >(self._as_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn as_superclass<A>(&self) -> &A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::as_superclass::<A::Class>(self._as_inner()).into()
            }
            #[inline]
            #[track_caller]
            pub fn try_as_superclass<A>(&self) -> Option<&A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_superclass::<A::Class>(self._as_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn into_supertype<A>(self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if C::KIND.is_mixin()
                            && A::TYPE.const_eq(::classes::object::Object::TYPE)
                        {} else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_supertype::<A::Class>(self._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn to_supertype<A>(&self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if C::KIND.is_mixin()
                            && A::TYPE.const_eq(::classes::object::Object::TYPE)
                        {} else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::to_supertype::<A::Class>(self._as_inner()).into()
            }
            #[inline]
            #[track_caller]
            pub fn try_into_supertype<A>(self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_supertype::<A::Class>(self._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn try_to_supertype<A>(&self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                ::classes::ptr::RcDyn::try_to_supertype::<A::Class>(self._as_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn try_into_subtype<D>(self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_subtype::<D::Class>(self._into_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn try_to_subtype<D>(&self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_subtype::<
                    D::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn into_subtype<D>(self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self._into_inner())
                    .into()
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn to_subtype<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::into_subtype::<
                    D::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn upcast<A, I>(&self) -> I
            where
                A: ::classes::class::IsClass<Class: ::classes::class::HasImpl<I::Class>>,
                I: ::classes::class::ClassRc,
                for<'a> &'a I: From<&'a ::classes::ptr::RcDyn<I::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::upcast::<
                    A::Class,
                    I::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn upcast_unchecked<A, I>(&self) -> I
            where
                A: ::classes::class::IsClass<Class: ::classes::class::HasImpl<I::Class>>,
                I: ::classes::class::ClassRc,
                for<'a> &'a I: From<&'a ::classes::ptr::RcDyn<I::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::upcast_unchecked::<
                        A::Class,
                        I::Class,
                    >(self.clone()._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_upcast<A, I>(&self) -> Option<I>
            where
                A: ::classes::class::IsClass<Class: ::classes::class::HasImpl<I::Class>>,
                I: ::classes::class::ClassRc,
                for<'a> &'a I: From<&'a ::classes::ptr::RcDyn<I::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_upcast::<
                    A::Class,
                    I::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub unsafe fn downcast_unchecked<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<
                    Class: ::classes::class::HasImpl<self::BuildContext>,
                >,
                S: ::classes::class::ClassRc,
                for<'a> &'a S: From<&'a ::classes::ptr::RcDyn<S::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        S::Vtable,
                        <B::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::downcast_unchecked::<
                        B::Class,
                        S::Class,
                    >(self.clone()._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast<B, S>(&self) -> Option<S>
            where
                B: ::classes::class::IsClass<
                    Class: ::classes::class::HasImpl<self::BuildContext>,
                >,
                S: ::classes::class::ClassRc,
                for<'a> &'a S: From<&'a ::classes::ptr::RcDyn<S::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        S::Vtable,
                        <B::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_downcast::<
                    B::Class,
                    S::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn downcast<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<
                    Class: ::classes::class::HasImpl<self::BuildContext>,
                >,
                S: ::classes::class::ClassRc,
                for<'a> &'a S: From<&'a ::classes::ptr::RcDyn<S::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        S::Vtable,
                        <B::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::downcast::<
                    B::Class,
                    S::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            pub fn try_cast_mixin<M>(&self) -> Option<M>
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_mixin::<
                    M::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn cast_mixin<M>(&self) -> M
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::into_mixin::<M::Class>(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn cast_mixin_unchecked<M>(
                &self,
                instance: ::classes::vtable::MixinInstanceType,
            ) -> M
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                unsafe {
                    ::classes::ptr::RcDyn::into_mixin_unchecked::<
                        M::Class,
                    >(self.clone()._into_inner(), instance)
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast_ty(&self, ty: ::classes::vtable::Type) -> Option<&Self> {
                ::classes::ptr::RcDyn::try_downcast_ty(self._as_inner(), ty)
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn downcast_ty(&self, ty: ::classes::vtable::Type) -> &Self {
                ::classes::ptr::RcDyn::downcast_ty(self._as_inner(), ty).into()
            }
            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[track_caller]
            pub unsafe fn into_subclass_unchecked<D>(self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_subclass_unchecked::<
                        D::Class,
                    >(self._into_inner())
                }
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn into_subclass<D>(self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_subclass::<D::Class>(self._into_inner())
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn try_into_subclass<D>(self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_into_subclass::<D::Class>(self._into_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[track_caller]
            pub unsafe fn as_subclass_unchecked<D>(&self) -> &D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<
                        D::Class,
                    >(self._as_inner())
                }
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn as_subclass<D>(&self) -> &D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::as_subclass::<D::Class>(self._as_inner()).into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn try_as_subclass<D>(&self) -> Option<&D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[track_caller]
            pub unsafe fn to_subclass_unchecked<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<
                        D::Class,
                    >(self._as_inner())
                }
                    .clone()
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn to_subclass<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::as_subclass::<D::Class>(self._as_inner())
                    .clone()
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            pub fn try_to_subclass<D>(&self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner())
                    .cloned()
                    .map(Into::into)
            }
            #[inline]
            pub const fn ty(&self) -> ::classes::vtable::Type {
                self.0.vtable().ty()
            }
            #[inline]
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::BuildContext> {
                ::classes::ptr::RcDyn::as_ptr(this._as_inner())
            }
            #[inline]
            pub fn is_subtype_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subtype_of(C::TYPE)
            }
            #[inline]
            pub fn is_subclass_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subclass_of(C::TYPE)
            }
            #[inline]
            pub fn is_subtype_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subtype_of(ty)
            }
            #[inline]
            pub fn is_subclass_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subclass_of(ty)
            }
        }
        impl<V> self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
            #[inline]
            pub fn upgrade(
                &self,
            ) -> Option<
                self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V>,
            > {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner())
                    .map(self::BuildContext::_from_inner)
            }
            #[inline]
            pub fn strong_count(&self) -> usize {
                ::classes::ptr::WeakDyn::strong_count(self._as_inner())
            }
            #[inline]
            pub fn weak_count(&self) -> usize {
                ::classes::ptr::WeakDyn::weak_count(self._as_inner())
            }
            #[inline]
            pub const fn ty(&self) -> ::classes::vtable::Type {
                self.0.vtable().ty()
            }
            #[inline]
            pub fn is_subtype_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subtype_of(C::TYPE)
            }
            #[inline]
            pub fn is_subclass_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subclass_of(C::TYPE)
            }
            #[inline]
            pub fn is_subtype_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subtype_of(ty)
            }
            #[inline]
            pub fn is_subclass_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subclass_of(ty)
            }
        }
        type Super = ::classes::object::Object;
        unsafe impl ::classes::class::HasSuper for self::BuildContext {
            type Super = Object;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)] match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            type Super = Object<::classes::ptr::RcDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            type Target = Object<::classes::ptr::RcDyn<Object>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
            type Super = Object<::classes::ptr::WeakDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::BuildContext {
            type SuperData = ::classes::prelude::CData<Object>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::BuildContext {
            type SuperVtable = ::classes::prelude::CVtable<Object>;
        }
        impl<V> self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            #[inline]
            pub fn as_super(&self) -> &Object<::classes::ptr::RcDyn<Object>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Object<::classes::ptr::RcDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::RcDyn::into_super(self.clone()._into_inner()),
                )
            }
            #[inline]
            pub fn into_super(self) -> Object<::classes::ptr::RcDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::RcDyn::into_super(self._into_inner()),
                )
            }
        }
        impl self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Object<::classes::ptr::RcDyn<Object>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V> {
            #[inline]
            pub fn as_super(&self) -> &Object<::classes::ptr::WeakDyn<Object>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::WeakDyn::into_super(self.clone()._into_inner()),
                )
            }
            #[inline]
            pub fn into_super(self) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::WeakDyn::into_super(self._into_inner()),
                )
            }
        }
        impl vtable::BuildContext {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Object> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V>>
        for Object<::classes::ptr::RcDyn<Object>, V> {
            fn from(
                class: self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V>,
            ) -> Object<::classes::ptr::RcDyn<Object>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Object<::classes::ptr::RcDyn<Object>, V>>
        for self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V> {
            type Error = Object<::classes::ptr::RcDyn<Object>, V>;
            fn try_from(
                class: Object<::classes::ptr::RcDyn<Object>, V>,
            ) -> ::core::result::Result<
                self::BuildContext<::classes::ptr::RcDyn<self::BuildContext>, V>,
                Self::Error,
            > {
                class.try_as_subclass().cloned().ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V>>
        for Object<::classes::ptr::WeakDyn<Object>, V> {
            fn from(
                class: self::BuildContext<::classes::ptr::WeakDyn<self::BuildContext>, V>,
            ) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                class.into_super()
            }
        }
        mod data {
            use super::*;
            use ::classes::get_set::{New, NewCopy};
            use ::classes::prelude::*;
            use ::classes::ptr::RcDyn;
            pub(super) type Super = ::classes::prelude::CData<super::Super>;
            #[repr(C)]
            pub struct BuildContext {
                pub(super) _super: Super,
            }
            impl BuildContext {
                #[cold]
                #[inline(never)]
                pub fn _delegate_ctor<
                    _S: ::classes::class::IsClass,
                    _F: FnOnce(
                            ::classes::prelude::CRcUninit<_S>,
                        ) -> ::classes::prelude::CRc<_S>,
                >(
                    mut _self: ::classes::prelude::CRcUninit<Self>,
                    new: _F,
                ) -> ::classes::prelude::CRc<Self>
                where
                    ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                    for<'a> &'a ::classes::prelude::CRc<
                        _S,
                    >: From<
                        &'a ::classes::ptr::RcDyn<
                            <::classes::prelude::CRc<
                                _S,
                            > as ::classes::class::IsClass>::Class,
                        >,
                    >,
                {
                    let _ = new;
                    {
                        ::core::panicking::panic_fmt(format_args!("unsupported"));
                    }
                }
            }
        }
        mod vtable {
            use super::*;
            use ::classes::class::{
                ClassVtable, ClassVtableBase, NonVirtual, Virtual, VtableHasImpl,
                VtableHasSuper,
            };
            use ::classes::prelude::*;
            use ::classes::vtable::{MixinVtableHeader, TypeInfo, VtableHeader};
            pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
            #[repr(C)]
            pub struct BuildContext {
                pub(super) _super: Super,
                pub widget: fn(&::classes::prelude::CRc<Self>) -> CRc<Widget>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for BuildContext {
                #[inline]
                fn clone(&self) -> BuildContext {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>) -> CRc<Widget>,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for BuildContext {}
            impl BuildContext {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout {
                        this: self,
                        offset,
                    }
                }
            }
            pub struct DebugVtableLayout<'a> {
                this: &'a self::BuildContext,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("BuildContext");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self
                            .this
                            ._super
                            .debug_vtable_layout(
                                self.offset + { builtin # offset_of(BuildContext, _super) },
                            ),
                    );
                    dbg.field(
                        "widget",
                        &(self.offset + { builtin # offset_of(BuildContext, widget) }),
                    );
                    dbg.field(
                        "\'end",
                        &(self.offset + ::core::mem::size_of::<BuildContext>()),
                    );
                    dbg.finish()
                }
            }
            pub(super) mod opt {
                use super::*;
                use ::classes::class::{
                    ClassVtable, NonVirtual, Virtual, VtableHasImpl, VtableHasSuper,
                };
                use ::classes::prelude::*;
                use ::classes::vtable::{MixinVtableHeader, VtableHeaderOpt};
                pub(in super::super) type Super = ::classes::prelude::CVtableOpt<
                    super::super::Super,
                >;
                #[repr(C)]
                pub struct BuildContext {
                    pub(in super::super) _super: Super,
                    pub widget: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>) -> CRc<Widget>,
                    >,
                }
                #[automatically_derived]
                impl ::core::default::Default for BuildContext {
                    #[inline]
                    fn default() -> BuildContext {
                        BuildContext {
                            _super: ::core::default::Default::default(),
                            widget: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for BuildContext {
                    #[inline]
                    fn clone(&self) -> BuildContext {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<
                                fn(&::classes::prelude::CRc<Self>) -> CRc<Widget>,
                            >,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for BuildContext {}
                impl BuildContext {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
                        widget: ::core::option::Option::None,
                    };
                    pub const fn init_mixin_header(
                        mixin_header: &mut [::core::mem::MaybeUninit<
                            ::classes::vtable::MixinVtableHeader,
                        >],
                    ) {
                        Super::init_mixin_header(mixin_header);
                    }
                    pub const fn init_header(
                        &mut self,
                        ty: ::core::option::Option<::classes::vtable::Type>,
                        offset: usize,
                    ) {
                        let ty = match ty {
                            ::core::option::Option::None => Self::TYPE,
                            ::core::option::Option::Some(ty) => ty,
                        };
                        self._super
                            .init_header(::core::option::Option::Some(ty), offset);
                    }
                    #[allow(unused_unsafe)]
                    pub const fn init<V: ::classes::class::ClassVtableOpt>(
                        _self: &mut V,
                    ) {
                        Super::init(_self);
                    }
                    #[track_caller]
                    pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                        ::classes::prelude::CVtable::<Self> {
                            _super: self._super.assert_init(),
                            widget: self
                                .widget
                                .expect(
                                    "cannot instantiate because method `BuildContext::widget` is not implemented",
                                ),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<0usize> = ::classes::vtable::TypeInfo::new_abstract_class::<
                super::BuildContext,
            >(
                ::core::option::Option::Some(Super::TYPE),
                [],
                MODULE_PATH,
                "BuildContext",
            );
        }
        const _: () = {
            if !(::core::mem::size_of::<vtable::BuildContext>()
                == ::core::mem::size_of::<vtable::opt::BuildContext>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: BuildContext != size of vtable :: opt :: BuildContext",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::BuildContext, widget) }
                == { builtin # offset_of(vtable::opt::BuildContext, widget) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: BuildContext::widget != offset of vtable :: opt :: BuildContext::widget",
                        ),
                    );
                }
            }
        };
        impl BuildContext<::classes::ptr::RcDyn<BuildContext>> {
            #[inline]
            pub fn widget(&self) -> CRc<Widget> {
                { (self.0.vtable().widget)(self) }
            }
        }
        impl BuildContext<
            ::classes::ptr::RcDyn<BuildContext>,
            ::classes::class::NonVirtual,
        > {}
        impl BuildContext<::classes::ptr::RcDyn<BuildContext>> {}
    }
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _Element::Element;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Element {
        use super::*;
        use ::core::ptr::NonNull;
        use ::classes::class::{ConcreteClass, NonVirtual, Virtual};
        use ::classes::get_set::{GetSet, GetSetCopy};
        use ::classes::prelude::*;
        use ::classes::ptr::RcDyn;
        use ::classes::vtable::{
            MaybeUninitVtableWithMixinHeader, VtableHeader, VtableWithMixinHeader,
        };
        #[repr(transparent)]
        pub struct Element<
            T = ::classes::class::ClassMarker,
            V = ::classes::class::Virtual,
        >(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::Element<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::Element<T, V> {}
        impl<T, V> self::Element<T, V> {
            #[doc(hidden)]
            #[inline]
            pub fn _into_inner(self) -> T {
                self.0
            }
            #[doc(hidden)]
            #[inline]
            pub fn _as_inner(&self) -> &T {
                &self.0
            }
            #[doc(hidden)]
            #[inline]
            pub fn _from_inner(inner: T) -> Self {
                Self(inner, ::core::marker::PhantomData)
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::Element>>
        for self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            fn from(inner: ::classes::ptr::RcDyn<self::Element>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<self::Element<::classes::ptr::RcDyn<self::Element>, V>>
        for ::classes::ptr::RcDyn<self::Element> {
            fn from(
                this: self::Element<::classes::ptr::RcDyn<self::Element>, V>,
            ) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::Element>>
        for self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            fn from(inner: ::classes::ptr::WeakDyn<self::Element>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<self::Element<::classes::ptr::WeakDyn<self::Element>, V>>
        for ::classes::ptr::WeakDyn<self::Element> {
            fn from(
                this: self::Element<::classes::ptr::WeakDyn<self::Element>, V>,
            ) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::Element<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::Element<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak
        for self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            type Upgraded = Self;
            type UpgradedOpt = Self;
            type DowngradeFrom = Self;
            fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<Self> {
                ::classes::ptr::RcDyn::as_ptr(this._as_inner())
            }
            fn vtable(this: &Self) -> &Self::Vtable {
                this._as_inner().vtable()
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
        impl<V> ::classes::class::ClassRcWeak
        for self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            type Upgraded = Option<
                self::Element<::classes::ptr::RcDyn<self::Element>, V>,
            >;
            type UpgradedOpt = self::Element<::classes::ptr::RcDyn<self::Element>, V>;
            type DowngradeFrom = self::Element<::classes::ptr::RcDyn<self::Element>, V>;
            fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<Self> {
                this._as_inner().as_ptr()
            }
            fn vtable(this: &Self) -> &Self::Vtable {
                this._as_inner().vtable()
            }
            fn upgrade(this: &Self) -> Self::Upgraded {
                this.upgrade()
            }
            fn upgrade_opt(this: Option<&Self>) -> Option<Self::UpgradedOpt> {
                this.and_then(|this| this.upgrade())
            }
            fn downgrade_from(from: &Self::DowngradeFrom) -> Self {
                self::Element::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
        for self::Element<::classes::ptr::RcDyn<self::Element>, V>
        where
            for<'a> &'a C: ::core::convert::From<&'a ::classes::ptr::RcDyn<C::Class>>,
        {
            fn eq(&self, other: &C) -> bool {
                type CRcEqHash = ::classes::prelude::CRc<::classes::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    let other = ::classes::class::ClassRc::to_supertype::<
                        ::classes::prelude::CRc<::classes::object::Object>,
                    >(other);
                    CRcEqHash::eq(&this, &other)
                } else {
                    ::classes::class::ClassRcWeak::as_ptr(self)
                        == ::classes::class::ClassRcWeak::as_ptr(other)
                }
            }
        }
        impl<V> ::core::cmp::Eq
        for self::Element<::classes::ptr::RcDyn<self::Element>, V>
        where
            for<'a> &'a Self: ::core::convert::From<
                &'a ::classes::ptr::RcDyn<self::Element>,
            >,
        {}
        impl<V> ::core::hash::Hash
        for self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                type CRcEqHash = ::classes::prelude::CRc<::classes::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    CRcEqHash::hash(&this, state);
                } else {
                    ::core::hash::Hash::hash(
                        &::classes::class::ClassRcWeak::as_ptr(self),
                        state,
                    );
                }
            }
        }
        impl<V, C: ::classes::class::ClassRcWeak> ::core::cmp::PartialEq<C>
        for self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq
        for self::Element<::classes::ptr::WeakDyn<self::Element>, V> {}
        impl<V> ::core::hash::Hash
        for self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(
                    &::classes::class::ClassRcWeak::as_ptr(self),
                    state,
                );
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug
        for self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                type CRcFormat = ::classes::prelude::CRc<::classes::fmt::Format>;
                if let Some(this) = self.try_to_supertype::<CRcFormat>() {
                    CRcFormat::fmt_debug(&this, f)
                } else {
                    ::core::fmt::Display::fmt(
                        &::classes::class::ClassRcWeak::as_ptr(self),
                        f,
                    )
                }
            }
        }
        impl<V> ::core::fmt::Debug
        for self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if let Some(this) = self.upgrade() {
                    ::core::fmt::Debug::fmt(&this, f)
                } else {
                    ::core::fmt::Display::fmt(
                        &::classes::class::ClassRcWeak::as_ptr(self),
                        f,
                    )
                }
            }
        }
        impl<T, V> ::classes::class::IsClass for self::Element<T, V> {
            type Class = self::Element;
        }
        impl ::classes::class::IsClass for data::Element {
            type Class = self::Element;
        }
        impl self::Element {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::Element {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::Element {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize = <vtable::Element as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::Element {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::Element {
            type Class = self::Element;
        }
        impl ::classes::class::IsClass for vtable::opt::Element {
            type Class = self::Element;
        }
        impl ::classes::class::ClassDataBase for data::Element {
            type Vtable = vtable::Element;
        }
        impl ::classes::class::ClassVtableBase for vtable::Element {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::Element;
            type Opt = vtable::opt::Element;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::Element<T, V> {
            type DataBase = data::Element;
            type Data = data::Element;
            type VtableBase = vtable::Element;
            type Vtable = vtable::Element;
            type VtableOpt = vtable::opt::Element;
        }
        impl ::classes::class::ClassData for data::Element {}
        unsafe impl ::classes::class::ClassVtable for vtable::Element {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::Element {
            type VtableBase = vtable::Element;
            type Vtable = vtable::Element;
        }
        impl<V> ::classes::class::Class
        for self::Element<::classes::class::ClassMarker, V> {
            type Rc = self::Element<::classes::ptr::RcDyn<self::Element>, V>;
            type Weak = self::Element<::classes::ptr::WeakDyn<self::Element>, V>;
            type Ptr = ::classes::ptr::PtrDyn<vtable::Element>;
        }
        impl<V> self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            pub fn downgrade(
                this: &Self,
            ) -> self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
                self::Element::_from_inner(
                    ::classes::ptr::RcDyn::downgrade(this._as_inner()),
                )
            }
        }
        impl vtable::Element {
            #[inline]
            const fn cast_header(
                this: *const Self,
            ) -> *const ::classes::vtable::VtableHeader {
                this.cast()
            }
            pub const fn header(&self) -> &::classes::vtable::VtableHeader {
                unsafe { &*Self::cast_header(self) }
            }
            #[track_caller]
            pub const fn ty(&self) -> ::classes::vtable::Type {
                self.object_ty().as_type()
            }
            #[track_caller]
            pub const fn object_ty(&self) -> ::classes::vtable::ConcreteClassType {
                let offset = self.header().offset_of_object_header();
                unsafe { &*Self::cast_header(self).byte_offset(offset) }
                    .object_ty()
                    .expect("expect object type")
            }
        }
        impl<V> self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::Element<
                ::classes::ptr::RcDyn<self::Element>,
                ::classes::class::Virtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::Element<
                ::classes::ptr::RcDyn<self::Element>,
                ::classes::class::NonVirtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc
        for self::Element<::classes::ptr::RcDyn<self::Element>, V> {}
        impl<V> self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            #[inline]
            #[track_caller]
            pub fn try_into_superclass<A>(self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<
                    A::Class,
                >(self._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn into_superclass<A>(self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn into_superclass_unchecked<A>(self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<
                        A::Class,
                    >(self._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_to_superclass<A>(&self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<
                    A::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn to_superclass<A>(&self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<
                    A::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn to_superclass_unchecked<A>(&self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<
                        A::Class,
                    >(self.clone()._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn as_superclass_unchecked<A>(&self) -> &A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_superclass_unchecked::<
                        A::Class,
                    >(self._as_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn as_superclass<A>(&self) -> &A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::as_superclass::<A::Class>(self._as_inner()).into()
            }
            #[inline]
            #[track_caller]
            pub fn try_as_superclass<A>(&self) -> Option<&A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_superclass::<A::Class>(self._as_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn into_supertype<A>(self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if C::KIND.is_mixin()
                            && A::TYPE.const_eq(::classes::object::Object::TYPE)
                        {} else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_supertype::<A::Class>(self._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn to_supertype<A>(&self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if C::KIND.is_mixin()
                            && A::TYPE.const_eq(::classes::object::Object::TYPE)
                        {} else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::to_supertype::<A::Class>(self._as_inner()).into()
            }
            #[inline]
            #[track_caller]
            pub fn try_into_supertype<A>(self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_supertype::<A::Class>(self._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn try_to_supertype<A>(&self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                ::classes::ptr::RcDyn::try_to_supertype::<A::Class>(self._as_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn try_into_subtype<D>(self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_subtype::<D::Class>(self._into_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn try_to_subtype<D>(&self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_subtype::<
                    D::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn into_subtype<D>(self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self._into_inner())
                    .into()
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn to_subtype<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::into_subtype::<
                    D::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn upcast<A, I>(&self) -> I
            where
                A: ::classes::class::IsClass<Class: ::classes::class::HasImpl<I::Class>>,
                I: ::classes::class::ClassRc,
                for<'a> &'a I: From<&'a ::classes::ptr::RcDyn<I::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::upcast::<
                    A::Class,
                    I::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn upcast_unchecked<A, I>(&self) -> I
            where
                A: ::classes::class::IsClass<Class: ::classes::class::HasImpl<I::Class>>,
                I: ::classes::class::ClassRc,
                for<'a> &'a I: From<&'a ::classes::ptr::RcDyn<I::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::upcast_unchecked::<
                        A::Class,
                        I::Class,
                    >(self.clone()._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_upcast<A, I>(&self) -> Option<I>
            where
                A: ::classes::class::IsClass<Class: ::classes::class::HasImpl<I::Class>>,
                I: ::classes::class::ClassRc,
                for<'a> &'a I: From<&'a ::classes::ptr::RcDyn<I::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_upcast::<
                    A::Class,
                    I::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub unsafe fn downcast_unchecked<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<
                    Class: ::classes::class::HasImpl<self::Element>,
                >,
                S: ::classes::class::ClassRc,
                for<'a> &'a S: From<&'a ::classes::ptr::RcDyn<S::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        S::Vtable,
                        <B::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::downcast_unchecked::<
                        B::Class,
                        S::Class,
                    >(self.clone()._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast<B, S>(&self) -> Option<S>
            where
                B: ::classes::class::IsClass<
                    Class: ::classes::class::HasImpl<self::Element>,
                >,
                S: ::classes::class::ClassRc,
                for<'a> &'a S: From<&'a ::classes::ptr::RcDyn<S::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        S::Vtable,
                        <B::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_downcast::<
                    B::Class,
                    S::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn downcast<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<
                    Class: ::classes::class::HasImpl<self::Element>,
                >,
                S: ::classes::class::ClassRc,
                for<'a> &'a S: From<&'a ::classes::ptr::RcDyn<S::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        S::Vtable,
                        <B::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::downcast::<
                    B::Class,
                    S::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            pub fn try_cast_mixin<M>(&self) -> Option<M>
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_mixin::<
                    M::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn cast_mixin<M>(&self) -> M
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::into_mixin::<M::Class>(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn cast_mixin_unchecked<M>(
                &self,
                instance: ::classes::vtable::MixinInstanceType,
            ) -> M
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                unsafe {
                    ::classes::ptr::RcDyn::into_mixin_unchecked::<
                        M::Class,
                    >(self.clone()._into_inner(), instance)
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast_ty(&self, ty: ::classes::vtable::Type) -> Option<&Self> {
                ::classes::ptr::RcDyn::try_downcast_ty(self._as_inner(), ty)
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn downcast_ty(&self, ty: ::classes::vtable::Type) -> &Self {
                ::classes::ptr::RcDyn::downcast_ty(self._as_inner(), ty).into()
            }
            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[track_caller]
            pub unsafe fn into_subclass_unchecked<D>(self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_subclass_unchecked::<
                        D::Class,
                    >(self._into_inner())
                }
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn into_subclass<D>(self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_subclass::<D::Class>(self._into_inner())
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn try_into_subclass<D>(self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_into_subclass::<D::Class>(self._into_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[track_caller]
            pub unsafe fn as_subclass_unchecked<D>(&self) -> &D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<
                        D::Class,
                    >(self._as_inner())
                }
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn as_subclass<D>(&self) -> &D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::as_subclass::<D::Class>(self._as_inner()).into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn try_as_subclass<D>(&self) -> Option<&D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[track_caller]
            pub unsafe fn to_subclass_unchecked<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<
                        D::Class,
                    >(self._as_inner())
                }
                    .clone()
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn to_subclass<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::as_subclass::<D::Class>(self._as_inner())
                    .clone()
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            pub fn try_to_subclass<D>(&self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner())
                    .cloned()
                    .map(Into::into)
            }
            #[inline]
            pub const fn ty(&self) -> ::classes::vtable::Type {
                self.0.vtable().ty()
            }
            #[inline]
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::Element> {
                ::classes::ptr::RcDyn::as_ptr(this._as_inner())
            }
            #[inline]
            pub fn is_subtype_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subtype_of(C::TYPE)
            }
            #[inline]
            pub fn is_subclass_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subclass_of(C::TYPE)
            }
            #[inline]
            pub fn is_subtype_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subtype_of(ty)
            }
            #[inline]
            pub fn is_subclass_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subclass_of(ty)
            }
        }
        impl<V> self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            #[inline]
            pub fn upgrade(
                &self,
            ) -> Option<self::Element<::classes::ptr::RcDyn<self::Element>, V>> {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner())
                    .map(self::Element::_from_inner)
            }
            #[inline]
            pub fn strong_count(&self) -> usize {
                ::classes::ptr::WeakDyn::strong_count(self._as_inner())
            }
            #[inline]
            pub fn weak_count(&self) -> usize {
                ::classes::ptr::WeakDyn::weak_count(self._as_inner())
            }
            #[inline]
            pub const fn ty(&self) -> ::classes::vtable::Type {
                self.0.vtable().ty()
            }
            #[inline]
            pub fn is_subtype_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subtype_of(C::TYPE)
            }
            #[inline]
            pub fn is_subclass_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subclass_of(C::TYPE)
            }
            #[inline]
            pub fn is_subtype_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subtype_of(ty)
            }
            #[inline]
            pub fn is_subclass_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subclass_of(ty)
            }
        }
        type Super = ::classes::object::Object;
        unsafe impl ::classes::class::HasSuper for self::Element {
            type Super = Object;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)] match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            type Super = Object<::classes::ptr::RcDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref
        for self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            type Target = Object<::classes::ptr::RcDyn<Object>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            type Super = Object<::classes::ptr::WeakDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::Element {
            type SuperData = ::classes::prelude::CData<Object>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::Element {
            type SuperVtable = ::classes::prelude::CVtable<Object>;
        }
        impl<V> self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            #[inline]
            pub fn as_super(&self) -> &Object<::classes::ptr::RcDyn<Object>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Object<::classes::ptr::RcDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::RcDyn::into_super(self.clone()._into_inner()),
                )
            }
            #[inline]
            pub fn into_super(self) -> Object<::classes::ptr::RcDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::RcDyn::into_super(self._into_inner()),
                )
            }
        }
        impl self::Element<::classes::ptr::RcDyn<self::Element>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Object<::classes::ptr::RcDyn<Object>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            #[inline]
            pub fn as_super(&self) -> &Object<::classes::ptr::WeakDyn<Object>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::WeakDyn::into_super(self.clone()._into_inner()),
                )
            }
            #[inline]
            pub fn into_super(self) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::WeakDyn::into_super(self._into_inner()),
                )
            }
        }
        impl vtable::Element {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Object> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::Element<::classes::ptr::RcDyn<self::Element>, V>>
        for Object<::classes::ptr::RcDyn<Object>, V> {
            fn from(
                class: self::Element<::classes::ptr::RcDyn<self::Element>, V>,
            ) -> Object<::classes::ptr::RcDyn<Object>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Object<::classes::ptr::RcDyn<Object>, V>>
        for self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            type Error = Object<::classes::ptr::RcDyn<Object>, V>;
            fn try_from(
                class: Object<::classes::ptr::RcDyn<Object>, V>,
            ) -> ::core::result::Result<
                self::Element<::classes::ptr::RcDyn<self::Element>, V>,
                Self::Error,
            > {
                class.try_as_subclass().cloned().ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::Element<::classes::ptr::WeakDyn<self::Element>, V>>
        for Object<::classes::ptr::WeakDyn<Object>, V> {
            fn from(
                class: self::Element<::classes::ptr::WeakDyn<self::Element>, V>,
            ) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                class.into_super()
            }
        }
        impl ::classes::class::HasImpl<BuildContext> for self::Element {
            #[allow(unreachable_code)]
            fn to_impl(&self) -> BuildContext {
                match self.0 {}
            }
        }
        impl<
            V,
        > ::classes::class::HasImpl<BuildContext<::classes::ptr::RcDyn<BuildContext>, V>>
        for self::Element<::classes::ptr::RcDyn<self::Element>, V> {
            fn to_impl(&self) -> BuildContext<::classes::ptr::RcDyn<BuildContext>, V> {
                ::classes::ptr::RcDyn::into_impl::<
                    BuildContext,
                >(self.clone()._into_inner())
                    .into()
            }
        }
        impl<
            V,
        > ::classes::class::HasImpl<
            BuildContext<::classes::ptr::WeakDyn<BuildContext>, V>,
        > for self::Element<::classes::ptr::WeakDyn<self::Element>, V> {
            fn to_impl(&self) -> BuildContext<::classes::ptr::WeakDyn<BuildContext>, V> {
                ::classes::ptr::WeakDyn::into_impl::<
                    BuildContext,
                >(self.clone()._into_inner())
                    .into()
            }
        }
        unsafe impl ::classes::class::VtableHasImpl<BuildContext> for vtable::Element {
            const OFFSET: usize = { builtin # offset_of(vtable::Element, BuildContext) };
        }
        unsafe impl<
            V,
        > ::classes::class::VtableHasImpl<
            BuildContext<::classes::ptr::RcDyn<BuildContext>, V>,
        > for vtable::Element {
            const OFFSET: usize = { builtin # offset_of(vtable::Element, BuildContext) };
        }
        unsafe impl<
            V,
        > ::classes::class::VtableHasImpl<
            BuildContext<::classes::ptr::WeakDyn<BuildContext>, V>,
        > for vtable::Element {
            const OFFSET: usize = { builtin # offset_of(vtable::Element, BuildContext) };
        }
        impl<V> From<self::Element<::classes::ptr::RcDyn<self::Element>, V>>
        for super::BuildContext<::classes::ptr::RcDyn<super::BuildContext>, V> {
            fn from(
                class: self::Element<::classes::ptr::RcDyn<self::Element>, V>,
            ) -> super::BuildContext<::classes::ptr::RcDyn<super::BuildContext>, V> {
                ::classes::class::HasImpl::to_impl(&class)
            }
        }
        impl TryFrom<super::BuildContext<::classes::ptr::RcDyn<super::BuildContext>>>
        for self::Element<::classes::ptr::RcDyn<self::Element>> {
            type Error = super::BuildContext<::classes::ptr::RcDyn<super::BuildContext>>;
            fn try_from(
                class: BuildContext<::classes::ptr::RcDyn<BuildContext>>,
            ) -> ::core::result::Result<
                Element<::classes::ptr::RcDyn<Element>>,
                Self::Error,
            > {
                class.try_downcast::<Self, Self>().ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::Element<::classes::ptr::WeakDyn<self::Element>, V>>
        for super::BuildContext<::classes::ptr::WeakDyn<super::BuildContext>, V> {
            fn from(
                class: self::Element<::classes::ptr::WeakDyn<self::Element>, V>,
            ) -> super::BuildContext<::classes::ptr::WeakDyn<super::BuildContext>, V> {
                ::classes::class::HasImpl::to_impl(&class)
            }
        }
        mod data {
            use super::*;
            use ::classes::get_set::{New, NewCopy};
            use ::classes::prelude::*;
            use ::classes::ptr::RcDyn;
            pub(super) type Super = ::classes::prelude::CData<super::Super>;
            #[repr(C)]
            pub struct Element {
                pub(super) _super: Super,
                pub(super) parent: ::core::cell::Cell<Option<CWeak<Element>>>,
                pub(super) widget: ::core::cell::Cell<Option<CWeak<Widget>>>,
                pub(super) dirty: ::core::cell::Cell<bool>,
            }
            impl Element {
                #[cold]
                #[inline(never)]
                pub fn _delegate_ctor<
                    _S: ::classes::class::IsClass,
                    _F: FnOnce(
                            ::classes::prelude::CRcUninit<_S>,
                        ) -> ::classes::prelude::CRc<_S>,
                >(
                    mut _self: ::classes::prelude::CRcUninit<Self>,
                    new: _F,
                ) -> ::classes::prelude::CRc<Self>
                where
                    ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                    for<'a> &'a ::classes::prelude::CRc<
                        _S,
                    >: From<
                        &'a ::classes::ptr::RcDyn<
                            <::classes::prelude::CRc<
                                _S,
                            > as ::classes::class::IsClass>::Class,
                        >,
                    >,
                {
                    let _ = new;
                    {
                        ::core::panicking::panic_fmt(format_args!("unsupported"));
                    }
                }
                pub fn new(
                    mut _self: ::classes::prelude::CRcUninit<Self>,
                    widget: Option<CRc<Widget>>,
                ) -> ::classes::prelude::CRc<Self> {
                    unsafe {
                        ::core::ptr::write(
                            &raw mut (*_self.as_mut_ptr()).parent,
                            ::classes::get_set::New::new_cell(None),
                        );
                        ::core::ptr::write(
                            &raw mut (*_self.as_mut_ptr()).widget,
                            ::classes::get_set::New::new_cell(widget),
                        );
                        ::core::ptr::write(
                            &raw mut (*_self.as_mut_ptr()).dirty,
                            ::classes::get_set::NewCopy::new_cell(true),
                        );
                        let _ = |Self { _super, parent: _, widget: _, dirty: _ }: Self| ();
                        ::classes::prelude::CData::<
                            ::classes::object::Object,
                        >::new(_self.into_super())
                            .into_subclass_unchecked()
                    }
                }
                pub(super) fn rebuild(
                    _self: &::classes::prelude::CRc<Self>,
                    force: bool,
                ) {
                    if _self.get_dirty() || force {
                        _self.perform_rebuild();
                    }
                }
                pub(super) fn perform_rebuild(_self: &::classes::prelude::CRc<Self>) {
                    _self.set_dirty(false);
                }
                pub(super) fn widget(
                    _self: &::classes::prelude::CRc<Self>,
                ) -> CRc<Widget> {
                    _self.get_widget().unwrap()
                }
            }
        }
        mod vtable {
            use super::*;
            use ::classes::class::{
                ClassVtable, ClassVtableBase, NonVirtual, Virtual, VtableHasImpl,
                VtableHasSuper,
            };
            use ::classes::prelude::*;
            use ::classes::vtable::{MixinVtableHeader, TypeInfo, VtableHeader};
            pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
            #[repr(C)]
            pub struct Element {
                pub(super) _super: Super,
                pub rebuild: fn(&::classes::prelude::CRc<Self>, force: bool),
                pub perform_rebuild: fn(&::classes::prelude::CRc<Self>),
                pub BuildContext: ::classes::prelude::CVtable<BuildContext>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Element {
                #[inline]
                fn clone(&self) -> Element {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>, force: bool),
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>),
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        ::classes::prelude::CVtable<BuildContext>,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Element {}
            impl Element {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout {
                        this: self,
                        offset,
                    }
                }
            }
            pub struct DebugVtableLayout<'a> {
                this: &'a self::Element,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("Element");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self
                            .this
                            ._super
                            .debug_vtable_layout(
                                self.offset + { builtin # offset_of(Element, _super) },
                            ),
                    );
                    dbg.field(
                        "rebuild",
                        &(self.offset + { builtin # offset_of(Element, rebuild) }),
                    );
                    dbg.field(
                        "perform_rebuild",
                        &(self.offset
                            + { builtin # offset_of(Element, perform_rebuild) }),
                    );
                    dbg.field(
                        "BuildContext",
                        &self
                            .this
                            .BuildContext
                            .debug_vtable_layout(
                                self.offset + { builtin # offset_of(Element, BuildContext) },
                            ),
                    );
                    dbg.field(
                        "\'end",
                        &(self.offset + ::core::mem::size_of::<Element>()),
                    );
                    dbg.finish()
                }
            }
            pub(super) mod opt {
                use super::*;
                use ::classes::class::{
                    ClassVtable, NonVirtual, Virtual, VtableHasImpl, VtableHasSuper,
                };
                use ::classes::prelude::*;
                use ::classes::vtable::{MixinVtableHeader, VtableHeaderOpt};
                pub(in super::super) type Super = ::classes::prelude::CVtableOpt<
                    super::super::Super,
                >;
                #[repr(C)]
                pub struct Element {
                    pub(in super::super) _super: Super,
                    pub rebuild: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>, force: bool),
                    >,
                    pub perform_rebuild: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>),
                    >,
                    pub BuildContext: ::classes::prelude::CVtableOpt<BuildContext>,
                }
                #[automatically_derived]
                impl ::core::default::Default for Element {
                    #[inline]
                    fn default() -> Element {
                        Element {
                            _super: ::core::default::Default::default(),
                            rebuild: ::core::default::Default::default(),
                            perform_rebuild: ::core::default::Default::default(),
                            BuildContext: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Element {
                    #[inline]
                    fn clone(&self) -> Element {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<
                                fn(&::classes::prelude::CRc<Self>, force: bool),
                            >,
                        >;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<fn(&::classes::prelude::CRc<Self>)>,
                        >;
                        let _: ::core::clone::AssertParamIsClone<
                            ::classes::prelude::CVtableOpt<BuildContext>,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Element {}
                impl Element {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
                        rebuild: ::core::option::Option::None,
                        perform_rebuild: ::core::option::Option::None,
                        BuildContext: ::classes::prelude::CVtableOpt::<
                            BuildContext,
                        >::DEFAULT,
                    };
                    pub const fn init_mixin_header(
                        mixin_header: &mut [::core::mem::MaybeUninit<
                            ::classes::vtable::MixinVtableHeader,
                        >],
                    ) {
                        Super::init_mixin_header(mixin_header);
                    }
                    pub const fn init_header(
                        &mut self,
                        ty: ::core::option::Option<::classes::vtable::Type>,
                        offset: usize,
                    ) {
                        let ty = match ty {
                            ::core::option::Option::None => Self::TYPE,
                            ::core::option::Option::Some(ty) => ty,
                        };
                        self._super
                            .init_header(::core::option::Option::Some(ty), offset);
                        self.BuildContext
                            .init_header(
                                ::core::option::Option::None,
                                offset
                                    + {
                                        builtin # offset_of(
                                            ::classes::prelude::CVtable < Self >, BuildContext
                                        )
                                    },
                            );
                    }
                    #[allow(unused_unsafe)]
                    pub const fn init<V: ::classes::class::ClassVtableOpt>(
                        _self: &mut V,
                    ) {
                        Super::init(_self);
                        {
                            let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self);
                            ptr.rebuild = ::core::option::Option::Some(|this, force| {
                                ::classes::prelude::CData::<
                                    Self,
                                >::rebuild(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        force,
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset) {
                                ptr.rebuild = ::core::option::Option::Some(|this, force| {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::rebuild(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                            force,
                                        )
                                        .into()
                                });
                            }
                        }
                        {
                            let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self);
                            ptr.perform_rebuild = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<
                                    Self,
                                >::perform_rebuild(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset) {
                                ptr.perform_rebuild = ::core::option::Option::Some(|this| {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::perform_rebuild(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        )
                                        .into()
                                });
                            }
                        }
                        {
                            let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<
                                _,
                                ::classes::prelude::CVtableOpt<BuildContext>,
                            >(_self);
                            ptr.widget = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<
                                    Self,
                                >::widget(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<BuildContext>,
                            >(_self, &mut offset) {
                                ptr.widget = ::core::option::Option::Some(|this| {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::widget(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        )
                                        .into()
                                });
                            }
                        }
                    }
                    #[track_caller]
                    pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                        ::classes::prelude::CVtable::<Self> {
                            _super: self._super.assert_init(),
                            rebuild: self
                                .rebuild
                                .expect(
                                    "cannot instantiate because method `Element::rebuild` is not implemented",
                                ),
                            perform_rebuild: self
                                .perform_rebuild
                                .expect(
                                    "cannot instantiate because method `Element::perform_rebuild` is not implemented",
                                ),
                            BuildContext: self.BuildContext.assert_init(),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<1usize> = ::classes::vtable::TypeInfo::new_abstract_class::<
                super::Element,
            >(
                ::core::option::Option::Some(Super::TYPE),
                [
                    (
                        ::classes::prelude::CVtable::<BuildContext>::TYPE,
                        { builtin # offset_of(vtable::Element, BuildContext) },
                    ),
                ],
                MODULE_PATH,
                "Element",
            );
        }
        const _: () = {
            if !(::core::mem::size_of::<vtable::Element>()
                == ::core::mem::size_of::<vtable::opt::Element>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Element != size of vtable :: opt :: Element",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Element, rebuild) }
                == { builtin # offset_of(vtable::opt::Element, rebuild) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Element::rebuild != offset of vtable :: opt :: Element::rebuild",
                        ),
                    );
                }
            }
            if !(::core::mem::size_of::<vtable::Element>()
                == ::core::mem::size_of::<vtable::opt::Element>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Element != size of vtable :: opt :: Element",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Element, perform_rebuild) }
                == { builtin # offset_of(vtable::opt::Element, perform_rebuild) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Element::perform_rebuild != offset of vtable :: opt :: Element::perform_rebuild",
                        ),
                    );
                }
            }
            if !(::core::mem::size_of::<vtable::Element>()
                == ::core::mem::size_of::<vtable::opt::Element>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Element != size of vtable :: opt :: Element",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Element, BuildContext) }
                == { builtin # offset_of(vtable::opt::Element, BuildContext) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Element::BuildContext != offset of vtable :: opt :: Element::BuildContext",
                        ),
                    );
                }
            }
        };
        impl Element<::classes::ptr::RcDyn<Element>> {
            #[inline]
            pub fn rebuild(&self, force: bool) {
                { (self.0.vtable().rebuild)(self, force) }
            }
            #[inline]
            pub fn perform_rebuild(&self) {
                { (self.0.vtable().perform_rebuild)(self) }
            }
            #[inline]
            pub fn widget(&self) -> CRc<Widget> {
                { self.to_supertype::<::classes::prelude::CRc<BuildContext>>().widget() }
                    .try_into()
                    .unwrap()
            }
        }
        impl Element<::classes::ptr::RcDyn<Element>, ::classes::class::NonVirtual> {
            #[inline]
            pub fn rebuild(&self, force: bool) {
                { ::classes::prelude::CData::<Self>::rebuild(self.as_virtual(), force) }
            }
            #[inline]
            pub fn perform_rebuild(&self) {
                { ::classes::prelude::CData::<Self>::perform_rebuild(self.as_virtual()) }
            }
            #[inline]
            pub fn widget(&self) -> CRc<Widget> {
                { ::classes::prelude::CData::<Self>::widget(self.as_virtual()) }
            }
        }
        impl Element<::classes::ptr::RcDyn<Element>> {
            #[inline]
            pub(in super::super) fn get_parent(
                &self,
            ) -> <Option<CWeak<Element>> as ::classes::get_set::GetSet>::Get {
                ::classes::get_set::GetSet::cell_get(&self.0.parent)
            }
            #[inline]
            pub(in super::super) fn set_parent<
                _T: ::core::convert::Into<
                        <Option<CWeak<Element>> as ::classes::get_set::GetSet>::Set,
                    >,
            >(&self, parent: _T) {
                ::classes::get_set::GetSet::cell_set(&self.0.parent, parent.into());
            }
            #[inline]
            #[must_use]
            pub(in super::super) fn replace_parent<
                _T: ::core::convert::Into<
                        <Option<CWeak<Element>> as ::classes::get_set::GetSet>::Set,
                    >,
            >(
                &self,
                parent: _T,
            ) -> <Option<CWeak<Element>> as ::classes::get_set::GetSet>::Get {
                let old = self.get_parent();
                self.set_parent(parent);
                old
            }
            #[inline]
            pub(in super::super) fn update_parent_with<
                _T: ::core::convert::Into<
                        <Option<CWeak<Element>> as ::classes::get_set::GetSet>::Set,
                    >,
                _F: ::core::ops::FnOnce(
                        <Option<CWeak<Element>> as ::classes::get_set::GetSet>::Get,
                    ) -> _T,
            >(&self, f: _F) {
                self.set_parent(f(self.get_parent()));
            }
            #[inline]
            pub(in super::super) fn raw_get_parent(
                &self,
            ) -> &::core::cell::Cell<Option<CWeak<Element>>> {
                &self.0.parent
            }
            #[inline]
            pub(in super::super) fn get_widget(
                &self,
            ) -> <Option<CWeak<Widget>> as ::classes::get_set::GetSet>::Get {
                ::classes::get_set::GetSet::cell_get(&self.0.widget)
            }
            #[inline]
            pub(in super::super) fn set_widget<
                _T: ::core::convert::Into<
                        <Option<CWeak<Widget>> as ::classes::get_set::GetSet>::Set,
                    >,
            >(&self, widget: _T) {
                ::classes::get_set::GetSet::cell_set(&self.0.widget, widget.into());
            }
            #[inline]
            #[must_use]
            pub(in super::super) fn replace_widget<
                _T: ::core::convert::Into<
                        <Option<CWeak<Widget>> as ::classes::get_set::GetSet>::Set,
                    >,
            >(
                &self,
                widget: _T,
            ) -> <Option<CWeak<Widget>> as ::classes::get_set::GetSet>::Get {
                let old = self.get_widget();
                self.set_widget(widget);
                old
            }
            #[inline]
            pub(in super::super) fn update_widget_with<
                _T: ::core::convert::Into<
                        <Option<CWeak<Widget>> as ::classes::get_set::GetSet>::Set,
                    >,
                _F: ::core::ops::FnOnce(
                        <Option<CWeak<Widget>> as ::classes::get_set::GetSet>::Get,
                    ) -> _T,
            >(&self, f: _F) {
                self.set_widget(f(self.get_widget()));
            }
            #[inline]
            pub(in super::super) fn raw_get_widget(
                &self,
            ) -> &::core::cell::Cell<Option<CWeak<Widget>>> {
                &self.0.widget
            }
            #[inline]
            pub(in super::super) fn get_dirty(&self) -> bool {
                ::classes::get_set::GetSetCopy::cell_get(&self.0.dirty)
            }
            #[inline]
            pub(in super::super) fn set_dirty<_T: ::core::convert::Into<bool>>(
                &self,
                dirty: _T,
            ) {
                ::classes::get_set::GetSetCopy::cell_set(&self.0.dirty, dirty.into());
            }
            #[inline]
            #[must_use]
            pub(in super::super) fn replace_dirty<_T: ::core::convert::Into<bool>>(
                &self,
                dirty: _T,
            ) -> bool {
                let old = self.get_dirty();
                self.set_dirty(dirty);
                old
            }
            #[inline]
            pub(in super::super) fn update_dirty_with<
                _T: ::core::convert::Into<bool>,
                _F: ::core::ops::FnOnce(bool) -> _T,
            >(&self, f: _F) {
                self.set_dirty(f(self.get_dirty()));
            }
            #[inline]
            pub(in super::super) fn raw_get_dirty(&self) -> &::core::cell::Cell<bool> {
                &self.0.dirty
            }
        }
    }
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _Widget::Widget;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Widget {
        use super::*;
        use ::core::ptr::NonNull;
        use ::classes::class::{ConcreteClass, NonVirtual, Virtual};
        use ::classes::get_set::{GetSet, GetSetCopy};
        use ::classes::prelude::*;
        use ::classes::ptr::RcDyn;
        use ::classes::vtable::{
            MaybeUninitVtableWithMixinHeader, VtableHeader, VtableWithMixinHeader,
        };
        #[repr(transparent)]
        pub struct Widget<
            T = ::classes::class::ClassMarker,
            V = ::classes::class::Virtual,
        >(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::Widget<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::Widget<T, V> {}
        impl<T, V> self::Widget<T, V> {
            #[doc(hidden)]
            #[inline]
            pub fn _into_inner(self) -> T {
                self.0
            }
            #[doc(hidden)]
            #[inline]
            pub fn _as_inner(&self) -> &T {
                &self.0
            }
            #[doc(hidden)]
            #[inline]
            pub fn _from_inner(inner: T) -> Self {
                Self(inner, ::core::marker::PhantomData)
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::Widget>>
        for self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            fn from(inner: ::classes::ptr::RcDyn<self::Widget>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<self::Widget<::classes::ptr::RcDyn<self::Widget>, V>>
        for ::classes::ptr::RcDyn<self::Widget> {
            fn from(this: self::Widget<::classes::ptr::RcDyn<self::Widget>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::Widget>>
        for self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
            fn from(inner: ::classes::ptr::WeakDyn<self::Widget>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<self::Widget<::classes::ptr::WeakDyn<self::Widget>, V>>
        for ::classes::ptr::WeakDyn<self::Widget> {
            fn from(
                this: self::Widget<::classes::ptr::WeakDyn<self::Widget>, V>,
            ) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::Widget<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::Widget<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak
        for self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            type Upgraded = Self;
            type UpgradedOpt = Self;
            type DowngradeFrom = Self;
            fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<Self> {
                ::classes::ptr::RcDyn::as_ptr(this._as_inner())
            }
            fn vtable(this: &Self) -> &Self::Vtable {
                this._as_inner().vtable()
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
        impl<V> ::classes::class::ClassRcWeak
        for self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
            type Upgraded = Option<self::Widget<::classes::ptr::RcDyn<self::Widget>, V>>;
            type UpgradedOpt = self::Widget<::classes::ptr::RcDyn<self::Widget>, V>;
            type DowngradeFrom = self::Widget<::classes::ptr::RcDyn<self::Widget>, V>;
            fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<Self> {
                this._as_inner().as_ptr()
            }
            fn vtable(this: &Self) -> &Self::Vtable {
                this._as_inner().vtable()
            }
            fn upgrade(this: &Self) -> Self::Upgraded {
                this.upgrade()
            }
            fn upgrade_opt(this: Option<&Self>) -> Option<Self::UpgradedOpt> {
                this.and_then(|this| this.upgrade())
            }
            fn downgrade_from(from: &Self::DowngradeFrom) -> Self {
                self::Widget::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
        for self::Widget<::classes::ptr::RcDyn<self::Widget>, V>
        where
            for<'a> &'a C: ::core::convert::From<&'a ::classes::ptr::RcDyn<C::Class>>,
        {
            fn eq(&self, other: &C) -> bool {
                type CRcEqHash = ::classes::prelude::CRc<::classes::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    let other = ::classes::class::ClassRc::to_supertype::<
                        ::classes::prelude::CRc<::classes::object::Object>,
                    >(other);
                    CRcEqHash::eq(&this, &other)
                } else {
                    ::classes::class::ClassRcWeak::as_ptr(self)
                        == ::classes::class::ClassRcWeak::as_ptr(other)
                }
            }
        }
        impl<V> ::core::cmp::Eq for self::Widget<::classes::ptr::RcDyn<self::Widget>, V>
        where
            for<'a> &'a Self: ::core::convert::From<
                &'a ::classes::ptr::RcDyn<self::Widget>,
            >,
        {}
        impl<V> ::core::hash::Hash
        for self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                type CRcEqHash = ::classes::prelude::CRc<::classes::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    CRcEqHash::hash(&this, state);
                } else {
                    ::core::hash::Hash::hash(
                        &::classes::class::ClassRcWeak::as_ptr(self),
                        state,
                    );
                }
            }
        }
        impl<V, C: ::classes::class::ClassRcWeak> ::core::cmp::PartialEq<C>
        for self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq
        for self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {}
        impl<V> ::core::hash::Hash
        for self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(
                    &::classes::class::ClassRcWeak::as_ptr(self),
                    state,
                );
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug
        for self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                type CRcFormat = ::classes::prelude::CRc<::classes::fmt::Format>;
                if let Some(this) = self.try_to_supertype::<CRcFormat>() {
                    CRcFormat::fmt_debug(&this, f)
                } else {
                    ::core::fmt::Display::fmt(
                        &::classes::class::ClassRcWeak::as_ptr(self),
                        f,
                    )
                }
            }
        }
        impl<V> ::core::fmt::Debug
        for self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if let Some(this) = self.upgrade() {
                    ::core::fmt::Debug::fmt(&this, f)
                } else {
                    ::core::fmt::Display::fmt(
                        &::classes::class::ClassRcWeak::as_ptr(self),
                        f,
                    )
                }
            }
        }
        impl<T, V> ::classes::class::IsClass for self::Widget<T, V> {
            type Class = self::Widget;
        }
        impl ::classes::class::IsClass for data::Widget {
            type Class = self::Widget;
        }
        impl self::Widget {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::Widget {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::Widget {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize = <vtable::Widget as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::Widget {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::Widget {
            type Class = self::Widget;
        }
        impl ::classes::class::IsClass for vtable::opt::Widget {
            type Class = self::Widget;
        }
        impl ::classes::class::ClassDataBase for data::Widget {
            type Vtable = vtable::Widget;
        }
        impl ::classes::class::ClassVtableBase for vtable::Widget {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::Widget;
            type Opt = vtable::opt::Widget;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::Widget<T, V> {
            type DataBase = data::Widget;
            type Data = data::Widget;
            type VtableBase = vtable::Widget;
            type Vtable = vtable::Widget;
            type VtableOpt = vtable::opt::Widget;
        }
        impl ::classes::class::ClassData for data::Widget {}
        unsafe impl ::classes::class::ClassVtable for vtable::Widget {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::Widget {
            type VtableBase = vtable::Widget;
            type Vtable = vtable::Widget;
        }
        impl<V> ::classes::class::Class
        for self::Widget<::classes::class::ClassMarker, V> {
            type Rc = self::Widget<::classes::ptr::RcDyn<self::Widget>, V>;
            type Weak = self::Widget<::classes::ptr::WeakDyn<self::Widget>, V>;
            type Ptr = ::classes::ptr::PtrDyn<vtable::Widget>;
        }
        impl<V> self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            pub fn downgrade(
                this: &Self,
            ) -> self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
                self::Widget::_from_inner(
                    ::classes::ptr::RcDyn::downgrade(this._as_inner()),
                )
            }
        }
        impl vtable::Widget {
            #[inline]
            const fn cast_header(
                this: *const Self,
            ) -> *const ::classes::vtable::VtableHeader {
                this.cast()
            }
            pub const fn header(&self) -> &::classes::vtable::VtableHeader {
                unsafe { &*Self::cast_header(self) }
            }
            #[track_caller]
            pub const fn ty(&self) -> ::classes::vtable::Type {
                self.object_ty().as_type()
            }
            #[track_caller]
            pub const fn object_ty(&self) -> ::classes::vtable::ConcreteClassType {
                let offset = self.header().offset_of_object_header();
                unsafe { &*Self::cast_header(self).byte_offset(offset) }
                    .object_ty()
                    .expect("expect object type")
            }
        }
        impl<V> self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::Widget<
                ::classes::ptr::RcDyn<self::Widget>,
                ::classes::class::Virtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::Widget<
                ::classes::ptr::RcDyn<self::Widget>,
                ::classes::class::NonVirtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc
        for self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {}
        impl<V> self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            #[inline]
            #[track_caller]
            pub fn try_into_superclass<A>(self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<
                    A::Class,
                >(self._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn into_superclass<A>(self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn into_superclass_unchecked<A>(self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<
                        A::Class,
                    >(self._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_to_superclass<A>(&self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<
                    A::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn to_superclass<A>(&self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<
                    A::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn to_superclass_unchecked<A>(&self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<
                        A::Class,
                    >(self.clone()._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn as_superclass_unchecked<A>(&self) -> &A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_superclass_unchecked::<
                        A::Class,
                    >(self._as_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn as_superclass<A>(&self) -> &A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::as_superclass::<A::Class>(self._as_inner()).into()
            }
            #[inline]
            #[track_caller]
            pub fn try_as_superclass<A>(&self) -> Option<&A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_superclass::<A::Class>(self._as_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn into_supertype<A>(self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if C::KIND.is_mixin()
                            && A::TYPE.const_eq(::classes::object::Object::TYPE)
                        {} else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_supertype::<A::Class>(self._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn to_supertype<A>(&self) -> A
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if C::KIND.is_mixin()
                            && A::TYPE.const_eq(::classes::object::Object::TYPE)
                        {} else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        A::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::to_supertype::<A::Class>(self._as_inner()).into()
            }
            #[inline]
            #[track_caller]
            pub fn try_into_supertype<A>(self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_supertype::<A::Class>(self._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn try_to_supertype<A>(&self) -> Option<A>
            where
                A: ::classes::class::ClassRc,
                for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
            {
                ::classes::ptr::RcDyn::try_to_supertype::<A::Class>(self._as_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn try_into_subtype<D>(self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_subtype::<D::Class>(self._into_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn try_to_subtype<D>(&self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_subtype::<
                    D::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn into_subtype<D>(self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self._into_inner())
                    .into()
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn to_subtype<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::into_subtype::<
                    D::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn upcast<A, I>(&self) -> I
            where
                A: ::classes::class::IsClass<Class: ::classes::class::HasImpl<I::Class>>,
                I: ::classes::class::ClassRc,
                for<'a> &'a I: From<&'a ::classes::ptr::RcDyn<I::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::upcast::<
                    A::Class,
                    I::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn upcast_unchecked<A, I>(&self) -> I
            where
                A: ::classes::class::IsClass<Class: ::classes::class::HasImpl<I::Class>>,
                I: ::classes::class::ClassRc,
                for<'a> &'a I: From<&'a ::classes::ptr::RcDyn<I::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::upcast_unchecked::<
                        A::Class,
                        I::Class,
                    >(self.clone()._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_upcast<A, I>(&self) -> Option<I>
            where
                A: ::classes::class::IsClass<Class: ::classes::class::HasImpl<I::Class>>,
                I: ::classes::class::ClassRc,
                for<'a> &'a I: From<&'a ::classes::ptr::RcDyn<I::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_upcast::<
                    A::Class,
                    I::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub unsafe fn downcast_unchecked<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<
                    Class: ::classes::class::HasImpl<self::Widget>,
                >,
                S: ::classes::class::ClassRc,
                for<'a> &'a S: From<&'a ::classes::ptr::RcDyn<S::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        S::Vtable,
                        <B::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::downcast_unchecked::<
                        B::Class,
                        S::Class,
                    >(self.clone()._into_inner())
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast<B, S>(&self) -> Option<S>
            where
                B: ::classes::class::IsClass<
                    Class: ::classes::class::HasImpl<self::Widget>,
                >,
                S: ::classes::class::ClassRc,
                for<'a> &'a S: From<&'a ::classes::ptr::RcDyn<S::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        S::Vtable,
                        <B::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_downcast::<
                    B::Class,
                    S::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn downcast<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<
                    Class: ::classes::class::HasImpl<self::Widget>,
                >,
                S: ::classes::class::ClassRc,
                for<'a> &'a S: From<&'a ::classes::ptr::RcDyn<S::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        S::Vtable,
                        <B::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::downcast::<
                    B::Class,
                    S::Class,
                >(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            pub fn try_cast_mixin<M>(&self) -> Option<M>
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_mixin::<
                    M::Class,
                >(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn cast_mixin<M>(&self) -> M
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::into_mixin::<M::Class>(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            #[track_caller]
            pub unsafe fn cast_mixin_unchecked<M>(
                &self,
                instance: ::classes::vtable::MixinInstanceType,
            ) -> M
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                unsafe {
                    ::classes::ptr::RcDyn::into_mixin_unchecked::<
                        M::Class,
                    >(self.clone()._into_inner(), instance)
                }
                    .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast_ty(&self, ty: ::classes::vtable::Type) -> Option<&Self> {
                ::classes::ptr::RcDyn::try_downcast_ty(self._as_inner(), ty)
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn downcast_ty(&self, ty: ::classes::vtable::Type) -> &Self {
                ::classes::ptr::RcDyn::downcast_ty(self._as_inner(), ty).into()
            }
            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[track_caller]
            pub unsafe fn into_subclass_unchecked<D>(self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_subclass_unchecked::<
                        D::Class,
                    >(self._into_inner())
                }
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn into_subclass<D>(self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_subclass::<D::Class>(self._into_inner())
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn try_into_subclass<D>(self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_into_subclass::<D::Class>(self._into_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[track_caller]
            pub unsafe fn as_subclass_unchecked<D>(&self) -> &D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<
                        D::Class,
                    >(self._as_inner())
                }
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn as_subclass<D>(&self) -> &D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::as_subclass::<D::Class>(self._as_inner()).into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn try_as_subclass<D>(&self) -> Option<&D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner())
                    .map(Into::into)
            }
            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[track_caller]
            pub unsafe fn to_subclass_unchecked<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<
                        D::Class,
                    >(self._as_inner())
                }
                    .clone()
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn to_subclass<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::as_subclass::<D::Class>(self._as_inner())
                    .clone()
                    .into()
            }
            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            pub fn try_to_subclass<D>(&self) -> Option<D>
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                let _ = {
                    use ::classes::class::ClassVtable;
                    struct Assert<C: ClassVtable, A: ClassVtable>(
                        core::marker::PhantomData<(C, A)>,
                    );
                    impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                        const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("not a subclass"),
                                );
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner())
                    .cloned()
                    .map(Into::into)
            }
            #[inline]
            pub const fn ty(&self) -> ::classes::vtable::Type {
                self.0.vtable().ty()
            }
            #[inline]
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::Widget> {
                ::classes::ptr::RcDyn::as_ptr(this._as_inner())
            }
            #[inline]
            pub fn is_subtype_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subtype_of(C::TYPE)
            }
            #[inline]
            pub fn is_subclass_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subclass_of(C::TYPE)
            }
            #[inline]
            pub fn is_subtype_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subtype_of(ty)
            }
            #[inline]
            pub fn is_subclass_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subclass_of(ty)
            }
        }
        impl<V> self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
            #[inline]
            pub fn upgrade(
                &self,
            ) -> Option<self::Widget<::classes::ptr::RcDyn<self::Widget>, V>> {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner())
                    .map(self::Widget::_from_inner)
            }
            #[inline]
            pub fn strong_count(&self) -> usize {
                ::classes::ptr::WeakDyn::strong_count(self._as_inner())
            }
            #[inline]
            pub fn weak_count(&self) -> usize {
                ::classes::ptr::WeakDyn::weak_count(self._as_inner())
            }
            #[inline]
            pub const fn ty(&self) -> ::classes::vtable::Type {
                self.0.vtable().ty()
            }
            #[inline]
            pub fn is_subtype_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subtype_of(C::TYPE)
            }
            #[inline]
            pub fn is_subclass_of<C: ::classes::class::ClassVtable>(&self) -> bool {
                self.ty().is_subclass_of(C::TYPE)
            }
            #[inline]
            pub fn is_subtype_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subtype_of(ty)
            }
            #[inline]
            pub fn is_subclass_of_ty(&self, ty: ::classes::vtable::Type) -> bool {
                self.ty().is_subclass_of(ty)
            }
        }
        type Super = ::classes::object::Object;
        unsafe impl ::classes::class::HasSuper for self::Widget {
            type Super = Object;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)] match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            type Super = Object<::classes::ptr::RcDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref
        for self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            type Target = Object<::classes::ptr::RcDyn<Object>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
            type Super = Object<::classes::ptr::WeakDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::Widget {
            type SuperData = ::classes::prelude::CData<Object>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::Widget {
            type SuperVtable = ::classes::prelude::CVtable<Object>;
        }
        impl<V> self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            #[inline]
            pub fn as_super(&self) -> &Object<::classes::ptr::RcDyn<Object>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Object<::classes::ptr::RcDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::RcDyn::into_super(self.clone()._into_inner()),
                )
            }
            #[inline]
            pub fn into_super(self) -> Object<::classes::ptr::RcDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::RcDyn::into_super(self._into_inner()),
                )
            }
        }
        impl self::Widget<::classes::ptr::RcDyn<self::Widget>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Object<::classes::ptr::RcDyn<Object>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::Widget<::classes::ptr::WeakDyn<self::Widget>, V> {
            #[inline]
            pub fn as_super(&self) -> &Object<::classes::ptr::WeakDyn<Object>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::WeakDyn::into_super(self.clone()._into_inner()),
                )
            }
            #[inline]
            pub fn into_super(self) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                Object::_from_inner(
                    ::classes::ptr::WeakDyn::into_super(self._into_inner()),
                )
            }
        }
        impl vtable::Widget {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Object> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::Widget<::classes::ptr::RcDyn<self::Widget>, V>>
        for Object<::classes::ptr::RcDyn<Object>, V> {
            fn from(
                class: self::Widget<::classes::ptr::RcDyn<self::Widget>, V>,
            ) -> Object<::classes::ptr::RcDyn<Object>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Object<::classes::ptr::RcDyn<Object>, V>>
        for self::Widget<::classes::ptr::RcDyn<self::Widget>, V> {
            type Error = Object<::classes::ptr::RcDyn<Object>, V>;
            fn try_from(
                class: Object<::classes::ptr::RcDyn<Object>, V>,
            ) -> ::core::result::Result<
                self::Widget<::classes::ptr::RcDyn<self::Widget>, V>,
                Self::Error,
            > {
                class.try_as_subclass().cloned().ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::Widget<::classes::ptr::WeakDyn<self::Widget>, V>>
        for Object<::classes::ptr::WeakDyn<Object>, V> {
            fn from(
                class: self::Widget<::classes::ptr::WeakDyn<self::Widget>, V>,
            ) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                class.into_super()
            }
        }
        mod data {
            use super::*;
            use ::classes::get_set::{New, NewCopy};
            use ::classes::prelude::*;
            use ::classes::ptr::RcDyn;
            pub(super) type Super = ::classes::prelude::CData<super::Super>;
            #[repr(C)]
            pub struct Widget {
                pub(super) _super: Super,
            }
            impl Widget {
                #[cold]
                #[inline(never)]
                pub fn _delegate_ctor<
                    _S: ::classes::class::IsClass,
                    _F: FnOnce(
                            ::classes::prelude::CRcUninit<_S>,
                        ) -> ::classes::prelude::CRc<_S>,
                >(
                    mut _self: ::classes::prelude::CRcUninit<Self>,
                    new: _F,
                ) -> ::classes::prelude::CRc<Self>
                where
                    ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                    for<'a> &'a ::classes::prelude::CRc<
                        _S,
                    >: From<
                        &'a ::classes::ptr::RcDyn<
                            <::classes::prelude::CRc<
                                _S,
                            > as ::classes::class::IsClass>::Class,
                        >,
                    >,
                {
                    let _ = new;
                    {
                        ::core::panicking::panic_fmt(format_args!("unsupported"));
                    }
                }
            }
        }
        mod vtable {
            use super::*;
            use ::classes::class::{
                ClassVtable, ClassVtableBase, NonVirtual, Virtual, VtableHasImpl,
                VtableHasSuper,
            };
            use ::classes::prelude::*;
            use ::classes::vtable::{MixinVtableHeader, TypeInfo, VtableHeader};
            pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
            #[repr(C)]
            pub struct Widget {
                pub(super) _super: Super,
                pub create_element: fn(&::classes::prelude::CRc<Self>) -> CRc<Element>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Widget {
                #[inline]
                fn clone(&self) -> Widget {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>) -> CRc<Element>,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Widget {}
            impl Widget {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout {
                        this: self,
                        offset,
                    }
                }
            }
            pub struct DebugVtableLayout<'a> {
                this: &'a self::Widget,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("Widget");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self
                            .this
                            ._super
                            .debug_vtable_layout(
                                self.offset + { builtin # offset_of(Widget, _super) },
                            ),
                    );
                    dbg.field(
                        "create_element",
                        &(self.offset + { builtin # offset_of(Widget, create_element) }),
                    );
                    dbg.field(
                        "\'end",
                        &(self.offset + ::core::mem::size_of::<Widget>()),
                    );
                    dbg.finish()
                }
            }
            pub(super) mod opt {
                use super::*;
                use ::classes::class::{
                    ClassVtable, NonVirtual, Virtual, VtableHasImpl, VtableHasSuper,
                };
                use ::classes::prelude::*;
                use ::classes::vtable::{MixinVtableHeader, VtableHeaderOpt};
                pub(in super::super) type Super = ::classes::prelude::CVtableOpt<
                    super::super::Super,
                >;
                #[repr(C)]
                pub struct Widget {
                    pub(in super::super) _super: Super,
                    pub create_element: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>) -> CRc<Element>,
                    >,
                }
                #[automatically_derived]
                impl ::core::default::Default for Widget {
                    #[inline]
                    fn default() -> Widget {
                        Widget {
                            _super: ::core::default::Default::default(),
                            create_element: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Widget {
                    #[inline]
                    fn clone(&self) -> Widget {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<
                                fn(&::classes::prelude::CRc<Self>) -> CRc<Element>,
                            >,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Widget {}
                impl Widget {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
                        create_element: ::core::option::Option::None,
                    };
                    pub const fn init_mixin_header(
                        mixin_header: &mut [::core::mem::MaybeUninit<
                            ::classes::vtable::MixinVtableHeader,
                        >],
                    ) {
                        Super::init_mixin_header(mixin_header);
                    }
                    pub const fn init_header(
                        &mut self,
                        ty: ::core::option::Option<::classes::vtable::Type>,
                        offset: usize,
                    ) {
                        let ty = match ty {
                            ::core::option::Option::None => Self::TYPE,
                            ::core::option::Option::Some(ty) => ty,
                        };
                        self._super
                            .init_header(::core::option::Option::Some(ty), offset);
                    }
                    #[allow(unused_unsafe)]
                    pub const fn init<V: ::classes::class::ClassVtableOpt>(
                        _self: &mut V,
                    ) {
                        Super::init(_self);
                    }
                    #[track_caller]
                    pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                        ::classes::prelude::CVtable::<Self> {
                            _super: self._super.assert_init(),
                            create_element: self
                                .create_element
                                .expect(
                                    "cannot instantiate because method `Widget::create_element` is not implemented",
                                ),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<0usize> = ::classes::vtable::TypeInfo::new_abstract_class::<
                super::Widget,
            >(::core::option::Option::Some(Super::TYPE), [], MODULE_PATH, "Widget");
        }
        const _: () = {
            if !(::core::mem::size_of::<vtable::Widget>()
                == ::core::mem::size_of::<vtable::opt::Widget>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Widget != size of vtable :: opt :: Widget",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Widget, create_element) }
                == { builtin # offset_of(vtable::opt::Widget, create_element) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Widget::create_element != offset of vtable :: opt :: Widget::create_element",
                        ),
                    );
                }
            }
        };
        impl Widget<::classes::ptr::RcDyn<Widget>> {
            #[inline]
            pub fn create_element(&self) -> CRc<Element> {
                { (self.0.vtable().create_element)(self) }
            }
        }
        impl Widget<::classes::ptr::RcDyn<Widget>, ::classes::class::NonVirtual> {}
        impl Widget<::classes::ptr::RcDyn<Widget>> {}
    }
}
