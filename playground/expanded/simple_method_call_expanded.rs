#![feature(prelude_import)]
//! 包含普通方法调用的 DSL 程序，用于验证指针信息通过方法调用的传播
//!
//! 这个程序测试以下场景：
//! 1. 类实例的普通方法调用（非 getter/setter）
//! 2. 方法内部访问字段（通过 getter/setter）
//! 3. 方法返回类引用
//! 4. 方法参数传递类引用
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use classes_macros::classes;
use classes::prelude::CRc;
#[allow(unused_imports)]
use _classes::Point;
#[allow(unused_imports)]
use _classes::Container;
use ::classes::prelude::*;
const MODULE_PATH: &str = "simple_method_call";
#[allow(unused_macros)]
mod _classes {
    use super::*;
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _Point::Point;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Point {
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
        pub struct Point<
            T = ::classes::class::ClassMarker,
            V = ::classes::class::Virtual,
        >(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::Point<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::Point<T, V> {}
        impl<T, V> self::Point<T, V> {
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
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::Point>>
        for self::Point<::classes::ptr::RcDyn<self::Point>, V> {
            fn from(inner: ::classes::ptr::RcDyn<self::Point>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::Point<::classes::ptr::RcDyn<self::Point>, V>>
        for ::classes::ptr::RcDyn<self::Point> {
            fn from(this: self::Point<::classes::ptr::RcDyn<self::Point>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::Point>>
        for self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
            fn from(inner: ::classes::ptr::WeakDyn<self::Point>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<self::Point<::classes::ptr::WeakDyn<self::Point>, V>>
        for ::classes::ptr::WeakDyn<self::Point> {
            fn from(this: self::Point<::classes::ptr::WeakDyn<self::Point>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::Point<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::Point<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak
        for self::Point<::classes::ptr::RcDyn<self::Point>, V> {
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
        for self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
            type Upgraded = Option<self::Point<::classes::ptr::RcDyn<self::Point>, V>>;
            type UpgradedOpt = self::Point<::classes::ptr::RcDyn<self::Point>, V>;
            type DowngradeFrom = self::Point<::classes::ptr::RcDyn<self::Point>, V>;
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
                self::Point::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
        for self::Point<::classes::ptr::RcDyn<self::Point>, V>
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
        impl<V> ::core::cmp::Eq for self::Point<::classes::ptr::RcDyn<self::Point>, V>
        where
            for<'a> &'a Self: ::core::convert::From<
                &'a ::classes::ptr::RcDyn<self::Point>,
            >,
        {}
        impl<V> ::core::hash::Hash
        for self::Point<::classes::ptr::RcDyn<self::Point>, V> {
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
        for self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq
        for self::Point<::classes::ptr::WeakDyn<self::Point>, V> {}
        impl<V> ::core::hash::Hash
        for self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(
                    &::classes::class::ClassRcWeak::as_ptr(self),
                    state,
                );
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Point<::classes::ptr::RcDyn<self::Point>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug
        for self::Point<::classes::ptr::RcDyn<self::Point>, V> {
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
        for self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
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
        impl<T, V> ::classes::class::IsClass for self::Point<T, V> {
            type Class = self::Point;
        }
        impl ::classes::class::IsClass for data::Point {
            type Class = self::Point;
        }
        impl self::Point {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::Point {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::Point {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize = <vtable::Point as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::Point {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::Point {
            type Class = self::Point;
        }
        impl ::classes::class::IsClass for vtable::opt::Point {
            type Class = self::Point;
        }
        impl ::classes::class::ClassDataBase for data::Point {
            type Vtable = vtable::Point;
        }
        impl ::classes::class::ClassVtableBase for vtable::Point {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::Point;
            type Opt = vtable::opt::Point;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::Point<T, V> {
            type DataBase = data::Point;
            type Data = data::Point;
            type VtableBase = vtable::Point;
            type Vtable = vtable::Point;
            type VtableOpt = vtable::opt::Point;
        }
        impl ::classes::class::ClassData for data::Point {}
        unsafe impl ::classes::class::ClassVtable for vtable::Point {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::Point {
            type VtableBase = vtable::Point;
            type Vtable = vtable::Point;
        }
        impl<V> ::classes::class::Class
        for self::Point<::classes::class::ClassMarker, V> {
            type Rc = self::Point<::classes::ptr::RcDyn<self::Point>, V>;
            type Weak = self::Point<::classes::ptr::WeakDyn<self::Point>, V>;
            type Ptr = ::classes::ptr::PtrDyn<vtable::Point>;
        }
        impl<V> self::Point<::classes::ptr::RcDyn<self::Point>, V> {
            pub fn downgrade(
                this: &Self,
            ) -> self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
                self::Point::_from_inner(
                    ::classes::ptr::RcDyn::downgrade(this._as_inner()),
                )
            }
        }
        impl vtable::Point {
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
        impl<V> self::Point<::classes::ptr::RcDyn<self::Point>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::Point<
                ::classes::ptr::RcDyn<self::Point>,
                ::classes::class::Virtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::Point<
                ::classes::ptr::RcDyn<self::Point>,
                ::classes::class::NonVirtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc
        for self::Point<::classes::ptr::RcDyn<self::Point>, V> {}
        impl<V> self::Point<::classes::ptr::RcDyn<self::Point>, V> {
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
                    Class: ::classes::class::HasImpl<self::Point>,
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
                    Class: ::classes::class::HasImpl<self::Point>,
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
                    Class: ::classes::class::HasImpl<self::Point>,
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
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::Point> {
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
        impl<V> self::Point<::classes::ptr::RcDyn<self::Point>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
            #[inline]
            pub fn upgrade(
                &self,
            ) -> Option<self::Point<::classes::ptr::RcDyn<self::Point>, V>> {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner())
                    .map(self::Point::_from_inner)
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
        unsafe impl ::classes::class::HasSuper for self::Point {
            type Super = Object;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)] match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Point<::classes::ptr::RcDyn<self::Point>, V> {
            type Super = Object<::classes::ptr::RcDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref
        for self::Point<::classes::ptr::RcDyn<self::Point>, V> {
            type Target = Object<::classes::ptr::RcDyn<Object>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
            type Super = Object<::classes::ptr::WeakDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::Point {
            type SuperData = ::classes::prelude::CData<Object>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::Point {
            type SuperVtable = ::classes::prelude::CVtable<Object>;
        }
        impl<V> self::Point<::classes::ptr::RcDyn<self::Point>, V> {
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
        impl self::Point<::classes::ptr::RcDyn<self::Point>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Object<::classes::ptr::RcDyn<Object>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::Point<::classes::ptr::WeakDyn<self::Point>, V> {
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
        impl vtable::Point {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Object> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::Point<::classes::ptr::RcDyn<self::Point>, V>>
        for Object<::classes::ptr::RcDyn<Object>, V> {
            fn from(
                class: self::Point<::classes::ptr::RcDyn<self::Point>, V>,
            ) -> Object<::classes::ptr::RcDyn<Object>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Object<::classes::ptr::RcDyn<Object>, V>>
        for self::Point<::classes::ptr::RcDyn<self::Point>, V> {
            type Error = Object<::classes::ptr::RcDyn<Object>, V>;
            fn try_from(
                class: Object<::classes::ptr::RcDyn<Object>, V>,
            ) -> ::core::result::Result<
                self::Point<::classes::ptr::RcDyn<self::Point>, V>,
                Self::Error,
            > {
                class.try_as_subclass().cloned().ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::Point<::classes::ptr::WeakDyn<self::Point>, V>>
        for Object<::classes::ptr::WeakDyn<Object>, V> {
            fn from(
                class: self::Point<::classes::ptr::WeakDyn<self::Point>, V>,
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
            pub struct Point {
                pub(super) _super: Super,
                pub(super) x: ::core::cell::Cell<i32>,
                pub(super) y: ::core::cell::Cell<i32>,
            }
            impl Point {
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
                    x: i32,
                    y: i32,
                ) -> ::classes::prelude::CRc<Self> {
                    unsafe {
                        ::core::ptr::write(
                            &raw mut (*_self.as_mut_ptr()).x,
                            ::classes::get_set::NewCopy::new_cell(x),
                        );
                        ::core::ptr::write(
                            &raw mut (*_self.as_mut_ptr()).y,
                            ::classes::get_set::NewCopy::new_cell(y),
                        );
                        let _ = |Self { _super, x: _, y: _ }: Self| ();
                        ::classes::prelude::CData::<
                            ::classes::object::Object,
                        >::new(_self.into_super())
                            .into_subclass_unchecked()
                    }
                }
                pub(super) fn sum_coords(_self: &::classes::prelude::CRc<Self>) -> i32 {
                    _self.get_x() + _self.get_y()
                }
                pub(super) fn distance_squared(
                    _self: &::classes::prelude::CRc<Self>,
                    other: &CRc<Point>,
                ) -> i32 {
                    let dx = _self.get_x() - other.get_x();
                    let dy = _self.get_y() - other.get_y();
                    dx * dx + dy * dy
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
            pub struct Point {
                pub(super) _super: Super,
                pub sum_coords: fn(&::classes::prelude::CRc<Self>) -> i32,
                pub distance_squared: fn(
                    &::classes::prelude::CRc<Self>,
                    other: &CRc<Point>,
                ) -> i32,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Point {
                #[inline]
                fn clone(&self) -> Point {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>) -> i32,
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>, other: &CRc<Point>) -> i32,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Point {}
            impl Point {
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
                this: &'a self::Point,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("Point");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self
                            .this
                            ._super
                            .debug_vtable_layout(
                                self.offset + { builtin # offset_of(Point, _super) },
                            ),
                    );
                    dbg.field(
                        "sum_coords",
                        &(self.offset + { builtin # offset_of(Point, sum_coords) }),
                    );
                    dbg.field(
                        "distance_squared",
                        &(self.offset + { builtin # offset_of(Point, distance_squared) }),
                    );
                    dbg.field("\'end", &(self.offset + ::core::mem::size_of::<Point>()));
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
                pub struct Point {
                    pub(in super::super) _super: Super,
                    pub sum_coords: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>) -> i32,
                    >,
                    pub distance_squared: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>, other: &CRc<Point>) -> i32,
                    >,
                }
                #[automatically_derived]
                impl ::core::default::Default for Point {
                    #[inline]
                    fn default() -> Point {
                        Point {
                            _super: ::core::default::Default::default(),
                            sum_coords: ::core::default::Default::default(),
                            distance_squared: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Point {
                    #[inline]
                    fn clone(&self) -> Point {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<
                                fn(&::classes::prelude::CRc<Self>) -> i32,
                            >,
                        >;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<
                                fn(
                                    &::classes::prelude::CRc<Self>,
                                    other: &CRc<Point>,
                                ) -> i32,
                            >,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Point {}
                impl Point {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
                        sum_coords: ::core::option::Option::None,
                        distance_squared: ::core::option::Option::None,
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
                            ptr.sum_coords = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<
                                    Self,
                                >::sum_coords(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset) {
                                ptr.sum_coords = ::core::option::Option::Some(|this| {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::sum_coords(
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
                            ptr.distance_squared = ::core::option::Option::Some(|
                                this,
                                other|
                            {
                                ::classes::prelude::CData::<
                                    Self,
                                >::distance_squared(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        other,
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset) {
                                ptr.distance_squared = ::core::option::Option::Some(|
                                    this,
                                    other|
                                {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::distance_squared(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                            other,
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
                            sum_coords: self
                                .sum_coords
                                .expect(
                                    "cannot instantiate because method `Point::sum_coords` is not implemented",
                                ),
                            distance_squared: self
                                .distance_squared
                                .expect(
                                    "cannot instantiate because method `Point::distance_squared` is not implemented",
                                ),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<0usize> = ::classes::vtable::TypeInfo::new_concrete_class::<
                super::Point,
            >(::core::option::Option::Some(Super::TYPE), [], MODULE_PATH, "Point");
        }
        const _: () = {
            if !(::core::mem::size_of::<vtable::Point>()
                == ::core::mem::size_of::<vtable::opt::Point>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Point != size of vtable :: opt :: Point",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Point, sum_coords) }
                == { builtin # offset_of(vtable::opt::Point, sum_coords) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Point::sum_coords != offset of vtable :: opt :: Point::sum_coords",
                        ),
                    );
                }
            }
            if !(::core::mem::size_of::<vtable::Point>()
                == ::core::mem::size_of::<vtable::opt::Point>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Point != size of vtable :: opt :: Point",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Point, distance_squared) }
                == { builtin # offset_of(vtable::opt::Point, distance_squared) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Point::distance_squared != offset of vtable :: opt :: Point::distance_squared",
                        ),
                    );
                }
            }
        };
        static VTABLE: ::classes::vtable::VtableWithMixinHeader<
            vtable::Point,
            { vtable::Point::MIXIN_HEADER_ENTRIES },
        > = {
            let mut vtable = ::classes::vtable::MaybeUninitVtableWithMixinHeader::new(
                vtable::opt::Point::DEFAULT,
            );
            vtable::opt::Point::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(::core::option::Option::None, 0);
            vtable::opt::Point::init(vtable_opt);
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            ::classes::vtable::VtableWithMixinHeader::new(
                headers,
                vtable_opt.assert_init(),
            )
        };
        unsafe impl ::classes::class::ConcreteClass for self::Point {
            const VTABLE: ::core::ptr::NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }
        impl self::Point {
            pub const fn vtable<'a>() -> &'a ::classes::vtable::VtableWithMixinHeader<
                vtable::Point,
                { vtable::Point::MIXIN_HEADER_ENTRIES },
            > {
                &VTABLE
            }
        }
        impl Point<::classes::ptr::RcDyn<Point>> {
            #[inline]
            pub fn new(x: i32, y: i32) -> Self {
                ::classes::prelude::CData::<
                    Self,
                >::new(::classes::prelude::CRcUninit::<Self>::new_uninit(), x, y)
            }
            #[inline]
            pub fn sum_coords(&self) -> i32 {
                { (self.0.vtable().sum_coords)(self) }
            }
            #[inline]
            pub fn distance_squared(&self, other: &CRc<Point>) -> i32 {
                { (self.0.vtable().distance_squared)(self, other) }
            }
        }
        impl Point<::classes::ptr::RcDyn<Point>, ::classes::class::NonVirtual> {
            #[inline]
            pub fn sum_coords(&self) -> i32 {
                { ::classes::prelude::CData::<Self>::sum_coords(self.as_virtual()) }
            }
            #[inline]
            pub fn distance_squared(&self, other: &CRc<Point>) -> i32 {
                {
                    ::classes::prelude::CData::<
                        Self,
                    >::distance_squared(self.as_virtual(), other)
                }
            }
        }
        impl Point<::classes::ptr::RcDyn<Point>> {
            #[inline]
            pub(in super::super) fn get_x(&self) -> i32 {
                ::classes::get_set::GetSetCopy::cell_get(&self.0.x)
            }
            #[inline]
            pub(in super::super) fn set_x<_T: ::core::convert::Into<i32>>(&self, x: _T) {
                ::classes::get_set::GetSetCopy::cell_set(&self.0.x, x.into());
            }
            #[inline]
            #[must_use]
            pub(in super::super) fn replace_x<_T: ::core::convert::Into<i32>>(
                &self,
                x: _T,
            ) -> i32 {
                let old = self.get_x();
                self.set_x(x);
                old
            }
            #[inline]
            pub(in super::super) fn update_x_with<
                _T: ::core::convert::Into<i32>,
                _F: ::core::ops::FnOnce(i32) -> _T,
            >(&self, f: _F) {
                self.set_x(f(self.get_x()));
            }
            #[inline]
            pub(in super::super) fn raw_get_x(&self) -> &::core::cell::Cell<i32> {
                &self.0.x
            }
            #[inline]
            pub(in super::super) fn get_y(&self) -> i32 {
                ::classes::get_set::GetSetCopy::cell_get(&self.0.y)
            }
            #[inline]
            pub(in super::super) fn set_y<_T: ::core::convert::Into<i32>>(&self, y: _T) {
                ::classes::get_set::GetSetCopy::cell_set(&self.0.y, y.into());
            }
            #[inline]
            #[must_use]
            pub(in super::super) fn replace_y<_T: ::core::convert::Into<i32>>(
                &self,
                y: _T,
            ) -> i32 {
                let old = self.get_y();
                self.set_y(y);
                old
            }
            #[inline]
            pub(in super::super) fn update_y_with<
                _T: ::core::convert::Into<i32>,
                _F: ::core::ops::FnOnce(i32) -> _T,
            >(&self, f: _F) {
                self.set_y(f(self.get_y()));
            }
            #[inline]
            pub(in super::super) fn raw_get_y(&self) -> &::core::cell::Cell<i32> {
                &self.0.y
            }
        }
    }
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _Container::Container;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Container {
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
        pub struct Container<
            T = ::classes::class::ClassMarker,
            V = ::classes::class::Virtual,
        >(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::Container<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::Container<T, V> {}
        impl<T, V> self::Container<T, V> {
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
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::Container>>
        for self::Container<::classes::ptr::RcDyn<self::Container>, V> {
            fn from(inner: ::classes::ptr::RcDyn<self::Container>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<
            self::Container<::classes::ptr::RcDyn<self::Container>, V>,
        > for ::classes::ptr::RcDyn<self::Container> {
            fn from(
                this: self::Container<::classes::ptr::RcDyn<self::Container>, V>,
            ) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::Container>>
        for self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
            fn from(inner: ::classes::ptr::WeakDyn<self::Container>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<
            V,
        > ::core::convert::From<
            self::Container<::classes::ptr::WeakDyn<self::Container>, V>,
        > for ::classes::ptr::WeakDyn<self::Container> {
            fn from(
                this: self::Container<::classes::ptr::WeakDyn<self::Container>, V>,
            ) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::Container<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::Container<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak
        for self::Container<::classes::ptr::RcDyn<self::Container>, V> {
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
        for self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
            type Upgraded = Option<
                self::Container<::classes::ptr::RcDyn<self::Container>, V>,
            >;
            type UpgradedOpt = self::Container<
                ::classes::ptr::RcDyn<self::Container>,
                V,
            >;
            type DowngradeFrom = self::Container<
                ::classes::ptr::RcDyn<self::Container>,
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
                self::Container::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
        for self::Container<::classes::ptr::RcDyn<self::Container>, V>
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
        for self::Container<::classes::ptr::RcDyn<self::Container>, V>
        where
            for<'a> &'a Self: ::core::convert::From<
                &'a ::classes::ptr::RcDyn<self::Container>,
            >,
        {}
        impl<V> ::core::hash::Hash
        for self::Container<::classes::ptr::RcDyn<self::Container>, V> {
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
        for self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq
        for self::Container<::classes::ptr::WeakDyn<self::Container>, V> {}
        impl<V> ::core::hash::Hash
        for self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(
                    &::classes::class::ClassRcWeak::as_ptr(self),
                    state,
                );
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Container<::classes::ptr::RcDyn<self::Container>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer
        for self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug
        for self::Container<::classes::ptr::RcDyn<self::Container>, V> {
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
        for self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
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
        impl<T, V> ::classes::class::IsClass for self::Container<T, V> {
            type Class = self::Container;
        }
        impl ::classes::class::IsClass for data::Container {
            type Class = self::Container;
        }
        impl self::Container {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::Container {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::Container {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize = <vtable::Container as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::Container {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::Container {
            type Class = self::Container;
        }
        impl ::classes::class::IsClass for vtable::opt::Container {
            type Class = self::Container;
        }
        impl ::classes::class::ClassDataBase for data::Container {
            type Vtable = vtable::Container;
        }
        impl ::classes::class::ClassVtableBase for vtable::Container {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::Container;
            type Opt = vtable::opt::Container;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::Container<T, V> {
            type DataBase = data::Container;
            type Data = data::Container;
            type VtableBase = vtable::Container;
            type Vtable = vtable::Container;
            type VtableOpt = vtable::opt::Container;
        }
        impl ::classes::class::ClassData for data::Container {}
        unsafe impl ::classes::class::ClassVtable for vtable::Container {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::Container {
            type VtableBase = vtable::Container;
            type Vtable = vtable::Container;
        }
        impl<V> ::classes::class::Class
        for self::Container<::classes::class::ClassMarker, V> {
            type Rc = self::Container<::classes::ptr::RcDyn<self::Container>, V>;
            type Weak = self::Container<::classes::ptr::WeakDyn<self::Container>, V>;
            type Ptr = ::classes::ptr::PtrDyn<vtable::Container>;
        }
        impl<V> self::Container<::classes::ptr::RcDyn<self::Container>, V> {
            pub fn downgrade(
                this: &Self,
            ) -> self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
                self::Container::_from_inner(
                    ::classes::ptr::RcDyn::downgrade(this._as_inner()),
                )
            }
        }
        impl vtable::Container {
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
        impl<V> self::Container<::classes::ptr::RcDyn<self::Container>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::Container<
                ::classes::ptr::RcDyn<self::Container>,
                ::classes::class::Virtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::Container<
                ::classes::ptr::RcDyn<self::Container>,
                ::classes::class::NonVirtual,
            > {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc
        for self::Container<::classes::ptr::RcDyn<self::Container>, V> {}
        impl<V> self::Container<::classes::ptr::RcDyn<self::Container>, V> {
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
                    Class: ::classes::class::HasImpl<self::Container>,
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
                    Class: ::classes::class::HasImpl<self::Container>,
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
                    Class: ::classes::class::HasImpl<self::Container>,
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
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::Container> {
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
        impl<V> self::Container<::classes::ptr::RcDyn<self::Container>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
            #[inline]
            pub fn upgrade(
                &self,
            ) -> Option<self::Container<::classes::ptr::RcDyn<self::Container>, V>> {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner())
                    .map(self::Container::_from_inner)
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
        unsafe impl ::classes::class::HasSuper for self::Container {
            type Super = Object;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)] match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Container<::classes::ptr::RcDyn<self::Container>, V> {
            type Super = Object<::classes::ptr::RcDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref
        for self::Container<::classes::ptr::RcDyn<self::Container>, V> {
            type Target = Object<::classes::ptr::RcDyn<Object>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
        for self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
            type Super = Object<::classes::ptr::WeakDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::Container {
            type SuperData = ::classes::prelude::CData<Object>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::Container {
            type SuperVtable = ::classes::prelude::CVtable<Object>;
        }
        impl<V> self::Container<::classes::ptr::RcDyn<self::Container>, V> {
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
        impl self::Container<::classes::ptr::RcDyn<self::Container>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Object<::classes::ptr::RcDyn<Object>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::Container<::classes::ptr::WeakDyn<self::Container>, V> {
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
        impl vtable::Container {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Object> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::Container<::classes::ptr::RcDyn<self::Container>, V>>
        for Object<::classes::ptr::RcDyn<Object>, V> {
            fn from(
                class: self::Container<::classes::ptr::RcDyn<self::Container>, V>,
            ) -> Object<::classes::ptr::RcDyn<Object>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Object<::classes::ptr::RcDyn<Object>, V>>
        for self::Container<::classes::ptr::RcDyn<self::Container>, V> {
            type Error = Object<::classes::ptr::RcDyn<Object>, V>;
            fn try_from(
                class: Object<::classes::ptr::RcDyn<Object>, V>,
            ) -> ::core::result::Result<
                self::Container<::classes::ptr::RcDyn<self::Container>, V>,
                Self::Error,
            > {
                class.try_as_subclass().cloned().ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::Container<::classes::ptr::WeakDyn<self::Container>, V>>
        for Object<::classes::ptr::WeakDyn<Object>, V> {
            fn from(
                class: self::Container<::classes::ptr::WeakDyn<self::Container>, V>,
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
            pub struct Container {
                pub(super) _super: Super,
                pub(super) point: ::core::cell::Cell<::core::option::Option<CRc<Point>>>,
            }
            impl Container {
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
                ) -> ::classes::prelude::CRc<Self> {
                    unsafe {
                        ::core::ptr::write(
                            &raw mut (*_self.as_mut_ptr()).point,
                            ::core::cell::Cell::new(::core::option::Option::None),
                        );
                        let _ = |Self { _super, point: _ }: Self| ();
                        ::classes::prelude::CData::<
                            ::classes::object::Object,
                        >::new(_self.into_super())
                            .into_subclass_unchecked()
                    }
                }
                pub(super) fn get_internal_point(
                    _self: &::classes::prelude::CRc<Self>,
                ) -> CRc<Point> {
                    _self.get_point()
                }
                pub(super) fn get_point_sum(
                    _self: &::classes::prelude::CRc<Self>,
                ) -> i32 {
                    let p = _self.get_point();
                    p.sum_coords()
                }
                pub(super) fn distance_to(
                    _self: &::classes::prelude::CRc<Self>,
                    other: &CRc<Point>,
                ) -> i32 {
                    let internal = _self.get_point();
                    internal.distance_squared(other)
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
            pub struct Container {
                pub(super) _super: Super,
                pub get_internal_point: fn(&::classes::prelude::CRc<Self>) -> CRc<Point>,
                pub get_point_sum: fn(&::classes::prelude::CRc<Self>) -> i32,
                pub distance_to: fn(
                    &::classes::prelude::CRc<Self>,
                    other: &CRc<Point>,
                ) -> i32,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Container {
                #[inline]
                fn clone(&self) -> Container {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>) -> CRc<Point>,
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>) -> i32,
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>, other: &CRc<Point>) -> i32,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Container {}
            impl Container {
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
                this: &'a self::Container,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("Container");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self
                            .this
                            ._super
                            .debug_vtable_layout(
                                self.offset + { builtin # offset_of(Container, _super) },
                            ),
                    );
                    dbg.field(
                        "get_internal_point",
                        &(self.offset
                            + { builtin # offset_of(Container, get_internal_point) }),
                    );
                    dbg.field(
                        "get_point_sum",
                        &(self.offset
                            + { builtin # offset_of(Container, get_point_sum) }),
                    );
                    dbg.field(
                        "distance_to",
                        &(self.offset + { builtin # offset_of(Container, distance_to) }),
                    );
                    dbg.field(
                        "\'end",
                        &(self.offset + ::core::mem::size_of::<Container>()),
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
                pub struct Container {
                    pub(in super::super) _super: Super,
                    pub get_internal_point: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>) -> CRc<Point>,
                    >,
                    pub get_point_sum: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>) -> i32,
                    >,
                    pub distance_to: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>, other: &CRc<Point>) -> i32,
                    >,
                }
                #[automatically_derived]
                impl ::core::default::Default for Container {
                    #[inline]
                    fn default() -> Container {
                        Container {
                            _super: ::core::default::Default::default(),
                            get_internal_point: ::core::default::Default::default(),
                            get_point_sum: ::core::default::Default::default(),
                            distance_to: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Container {
                    #[inline]
                    fn clone(&self) -> Container {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<
                                fn(&::classes::prelude::CRc<Self>) -> CRc<Point>,
                            >,
                        >;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<
                                fn(&::classes::prelude::CRc<Self>) -> i32,
                            >,
                        >;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<
                                fn(
                                    &::classes::prelude::CRc<Self>,
                                    other: &CRc<Point>,
                                ) -> i32,
                            >,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Container {}
                impl Container {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
                        get_internal_point: ::core::option::Option::None,
                        get_point_sum: ::core::option::Option::None,
                        distance_to: ::core::option::Option::None,
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
                            ptr.get_internal_point = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<
                                    Self,
                                >::get_internal_point(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset) {
                                ptr.get_internal_point = ::core::option::Option::Some(|
                                    this|
                                {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::get_internal_point(
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
                            ptr.get_point_sum = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<
                                    Self,
                                >::get_point_sum(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset) {
                                ptr.get_point_sum = ::core::option::Option::Some(|this| {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::get_point_sum(
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
                            ptr.distance_to = ::core::option::Option::Some(|this, other| {
                                ::classes::prelude::CData::<
                                    Self,
                                >::distance_to(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        other,
                                    )
                                    .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset) {
                                ptr.distance_to = ::core::option::Option::Some(|
                                    this,
                                    other|
                                {
                                    ::classes::prelude::CData::<
                                        Self,
                                    >::distance_to(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                            other,
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
                            get_internal_point: self
                                .get_internal_point
                                .expect(
                                    "cannot instantiate because method `Container::get_internal_point` is not implemented",
                                ),
                            get_point_sum: self
                                .get_point_sum
                                .expect(
                                    "cannot instantiate because method `Container::get_point_sum` is not implemented",
                                ),
                            distance_to: self
                                .distance_to
                                .expect(
                                    "cannot instantiate because method `Container::distance_to` is not implemented",
                                ),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<0usize> = ::classes::vtable::TypeInfo::new_concrete_class::<
                super::Container,
            >(::core::option::Option::Some(Super::TYPE), [], MODULE_PATH, "Container");
        }
        const _: () = {
            if !(::core::mem::size_of::<vtable::Container>()
                == ::core::mem::size_of::<vtable::opt::Container>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Container != size of vtable :: opt :: Container",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Container, get_internal_point) }
                == { builtin # offset_of(vtable::opt::Container, get_internal_point) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Container::get_internal_point != offset of vtable :: opt :: Container::get_internal_point",
                        ),
                    );
                }
            }
            if !(::core::mem::size_of::<vtable::Container>()
                == ::core::mem::size_of::<vtable::opt::Container>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Container != size of vtable :: opt :: Container",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Container, get_point_sum) }
                == { builtin # offset_of(vtable::opt::Container, get_point_sum) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Container::get_point_sum != offset of vtable :: opt :: Container::get_point_sum",
                        ),
                    );
                }
            }
            if !(::core::mem::size_of::<vtable::Container>()
                == ::core::mem::size_of::<vtable::opt::Container>())
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "size of vtable :: Container != size of vtable :: opt :: Container",
                        ),
                    );
                }
            }
            if !({ builtin # offset_of(vtable::Container, distance_to) }
                == { builtin # offset_of(vtable::opt::Container, distance_to) })
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "offset of vtable :: Container::distance_to != offset of vtable :: opt :: Container::distance_to",
                        ),
                    );
                }
            }
        };
        static VTABLE: ::classes::vtable::VtableWithMixinHeader<
            vtable::Container,
            { vtable::Container::MIXIN_HEADER_ENTRIES },
        > = {
            let mut vtable = ::classes::vtable::MaybeUninitVtableWithMixinHeader::new(
                vtable::opt::Container::DEFAULT,
            );
            vtable::opt::Container::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(::core::option::Option::None, 0);
            vtable::opt::Container::init(vtable_opt);
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            ::classes::vtable::VtableWithMixinHeader::new(
                headers,
                vtable_opt.assert_init(),
            )
        };
        unsafe impl ::classes::class::ConcreteClass for self::Container {
            const VTABLE: ::core::ptr::NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }
        impl self::Container {
            pub const fn vtable<'a>() -> &'a ::classes::vtable::VtableWithMixinHeader<
                vtable::Container,
                { vtable::Container::MIXIN_HEADER_ENTRIES },
            > {
                &VTABLE
            }
        }
        impl Container<::classes::ptr::RcDyn<Container>> {
            #[inline]
            pub fn new() -> Self {
                ::classes::prelude::CData::<
                    Self,
                >::new(::classes::prelude::CRcUninit::<Self>::new_uninit())
            }
            #[inline]
            pub fn get_internal_point(&self) -> CRc<Point> {
                { (self.0.vtable().get_internal_point)(self) }
            }
            #[inline]
            pub fn get_point_sum(&self) -> i32 {
                { (self.0.vtable().get_point_sum)(self) }
            }
            #[inline]
            pub fn distance_to(&self, other: &CRc<Point>) -> i32 {
                { (self.0.vtable().distance_to)(self, other) }
            }
        }
        impl Container<::classes::ptr::RcDyn<Container>, ::classes::class::NonVirtual> {
            #[inline]
            pub fn get_internal_point(&self) -> CRc<Point> {
                {
                    ::classes::prelude::CData::<
                        Self,
                    >::get_internal_point(self.as_virtual())
                }
            }
            #[inline]
            pub fn get_point_sum(&self) -> i32 {
                { ::classes::prelude::CData::<Self>::get_point_sum(self.as_virtual()) }
            }
            #[inline]
            pub fn distance_to(&self, other: &CRc<Point>) -> i32 {
                {
                    ::classes::prelude::CData::<
                        Self,
                    >::distance_to(self.as_virtual(), other)
                }
            }
        }
        impl Container<::classes::ptr::RcDyn<Container>> {
            #[inline]
            pub(in super::super) fn get_point(
                &self,
            ) -> <CRc<Point> as ::classes::get_set::GetSet>::OptionGet {
                ::classes::get_set::GetSet::cell_option_get(&self.0.point)
            }
            #[inline]
            pub(in super::super) fn set_point(
                &self,
                point: <CRc<Point> as ::classes::get_set::GetSet>::Set,
            ) {
                ::classes::get_set::GetSet::cell_option_set(&self.0.point, point);
            }
            #[inline]
            #[must_use]
            pub(in super::super) fn replace_point(
                &self,
                point: <CRc<Point> as ::classes::get_set::GetSet>::Set,
            ) -> <::core::option::Option<
                CRc<Point>,
            > as ::classes::get_set::GetSet>::Get {
                let old = ::classes::get_set::GetSet::try_cell_option_get(&self.0.point);
                ::classes::get_set::GetSet::cell_option_set(&self.0.point, point);
                old
            }
            #[inline]
            pub(in super::super) fn update_point_with<
                _F: ::core::ops::FnOnce(
                        <::core::option::Option<
                            CRc<Point>,
                        > as ::classes::get_set::GetSet>::Get,
                    ) -> <CRc<Point> as ::classes::get_set::GetSet>::Set,
            >(&self, f: _F) {
                ::classes::get_set::GetSet::cell_option_set(
                    &self.0.point,
                    f(::classes::get_set::GetSet::try_cell_option_get(&self.0.point)),
                );
            }
            #[inline]
            pub(in super::super) fn raw_get_point(
                &self,
            ) -> &::core::cell::Cell<::core::option::Option<CRc<Point>>> {
                &self.0.point
            }
        }
    }
}
fn main() {
    let _p1 = Point::new(10, 20);
    let _p2 = Point::new(30, 40);
    let _sum1 = _p1.sum_coords();
    let _dist = _p1.distance_squared(&_p2);
    let mut _c1 = Container::new();
    let mut _c2 = Container::new();
    _c1.set_point(_p1);
    let _p3 = _c1.get_internal_point();
    let _sum2 = _c1.get_point_sum();
    let _dist2 = _c1.distance_to(&_p2);
    _c2.set_point(_p2);
    let _ = _sum1;
    let _ = _dist;
    std::mem::drop(_p3);
    let _ = _sum2;
    let _ = _dist2;
    std::mem::drop(_c1);
    std::mem::drop(_c2);
}
