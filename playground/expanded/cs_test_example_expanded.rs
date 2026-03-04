#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use ::classes::prelude::*;
#[allow(unused_imports)]
use _classes::Cross;
#[allow(unused_imports)]
use _classes::Number;
#[allow(unused_imports)]
use _classes::One;
#[allow(unused_imports)]
use _classes::Two;
use classes::prelude::CRc;
use classes_macros::classes;
const MODULE_PATH: &str = "cs_test_example";
#[allow(unused_macros)]
mod _classes {
    use super::*;
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _Number::Number;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Number {
        use super::*;
        use ::classes::class::{ConcreteClass, NonVirtual, Virtual};
        use ::classes::get_set::{GetSet, GetSetCopy};
        use ::classes::prelude::*;
        use ::classes::ptr::RcDyn;
        use ::classes::vtable::{
            MaybeUninitVtableWithMixinHeader, VtableHeader, VtableWithMixinHeader,
        };
        use ::core::ptr::NonNull;
        #[repr(transparent)]
        pub struct Number<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual>(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::Number<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::Number<T, V> {}
        impl<T, V> self::Number<T, V> {
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
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::Number>>
            for self::Number<::classes::ptr::RcDyn<self::Number>, V>
        {
            fn from(inner: ::classes::ptr::RcDyn<self::Number>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::Number<::classes::ptr::RcDyn<self::Number>, V>>
            for ::classes::ptr::RcDyn<self::Number>
        {
            fn from(this: self::Number<::classes::ptr::RcDyn<self::Number>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::Number>>
            for self::Number<::classes::ptr::WeakDyn<self::Number>, V>
        {
            fn from(inner: ::classes::ptr::WeakDyn<self::Number>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::Number<::classes::ptr::WeakDyn<self::Number>, V>>
            for ::classes::ptr::WeakDyn<self::Number>
        {
            fn from(this: self::Number<::classes::ptr::WeakDyn<self::Number>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::Number<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::Number<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak for self::Number<::classes::ptr::RcDyn<self::Number>, V> {
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
        impl<V> ::classes::class::ClassRcWeak for self::Number<::classes::ptr::WeakDyn<self::Number>, V> {
            type Upgraded = Option<self::Number<::classes::ptr::RcDyn<self::Number>, V>>;
            type UpgradedOpt = self::Number<::classes::ptr::RcDyn<self::Number>, V>;
            type DowngradeFrom = self::Number<::classes::ptr::RcDyn<self::Number>, V>;
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
                self::Number::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
            for self::Number<::classes::ptr::RcDyn<self::Number>, V>
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
        impl<V> ::core::cmp::Eq for self::Number<::classes::ptr::RcDyn<self::Number>, V> where
            for<'a> &'a Self: ::core::convert::From<&'a ::classes::ptr::RcDyn<self::Number>>
        {
        }
        impl<V> ::core::hash::Hash for self::Number<::classes::ptr::RcDyn<self::Number>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                type CRcEqHash = ::classes::prelude::CRc<::classes::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    CRcEqHash::hash(&this, state);
                } else {
                    ::core::hash::Hash::hash(&::classes::class::ClassRcWeak::as_ptr(self), state);
                }
            }
        }
        impl<V, C: ::classes::class::ClassRcWeak> ::core::cmp::PartialEq<C>
            for self::Number<::classes::ptr::WeakDyn<self::Number>, V>
        {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq for self::Number<::classes::ptr::WeakDyn<self::Number>, V> {}
        impl<V> ::core::hash::Hash for self::Number<::classes::ptr::WeakDyn<self::Number>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(&::classes::class::ClassRcWeak::as_ptr(self), state);
            }
        }
        impl<V> ::core::fmt::Pointer for self::Number<::classes::ptr::RcDyn<self::Number>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer for self::Number<::classes::ptr::WeakDyn<self::Number>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug for self::Number<::classes::ptr::RcDyn<self::Number>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                type CRcFormat = ::classes::prelude::CRc<::classes::fmt::Format>;
                if let Some(this) = self.try_to_supertype::<CRcFormat>() {
                    CRcFormat::fmt_debug(&this, f)
                } else {
                    ::core::fmt::Display::fmt(&::classes::class::ClassRcWeak::as_ptr(self), f)
                }
            }
        }
        impl<V> ::core::fmt::Debug for self::Number<::classes::ptr::WeakDyn<self::Number>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if let Some(this) = self.upgrade() {
                    ::core::fmt::Debug::fmt(&this, f)
                } else {
                    ::core::fmt::Display::fmt(&::classes::class::ClassRcWeak::as_ptr(self), f)
                }
            }
        }
        impl<T, V> ::classes::class::IsClass for self::Number<T, V> {
            type Class = self::Number;
        }
        impl ::classes::class::IsClass for data::Number {
            type Class = self::Number;
        }
        impl self::Number {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::Number {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::Number {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize =
                <vtable::Number as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::Number {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::Number {
            type Class = self::Number;
        }
        impl ::classes::class::IsClass for vtable::opt::Number {
            type Class = self::Number;
        }
        impl ::classes::class::ClassDataBase for data::Number {
            type Vtable = vtable::Number;
        }
        impl ::classes::class::ClassVtableBase for vtable::Number {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::Number;
            type Opt = vtable::opt::Number;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::Number<T, V> {
            type DataBase = data::Number;
            type Data = data::Number;
            type VtableBase = vtable::Number;
            type Vtable = vtable::Number;
            type VtableOpt = vtable::opt::Number;
        }
        impl ::classes::class::ClassData for data::Number {}
        unsafe impl ::classes::class::ClassVtable for vtable::Number {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::Number {
            type VtableBase = vtable::Number;
            type Vtable = vtable::Number;
        }
        impl<V> ::classes::class::Class for self::Number<::classes::class::ClassMarker, V> {
            type Rc = self::Number<::classes::ptr::RcDyn<self::Number>, V>;
            type Weak = self::Number<::classes::ptr::WeakDyn<self::Number>, V>;
            type Ptr = ::classes::ptr::PtrDyn<vtable::Number>;
        }
        impl<V> self::Number<::classes::ptr::RcDyn<self::Number>, V> {
            pub fn downgrade(
                this: &Self,
            ) -> self::Number<::classes::ptr::WeakDyn<self::Number>, V> {
                self::Number::_from_inner(::classes::ptr::RcDyn::downgrade(this._as_inner()))
            }
        }
        impl vtable::Number {
            #[inline]
            const fn cast_header(this: *const Self) -> *const ::classes::vtable::VtableHeader {
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
        impl<V> self::Number<::classes::ptr::RcDyn<self::Number>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::Number<::classes::ptr::RcDyn<self::Number>, ::classes::class::Virtual>
            {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::Number<::classes::ptr::RcDyn<self::Number>, ::classes::class::NonVirtual>
            {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc for self::Number<::classes::ptr::RcDyn<self::Number>, V> {}
        impl<V> self::Number<::classes::ptr::RcDyn<self::Number>, V> {
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<A::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<A::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<A::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<A::Class>(
                        self.clone()._into_inner(),
                    )
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_superclass_unchecked::<A::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                        {
                        } else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_supertype::<A::Class>(self._into_inner()).into()
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
                        {
                        } else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                ::classes::ptr::RcDyn::try_into_subtype::<D::Class>(self.clone()._into_inner())
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
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self._into_inner()).into()
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn to_subtype<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self.clone()._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::upcast::<A::Class, I::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::upcast_unchecked::<A::Class, I::Class>(
                        self.clone()._into_inner(),
                    )
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_upcast::<A::Class, I::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub unsafe fn downcast_unchecked<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::Number>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::downcast_unchecked::<B::Class, S::Class>(
                        self.clone()._into_inner(),
                    )
                }
                .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast<B, S>(&self) -> Option<S>
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::Number>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_downcast::<B::Class, S::Class>(
                    self.clone()._into_inner(),
                )
                .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn downcast<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::Number>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::downcast::<B::Class, S::Class>(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            pub fn try_cast_mixin<M>(&self) -> Option<M>
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_mixin::<M::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn cast_mixin<M>(&self) -> M
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::into_mixin::<M::Class>(self.clone()._into_inner()).into()
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
                    ::classes::ptr::RcDyn::into_mixin_unchecked::<M::Class>(
                        self.clone()._into_inner(),
                        instance,
                    )
                }
                .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast_ty(&self, ty: ::classes::vtable::Type) -> Option<&Self> {
                ::classes::ptr::RcDyn::try_downcast_ty(self._as_inner(), ty).map(Into::into)
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_subclass_unchecked::<D::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_subclass::<D::Class>(self._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<D::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner()).map(Into::into)
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<D::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::Number> {
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
        impl<V> self::Number<::classes::ptr::RcDyn<self::Number>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Number<::classes::ptr::WeakDyn<self::Number>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Number<::classes::ptr::WeakDyn<self::Number>, V> {
            #[inline]
            pub fn upgrade(&self) -> Option<self::Number<::classes::ptr::RcDyn<self::Number>, V>> {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner()).map(self::Number::_from_inner)
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
        unsafe impl ::classes::class::HasSuper for self::Number {
            type Super = Object;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)]
                match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper for self::Number<::classes::ptr::RcDyn<self::Number>, V> {
            type Super = Object<::classes::ptr::RcDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref for self::Number<::classes::ptr::RcDyn<self::Number>, V> {
            type Target = Object<::classes::ptr::RcDyn<Object>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper
            for self::Number<::classes::ptr::WeakDyn<self::Number>, V>
        {
            type Super = Object<::classes::ptr::WeakDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::Number {
            type SuperData = ::classes::prelude::CData<Object>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::Number {
            type SuperVtable = ::classes::prelude::CVtable<Object>;
        }
        impl<V> self::Number<::classes::ptr::RcDyn<self::Number>, V> {
            #[inline]
            pub fn as_super(&self) -> &Object<::classes::ptr::RcDyn<Object>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Object<::classes::ptr::RcDyn<Object>, V> {
                Object::_from_inner(::classes::ptr::RcDyn::into_super(
                    self.clone()._into_inner(),
                ))
            }
            #[inline]
            pub fn into_super(self) -> Object<::classes::ptr::RcDyn<Object>, V> {
                Object::_from_inner(::classes::ptr::RcDyn::into_super(self._into_inner()))
            }
        }
        impl self::Number<::classes::ptr::RcDyn<self::Number>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Object<::classes::ptr::RcDyn<Object>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::Number<::classes::ptr::WeakDyn<self::Number>, V> {
            #[inline]
            pub fn as_super(&self) -> &Object<::classes::ptr::WeakDyn<Object>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                Object::_from_inner(::classes::ptr::WeakDyn::into_super(
                    self.clone()._into_inner(),
                ))
            }
            #[inline]
            pub fn into_super(self) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                Object::_from_inner(::classes::ptr::WeakDyn::into_super(self._into_inner()))
            }
        }
        impl vtable::Number {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Object> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::Number<::classes::ptr::RcDyn<self::Number>, V>>
            for Object<::classes::ptr::RcDyn<Object>, V>
        {
            fn from(
                class: self::Number<::classes::ptr::RcDyn<self::Number>, V>,
            ) -> Object<::classes::ptr::RcDyn<Object>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Object<::classes::ptr::RcDyn<Object>, V>>
            for self::Number<::classes::ptr::RcDyn<self::Number>, V>
        {
            type Error = Object<::classes::ptr::RcDyn<Object>, V>;
            fn try_from(
                class: Object<::classes::ptr::RcDyn<Object>, V>,
            ) -> ::core::result::Result<
                self::Number<::classes::ptr::RcDyn<self::Number>, V>,
                Self::Error,
            > {
                class
                    .try_as_subclass()
                    .cloned()
                    .ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::Number<::classes::ptr::WeakDyn<self::Number>, V>>
            for Object<::classes::ptr::WeakDyn<Object>, V>
        {
            fn from(
                class: self::Number<::classes::ptr::WeakDyn<self::Number>, V>,
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
            pub struct Number {
                pub(super) _super: Super,
            }
            impl Number {
                #[cold]
                #[inline(never)]
                pub fn _delegate_ctor<
                    _S: ::classes::class::IsClass,
                    _F: FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
                >(
                    mut _self: ::classes::prelude::CRcUninit<Self>,
                    new: _F,
                ) -> ::classes::prelude::CRc<Self>
                where
                    ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                    for<'a> &'a ::classes::prelude::CRc<_S>: From<
                        &'a ::classes::ptr::RcDyn<
                            <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
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
                        let _ = |Self { _super }: Self| ();
                        ::classes::prelude::CData::<::classes::object::Object>::new(
                            _self.into_super(),
                        )
                        .into_subclass_unchecked()
                    }
                }
                pub(super) fn get(_self: &::classes::prelude::CRc<Self>) -> i32 {
                    return 0;
                }
            }
        }
        mod vtable {
            use super::*;
            use ::classes::class::{
                ClassVtable, ClassVtableBase, NonVirtual, Virtual, VtableHasImpl, VtableHasSuper,
            };
            use ::classes::prelude::*;
            use ::classes::vtable::{MixinVtableHeader, TypeInfo, VtableHeader};
            pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
            #[repr(C)]
            pub struct Number {
                pub(super) _super: Super,
                pub get: fn(&::classes::prelude::CRc<Self>) -> i32,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Number {
                #[inline]
                fn clone(&self) -> Number {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>) -> i32,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Number {}
            impl Number {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }
            pub struct DebugVtableLayout<'a> {
                this: &'a self::Number,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("Number");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self.this._super.debug_vtable_layout(
                            self.offset + {
                                builtin # offset_of(Number, _super)
                            },
                        ),
                    );
                    dbg.field(
                        "get",
                        &(self.offset + {
                            builtin # offset_of(Number, get)
                        }),
                    );
                    dbg.field("\'end", &(self.offset + ::core::mem::size_of::<Number>()));
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
                pub(in super::super) type Super =
                    ::classes::prelude::CVtableOpt<super::super::Super>;
                #[repr(C)]
                pub struct Number {
                    pub(in super::super) _super: Super,
                    pub get: ::core::option::Option<fn(&::classes::prelude::CRc<Self>) -> i32>,
                }
                #[automatically_derived]
                impl ::core::default::Default for Number {
                    #[inline]
                    fn default() -> Number {
                        Number {
                            _super: ::core::default::Default::default(),
                            get: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Number {
                    #[inline]
                    fn clone(&self) -> Number {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<fn(&::classes::prelude::CRc<Self>) -> i32>,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Number {}
                impl Number {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
                        get: ::core::option::Option::None,
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
                    pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                        Super::init(_self);
                        {
                            let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self);
                            ptr.get = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<Self>::get(&unsafe {
                                    this.try_to_subtype().unwrap_unchecked()
                                })
                                .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset)
                            {
                                ptr.get = ::core::option::Option::Some(|this| {
                                    ::classes::prelude::CData::<Self>::get(&unsafe {
                                        this.try_to_subtype().unwrap_unchecked()
                                    })
                                    .into()
                                });
                            }
                        }
                    }
                    #[track_caller]
                    pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                        ::classes::prelude::CVtable::<Self> {
                            _super: self._super.assert_init(),
                            get: self
                                .get
                                .expect(
                                    "cannot instantiate because method `Number::get` is not implemented",
                                ),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                ::classes::vtable::TypeInfo::new_concrete_class::<super::Number>(
                    ::core::option::Option::Some(Super::TYPE),
                    [],
                    MODULE_PATH,
                    "Number",
                );
        }
        const _: () = {
            if !(::core::mem::size_of::<vtable::Number>()
                == ::core::mem::size_of::<vtable::opt::Number>())
            {
                {
                    ::core::panicking::panic_fmt(format_args!(
                        "size of vtable :: Number != size of vtable :: opt :: Number",
                    ));
                }
            }
            if !({
                builtin # offset_of(vtable::Number, get)
            } == {
                builtin # offset_of(vtable::opt::Number, get)
            }) {
                {
                    ::core::panicking::panic_fmt(format_args!(
                        "offset of vtable :: Number::get != offset of vtable :: opt :: Number::get",
                    ));
                }
            }
        };
        static VTABLE: ::classes::vtable::VtableWithMixinHeader<
            vtable::Number,
            { vtable::Number::MIXIN_HEADER_ENTRIES },
        > = {
            let mut vtable = ::classes::vtable::MaybeUninitVtableWithMixinHeader::new(
                vtable::opt::Number::DEFAULT,
            );
            vtable::opt::Number::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(::core::option::Option::None, 0);
            vtable::opt::Number::init(vtable_opt);
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            ::classes::vtable::VtableWithMixinHeader::new(headers, vtable_opt.assert_init())
        };
        unsafe impl ::classes::class::ConcreteClass for self::Number {
            const VTABLE: ::core::ptr::NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }
        impl self::Number {
            pub const fn vtable<'a>() -> &'a ::classes::vtable::VtableWithMixinHeader<
                vtable::Number,
                { vtable::Number::MIXIN_HEADER_ENTRIES },
            > {
                &VTABLE
            }
        }
        impl Number<::classes::ptr::RcDyn<Number>> {
            #[inline]
            pub fn new() -> Self {
                ::classes::prelude::CData::<Self>::new(
                    ::classes::prelude::CRcUninit::<Self>::new_uninit(),
                )
            }
            #[inline]
            pub fn get(&self) -> i32 {
                {
                    (self.0.vtable().get)(self)
                }
            }
        }
        impl Number<::classes::ptr::RcDyn<Number>, ::classes::class::NonVirtual> {
            #[inline]
            pub fn get(&self) -> i32 {
                {
                    ::classes::prelude::CData::<Self>::get(self.as_virtual())
                }
            }
        }
        impl Number<::classes::ptr::RcDyn<Number>> {}
    }
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _One::One;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _One {
        use super::*;
        use ::classes::class::{ConcreteClass, NonVirtual, Virtual};
        use ::classes::get_set::{GetSet, GetSetCopy};
        use ::classes::prelude::*;
        use ::classes::ptr::RcDyn;
        use ::classes::vtable::{
            MaybeUninitVtableWithMixinHeader, VtableHeader, VtableWithMixinHeader,
        };
        use ::core::ptr::NonNull;
        #[repr(transparent)]
        pub struct One<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual>(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::One<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::One<T, V> {}
        impl<T, V> self::One<T, V> {
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
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::One>>
            for self::One<::classes::ptr::RcDyn<self::One>, V>
        {
            fn from(inner: ::classes::ptr::RcDyn<self::One>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::One<::classes::ptr::RcDyn<self::One>, V>>
            for ::classes::ptr::RcDyn<self::One>
        {
            fn from(this: self::One<::classes::ptr::RcDyn<self::One>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::One>>
            for self::One<::classes::ptr::WeakDyn<self::One>, V>
        {
            fn from(inner: ::classes::ptr::WeakDyn<self::One>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::One<::classes::ptr::WeakDyn<self::One>, V>>
            for ::classes::ptr::WeakDyn<self::One>
        {
            fn from(this: self::One<::classes::ptr::WeakDyn<self::One>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::One<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::One<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak for self::One<::classes::ptr::RcDyn<self::One>, V> {
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
        impl<V> ::classes::class::ClassRcWeak for self::One<::classes::ptr::WeakDyn<self::One>, V> {
            type Upgraded = Option<self::One<::classes::ptr::RcDyn<self::One>, V>>;
            type UpgradedOpt = self::One<::classes::ptr::RcDyn<self::One>, V>;
            type DowngradeFrom = self::One<::classes::ptr::RcDyn<self::One>, V>;
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
                self::One::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
            for self::One<::classes::ptr::RcDyn<self::One>, V>
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
        impl<V> ::core::cmp::Eq for self::One<::classes::ptr::RcDyn<self::One>, V> where
            for<'a> &'a Self: ::core::convert::From<&'a ::classes::ptr::RcDyn<self::One>>
        {
        }
        impl<V> ::core::hash::Hash for self::One<::classes::ptr::RcDyn<self::One>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                type CRcEqHash = ::classes::prelude::CRc<::classes::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    CRcEqHash::hash(&this, state);
                } else {
                    ::core::hash::Hash::hash(&::classes::class::ClassRcWeak::as_ptr(self), state);
                }
            }
        }
        impl<V, C: ::classes::class::ClassRcWeak> ::core::cmp::PartialEq<C>
            for self::One<::classes::ptr::WeakDyn<self::One>, V>
        {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq for self::One<::classes::ptr::WeakDyn<self::One>, V> {}
        impl<V> ::core::hash::Hash for self::One<::classes::ptr::WeakDyn<self::One>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(&::classes::class::ClassRcWeak::as_ptr(self), state);
            }
        }
        impl<V> ::core::fmt::Pointer for self::One<::classes::ptr::RcDyn<self::One>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer for self::One<::classes::ptr::WeakDyn<self::One>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug for self::One<::classes::ptr::RcDyn<self::One>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                type CRcFormat = ::classes::prelude::CRc<::classes::fmt::Format>;
                if let Some(this) = self.try_to_supertype::<CRcFormat>() {
                    CRcFormat::fmt_debug(&this, f)
                } else {
                    ::core::fmt::Display::fmt(&::classes::class::ClassRcWeak::as_ptr(self), f)
                }
            }
        }
        impl<V> ::core::fmt::Debug for self::One<::classes::ptr::WeakDyn<self::One>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if let Some(this) = self.upgrade() {
                    ::core::fmt::Debug::fmt(&this, f)
                } else {
                    ::core::fmt::Display::fmt(&::classes::class::ClassRcWeak::as_ptr(self), f)
                }
            }
        }
        impl<T, V> ::classes::class::IsClass for self::One<T, V> {
            type Class = self::One;
        }
        impl ::classes::class::IsClass for data::One {
            type Class = self::One;
        }
        impl self::One {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::One {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::One {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize =
                <vtable::One as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::One {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::One {
            type Class = self::One;
        }
        impl ::classes::class::IsClass for vtable::opt::One {
            type Class = self::One;
        }
        impl ::classes::class::ClassDataBase for data::One {
            type Vtable = vtable::One;
        }
        impl ::classes::class::ClassVtableBase for vtable::One {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::One;
            type Opt = vtable::opt::One;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::One<T, V> {
            type DataBase = data::One;
            type Data = data::One;
            type VtableBase = vtable::One;
            type Vtable = vtable::One;
            type VtableOpt = vtable::opt::One;
        }
        impl ::classes::class::ClassData for data::One {}
        unsafe impl ::classes::class::ClassVtable for vtable::One {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::One {
            type VtableBase = vtable::One;
            type Vtable = vtable::One;
        }
        impl<V> ::classes::class::Class for self::One<::classes::class::ClassMarker, V> {
            type Rc = self::One<::classes::ptr::RcDyn<self::One>, V>;
            type Weak = self::One<::classes::ptr::WeakDyn<self::One>, V>;
            type Ptr = ::classes::ptr::PtrDyn<vtable::One>;
        }
        impl<V> self::One<::classes::ptr::RcDyn<self::One>, V> {
            pub fn downgrade(this: &Self) -> self::One<::classes::ptr::WeakDyn<self::One>, V> {
                self::One::_from_inner(::classes::ptr::RcDyn::downgrade(this._as_inner()))
            }
        }
        impl vtable::One {
            #[inline]
            const fn cast_header(this: *const Self) -> *const ::classes::vtable::VtableHeader {
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
        impl<V> self::One<::classes::ptr::RcDyn<self::One>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::One<::classes::ptr::RcDyn<self::One>, ::classes::class::Virtual>
            {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::One<::classes::ptr::RcDyn<self::One>, ::classes::class::NonVirtual>
            {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc for self::One<::classes::ptr::RcDyn<self::One>, V> {}
        impl<V> self::One<::classes::ptr::RcDyn<self::One>, V> {
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<A::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<A::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<A::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<A::Class>(
                        self.clone()._into_inner(),
                    )
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_superclass_unchecked::<A::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                        {
                        } else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_supertype::<A::Class>(self._into_inner()).into()
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
                        {
                        } else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                ::classes::ptr::RcDyn::try_into_subtype::<D::Class>(self.clone()._into_inner())
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
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self._into_inner()).into()
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn to_subtype<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self.clone()._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::upcast::<A::Class, I::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::upcast_unchecked::<A::Class, I::Class>(
                        self.clone()._into_inner(),
                    )
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_upcast::<A::Class, I::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub unsafe fn downcast_unchecked<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::One>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::downcast_unchecked::<B::Class, S::Class>(
                        self.clone()._into_inner(),
                    )
                }
                .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast<B, S>(&self) -> Option<S>
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::One>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_downcast::<B::Class, S::Class>(
                    self.clone()._into_inner(),
                )
                .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn downcast<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::One>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::downcast::<B::Class, S::Class>(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            pub fn try_cast_mixin<M>(&self) -> Option<M>
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_mixin::<M::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn cast_mixin<M>(&self) -> M
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::into_mixin::<M::Class>(self.clone()._into_inner()).into()
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
                    ::classes::ptr::RcDyn::into_mixin_unchecked::<M::Class>(
                        self.clone()._into_inner(),
                        instance,
                    )
                }
                .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast_ty(&self, ty: ::classes::vtable::Type) -> Option<&Self> {
                ::classes::ptr::RcDyn::try_downcast_ty(self._as_inner(), ty).map(Into::into)
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_subclass_unchecked::<D::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_subclass::<D::Class>(self._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<D::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner()).map(Into::into)
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<D::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::One> {
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
        impl<V> self::One<::classes::ptr::RcDyn<self::One>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::One<::classes::ptr::WeakDyn<self::One>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::One<::classes::ptr::WeakDyn<self::One>, V> {
            #[inline]
            pub fn upgrade(&self) -> Option<self::One<::classes::ptr::RcDyn<self::One>, V>> {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner()).map(self::One::_from_inner)
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
        type Super = Number;
        unsafe impl ::classes::class::HasSuper for self::One {
            type Super = Number;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)]
                match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper for self::One<::classes::ptr::RcDyn<self::One>, V> {
            type Super = Number<::classes::ptr::RcDyn<Number>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref for self::One<::classes::ptr::RcDyn<self::One>, V> {
            type Target = Number<::classes::ptr::RcDyn<Number>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper for self::One<::classes::ptr::WeakDyn<self::One>, V> {
            type Super = Number<::classes::ptr::WeakDyn<Number>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::One {
            type SuperData = ::classes::prelude::CData<Number>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::One {
            type SuperVtable = ::classes::prelude::CVtable<Number>;
        }
        impl<V> self::One<::classes::ptr::RcDyn<self::One>, V> {
            #[inline]
            pub fn as_super(&self) -> &Number<::classes::ptr::RcDyn<Number>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Number<::classes::ptr::RcDyn<Number>, V> {
                Number::_from_inner(::classes::ptr::RcDyn::into_super(
                    self.clone()._into_inner(),
                ))
            }
            #[inline]
            pub fn into_super(self) -> Number<::classes::ptr::RcDyn<Number>, V> {
                Number::_from_inner(::classes::ptr::RcDyn::into_super(self._into_inner()))
            }
        }
        impl self::One<::classes::ptr::RcDyn<self::One>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Number<::classes::ptr::RcDyn<Number>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::One<::classes::ptr::WeakDyn<self::One>, V> {
            #[inline]
            pub fn as_super(&self) -> &Number<::classes::ptr::WeakDyn<Number>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Number<::classes::ptr::WeakDyn<Number>, V> {
                Number::_from_inner(::classes::ptr::WeakDyn::into_super(
                    self.clone()._into_inner(),
                ))
            }
            #[inline]
            pub fn into_super(self) -> Number<::classes::ptr::WeakDyn<Number>, V> {
                Number::_from_inner(::classes::ptr::WeakDyn::into_super(self._into_inner()))
            }
        }
        impl vtable::One {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Number> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::One<::classes::ptr::RcDyn<self::One>, V>>
            for Number<::classes::ptr::RcDyn<Number>, V>
        {
            fn from(
                class: self::One<::classes::ptr::RcDyn<self::One>, V>,
            ) -> Number<::classes::ptr::RcDyn<Number>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Number<::classes::ptr::RcDyn<Number>, V>>
            for self::One<::classes::ptr::RcDyn<self::One>, V>
        {
            type Error = Number<::classes::ptr::RcDyn<Number>, V>;
            fn try_from(
                class: Number<::classes::ptr::RcDyn<Number>, V>,
            ) -> ::core::result::Result<self::One<::classes::ptr::RcDyn<self::One>, V>, Self::Error>
            {
                class
                    .try_as_subclass()
                    .cloned()
                    .ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::One<::classes::ptr::WeakDyn<self::One>, V>>
            for Number<::classes::ptr::WeakDyn<Number>, V>
        {
            fn from(
                class: self::One<::classes::ptr::WeakDyn<self::One>, V>,
            ) -> Number<::classes::ptr::WeakDyn<Number>, V> {
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
            pub struct One {
                pub(super) _super: Super,
            }
            impl One {
                #[cold]
                #[inline(never)]
                pub fn _delegate_ctor<
                    _S: ::classes::class::IsClass,
                    _F: FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
                >(
                    mut _self: ::classes::prelude::CRcUninit<Self>,
                    new: _F,
                ) -> ::classes::prelude::CRc<Self>
                where
                    ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                    for<'a> &'a ::classes::prelude::CRc<_S>: From<
                        &'a ::classes::ptr::RcDyn<
                            <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
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
                        let _ = |Self { _super }: Self| ();
                        Super::new(_self.into_super()).into_subclass_unchecked()
                    }
                }
                pub(super) fn get(_self: &::classes::prelude::CRc<Self>) -> i32 {
                    return 1;
                }
            }
        }
        mod vtable {
            use super::*;
            use ::classes::class::{
                ClassVtable, ClassVtableBase, NonVirtual, Virtual, VtableHasImpl, VtableHasSuper,
            };
            use ::classes::prelude::*;
            use ::classes::vtable::{MixinVtableHeader, TypeInfo, VtableHeader};
            pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
            #[repr(C)]
            pub struct One {
                pub(super) _super: Super,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for One {
                #[inline]
                fn clone(&self) -> One {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for One {}
            impl One {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }
            pub struct DebugVtableLayout<'a> {
                this: &'a self::One,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("One");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self.this._super.debug_vtable_layout(
                            self.offset + {
                                builtin # offset_of(One, _super)
                            },
                        ),
                    );
                    dbg.field("\'end", &(self.offset + ::core::mem::size_of::<One>()));
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
                pub(in super::super) type Super =
                    ::classes::prelude::CVtableOpt<super::super::Super>;
                #[repr(C)]
                pub struct One {
                    pub(in super::super) _super: Super,
                }
                #[automatically_derived]
                impl ::core::default::Default for One {
                    #[inline]
                    fn default() -> One {
                        One {
                            _super: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for One {
                    #[inline]
                    fn clone(&self) -> One {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for One {}
                impl One {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
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
                    pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                        Super::init(_self);
                        {
                            let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<
                                _,
                                ::classes::prelude::CVtableOpt<Super>,
                            >(_self);
                            ptr.get = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<Self>::get(&unsafe {
                                    this.try_to_subtype().unwrap_unchecked()
                                })
                                .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Super>,
                            >(_self, &mut offset)
                            {
                                ptr.get = ::core::option::Option::Some(|this| {
                                    ::classes::prelude::CData::<Self>::get(&unsafe {
                                        this.try_to_subtype().unwrap_unchecked()
                                    })
                                    .into()
                                });
                            }
                        }
                    }
                    #[track_caller]
                    pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                        ::classes::prelude::CVtable::<Self> {
                            _super: self._super.assert_init(),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                ::classes::vtable::TypeInfo::new_concrete_class::<super::One>(
                    ::core::option::Option::Some(Super::TYPE),
                    [],
                    MODULE_PATH,
                    "One",
                );
        }
        const _: () = {};
        static VTABLE: ::classes::vtable::VtableWithMixinHeader<
            vtable::One,
            { vtable::One::MIXIN_HEADER_ENTRIES },
        > = {
            let mut vtable =
                ::classes::vtable::MaybeUninitVtableWithMixinHeader::new(vtable::opt::One::DEFAULT);
            vtable::opt::One::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(::core::option::Option::None, 0);
            vtable::opt::One::init(vtable_opt);
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            ::classes::vtable::VtableWithMixinHeader::new(headers, vtable_opt.assert_init())
        };
        unsafe impl ::classes::class::ConcreteClass for self::One {
            const VTABLE: ::core::ptr::NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }
        impl self::One {
            pub const fn vtable<'a>() -> &'a ::classes::vtable::VtableWithMixinHeader<
                vtable::One,
                { vtable::One::MIXIN_HEADER_ENTRIES },
            > {
                &VTABLE
            }
        }
        impl One<::classes::ptr::RcDyn<One>> {
            #[inline]
            pub fn new() -> Self {
                ::classes::prelude::CData::<Self>::new(
                    ::classes::prelude::CRcUninit::<Self>::new_uninit(),
                )
            }
            #[inline]
            pub fn get(&self) -> i32 {
                { self.as_super().get() }.try_into().unwrap()
            }
        }
        impl One<::classes::ptr::RcDyn<One>, ::classes::class::NonVirtual> {
            #[inline]
            pub fn get(&self) -> i32 {
                {
                    ::classes::prelude::CData::<Self>::get(self.as_virtual())
                }
            }
        }
        impl One<::classes::ptr::RcDyn<One>> {}
    }
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _Two::Two;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Two {
        use super::*;
        use ::classes::class::{ConcreteClass, NonVirtual, Virtual};
        use ::classes::get_set::{GetSet, GetSetCopy};
        use ::classes::prelude::*;
        use ::classes::ptr::RcDyn;
        use ::classes::vtable::{
            MaybeUninitVtableWithMixinHeader, VtableHeader, VtableWithMixinHeader,
        };
        use ::core::ptr::NonNull;
        #[repr(transparent)]
        pub struct Two<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual>(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::Two<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::Two<T, V> {}
        impl<T, V> self::Two<T, V> {
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
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::Two>>
            for self::Two<::classes::ptr::RcDyn<self::Two>, V>
        {
            fn from(inner: ::classes::ptr::RcDyn<self::Two>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::Two<::classes::ptr::RcDyn<self::Two>, V>>
            for ::classes::ptr::RcDyn<self::Two>
        {
            fn from(this: self::Two<::classes::ptr::RcDyn<self::Two>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::Two>>
            for self::Two<::classes::ptr::WeakDyn<self::Two>, V>
        {
            fn from(inner: ::classes::ptr::WeakDyn<self::Two>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::Two<::classes::ptr::WeakDyn<self::Two>, V>>
            for ::classes::ptr::WeakDyn<self::Two>
        {
            fn from(this: self::Two<::classes::ptr::WeakDyn<self::Two>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::Two<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::Two<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak for self::Two<::classes::ptr::RcDyn<self::Two>, V> {
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
        impl<V> ::classes::class::ClassRcWeak for self::Two<::classes::ptr::WeakDyn<self::Two>, V> {
            type Upgraded = Option<self::Two<::classes::ptr::RcDyn<self::Two>, V>>;
            type UpgradedOpt = self::Two<::classes::ptr::RcDyn<self::Two>, V>;
            type DowngradeFrom = self::Two<::classes::ptr::RcDyn<self::Two>, V>;
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
                self::Two::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
            for self::Two<::classes::ptr::RcDyn<self::Two>, V>
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
        impl<V> ::core::cmp::Eq for self::Two<::classes::ptr::RcDyn<self::Two>, V> where
            for<'a> &'a Self: ::core::convert::From<&'a ::classes::ptr::RcDyn<self::Two>>
        {
        }
        impl<V> ::core::hash::Hash for self::Two<::classes::ptr::RcDyn<self::Two>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                type CRcEqHash = ::classes::prelude::CRc<::classes::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    CRcEqHash::hash(&this, state);
                } else {
                    ::core::hash::Hash::hash(&::classes::class::ClassRcWeak::as_ptr(self), state);
                }
            }
        }
        impl<V, C: ::classes::class::ClassRcWeak> ::core::cmp::PartialEq<C>
            for self::Two<::classes::ptr::WeakDyn<self::Two>, V>
        {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq for self::Two<::classes::ptr::WeakDyn<self::Two>, V> {}
        impl<V> ::core::hash::Hash for self::Two<::classes::ptr::WeakDyn<self::Two>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(&::classes::class::ClassRcWeak::as_ptr(self), state);
            }
        }
        impl<V> ::core::fmt::Pointer for self::Two<::classes::ptr::RcDyn<self::Two>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer for self::Two<::classes::ptr::WeakDyn<self::Two>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug for self::Two<::classes::ptr::RcDyn<self::Two>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                type CRcFormat = ::classes::prelude::CRc<::classes::fmt::Format>;
                if let Some(this) = self.try_to_supertype::<CRcFormat>() {
                    CRcFormat::fmt_debug(&this, f)
                } else {
                    ::core::fmt::Display::fmt(&::classes::class::ClassRcWeak::as_ptr(self), f)
                }
            }
        }
        impl<V> ::core::fmt::Debug for self::Two<::classes::ptr::WeakDyn<self::Two>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if let Some(this) = self.upgrade() {
                    ::core::fmt::Debug::fmt(&this, f)
                } else {
                    ::core::fmt::Display::fmt(&::classes::class::ClassRcWeak::as_ptr(self), f)
                }
            }
        }
        impl<T, V> ::classes::class::IsClass for self::Two<T, V> {
            type Class = self::Two;
        }
        impl ::classes::class::IsClass for data::Two {
            type Class = self::Two;
        }
        impl self::Two {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::Two {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::Two {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize =
                <vtable::Two as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::Two {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::Two {
            type Class = self::Two;
        }
        impl ::classes::class::IsClass for vtable::opt::Two {
            type Class = self::Two;
        }
        impl ::classes::class::ClassDataBase for data::Two {
            type Vtable = vtable::Two;
        }
        impl ::classes::class::ClassVtableBase for vtable::Two {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::Two;
            type Opt = vtable::opt::Two;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::Two<T, V> {
            type DataBase = data::Two;
            type Data = data::Two;
            type VtableBase = vtable::Two;
            type Vtable = vtable::Two;
            type VtableOpt = vtable::opt::Two;
        }
        impl ::classes::class::ClassData for data::Two {}
        unsafe impl ::classes::class::ClassVtable for vtable::Two {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::Two {
            type VtableBase = vtable::Two;
            type Vtable = vtable::Two;
        }
        impl<V> ::classes::class::Class for self::Two<::classes::class::ClassMarker, V> {
            type Rc = self::Two<::classes::ptr::RcDyn<self::Two>, V>;
            type Weak = self::Two<::classes::ptr::WeakDyn<self::Two>, V>;
            type Ptr = ::classes::ptr::PtrDyn<vtable::Two>;
        }
        impl<V> self::Two<::classes::ptr::RcDyn<self::Two>, V> {
            pub fn downgrade(this: &Self) -> self::Two<::classes::ptr::WeakDyn<self::Two>, V> {
                self::Two::_from_inner(::classes::ptr::RcDyn::downgrade(this._as_inner()))
            }
        }
        impl vtable::Two {
            #[inline]
            const fn cast_header(this: *const Self) -> *const ::classes::vtable::VtableHeader {
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
        impl<V> self::Two<::classes::ptr::RcDyn<self::Two>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::Two<::classes::ptr::RcDyn<self::Two>, ::classes::class::Virtual>
            {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::Two<::classes::ptr::RcDyn<self::Two>, ::classes::class::NonVirtual>
            {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc for self::Two<::classes::ptr::RcDyn<self::Two>, V> {}
        impl<V> self::Two<::classes::ptr::RcDyn<self::Two>, V> {
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<A::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<A::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<A::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<A::Class>(
                        self.clone()._into_inner(),
                    )
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_superclass_unchecked::<A::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                        {
                        } else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_supertype::<A::Class>(self._into_inner()).into()
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
                        {
                        } else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                ::classes::ptr::RcDyn::try_into_subtype::<D::Class>(self.clone()._into_inner())
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
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self._into_inner()).into()
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn to_subtype<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self.clone()._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::upcast::<A::Class, I::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::upcast_unchecked::<A::Class, I::Class>(
                        self.clone()._into_inner(),
                    )
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_upcast::<A::Class, I::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub unsafe fn downcast_unchecked<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::Two>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::downcast_unchecked::<B::Class, S::Class>(
                        self.clone()._into_inner(),
                    )
                }
                .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast<B, S>(&self) -> Option<S>
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::Two>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_downcast::<B::Class, S::Class>(
                    self.clone()._into_inner(),
                )
                .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn downcast<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::Two>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::downcast::<B::Class, S::Class>(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            pub fn try_cast_mixin<M>(&self) -> Option<M>
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_mixin::<M::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn cast_mixin<M>(&self) -> M
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::into_mixin::<M::Class>(self.clone()._into_inner()).into()
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
                    ::classes::ptr::RcDyn::into_mixin_unchecked::<M::Class>(
                        self.clone()._into_inner(),
                        instance,
                    )
                }
                .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast_ty(&self, ty: ::classes::vtable::Type) -> Option<&Self> {
                ::classes::ptr::RcDyn::try_downcast_ty(self._as_inner(), ty).map(Into::into)
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_subclass_unchecked::<D::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_subclass::<D::Class>(self._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<D::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner()).map(Into::into)
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<D::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::Two> {
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
        impl<V> self::Two<::classes::ptr::RcDyn<self::Two>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Two<::classes::ptr::WeakDyn<self::Two>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Two<::classes::ptr::WeakDyn<self::Two>, V> {
            #[inline]
            pub fn upgrade(&self) -> Option<self::Two<::classes::ptr::RcDyn<self::Two>, V>> {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner()).map(self::Two::_from_inner)
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
        type Super = Number;
        unsafe impl ::classes::class::HasSuper for self::Two {
            type Super = Number;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)]
                match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper for self::Two<::classes::ptr::RcDyn<self::Two>, V> {
            type Super = Number<::classes::ptr::RcDyn<Number>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref for self::Two<::classes::ptr::RcDyn<self::Two>, V> {
            type Target = Number<::classes::ptr::RcDyn<Number>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper for self::Two<::classes::ptr::WeakDyn<self::Two>, V> {
            type Super = Number<::classes::ptr::WeakDyn<Number>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::Two {
            type SuperData = ::classes::prelude::CData<Number>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::Two {
            type SuperVtable = ::classes::prelude::CVtable<Number>;
        }
        impl<V> self::Two<::classes::ptr::RcDyn<self::Two>, V> {
            #[inline]
            pub fn as_super(&self) -> &Number<::classes::ptr::RcDyn<Number>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Number<::classes::ptr::RcDyn<Number>, V> {
                Number::_from_inner(::classes::ptr::RcDyn::into_super(
                    self.clone()._into_inner(),
                ))
            }
            #[inline]
            pub fn into_super(self) -> Number<::classes::ptr::RcDyn<Number>, V> {
                Number::_from_inner(::classes::ptr::RcDyn::into_super(self._into_inner()))
            }
        }
        impl self::Two<::classes::ptr::RcDyn<self::Two>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Number<::classes::ptr::RcDyn<Number>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::Two<::classes::ptr::WeakDyn<self::Two>, V> {
            #[inline]
            pub fn as_super(&self) -> &Number<::classes::ptr::WeakDyn<Number>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Number<::classes::ptr::WeakDyn<Number>, V> {
                Number::_from_inner(::classes::ptr::WeakDyn::into_super(
                    self.clone()._into_inner(),
                ))
            }
            #[inline]
            pub fn into_super(self) -> Number<::classes::ptr::WeakDyn<Number>, V> {
                Number::_from_inner(::classes::ptr::WeakDyn::into_super(self._into_inner()))
            }
        }
        impl vtable::Two {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Number> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::Two<::classes::ptr::RcDyn<self::Two>, V>>
            for Number<::classes::ptr::RcDyn<Number>, V>
        {
            fn from(
                class: self::Two<::classes::ptr::RcDyn<self::Two>, V>,
            ) -> Number<::classes::ptr::RcDyn<Number>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Number<::classes::ptr::RcDyn<Number>, V>>
            for self::Two<::classes::ptr::RcDyn<self::Two>, V>
        {
            type Error = Number<::classes::ptr::RcDyn<Number>, V>;
            fn try_from(
                class: Number<::classes::ptr::RcDyn<Number>, V>,
            ) -> ::core::result::Result<self::Two<::classes::ptr::RcDyn<self::Two>, V>, Self::Error>
            {
                class
                    .try_as_subclass()
                    .cloned()
                    .ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::Two<::classes::ptr::WeakDyn<self::Two>, V>>
            for Number<::classes::ptr::WeakDyn<Number>, V>
        {
            fn from(
                class: self::Two<::classes::ptr::WeakDyn<self::Two>, V>,
            ) -> Number<::classes::ptr::WeakDyn<Number>, V> {
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
            pub struct Two {
                pub(super) _super: Super,
            }
            impl Two {
                #[cold]
                #[inline(never)]
                pub fn _delegate_ctor<
                    _S: ::classes::class::IsClass,
                    _F: FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
                >(
                    mut _self: ::classes::prelude::CRcUninit<Self>,
                    new: _F,
                ) -> ::classes::prelude::CRc<Self>
                where
                    ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                    for<'a> &'a ::classes::prelude::CRc<_S>: From<
                        &'a ::classes::ptr::RcDyn<
                            <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
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
                        let _ = |Self { _super }: Self| ();
                        Super::new(_self.into_super()).into_subclass_unchecked()
                    }
                }
                pub(super) fn get(_self: &::classes::prelude::CRc<Self>) -> i32 {
                    return 2;
                }
            }
        }
        mod vtable {
            use super::*;
            use ::classes::class::{
                ClassVtable, ClassVtableBase, NonVirtual, Virtual, VtableHasImpl, VtableHasSuper,
            };
            use ::classes::prelude::*;
            use ::classes::vtable::{MixinVtableHeader, TypeInfo, VtableHeader};
            pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
            #[repr(C)]
            pub struct Two {
                pub(super) _super: Super,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Two {
                #[inline]
                fn clone(&self) -> Two {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Two {}
            impl Two {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }
            pub struct DebugVtableLayout<'a> {
                this: &'a self::Two,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("Two");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self.this._super.debug_vtable_layout(
                            self.offset + {
                                builtin # offset_of(Two, _super)
                            },
                        ),
                    );
                    dbg.field("\'end", &(self.offset + ::core::mem::size_of::<Two>()));
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
                pub(in super::super) type Super =
                    ::classes::prelude::CVtableOpt<super::super::Super>;
                #[repr(C)]
                pub struct Two {
                    pub(in super::super) _super: Super,
                }
                #[automatically_derived]
                impl ::core::default::Default for Two {
                    #[inline]
                    fn default() -> Two {
                        Two {
                            _super: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Two {
                    #[inline]
                    fn clone(&self) -> Two {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Two {}
                impl Two {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
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
                    pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                        Super::init(_self);
                        {
                            let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<
                                _,
                                ::classes::prelude::CVtableOpt<Super>,
                            >(_self);
                            ptr.get = ::core::option::Option::Some(|this| {
                                ::classes::prelude::CData::<Self>::get(&unsafe {
                                    this.try_to_subtype().unwrap_unchecked()
                                })
                                .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Super>,
                            >(_self, &mut offset)
                            {
                                ptr.get = ::core::option::Option::Some(|this| {
                                    ::classes::prelude::CData::<Self>::get(&unsafe {
                                        this.try_to_subtype().unwrap_unchecked()
                                    })
                                    .into()
                                });
                            }
                        }
                    }
                    #[track_caller]
                    pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                        ::classes::prelude::CVtable::<Self> {
                            _super: self._super.assert_init(),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                ::classes::vtable::TypeInfo::new_concrete_class::<super::Two>(
                    ::core::option::Option::Some(Super::TYPE),
                    [],
                    MODULE_PATH,
                    "Two",
                );
        }
        const _: () = {};
        static VTABLE: ::classes::vtable::VtableWithMixinHeader<
            vtable::Two,
            { vtable::Two::MIXIN_HEADER_ENTRIES },
        > = {
            let mut vtable =
                ::classes::vtable::MaybeUninitVtableWithMixinHeader::new(vtable::opt::Two::DEFAULT);
            vtable::opt::Two::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(::core::option::Option::None, 0);
            vtable::opt::Two::init(vtable_opt);
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            ::classes::vtable::VtableWithMixinHeader::new(headers, vtable_opt.assert_init())
        };
        unsafe impl ::classes::class::ConcreteClass for self::Two {
            const VTABLE: ::core::ptr::NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }
        impl self::Two {
            pub const fn vtable<'a>() -> &'a ::classes::vtable::VtableWithMixinHeader<
                vtable::Two,
                { vtable::Two::MIXIN_HEADER_ENTRIES },
            > {
                &VTABLE
            }
        }
        impl Two<::classes::ptr::RcDyn<Two>> {
            #[inline]
            pub fn new() -> Self {
                ::classes::prelude::CData::<Self>::new(
                    ::classes::prelude::CRcUninit::<Self>::new_uninit(),
                )
            }
            #[inline]
            pub fn get(&self) -> i32 {
                { self.as_super().get() }.try_into().unwrap()
            }
        }
        impl Two<::classes::ptr::RcDyn<Two>, ::classes::class::NonVirtual> {
            #[inline]
            pub fn get(&self) -> i32 {
                {
                    ::classes::prelude::CData::<Self>::get(self.as_virtual())
                }
            }
        }
        impl Two<::classes::ptr::RcDyn<Two>> {}
    }
    use ::classes::prelude::*;
    #[allow(unused_imports)]
    pub(super) use _Cross::Cross;
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Cross {
        use super::*;
        use ::classes::class::{ConcreteClass, NonVirtual, Virtual};
        use ::classes::get_set::{GetSet, GetSetCopy};
        use ::classes::prelude::*;
        use ::classes::ptr::RcDyn;
        use ::classes::vtable::{
            MaybeUninitVtableWithMixinHeader, VtableHeader, VtableWithMixinHeader,
        };
        use ::core::ptr::NonNull;
        #[repr(transparent)]
        pub struct Cross<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual>(
            T,
            ::core::marker::PhantomData<V>,
        );
        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::Cross<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::Cross<T, V> {}
        impl<T, V> self::Cross<T, V> {
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
        impl<V> ::core::convert::From<::classes::ptr::RcDyn<self::Cross>>
            for self::Cross<::classes::ptr::RcDyn<self::Cross>, V>
        {
            fn from(inner: ::classes::ptr::RcDyn<self::Cross>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::Cross<::classes::ptr::RcDyn<self::Cross>, V>>
            for ::classes::ptr::RcDyn<self::Cross>
        {
            fn from(this: self::Cross<::classes::ptr::RcDyn<self::Cross>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<V> ::core::convert::From<::classes::ptr::WeakDyn<self::Cross>>
            for self::Cross<::classes::ptr::WeakDyn<self::Cross>, V>
        {
            fn from(inner: ::classes::ptr::WeakDyn<self::Cross>) -> Self {
                Self::_from_inner(inner)
            }
        }
        impl<V> ::core::convert::From<self::Cross<::classes::ptr::WeakDyn<self::Cross>, V>>
            for ::classes::ptr::WeakDyn<self::Cross>
        {
            fn from(this: self::Cross<::classes::ptr::WeakDyn<self::Cross>, V>) -> Self {
                this._into_inner()
            }
        }
        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::Cross<T, V> {
            fn from(inner: &'a T) -> Self {
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }
        impl<T, V> ::core::borrow::Borrow<T> for self::Cross<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }
        impl<V> ::classes::class::ClassRcWeak for self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
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
        impl<V> ::classes::class::ClassRcWeak for self::Cross<::classes::ptr::WeakDyn<self::Cross>, V> {
            type Upgraded = Option<self::Cross<::classes::ptr::RcDyn<self::Cross>, V>>;
            type UpgradedOpt = self::Cross<::classes::ptr::RcDyn<self::Cross>, V>;
            type DowngradeFrom = self::Cross<::classes::ptr::RcDyn<self::Cross>, V>;
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
                self::Cross::downgrade(from)
            }
        }
        impl<V, C: ::classes::class::ClassRc> ::core::cmp::PartialEq<C>
            for self::Cross<::classes::ptr::RcDyn<self::Cross>, V>
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
        impl<V> ::core::cmp::Eq for self::Cross<::classes::ptr::RcDyn<self::Cross>, V> where
            for<'a> &'a Self: ::core::convert::From<&'a ::classes::ptr::RcDyn<self::Cross>>
        {
        }
        impl<V> ::core::hash::Hash for self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                type CRcEqHash = ::classes::prelude::CRc<::classes::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    CRcEqHash::hash(&this, state);
                } else {
                    ::core::hash::Hash::hash(&::classes::class::ClassRcWeak::as_ptr(self), state);
                }
            }
        }
        impl<V, C: ::classes::class::ClassRcWeak> ::core::cmp::PartialEq<C>
            for self::Cross<::classes::ptr::WeakDyn<self::Cross>, V>
        {
            fn eq(&self, other: &C) -> bool {
                ::classes::class::ClassRcWeak::as_ptr(self)
                    == ::classes::class::ClassRcWeak::as_ptr(other)
            }
        }
        impl<V> ::core::cmp::Eq for self::Cross<::classes::ptr::WeakDyn<self::Cross>, V> {}
        impl<V> ::core::hash::Hash for self::Cross<::classes::ptr::WeakDyn<self::Cross>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(&::classes::class::ClassRcWeak::as_ptr(self), state);
            }
        }
        impl<V> ::core::fmt::Pointer for self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Pointer for self::Cross<::classes::ptr::WeakDyn<self::Cross>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::classes::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }
        impl<V> ::core::fmt::Debug for self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                type CRcFormat = ::classes::prelude::CRc<::classes::fmt::Format>;
                if let Some(this) = self.try_to_supertype::<CRcFormat>() {
                    CRcFormat::fmt_debug(&this, f)
                } else {
                    ::core::fmt::Display::fmt(&::classes::class::ClassRcWeak::as_ptr(self), f)
                }
            }
        }
        impl<V> ::core::fmt::Debug for self::Cross<::classes::ptr::WeakDyn<self::Cross>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if let Some(this) = self.upgrade() {
                    ::core::fmt::Debug::fmt(&this, f)
                } else {
                    ::core::fmt::Display::fmt(&::classes::class::ClassRcWeak::as_ptr(self), f)
                }
            }
        }
        impl<T, V> ::classes::class::IsClass for self::Cross<T, V> {
            type Class = self::Cross;
        }
        impl ::classes::class::IsClass for data::Cross {
            type Class = self::Cross;
        }
        impl self::Cross {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::Cross {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::Cross {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize =
                <vtable::Cross as ::classes::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::Cross {
            pub const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
        }
        impl ::classes::class::IsClass for vtable::Cross {
            type Class = self::Cross;
        }
        impl ::classes::class::IsClass for vtable::opt::Cross {
            type Class = self::Cross;
        }
        impl ::classes::class::ClassDataBase for data::Cross {
            type Vtable = vtable::Cross;
        }
        impl ::classes::class::ClassVtableBase for vtable::Cross {
            const TYPE: ::classes::vtable::Type = vtable::TYPE.as_type();
            type Data = data::Cross;
            type Opt = vtable::opt::Cross;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;
            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }
        impl<T, V> ::classes::class::ClassImpl for self::Cross<T, V> {
            type DataBase = data::Cross;
            type Data = data::Cross;
            type VtableBase = vtable::Cross;
            type Vtable = vtable::Cross;
            type VtableOpt = vtable::opt::Cross;
        }
        impl ::classes::class::ClassData for data::Cross {}
        unsafe impl ::classes::class::ClassVtable for vtable::Cross {}
        impl ::classes::class::ClassVtableOpt for vtable::opt::Cross {
            type VtableBase = vtable::Cross;
            type Vtable = vtable::Cross;
        }
        impl<V> ::classes::class::Class for self::Cross<::classes::class::ClassMarker, V> {
            type Rc = self::Cross<::classes::ptr::RcDyn<self::Cross>, V>;
            type Weak = self::Cross<::classes::ptr::WeakDyn<self::Cross>, V>;
            type Ptr = ::classes::ptr::PtrDyn<vtable::Cross>;
        }
        impl<V> self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
            pub fn downgrade(this: &Self) -> self::Cross<::classes::ptr::WeakDyn<self::Cross>, V> {
                self::Cross::_from_inner(::classes::ptr::RcDyn::downgrade(this._as_inner()))
            }
        }
        impl vtable::Cross {
            #[inline]
            const fn cast_header(this: *const Self) -> *const ::classes::vtable::VtableHeader {
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
        impl<V> self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::Cross<::classes::ptr::RcDyn<self::Cross>, ::classes::class::Virtual>
            {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::Cross<::classes::ptr::RcDyn<self::Cross>, ::classes::class::NonVirtual>
            {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> ::classes::class::ClassRc for self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {}
        impl<V> self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<A::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<A::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_into_superclass::<A::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_superclass::<A::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_superclass_unchecked::<A::Class>(
                        self.clone()._into_inner(),
                    )
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_superclass_unchecked::<A::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                        {
                        } else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::into_supertype::<A::Class>(self._into_inner()).into()
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
                        {
                        } else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                            {
                                ::core::panicking::panic_fmt(format_args!("not a subtype"));
                            }
                        };
                    }
                    Assert::<<Self as ::classes::class::ClassImpl>::Vtable, A::Vtable>::CHECK
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
                ::classes::ptr::RcDyn::try_into_subtype::<D::Class>(self.clone()._into_inner())
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
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self._into_inner()).into()
            }
            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn to_subtype<D>(&self) -> D
            where
                D: ::classes::class::ClassRc,
                for<'a> &'a D: From<&'a ::classes::ptr::RcDyn<D::Class>>,
            {
                ::classes::ptr::RcDyn::into_subtype::<D::Class>(self.clone()._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::upcast::<A::Class, I::Class>(self.clone()._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::upcast_unchecked::<A::Class, I::Class>(
                        self.clone()._into_inner(),
                    )
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <Self as ::classes::class::ClassImpl>::Vtable,
                        <A::Class as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_upcast::<A::Class, I::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub unsafe fn downcast_unchecked<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::Cross>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::downcast_unchecked::<B::Class, S::Class>(
                        self.clone()._into_inner(),
                    )
                }
                .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast<B, S>(&self) -> Option<S>
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::Cross>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::try_downcast::<B::Class, S::Class>(
                    self.clone()._into_inner(),
                )
                .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn downcast<B, S>(&self) -> S
            where
                B: ::classes::class::IsClass<Class: ::classes::class::HasImpl<self::Cross>>,
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<S::Vtable, <B::Class as ::classes::class::ClassImpl>::Vtable>::CHECK
                };
                ::classes::ptr::RcDyn::downcast::<B::Class, S::Class>(self.clone()._into_inner())
                    .into()
            }
            #[inline]
            pub fn try_cast_mixin<M>(&self) -> Option<M>
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::try_into_mixin::<M::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }
            #[inline]
            #[track_caller]
            pub fn cast_mixin<M>(&self) -> M
            where
                M: ::classes::class::IsClass<Class: ::classes::class::MixinClassImpl>
                    + From<::classes::ptr::RcDyn<M::Class>>,
            {
                ::classes::ptr::RcDyn::into_mixin::<M::Class>(self.clone()._into_inner()).into()
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
                    ::classes::ptr::RcDyn::into_mixin_unchecked::<M::Class>(
                        self.clone()._into_inner(),
                        instance,
                    )
                }
                .into()
            }
            #[inline]
            #[track_caller]
            pub fn try_downcast_ty(&self, ty: ::classes::vtable::Type) -> Option<&Self> {
                ::classes::ptr::RcDyn::try_downcast_ty(self._as_inner(), ty).map(Into::into)
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::into_subclass_unchecked::<D::Class>(self._into_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::into_subclass::<D::Class>(self._into_inner()).into()
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<D::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                ::classes::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner()).map(Into::into)
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
                            }
                        };
                    }
                    Assert::<
                        <D::Class as ::classes::class::ClassImpl>::Vtable,
                        <Self as ::classes::class::ClassImpl>::Vtable,
                    >::CHECK
                };
                unsafe {
                    ::classes::ptr::RcDyn::as_subclass_unchecked::<D::Class>(self._as_inner())
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
                                ::core::panicking::panic_fmt(format_args!("not a subclass"));
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
            pub fn as_ptr(this: &Self) -> ::classes::prelude::CPtr<self::Cross> {
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
        impl<V> self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Cross<::classes::ptr::WeakDyn<self::Cross>, V> {
            #[inline]
            pub fn to_impl<A: ::classes::class::ClassImpl>(&self) -> A
            where
                Self: ::classes::class::HasImpl<A>,
            {
                ::classes::class::HasImpl::to_impl(self)
            }
        }
        impl<V> self::Cross<::classes::ptr::WeakDyn<self::Cross>, V> {
            #[inline]
            pub fn upgrade(&self) -> Option<self::Cross<::classes::ptr::RcDyn<self::Cross>, V>> {
                ::classes::ptr::WeakDyn::upgrade(self._as_inner()).map(self::Cross::_from_inner)
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
        unsafe impl ::classes::class::HasSuper for self::Cross {
            type Super = Object;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)]
                match self._into_inner() {}
            }
        }
        unsafe impl<V> ::classes::class::HasSuper for self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
            type Super = Object<::classes::ptr::RcDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref for self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
            type Target = Object<::classes::ptr::RcDyn<Object>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> ::classes::class::HasSuper for self::Cross<::classes::ptr::WeakDyn<self::Cross>, V> {
            type Super = Object<::classes::ptr::WeakDyn<Object>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl ::classes::class::DataHasSuper for data::Cross {
            type SuperData = ::classes::prelude::CData<Object>;
        }
        unsafe impl ::classes::class::VtableHasSuper for vtable::Cross {
            type SuperVtable = ::classes::prelude::CVtable<Object>;
        }
        impl<V> self::Cross<::classes::ptr::RcDyn<self::Cross>, V> {
            #[inline]
            pub fn as_super(&self) -> &Object<::classes::ptr::RcDyn<Object>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Object<::classes::ptr::RcDyn<Object>, V> {
                Object::_from_inner(::classes::ptr::RcDyn::into_super(
                    self.clone()._into_inner(),
                ))
            }
            #[inline]
            pub fn into_super(self) -> Object<::classes::ptr::RcDyn<Object>, V> {
                Object::_from_inner(::classes::ptr::RcDyn::into_super(self._into_inner()))
            }
        }
        impl self::Cross<::classes::ptr::RcDyn<self::Cross>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &Object<::classes::ptr::RcDyn<Object>, ::classes::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }
        impl<V> self::Cross<::classes::ptr::WeakDyn<self::Cross>, V> {
            #[inline]
            pub fn as_super(&self) -> &Object<::classes::ptr::WeakDyn<Object>, V> {
                ::classes::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                Object::_from_inner(::classes::ptr::WeakDyn::into_super(
                    self.clone()._into_inner(),
                ))
            }
            #[inline]
            pub fn into_super(self) -> Object<::classes::ptr::WeakDyn<Object>, V> {
                Object::_from_inner(::classes::ptr::WeakDyn::into_super(self._into_inner()))
            }
        }
        impl vtable::Cross {
            pub const fn as_super(&self) -> &::classes::prelude::CVtable<Object> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl<V> From<self::Cross<::classes::ptr::RcDyn<self::Cross>, V>>
            for Object<::classes::ptr::RcDyn<Object>, V>
        {
            fn from(
                class: self::Cross<::classes::ptr::RcDyn<self::Cross>, V>,
            ) -> Object<::classes::ptr::RcDyn<Object>, V> {
                class.into_super()
            }
        }
        impl<V> TryFrom<Object<::classes::ptr::RcDyn<Object>, V>>
            for self::Cross<::classes::ptr::RcDyn<self::Cross>, V>
        {
            type Error = Object<::classes::ptr::RcDyn<Object>, V>;
            fn try_from(
                class: Object<::classes::ptr::RcDyn<Object>, V>,
            ) -> ::core::result::Result<
                self::Cross<::classes::ptr::RcDyn<self::Cross>, V>,
                Self::Error,
            > {
                class
                    .try_as_subclass()
                    .cloned()
                    .ok_or_else(|| class.clone())
            }
        }
        impl<V> From<self::Cross<::classes::ptr::WeakDyn<self::Cross>, V>>
            for Object<::classes::ptr::WeakDyn<Object>, V>
        {
            fn from(
                class: self::Cross<::classes::ptr::WeakDyn<self::Cross>, V>,
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
            pub struct Cross {
                pub(super) _super: Super,
            }
            impl Cross {
                #[cold]
                #[inline(never)]
                pub fn _delegate_ctor<
                    _S: ::classes::class::IsClass,
                    _F: FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
                >(
                    mut _self: ::classes::prelude::CRcUninit<Self>,
                    new: _F,
                ) -> ::classes::prelude::CRc<Self>
                where
                    ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                    for<'a> &'a ::classes::prelude::CRc<_S>: From<
                        &'a ::classes::ptr::RcDyn<
                            <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
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
                        let _ = |Self { _super }: Self| ();
                        ::classes::prelude::CData::<::classes::object::Object>::new(
                            _self.into_super(),
                        )
                        .into_subclass_unchecked()
                    }
                }
                pub(super) fn id(
                    _self: &::classes::prelude::CRc<Self>,
                    n: CRc<Number>,
                ) -> CRc<Number> {
                    return n;
                }
            }
        }
        mod vtable {
            use super::*;
            use ::classes::class::{
                ClassVtable, ClassVtableBase, NonVirtual, Virtual, VtableHasImpl, VtableHasSuper,
            };
            use ::classes::prelude::*;
            use ::classes::vtable::{MixinVtableHeader, TypeInfo, VtableHeader};
            pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
            #[repr(C)]
            pub struct Cross {
                pub(super) _super: Super,
                pub id: fn(&::classes::prelude::CRc<Self>, n: CRc<Number>) -> CRc<Number>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Cross {
                #[inline]
                fn clone(&self) -> Cross {
                    let _: ::core::clone::AssertParamIsClone<Super>;
                    let _: ::core::clone::AssertParamIsClone<
                        fn(&::classes::prelude::CRc<Self>, n: CRc<Number>) -> CRc<Number>,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Cross {}
            impl Cross {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }
            pub struct DebugVtableLayout<'a> {
                this: &'a self::Cross,
                offset: usize,
            }
            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut dbg = f.debug_struct("Cross");
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self.this._super.debug_vtable_layout(
                            self.offset + {
                                builtin # offset_of(Cross, _super)
                            },
                        ),
                    );
                    dbg.field(
                        "id",
                        &(self.offset + {
                            builtin # offset_of(Cross, id)
                        }),
                    );
                    dbg.field("\'end", &(self.offset + ::core::mem::size_of::<Cross>()));
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
                pub(in super::super) type Super =
                    ::classes::prelude::CVtableOpt<super::super::Super>;
                #[repr(C)]
                pub struct Cross {
                    pub(in super::super) _super: Super,
                    pub id: ::core::option::Option<
                        fn(&::classes::prelude::CRc<Self>, n: CRc<Number>) -> CRc<Number>,
                    >,
                }
                #[automatically_derived]
                impl ::core::default::Default for Cross {
                    #[inline]
                    fn default() -> Cross {
                        Cross {
                            _super: ::core::default::Default::default(),
                            id: ::core::default::Default::default(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Cross {
                    #[inline]
                    fn clone(&self) -> Cross {
                        let _: ::core::clone::AssertParamIsClone<Super>;
                        let _: ::core::clone::AssertParamIsClone<
                            ::core::option::Option<
                                fn(&::classes::prelude::CRc<Self>, n: CRc<Number>) -> CRc<Number>,
                            >,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Cross {}
                impl Cross {
                    pub const DEFAULT: Self = Self {
                        _super: Super::DEFAULT,
                        id: ::core::option::Option::None,
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
                    pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                        Super::init(_self);
                        {
                            let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self);
                            ptr.id = ::core::option::Option::Some(|this, n| {
                                ::classes::prelude::CData::<Self>::id(
                                    &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    n,
                                )
                                .into()
                            });
                            while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<
                                _,
                                ::classes::prelude::CVtableOpt<Self>,
                            >(_self, &mut offset)
                            {
                                ptr.id = ::core::option::Option::Some(|this, n| {
                                    ::classes::prelude::CData::<Self>::id(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        n,
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
                            id: self.id.expect(
                                "cannot instantiate because method `Cross::id` is not implemented",
                            ),
                        }
                    }
                }
            }
            pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                ::classes::vtable::TypeInfo::new_concrete_class::<super::Cross>(
                    ::core::option::Option::Some(Super::TYPE),
                    [],
                    MODULE_PATH,
                    "Cross",
                );
        }
        const _: () = {
            if !(::core::mem::size_of::<vtable::Cross>()
                == ::core::mem::size_of::<vtable::opt::Cross>())
            {
                {
                    ::core::panicking::panic_fmt(format_args!(
                        "size of vtable :: Cross != size of vtable :: opt :: Cross",
                    ));
                }
            }
            if !({
                builtin # offset_of(vtable::Cross, id)
            } == {
                builtin # offset_of(vtable::opt::Cross, id)
            }) {
                {
                    ::core::panicking::panic_fmt(format_args!(
                        "offset of vtable :: Cross::id != offset of vtable :: opt :: Cross::id",
                    ));
                }
            }
        };
        static VTABLE: ::classes::vtable::VtableWithMixinHeader<
            vtable::Cross,
            { vtable::Cross::MIXIN_HEADER_ENTRIES },
        > = {
            let mut vtable = ::classes::vtable::MaybeUninitVtableWithMixinHeader::new(
                vtable::opt::Cross::DEFAULT,
            );
            vtable::opt::Cross::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(::core::option::Option::None, 0);
            vtable::opt::Cross::init(vtable_opt);
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            ::classes::vtable::VtableWithMixinHeader::new(headers, vtable_opt.assert_init())
        };
        unsafe impl ::classes::class::ConcreteClass for self::Cross {
            const VTABLE: ::core::ptr::NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }
        impl self::Cross {
            pub const fn vtable<'a>() -> &'a ::classes::vtable::VtableWithMixinHeader<
                vtable::Cross,
                { vtable::Cross::MIXIN_HEADER_ENTRIES },
            > {
                &VTABLE
            }
        }
        impl Cross<::classes::ptr::RcDyn<Cross>> {
            #[inline]
            pub fn new() -> Self {
                ::classes::prelude::CData::<Self>::new(
                    ::classes::prelude::CRcUninit::<Self>::new_uninit(),
                )
            }
            #[inline]
            pub fn id(&self, n: CRc<Number>) -> CRc<Number> {
                {
                    (self.0.vtable().id)(self, n)
                }
            }
        }
        impl Cross<::classes::ptr::RcDyn<Cross>, ::classes::class::NonVirtual> {
            #[inline]
            pub fn id(&self, n: CRc<Number>) -> CRc<Number> {
                {
                    ::classes::prelude::CData::<Self>::id(self.as_virtual(), n)
                }
            }
        }
        impl Cross<::classes::ptr::RcDyn<Cross>> {}
    }
}
fn main() {
    let one = One::new();
    let two = Two::new();
    let n1: CRc<Number> = one.as_super().clone();
    let n2: CRc<Number> = two.as_super().clone();
    let cross = Cross::new();
    let x = cross.id(n1);
    let y = cross.id(n2);
    {
        ::std::io::_print(format_args!("x{0} y{1}\n", x.get(), y.get()));
    };
}
