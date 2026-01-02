#![feature(prelude_import)]
//! 示例1: 简单的类定义和继承
//!
//! 这是一个用rustdsl DSL语法编写的简单示例程序
//! 用于测试expand和rupta分析
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use classes::prelude::CRc;
use classes_macros::classes;
#[allow(unused_imports)]
use _classes::Animal;
#[allow(unused_imports)]
use _classes::Dog;
use ::classes::prelude::*;
const MODULE_PATH: &str = "example1";
#[allow(unused_macros)]
mod _classes {
    use super::*;
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _Animal::Animal;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Animal {
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
        pub struct Animal<
            T = ::classes::class::ClassMarker,
            V = ::classes::class::Virtual,
        >(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::Animal<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::Animal<T, V> {}
        impl<T, V> self::Animal<T, V> {
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
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::Animal>>
        for self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
            fn from(inner: ::classes::ptr::RcDyn<self::Animal>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<self::Animal<::classes::ptr::RcDyn<self::Animal>, V>>
        for ::classes::ptr::RcDyn<self::Animal> {
            fn from(this: self::Animal<::classes::ptr::RcDyn<self::Animal>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::Animal>>
        for self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
            fn from(inner: ::classes::ptr::WeakDyn<self::Animal>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<self::Animal<::classes::ptr::WeakDyn<self::Animal>, V>>
        for ::classes::ptr::WeakDyn<self::Animal> {
            fn from(
                this: self::Animal<::classes::ptr::WeakDyn<self::Animal>, V>,
            ) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::Animal<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::Animal<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak
        for self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
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
        for self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
            type Upgraded = Option<self::Animal<::classes::ptr::RcDyn<self::Animal>, V>>;
            type UpgradedOpt = self::Animal<::classes::ptr::RcDyn<self::Animal>, V>;
            type DowngradeFrom = self::Animal<::classes::ptr::RcDyn<self::Animal>, V>;
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
                self::Animal::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
        for self::Animal<::classes::ptr::RcDyn<self::Animal>, V>
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
        impl<V> ::core::cmp::Eq for self::Animal<::classes::ptr::RcDyn<self::Animal>, V>
        where
            for<'a> &'a Self: ::core::convert::From<
                &'a ::classes::ptr::RcDyn<self::Animal>,
            >,
        {}
        impl<V> ::core::hash::Hash
        for self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
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
        for self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq
        for self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {}
        impl<V> ::core::hash::Hash
        for self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(
                    &::classes::class::ClassRcWeak::as_ptr(self),
                    state,
                );
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug
        for self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
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
        for self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
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
        impl<T, V> ::classes::class::IsClass for self::Animal<T, V> {
            type Class = self::Animal;
        }
        impl ::classes::class::IsClass for data::Animal {
            type Class = self::Animal;
        }
        impl self::Animal {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::Animal {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::Animal {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize = <vtable::Animal as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::Animal {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::Animal {
            type Class = self::Animal;
        }
        impl ::classes::class::IsClass for vtable::opt::Animal {
            type Class = self::Animal;
        }
        impl ::classes::class::ClassDataBase for data::Animal {
            type Vtable = vtable::Animal;
        }
        impl ::classes::class::ClassVtableBase for vtable::Animal {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::Animal;
            type Opt = vtable::opt::Animal;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::Animal<T, V> {
            type DataBase = data::Animal;
            type Data = data::Animal;
            type VtableBase = vtable::Animal;
            type Vtable = vtable::Animal;
            type VtableOpt = vtable::opt::Animal;
        }
        impl ::classes::class::ClassData for data::Animal {}
        unsafe impl ::classes::class::ClassVtable for vtable::Animal {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::Animal {
            type VtableBase = vtable::Animal;
            type Vtable = vtable::Animal;
        }
        impl<V> ::classes::class::Class
        for self::Animal<::classes::class::ClassMarker, V> {
            type Rc = self::Animal<::classes::ptr::RcDyn<self::Animal>, V>;
            type Weak = self::Animal<::classes::ptr::WeakDyn<self::Animal>, V>;
            type Ptr = ::classes::ptr::PtrDyn<vtable::Animal>;
        }
        impl<V> self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
            pub fn downgrade(
                this: &Self,
            ) -> self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
                self::Animal::_from_inner(
                    ::classes::ptr::RcDyn::downgrade(this._as_inner()),
                )
            }
        }
        impl vtable::Animal {
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
        impl<V> self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::Animal<
                ::classes::ptr::RcDyn<self::Animal>,
                ::classes::class::Virtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::Animal<
                ::classes::ptr::RcDyn<self::Animal>,
                ::classes::class::NonVirtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc
        for self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {}
        impl<V> self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
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
                    Class: ::classes::class::HasImpl<self::Animal>,
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
                    Class: ::classes::class::HasImpl<self::Animal>,
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
                    Class: ::classes::class::HasImpl<self::Animal>,
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
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::Animal> {
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
        impl<V> self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
            #[inline]
            pub fn upgrade(
                &self,
            ) -> Option<self::Animal<::classes::ptr::RcDyn<self::Animal>, V>> {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner())
                    .map(self::Animal::_from_inner)
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
        unsafe impl ::classes::class::HasSuper for self::Animal {
            type Super = Object;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)] match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
            type Super = Object<::classes::ptr::RcDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref
        for self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
            type Target = Object<::classes::ptr::RcDyn<Object>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
            type Super = Object<::classes::ptr::WeakDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::Animal {
            type SuperData = ::classes::prelude::CData<Object>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::Animal {
            type SuperVtable = ::classes::prelude::CVtable<Object>;
        }
        impl<V> self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
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
        impl self::Animal<::classes::ptr::RcDyn<self::Animal>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Object<::classes::ptr::RcDyn<Object>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::Animal<::classes::ptr::WeakDyn<self::Animal>, V> {
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
        impl vtable::Animal {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Object> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::Animal<::classes::ptr::RcDyn<self::Animal>, V>>
        for Object<::classes::ptr::RcDyn<Object>, V> {
            fn from(
                class: self::Animal<::classes::ptr::RcDyn<self::Animal>, V>,
            ) -> Object<::classes::ptr::RcDyn<Object>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Object<::classes::ptr::RcDyn<Object>, V>>
        for self::Animal<::classes::ptr::RcDyn<self::Animal>, V> {
            type Error = Object<::classes::ptr::RcDyn<Object>, V>;
            fn try_from(
                class: Object<::classes::ptr::RcDyn<Object>, V>,
            ) -> ::core::result::Result<
                self::Animal<::classes::ptr::RcDyn<self::Animal>, V>,
                Self::Error,
            > {
                class.try_as_subclass().cloned().ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::Animal<::classes::ptr::WeakDyn<self::Animal>, V>>
        for Object<::classes::ptr::WeakDyn<Object>, V> {
            fn from(
                class: self::Animal<::classes::ptr::WeakDyn<self::Animal>, V>,
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
            pub struct Animal {
                pub(super) _super: Super,
                pub(super) name: String,
            }
            impl Animal {
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
                    name: String,
                ) -> ::classes::prelude::CRc<Self> {
                    unsafe {
                        ::core::ptr::write(&raw mut (*_self.as_mut_ptr()).name, name);
                        let _ = |Self { _super, name: _ }: Self| ();
                        ::classes::prelude::CData::<
                            ::classes::object::Object,
                        >::new(_self.into_super())
                            .into_subclass_unchecked()
                    }
                }
                pub(super) fn speak(_self: &::classes::prelude::CRc<Self>) {
                    {
                        ::std::io::_print(
                            format_args!("Animal {0} makes a sound\n", _self.get_name()),
                        );
                    };
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
            pub struct Animal {
                pub(super) _super: Super,
                pub speak: fn(&::classes::prelude::CRc<Self>),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Animal {
                #[inline]
                fn clone(&self) -> Animal {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>),
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Animal {}
            impl Animal {
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
                this: &'a self::Animal,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("Animal");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self
                            .this
                            ._super
                            .debug_vtable_layout(
                                self.offset + { builtin # offset_of(Animal, _super) },
                            ),
                    );
                    dbg.field(
                        "speak",
                        &(self.offset + { builtin # offset_of(Animal, speak) }),
                    );
                    dbg.field(
                        "\'end",
                        &(self.offset + ::core::mem::size_of::<Animal>()),
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
                pub struct Animal {
                    pub(in super::super) _super: Super,
                    pub speak: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>),
                    >,
                }
                #[automatically_derived]
                impl ::core::default::Default for Animal {
                    #[inline]
                    fn default() -> Animal {
                        Animal {
                            _super: ::core::default::Default::default(),
                            speak: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Animal {
                    #[inline]
                    fn clone(&self) -> Animal {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<fn(&::classes::prelude::CRc<Self>)>,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Animal {}
                impl Animal {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
                        speak: ::core::option::Option::None,
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
                        {
                            let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self);
                            ptr.speak = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<
                                    Self,
                                >::speak(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset) {
                                ptr.speak = ::core::option::Option::Some(|this| {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::speak(
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
                            speak: self
                                .speak
                                .expect(
                                    "cannot instantiate because method `Animal::speak` is not implemented",
                                ),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<0usize> = ::classes::vtable::TypeInfo::new_concrete_class::<
                super::Animal,
            >(::core::option::Option::Some(Super::TYPE), [], MODULE_PATH, "Animal");
        }
        const _: () = {
            if !(::core::mem::size_of::<vtable::Animal>()
                == ::core::mem::size_of::<vtable::opt::Animal>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Animal != size of vtable :: opt :: Animal",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Animal, speak) }
                == { builtin # offset_of(vtable::opt::Animal, speak) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Animal::speak != offset of vtable :: opt :: Animal::speak",
                        ),
                    );
                }
            }
        };
        static VTABLE: ::classes::vtable::VtableWithMixinHeader<
            vtable::Animal,
            { vtable::Animal::MIXIN_HEADER_ENTRIES },
        > = {
            let mut vtable = ::classes::vtable::MaybeUninitVtableWithMixinHeader::new(
                vtable::opt::Animal::DEFAULT,
            );
            vtable::opt::Animal::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(::core::option::Option::None, 0);
            vtable::opt::Animal::init(vtable_opt);
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            ::classes::vtable::VtableWithMixinHeader::new(
                headers,
                vtable_opt.assert_init(),
            )
        };
        unsafe impl ::classes::class::ConcreteClass for self::Animal {
            const VTABLE: ::core::ptr::NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }
        impl self::Animal {
            pub const fn vtable<'a>() -> &'a ::classes::vtable::VtableWithMixinHeader<
                vtable::Animal,
                { vtable::Animal::MIXIN_HEADER_ENTRIES },
            > {
                &VTABLE
            }
        }
        impl Animal<::classes::ptr::RcDyn<Animal>> {
            #[inline]
            pub fn new(name: String) -> Self {
                ::classes::prelude::CData::<
                    Self,
                >::new(::classes::prelude::CRcUninit::<Self>::new_uninit(), name)
            }
            #[inline]
            pub fn speak(&self) {
                { (self.0.vtable().speak)(self) }
            }
        }
        impl Animal<::classes::ptr::RcDyn<Animal>, ::classes::class::NonVirtual> {
            #[inline]
            pub fn speak(&self) {
                { ::classes::prelude::CData::<Self>::speak(self.as_virtual()) }
            }
        }
        impl Animal<::classes::ptr::RcDyn<Animal>> {
            #[inline]
            pub(in super::super) fn get_name(&self) -> &String {
                &self.0.name
            }
            #[inline]
            pub(in super::super) fn raw_get_name(&self) -> &String {
                &self.0.name
            }
        }
    }
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _Dog::Dog;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Dog {
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
        pub struct Dog<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual>(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::Dog<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::Dog<T, V> {}
        impl<T, V> self::Dog<T, V> {
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
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::Dog>>
        for self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
            fn from(inner: ::classes::ptr::RcDyn<self::Dog>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::Dog<::classes::ptr::RcDyn<self::Dog>, V>>
        for ::classes::ptr::RcDyn<self::Dog> {
            fn from(this: self::Dog<::classes::ptr::RcDyn<self::Dog>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::Dog>>
        for self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
            fn from(inner: ::classes::ptr::WeakDyn<self::Dog>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::Dog<::classes::ptr::WeakDyn<self::Dog>, V>>
        for ::classes::ptr::WeakDyn<self::Dog> {
            fn from(this: self::Dog<::classes::ptr::WeakDyn<self::Dog>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::Dog<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::Dog<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak
        for self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
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
        for self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
            type Upgraded = Option<self::Dog<::classes::ptr::RcDyn<self::Dog>, V>>;
            type UpgradedOpt = self::Dog<::classes::ptr::RcDyn<self::Dog>, V>;
            type DowngradeFrom = self::Dog<::classes::ptr::RcDyn<self::Dog>, V>;
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
                self::Dog::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
        for self::Dog<::classes::ptr::RcDyn<self::Dog>, V>
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
        impl<V> ::core::cmp::Eq for self::Dog<::classes::ptr::RcDyn<self::Dog>, V>
        where
            for<'a> &'a Self: ::core::convert::From<
                &'a ::classes::ptr::RcDyn<self::Dog>,
            >,
        {}
        impl<V> ::core::hash::Hash for self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
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
        for self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq for self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {}
        impl<V> ::core::hash::Hash for self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(
                    &::classes::class::ClassRcWeak::as_ptr(self),
                    state,
                );
            }
        }
        impl<V> ::core::fmt::Pointer for self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug for self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
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
        impl<V> ::core::fmt::Debug for self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
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
        impl<T, V> ::classes::class::IsClass for self::Dog<T, V> {
            type Class = self::Dog;
        }
        impl ::classes::class::IsClass for data::Dog {
            type Class = self::Dog;
        }
        impl self::Dog {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::Dog {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::Dog {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize = <vtable::Dog as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::Dog {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::Dog {
            type Class = self::Dog;
        }
        impl ::classes::class::IsClass for vtable::opt::Dog {
            type Class = self::Dog;
        }
        impl ::classes::class::ClassDataBase for data::Dog {
            type Vtable = vtable::Dog;
        }
        impl ::classes::class::ClassVtableBase for vtable::Dog {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::Dog;
            type Opt = vtable::opt::Dog;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::Dog<T, V> {
            type DataBase = data::Dog;
            type Data = data::Dog;
            type VtableBase = vtable::Dog;
            type Vtable = vtable::Dog;
            type VtableOpt = vtable::opt::Dog;
        }
        impl ::classes::class::ClassData for data::Dog {}
        unsafe impl ::classes::class::ClassVtable for vtable::Dog {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::Dog {
            type VtableBase = vtable::Dog;
            type Vtable = vtable::Dog;
        }
        impl<V> ::classes::class::Class for self::Dog<::classes::class::ClassMarker, V> {
            type Rc = self::Dog<::classes::ptr::RcDyn<self::Dog>, V>;
            type Weak = self::Dog<::classes::ptr::WeakDyn<self::Dog>, V>;
            type Ptr = ::classes::ptr::PtrDyn<vtable::Dog>;
        }
        impl<V> self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
            pub fn downgrade(
                this: &Self,
            ) -> self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
                self::Dog::_from_inner(
                    ::classes::ptr::RcDyn::downgrade(this._as_inner()),
                )
            }
        }
        impl vtable::Dog {
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
        impl<V> self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::Dog<
                ::classes::ptr::RcDyn<self::Dog>,
                ::classes::class::Virtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::Dog<
                ::classes::ptr::RcDyn<self::Dog>,
                ::classes::class::NonVirtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc
        for self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {}
        impl<V> self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
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
                    Class: ::classes::class::HasImpl<self::Dog>,
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
                    Class: ::classes::class::HasImpl<self::Dog>,
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
                    Class: ::classes::class::HasImpl<self::Dog>,
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
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::Dog> {
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
        impl<V> self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
            #[inline]
            pub fn upgrade(
                &self,
            ) -> Option<self::Dog<::classes::ptr::RcDyn<self::Dog>, V>> {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner())
                    .map(self::Dog::_from_inner)
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
        type Super = Animal;
        unsafe impl ::classes::class::HasSuper for self::Dog {
            type Super = Animal;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)] match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
            type Super = Animal<::classes::ptr::RcDyn<Animal>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref for self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
            type Target = Animal<::classes::ptr::RcDyn<Animal>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
            type Super = Animal<::classes::ptr::WeakDyn<Animal>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::Dog {
            type SuperData = ::classes::prelude::CData<Animal>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::Dog {
            type SuperVtable = ::classes::prelude::CVtable<Animal>;
        }
        impl<V> self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
            #[inline]
            pub fn as_super(&self) -> &Animal<::classes::ptr::RcDyn<Animal>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Animal<::classes::ptr::RcDyn<Animal>, V> {
                Animal::_from_inner(
                    ::classes::ptr::RcDyn::into_super(self.clone()._into_inner()),
                )
            }
            #[inline]
            pub fn into_super(self) -> Animal<::classes::ptr::RcDyn<Animal>, V> {
                Animal::_from_inner(
                    ::classes::ptr::RcDyn::into_super(self._into_inner()),
                )
            }
        }
        impl self::Dog<::classes::ptr::RcDyn<self::Dog>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Animal<::classes::ptr::RcDyn<Animal>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::Dog<::classes::ptr::WeakDyn<self::Dog>, V> {
            #[inline]
            pub fn as_super(&self) -> &Animal<::classes::ptr::WeakDyn<Animal>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Animal<::classes::ptr::WeakDyn<Animal>, V> {
                Animal::_from_inner(
                    ::classes::ptr::WeakDyn::into_super(self.clone()._into_inner()),
                )
            }
            #[inline]
            pub fn into_super(self) -> Animal<::classes::ptr::WeakDyn<Animal>, V> {
                Animal::_from_inner(
                    ::classes::ptr::WeakDyn::into_super(self._into_inner()),
                )
            }
        }
        impl vtable::Dog {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Animal> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::Dog<::classes::ptr::RcDyn<self::Dog>, V>>
        for Animal<::classes::ptr::RcDyn<Animal>, V> {
            fn from(
                class: self::Dog<::classes::ptr::RcDyn<self::Dog>, V>,
            ) -> Animal<::classes::ptr::RcDyn<Animal>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Animal<::classes::ptr::RcDyn<Animal>, V>>
        for self::Dog<::classes::ptr::RcDyn<self::Dog>, V> {
            type Error = Animal<::classes::ptr::RcDyn<Animal>, V>;
            fn try_from(
                class: Animal<::classes::ptr::RcDyn<Animal>, V>,
            ) -> ::core::result::Result<
                self::Dog<::classes::ptr::RcDyn<self::Dog>, V>,
                Self::Error,
            > {
                class.try_as_subclass().cloned().ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::Dog<::classes::ptr::WeakDyn<self::Dog>, V>>
        for Animal<::classes::ptr::WeakDyn<Animal>, V> {
            fn from(
                class: self::Dog<::classes::ptr::WeakDyn<self::Dog>, V>,
            ) -> Animal<::classes::ptr::WeakDyn<Animal>, V> {
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
            pub struct Dog {
                pub(super) _super: Super,
                pub(super) breed: String,
            }
            impl Dog {
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
                    name: String,
                    breed: String,
                ) -> ::classes::prelude::CRc<Self> {
                    unsafe {
                        ::core::ptr::write(&raw mut (*_self.as_mut_ptr()).breed, breed);
                        let _ = |Self { _super, breed: _ }: Self| ();
                        Super::new(_self.into_super(), name).into_subclass_unchecked()
                    }
                }
                pub(super) fn speak(_self: &::classes::prelude::CRc<Self>) {
                    {
                        ::std::io::_print(
                            format_args!("Dog {0} barks!\n", _self.get_name()),
                        );
                    };
                }
                pub(super) fn fetch(_self: &::classes::prelude::CRc<Self>) {
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0} the {1} is fetching!\n",
                                _self.get_name(),
                                _self.get_breed(),
                            ),
                        );
                    };
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
            pub struct Dog {
                pub(super) _super: Super,
                pub fetch: fn(&::classes::prelude::CRc<Self>),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Dog {
                #[inline]
                fn clone(&self) -> Dog {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>),
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Dog {}
            impl Dog {
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
                this: &'a self::Dog,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("Dog");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self
                            .this
                            ._super
                            .debug_vtable_layout(
                                self.offset + { builtin # offset_of(Dog, _super) },
                            ),
                    );
                    dbg.field(
                        "fetch",
                        &(self.offset + { builtin # offset_of(Dog, fetch) }),
                    );
                    dbg.field("\'end", &(self.offset + ::core::mem::size_of::<Dog>()));
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
                pub struct Dog {
                    pub(in super::super) _super: Super,
                    pub fetch: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>),
                    >,
                }
                #[automatically_derived]
                impl ::core::default::Default for Dog {
                    #[inline]
                    fn default() -> Dog {
                        Dog {
                            _super: ::core::default::Default::default(),
                            fetch: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Dog {
                    #[inline]
                    fn clone(&self) -> Dog {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<fn(&::classes::prelude::CRc<Self>)>,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Dog {}
                impl Dog {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
                        fetch: ::core::option::Option::None,
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
                        {
                            let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<
                                _,
                                ::classes::prelude::CVtableOpt<Super>,
                            >(_self);
                            ptr.speak = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<
                                    Self,
                                >::speak(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Super>,
                            >(_self, &mut offset) {
                                ptr.speak = ::core::option::Option::Some(|this| {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::speak(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
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
                            ptr.fetch = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<
                                    Self,
                                >::fetch(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset) {
                                ptr.fetch = ::core::option::Option::Some(|this| {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::fetch(
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
                            fetch: self
                                .fetch
                                .expect(
                                    "cannot instantiate because method `Dog::fetch` is not implemented",
                                ),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<0usize> = ::classes::vtable::TypeInfo::new_concrete_class::<
                super::Dog,
            >(::core::option::Option::Some(Super::TYPE), [], MODULE_PATH, "Dog");
        }
        const _: () = {
            if !(::core::mem::size_of::<vtable::Dog>()
                == ::core::mem::size_of::<vtable::opt::Dog>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Dog != size of vtable :: opt :: Dog",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Dog, fetch) }
                == { builtin # offset_of(vtable::opt::Dog, fetch) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Dog::fetch != offset of vtable :: opt :: Dog::fetch",
                        ),
                    );
                }
            }
        };
        static VTABLE: ::classes::vtable::VtableWithMixinHeader<
            vtable::Dog,
            { vtable::Dog::MIXIN_HEADER_ENTRIES },
        > = {
            let mut vtable = ::classes::vtable::MaybeUninitVtableWithMixinHeader::new(
                vtable::opt::Dog::DEFAULT,
            );
            vtable::opt::Dog::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(::core::option::Option::None, 0);
            vtable::opt::Dog::init(vtable_opt);
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            ::classes::vtable::VtableWithMixinHeader::new(
                headers,
                vtable_opt.assert_init(),
            )
        };
        unsafe impl ::classes::class::ConcreteClass for self::Dog {
            const VTABLE: ::core::ptr::NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }
        impl self::Dog {
            pub const fn vtable<'a>() -> &'a ::classes::vtable::VtableWithMixinHeader<
                vtable::Dog,
                { vtable::Dog::MIXIN_HEADER_ENTRIES },
            > {
                &VTABLE
            }
        }
        impl Dog<::classes::ptr::RcDyn<Dog>> {
            #[inline]
            pub fn new(name: String, breed: String) -> Self {
                ::classes::prelude::CData::<
                    Self,
                >::new(::classes::prelude::CRcUninit::<Self>::new_uninit(), name, breed)
            }
            #[inline]
            pub fn speak(&self) {
                { self.as_super().speak() }.try_into().unwrap()
            }
            #[inline]
            pub fn fetch(&self) {
                { (self.0.vtable().fetch)(self) }
            }
        }
        impl Dog<::classes::ptr::RcDyn<Dog>, ::classes::class::NonVirtual> {
            #[inline]
            pub fn speak(&self) {
                { ::classes::prelude::CData::<Self>::speak(self.as_virtual()) }
            }
            #[inline]
            pub fn fetch(&self) {
                { ::classes::prelude::CData::<Self>::fetch(self.as_virtual()) }
            }
        }
        impl Dog<::classes::ptr::RcDyn<Dog>> {
            #[inline]
            pub(in super::super) fn get_breed(&self) -> &String {
                &self.0.breed
            }
            #[inline]
            pub(in super::super) fn raw_get_breed(&self) -> &String {
                &self.0.breed
            }
        }
    }
}
fn main() {
    let dog = Dog::new("Buddy".to_string(), "Golden Retriever".to_string());
    dog.speak();
    dog.fetch();
    let animal: CRc<Animal> = dog.into_superclass();
    animal.speak();
}
