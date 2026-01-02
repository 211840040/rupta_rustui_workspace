#[macro_export]
macro_rules! _def_class {
    ($kind:ident $class:ident) => {
        #[repr(transparent)]
        pub struct $class<T = $crate::class::ClassMarker, V = $crate::class::Virtual>(
            T,
            ::core::marker::PhantomData<V>,
        );

        impl<T: ::core::clone::Clone, V> ::core::clone::Clone for self::$class<T, V> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), ::core::marker::PhantomData)
            }
        }
        impl<T: ::core::marker::Copy, V> ::core::marker::Copy for self::$class<T, V> {}

        impl<T, V> self::$class<T, V> {
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

        impl<V> ::core::convert::From<$crate::ptr::RcDyn<self::$class>>
            for self::$class<$crate::ptr::RcDyn<self::$class>, V>
        {
            fn from(inner: $crate::ptr::RcDyn<self::$class>) -> Self {
                Self::_from_inner(inner)
            }
        }

        impl<V> ::core::convert::From<self::$class<$crate::ptr::RcDyn<self::$class>, V>>
            for $crate::ptr::RcDyn<self::$class>
        {
            fn from(this: self::$class<$crate::ptr::RcDyn<self::$class>, V>) -> Self {
                this._into_inner()
            }
        }

        impl<V> ::core::convert::From<$crate::ptr::WeakDyn<self::$class>>
            for self::$class<$crate::ptr::WeakDyn<self::$class>, V>
        {
            fn from(inner: $crate::ptr::WeakDyn<self::$class>) -> Self {
                Self::_from_inner(inner)
            }
        }

        impl<V> ::core::convert::From<self::$class<$crate::ptr::WeakDyn<self::$class>, V>>
            for $crate::ptr::WeakDyn<self::$class>
        {
            fn from(this: self::$class<$crate::ptr::WeakDyn<self::$class>, V>) -> Self {
                this._into_inner()
            }
        }

        impl<'a, T, V> ::core::convert::From<&'a T> for &'a self::$class<T, V> {
            fn from(inner: &'a T) -> Self {
                // SAFETY: it is safe because `$class` is `repr(transparent)`
                // and there is no invariants inside.
                unsafe { &*core::ptr::from_ref(inner).cast() }
            }
        }

        impl<T, V> ::core::borrow::Borrow<T> for self::$class<T, V> {
            fn borrow(&self) -> &T {
                self._as_inner()
            }
        }

        impl<V> $crate::class::ClassRcWeak for self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            type Upgraded = Self;
            type UpgradedOpt = Self;
            type DowngradeFrom = Self;

            fn as_ptr(this: &Self) -> $crate::prelude::CPtr<Self> {
                $crate::ptr::RcDyn::as_ptr(this._as_inner())
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
        impl<V> $crate::class::ClassRcWeak for self::$class<$crate::ptr::WeakDyn<self::$class>, V> {
            type Upgraded = Option<self::$class<$crate::ptr::RcDyn<self::$class>, V>>;
            type UpgradedOpt = self::$class<$crate::ptr::RcDyn<self::$class>, V>;
            type DowngradeFrom = self::$class<$crate::ptr::RcDyn<self::$class>, V>;

            fn as_ptr(this: &Self) -> $crate::prelude::CPtr<Self> {
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
                self::$class::downgrade(from)
            }
        }

        impl<V, C: $crate::class::ClassRc> ::core::cmp::PartialEq<C>
            for self::$class<$crate::ptr::RcDyn<self::$class>, V>
        where
            for<'a> &'a C: ::core::convert::From<&'a $crate::ptr::RcDyn<C::Class>>,
        {
            fn eq(&self, other: &C) -> bool {
                type CRcEqHash = $crate::prelude::CRc<$crate::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    let other = $crate::class::ClassRc::to_supertype::<
                        $crate::prelude::CRc<$crate::object::Object>,
                    >(other);
                    CRcEqHash::eq(&this, &other)
                } else {
                    $crate::class::ClassRcWeak::as_ptr(self)
                        == $crate::class::ClassRcWeak::as_ptr(other)
                }
            }
        }

        impl<V> ::core::cmp::Eq for self::$class<$crate::ptr::RcDyn<self::$class>, V> where
            for<'a> &'a Self: ::core::convert::From<&'a $crate::ptr::RcDyn<self::$class>>
        {
        }

        impl<V> ::core::hash::Hash for self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                type CRcEqHash = $crate::prelude::CRc<$crate::eq_hash::EqHash>;
                if let Some(this) = self.try_to_supertype::<CRcEqHash>() {
                    CRcEqHash::hash(&this, state);
                } else {
                    ::core::hash::Hash::hash(&$crate::class::ClassRcWeak::as_ptr(self), state);
                }
            }
        }

        impl<V, C: $crate::class::ClassRcWeak> ::core::cmp::PartialEq<C>
            for self::$class<$crate::ptr::WeakDyn<self::$class>, V>
        {
            fn eq(&self, other: &C) -> bool {
                $crate::class::ClassRcWeak::as_ptr(self)
                    == $crate::class::ClassRcWeak::as_ptr(other)
            }
        }

        impl<V> ::core::cmp::Eq for self::$class<$crate::ptr::WeakDyn<self::$class>, V> {}

        impl<V> ::core::hash::Hash for self::$class<$crate::ptr::WeakDyn<self::$class>, V> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(&$crate::class::ClassRcWeak::as_ptr(self), state);
            }
        }

        impl<V> ::core::fmt::Pointer for self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                $crate::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }

        impl<V> ::core::fmt::Pointer for self::$class<$crate::ptr::WeakDyn<self::$class>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                $crate::class::ClassRcWeak::as_ptr(self).ptr().fmt(f)
            }
        }

        impl<V> ::core::fmt::Debug for self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                type CRcFormat = $crate::prelude::CRc<$crate::fmt::Format>;
                if let Some(this) = self.try_to_supertype::<CRcFormat>() {
                    CRcFormat::fmt_debug(&this, f)
                } else {
                    ::core::fmt::Display::fmt(&$crate::class::ClassRcWeak::as_ptr(self), f)
                }
            }
        }

        impl<V> ::core::fmt::Debug for self::$class<$crate::ptr::WeakDyn<self::$class>, V> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if let Some(this) = self.upgrade() {
                    ::core::fmt::Debug::fmt(&this, f)
                } else {
                    ::core::fmt::Display::fmt(&$crate::class::ClassRcWeak::as_ptr(self), f)
                }
            }
        }

        impl<T, V> $crate::class::IsClass for self::$class<T, V> {
            type Class = self::$class;
            // type Interface = interface::$class;
        }
        impl $crate::class::IsClass for data::$class {
            type Class = self::$class;
            // type Interface = interface::$class;
        }
        impl self::$class {
            pub const TYPE: $crate::vtable::Type = vtable::TYPE.as_type();
        }
        impl data::$class {
            pub const TYPE: $crate::vtable::Type = vtable::TYPE.as_type();
        }
        impl vtable::$class {
            pub const TYPE: $crate::vtable::Type = vtable::TYPE.as_type();
            pub const MIXIN_HEADER_ENTRIES: usize =
                <vtable::$class as $crate::class::ClassVtableBase>::MIXIN_HEADER_ENTRIES;
        }
        impl vtable::opt::$class {
            pub const TYPE: $crate::vtable::Type = vtable::TYPE.as_type();
        }

        impl $crate::class::IsClass for vtable::$class {
            type Class = self::$class;
        }
        impl $crate::class::IsClass for vtable::opt::$class {
            type Class = self::$class;
        }

        impl $crate::class::ClassDataBase for data::$class {
            type Vtable = vtable::$class;
        }
        impl $crate::class::ClassVtableBase for vtable::$class {
            const TYPE: $crate::vtable::Type = vtable::TYPE.as_type();

            type Data = data::$class;
            type Opt = vtable::opt::$class;
            type DebugVtableLayout<'a> = vtable::DebugVtableLayout<'a>;

            fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
                self.debug_vtable_layout(offset)
            }
        }

        $crate::_def_class! { impl ClassImpl for #[$kind] $class }

        // impl<T, V> $crate::class::ClassImpl for self::$class<T, V> {
        //     type Data = data::$class;
        //     type Vtable = vtable::$class;
        //     type VtableWithMixinHeader = $crate::vtable::VtableWithMixinHeader<
        //         vtable::$class,
        //         { <vtable::$class as $crate::class::ClassVtable>::MIXIN_HEADER_ENTRIES },
        //     >;
        //     type VtableOpt = vtable::opt::$class;
        // }
        // impl $crate::class::ClassImpl for interface::$class {
        //     type CData = $crate::class::InterfaceDataMarker<vtable::$class, vtable::opt::$class>;
        //     type CVtable = vtable::$class;
        //     type VtableOpt = vtable::opt::$class;
        // }

        impl<V> $crate::class::Class for self::$class<$crate::class::ClassMarker, V> {
            type Rc = self::$class<$crate::ptr::RcDyn<self::$class>, V>;

            type Weak = self::$class<$crate::ptr::WeakDyn<self::$class>, V>;

            type Ptr = $crate::ptr::PtrDyn<vtable::$class>;
        }
        // impl $crate::class::Class for interface::$class {
        //     type CRc = self::$class<$crate::ptr::RcDyn<interface::$class>>;
        //
        //     type CWeak = self::$class<$crate::ptr::WeakDyn<interface::$class>>;
        //
        //     type Ptr = $crate::ptr::PtrDyn<interface::$class>;
        // }

        impl<V> self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            pub fn downgrade(this: &Self) -> self::$class<$crate::ptr::WeakDyn<self::$class>, V> {
                self::$class::_from_inner($crate::ptr::RcDyn::downgrade(this._as_inner()))
            }
        }

        impl vtable::$class {
            #[inline]
            const fn cast_header(this: *const Self) -> *const $crate::vtable::VtableHeader {
                this.cast()
            }
            pub const fn header(&self) -> &$crate::vtable::VtableHeader {
                unsafe { &*Self::cast_header(self) }
            }
            #[track_caller]
            pub const fn ty(&self) -> $crate::vtable::Type {
                self.object_ty().as_type()
            }
            #[track_caller]
            pub const fn object_ty(&self) -> $crate::vtable::ConcreteClassType {
                let offset = self.header().offset_of_object_header();
                unsafe { &*Self::cast_header(self).byte_offset(offset) }
                    .object_ty()
                    .expect("expect object type")
            }
        }

        impl<V> self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            pub(in super::super) fn as_virtual(
                &self,
            ) -> &self::$class<$crate::ptr::RcDyn<self::$class>, $crate::class::Virtual> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub(in super::super) fn as_non_virtual(
                &self,
            ) -> &self::$class<$crate::ptr::RcDyn<self::$class>, $crate::class::NonVirtual> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }

        impl<V> $crate::class::ClassRc for self::$class<$crate::ptr::RcDyn<self::$class>, V> {}

        impl<V> self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub fn try_into_superclass<A>(self) -> Option<A>
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                $crate::ptr::RcDyn::try_into_superclass::<A::Class>(self._into_inner())
                    .map(Into::into)
            }

            #[inline]
            #[track_caller]
            pub fn into_superclass<A>(self) -> A
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                $crate::ptr::RcDyn::into_superclass::<A::Class>(self._into_inner()).into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub unsafe fn into_superclass_unchecked<A>(self) -> A
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                unsafe {
                    $crate::ptr::RcDyn::into_superclass_unchecked::<A::Class>(self._into_inner())
                }
                .into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub fn try_to_superclass<A>(&self) -> Option<A>
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                $crate::ptr::RcDyn::try_into_superclass::<A::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }

            #[inline]
            #[track_caller]
            pub fn to_superclass<A>(&self) -> A
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                $crate::ptr::RcDyn::into_superclass::<A::Class>(self.clone()._into_inner()).into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub unsafe fn to_superclass_unchecked<A>(&self) -> A
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                unsafe {
                    $crate::ptr::RcDyn::into_superclass_unchecked::<A::Class>(
                        self.clone()._into_inner(),
                    )
                }
                .into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub unsafe fn as_superclass_unchecked<A>(&self) -> &A
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                unsafe { $crate::ptr::RcDyn::as_superclass_unchecked::<A::Class>(self._as_inner()) }
                    .into()
            }

            #[inline]
            #[track_caller]
            pub fn as_superclass<A>(&self) -> &A
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                $crate::ptr::RcDyn::as_superclass::<A::Class>(self._as_inner()).into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub fn try_as_superclass<A>(&self) -> Option<&A>
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                $crate::ptr::RcDyn::try_as_superclass::<A::Class>(self._as_inner()).map(Into::into)
            }

            #[inline]
            #[track_caller]
            pub fn into_supertype<A>(self) -> A
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subtype!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                $crate::ptr::RcDyn::into_supertype::<A::Class>(self._into_inner()).into()
            }

            #[inline]
            #[track_caller]
            pub fn to_supertype<A>(&self) -> A
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::static_assert_subtype!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    A::Vtable
                );
                $crate::ptr::RcDyn::to_supertype::<A::Class>(self._as_inner()).into()
            }

            #[inline]
            #[track_caller]
            pub fn try_into_supertype<A>(self) -> Option<A>
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::ptr::RcDyn::try_into_supertype::<A::Class>(self._into_inner())
                    .map(Into::into)
            }

            #[inline]
            #[track_caller]
            pub fn try_to_supertype<A>(&self) -> Option<A>
            where
                A: $crate::class::ClassRc,
                for<'a> &'a A: From<&'a $crate::ptr::RcDyn<A::Class>>,
            {
                $crate::ptr::RcDyn::try_to_supertype::<A::Class>(self._as_inner()).map(Into::into)
            }

            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn try_into_subtype<D>(self) -> Option<D>
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::ptr::RcDyn::try_into_subtype::<D::Class>(self._into_inner()).map(Into::into)
            }

            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn try_to_subtype<D>(&self) -> Option<D>
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::ptr::RcDyn::try_into_subtype::<D::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }

            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn into_subtype<D>(self) -> D
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::ptr::RcDyn::into_subtype::<D::Class>(self._into_inner()).into()
            }

            /// Cast the `CRc` to its subtype `D`.
            #[inline]
            #[track_caller]
            pub fn to_subtype<D>(&self) -> D
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::ptr::RcDyn::into_subtype::<D::Class>(self.clone()._into_inner()).into()
            }

            #[inline]
            #[track_caller]
            pub fn upcast<A, I>(&self) -> I
            where
                A: $crate::class::IsClass<Class: $crate::class::HasImpl<I::Class>>,
                I: $crate::class::ClassRc,
                for<'a> &'a I: From<&'a $crate::ptr::RcDyn<I::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    <A::Class as $crate::class::ClassImpl>::Vtable
                );
                $crate::ptr::RcDyn::upcast::<A::Class, I::Class>(self.clone()._into_inner()).into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub unsafe fn upcast_unchecked<A, I>(&self) -> I
            where
                A: $crate::class::IsClass<Class: $crate::class::HasImpl<I::Class>>,
                I: $crate::class::ClassRc,
                for<'a> &'a I: From<&'a $crate::ptr::RcDyn<I::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    <A::Class as $crate::class::ClassImpl>::Vtable
                );
                unsafe {
                    $crate::ptr::RcDyn::upcast_unchecked::<A::Class, I::Class>(
                        self.clone()._into_inner(),
                    )
                }
                .into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub fn try_upcast<A, I>(&self) -> Option<I>
            where
                A: $crate::class::IsClass<Class: $crate::class::HasImpl<I::Class>>,
                I: $crate::class::ClassRc,
                for<'a> &'a I: From<&'a $crate::ptr::RcDyn<I::Class>>,
            {
                $crate::static_assert_subclass!(
                    <Self as $crate::class::ClassImpl>::Vtable,
                    <A::Class as $crate::class::ClassImpl>::Vtable
                );
                $crate::ptr::RcDyn::try_upcast::<A::Class, I::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub unsafe fn downcast_unchecked<B, S>(&self) -> S
            where
                B: $crate::class::IsClass<Class: $crate::class::HasImpl<self::$class>>,
                S: $crate::class::ClassRc,
                for<'a> &'a S: From<&'a $crate::ptr::RcDyn<S::Class>>,
            {
                $crate::static_assert_subclass!(
                    S::Vtable,
                    <B::Class as $crate::class::ClassImpl>::Vtable
                );
                unsafe {
                    $crate::ptr::RcDyn::downcast_unchecked::<B::Class, S::Class>(
                        self.clone()._into_inner(),
                    )
                }
                .into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub fn try_downcast<B, S>(&self) -> Option<S>
            where
                B: $crate::class::IsClass<Class: $crate::class::HasImpl<self::$class>>,
                S: $crate::class::ClassRc,
                for<'a> &'a S: From<&'a $crate::ptr::RcDyn<S::Class>>,
            {
                $crate::static_assert_subclass!(
                    S::Vtable,
                    <B::Class as $crate::class::ClassImpl>::Vtable
                );
                $crate::ptr::RcDyn::try_downcast::<B::Class, S::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }

            #[inline]
            #[track_caller]
            pub fn downcast<B, S>(&self) -> S
            where
                B: $crate::class::IsClass<Class: $crate::class::HasImpl<self::$class>>,
                S: $crate::class::ClassRc,
                for<'a> &'a S: From<&'a $crate::ptr::RcDyn<S::Class>>,
            {
                $crate::static_assert_subclass!(
                    S::Vtable,
                    <B::Class as $crate::class::ClassImpl>::Vtable
                );
                $crate::ptr::RcDyn::downcast::<B::Class, S::Class>(self.clone()._into_inner())
                    .into()
            }

            #[inline]
            pub fn try_cast_mixin<M>(&self) -> Option<M>
            where
                M: $crate::class::IsClass<Class: $crate::class::MixinClassImpl>
                    + From<$crate::ptr::RcDyn<M::Class>>,
            {
                $crate::ptr::RcDyn::try_into_mixin::<M::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }

            #[inline]
            #[track_caller]
            pub fn cast_mixin<M>(&self) -> M
            where
                M: $crate::class::IsClass<Class: $crate::class::MixinClassImpl>
                    + From<$crate::ptr::RcDyn<M::Class>>,
            {
                $crate::ptr::RcDyn::into_mixin::<M::Class>(self.clone()._into_inner()).into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub unsafe fn cast_mixin_unchecked<M>(
                &self,
                instance: $crate::vtable::MixinInstanceType,
            ) -> M
            where
                M: $crate::class::IsClass<Class: $crate::class::MixinClassImpl>
                    + From<$crate::ptr::RcDyn<M::Class>>,
            {
                unsafe {
                    $crate::ptr::RcDyn::into_mixin_unchecked::<M::Class>(
                        self.clone()._into_inner(),
                        instance,
                    )
                }
                .into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub fn try_downcast_ty(&self, ty: $crate::vtable::Type) -> Option<&Self> {
                $crate::ptr::RcDyn::try_downcast_ty(self._as_inner(), ty).map(Into::into)
            }

            #[inline]
            #[track_caller]
            pub fn downcast_ty(&self, ty: $crate::vtable::Type) -> &Self {
                $crate::ptr::RcDyn::downcast_ty(self._as_inner(), ty).into()
            }

            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub unsafe fn into_subclass_unchecked<D>(self) -> D
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::static_assert_subclass!(
                    <D::Class as $crate::class::ClassImpl>::Vtable,
                    <Self as $crate::class::ClassImpl>::Vtable
                );
                unsafe {
                    $crate::ptr::RcDyn::into_subclass_unchecked::<D::Class>(self._into_inner())
                }
                .into()
            }

            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn into_subclass<D>(self) -> D
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::static_assert_subclass!(
                    <D::Class as $crate::class::ClassImpl>::Vtable,
                    <Self as $crate::class::ClassImpl>::Vtable
                );
                $crate::ptr::RcDyn::into_subclass::<D::Class>(self._into_inner()).into()
            }

            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub fn try_into_subclass<D>(self) -> Option<D>
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::static_assert_subclass!(
                    <D::Class as $crate::class::ClassImpl>::Vtable,
                    <Self as $crate::class::ClassImpl>::Vtable
                );
                $crate::ptr::RcDyn::try_into_subclass::<D::Class>(self._into_inner())
                    .map(Into::into)
            }

            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub unsafe fn as_subclass_unchecked<D>(&self) -> &D
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::static_assert_subclass!(
                    <D::Class as $crate::class::ClassImpl>::Vtable,
                    <Self as $crate::class::ClassImpl>::Vtable
                );
                unsafe { $crate::ptr::RcDyn::as_subclass_unchecked::<D::Class>(self._as_inner()) }
                    .into()
            }

            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn as_subclass<D>(&self) -> &D
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::static_assert_subclass!(
                    <D::Class as $crate::class::ClassImpl>::Vtable,
                    <Self as $crate::class::ClassImpl>::Vtable
                );
                $crate::ptr::RcDyn::as_subclass::<D::Class>(self._as_inner()).into()
            }

            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub fn try_as_subclass<D>(&self) -> Option<&D>
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::static_assert_subclass!(
                    <D::Class as $crate::class::ClassImpl>::Vtable,
                    <Self as $crate::class::ClassImpl>::Vtable
                );
                $crate::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner()).map(Into::into)
            }

            /// Cast the `CRc` to its subclass `D`.
            ///
            /// # Safety
            /// `D` must be a superclass of `D`.
            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub unsafe fn to_subclass_unchecked<D>(&self) -> D
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::static_assert_subclass!(
                    <D::Class as $crate::class::ClassImpl>::Vtable,
                    <Self as $crate::class::ClassImpl>::Vtable
                );
                unsafe { $crate::ptr::RcDyn::as_subclass_unchecked::<D::Class>(self._as_inner()) }
                    .clone()
                    .into()
            }

            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            #[track_caller]
            pub fn to_subclass<D>(&self) -> D
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::static_assert_subclass!(
                    <D::Class as $crate::class::ClassImpl>::Vtable,
                    <Self as $crate::class::ClassImpl>::Vtable
                );
                $crate::ptr::RcDyn::as_subclass::<D::Class>(self._as_inner())
                    .clone()
                    .into()
            }

            /// Cast the `CRc` to its subclass `D`.
            #[inline]
            pub fn try_to_subclass<D>(&self) -> Option<D>
            where
                D: $crate::class::ClassRc,
                for<'a> &'a D: From<&'a $crate::ptr::RcDyn<D::Class>>,
            {
                $crate::static_assert_subclass!(
                    <D::Class as $crate::class::ClassImpl>::Vtable,
                    <Self as $crate::class::ClassImpl>::Vtable
                );
                $crate::ptr::RcDyn::try_as_subclass::<D::Class>(self._as_inner())
                    .cloned()
                    .map(Into::into)
            }

            #[inline]
            pub const fn ty(&self) -> $crate::vtable::Type {
                self.0.vtable().ty()
            }

            #[inline]
            pub fn as_ptr(this: &Self) -> $crate::prelude::CPtr<self::$class> {
                $crate::ptr::RcDyn::as_ptr(this._as_inner())
            }

            #[inline]
            pub fn is_subtype_of<C: $crate::class::ClassVtable>(&self) -> bool {
                self.ty().is_subtype_of(C::TYPE)
            }
            #[inline]
            pub fn is_subclass_of<C: $crate::class::ClassVtable>(&self) -> bool {
                self.ty().is_subclass_of(C::TYPE)
            }

            #[inline]
            pub fn is_subtype_of_ty(&self, ty: $crate::vtable::Type) -> bool {
                self.ty().is_subtype_of(ty)
            }
            #[inline]
            pub fn is_subclass_of_ty(&self, ty: $crate::vtable::Type) -> bool {
                self.ty().is_subclass_of(ty)
            }
        }

        impl<V> self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            #[inline]
            pub fn to_impl<A: $crate::class::ClassImpl>(&self) -> A
            where
                Self: $crate::class::HasImpl<A>,
            {
                $crate::class::HasImpl::to_impl(self)
            }
        }

        impl<V> self::$class<$crate::ptr::WeakDyn<self::$class>, V> {
            #[inline]
            pub fn to_impl<A: $crate::class::ClassImpl>(&self) -> A
            where
                Self: $crate::class::HasImpl<A>,
            {
                $crate::class::HasImpl::to_impl(self)
            }
        }

        impl<V> self::$class<$crate::ptr::WeakDyn<self::$class>, V> {
            #[inline]
            pub fn upgrade(&self) -> Option<self::$class<$crate::ptr::RcDyn<self::$class>, V>> {
                $crate::ptr::WeakDyn::upgrade(self._as_inner()).map(self::$class::_from_inner)
            }

            #[inline]
            pub fn strong_count(&self) -> usize {
                $crate::ptr::WeakDyn::strong_count(self._as_inner())
            }

            #[inline]
            pub fn weak_count(&self) -> usize {
                $crate::ptr::WeakDyn::weak_count(self._as_inner())
            }

            #[inline]
            pub const fn ty(&self) -> $crate::vtable::Type {
                self.0.vtable().ty()
            }

            #[inline]
            pub fn is_subtype_of<C: $crate::class::ClassVtable>(&self) -> bool {
                self.ty().is_subtype_of(C::TYPE)
            }
            #[inline]
            pub fn is_subclass_of<C: $crate::class::ClassVtable>(&self) -> bool {
                self.ty().is_subclass_of(C::TYPE)
            }

            #[inline]
            pub fn is_subtype_of_ty(&self, ty: $crate::vtable::Type) -> bool {
                self.ty().is_subtype_of(ty)
            }
            #[inline]
            pub fn is_subclass_of_ty(&self, ty: $crate::vtable::Type) -> bool {
                self.ty().is_subclass_of(ty)
            }
        }

        /*
        impl $crate::class::VtableSubtype<vtable::$class> for vtable::$class {
            fn upcast(&self) -> &vtable::$class {
                self
            }
        }
        impl $crate::class::Subtype<self::$class> for self::$class {}
        impl<V> $crate::class::Subtype<self::$class<$crate::ptr::RcDyn<self::$class>, V>>
            for self::$class<$crate::ptr::RcDyn<self::$class>, V>
        {
        }
        impl<V> $crate::class::Subtype<self::$class<$crate::ptr::WeakDyn<self::$class>, V>>
            for self::$class<$crate::ptr::WeakDyn<self::$class>, V>
        {
        }
        impl $crate::class::VtableSupertype<vtable::$class> for $crate::prelude::CVtable<$class> {
            unsafe fn downcast_unchecked(&self) -> &vtable::$class {
                self
            }
        }
        impl $crate::class::Supertype<self::$class> for self::$class {}
        impl<V> $crate::class::Supertype<self::$class<$crate::ptr::RcDyn<self::$class>, V>>
            for self::$class<$crate::ptr::RcDyn<self::$class>, V>
        {
        }
        impl<V> $crate::class::Supertype<self::$class<$crate::ptr::WeakDyn<self::$class>, V>>
            for self::$class<$crate::ptr::WeakDyn<self::$class>, V>
        {
        }
        */
    };
    (impl ClassImpl for #[class] $class:ident) => {
        impl<T, V> $crate::class::ClassImpl for self::$class<T, V> {
            type DataBase = data::$class;
            type Data = data::$class;
            type VtableBase = vtable::$class;
            type Vtable = vtable::$class;
            type VtableOpt = vtable::opt::$class;
        }
        impl $crate::class::ClassData for data::$class {}
        unsafe impl $crate::class::ClassVtable for vtable::$class {}
        impl $crate::class::ClassVtableOpt for vtable::opt::$class {
            type VtableBase = vtable::$class;
            type Vtable = vtable::$class;
        }
    };
    (impl ClassImpl for #[mixin] $class:ident) => {
        impl<T, V> $crate::class::ClassImpl for self::$class<T, V> {
            type DataBase = data::$class;
            type Data = $crate::vtable::MixinData<data::$class>;
            type VtableBase = vtable::$class;
            type Vtable = $crate::vtable::MixinVtable<vtable::$class>;
            type VtableOpt = vtable::opt::$class;
        }
        impl $crate::class::ClassVtableOpt for vtable::opt::$class {
            type VtableBase = vtable::$class;
            type Vtable = $crate::vtable::MixinVtable<vtable::$class>;
        }
        impl<T, V> $crate::class::MixinClassImpl for self::$class<T, V> {
            type DataWithoutSuper = data::$class;
            type VtableWithoutSuper = vtable::$class;
            // type InstanceData = data::$class;
            // type InstanceVtable = vtable::$class;
        }
    };
}

#[macro_export]
macro_rules! _def_concrete_class {
    ($class:ident) => {
        static VTABLE: $crate::vtable::VtableWithMixinHeader<
            vtable::$class,
            { vtable::$class::MIXIN_HEADER_ENTRIES },
        > = {
            let mut vtable =
                $crate::vtable::MaybeUninitVtableWithMixinHeader::new(vtable::opt::$class::DEFAULT);
            vtable::opt::$class::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(::core::option::Option::None, 0);
            vtable::opt::$class::init(vtable_opt);
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            $crate::vtable::VtableWithMixinHeader::new(headers, vtable_opt.assert_init())
        };
        unsafe impl $crate::class::ConcreteClass for self::$class {
            const VTABLE: ::core::ptr::NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }
        impl self::$class {
            pub const fn vtable<'a>() -> &'a $crate::vtable::VtableWithMixinHeader<
                vtable::$class,
                { vtable::$class::MIXIN_HEADER_ENTRIES },
            > {
                &VTABLE
            }
        }
    };
}

#[macro_export]
macro_rules! _def_mixin {
    ($class:ident $(on $( #[$kind:ident] $ons:ident),* )? $( implements $($impls:ident),* )?) => {
        // impl $crate::class::ClassData for data::$class<$crate::vtable::IgnoreSuper> {
        //     type Vtable = vtable::$class<$crate::vtable::IgnoreSuper>;
        //     type VtableOpt = vtable::opt::$class<$crate::vtable::IgnoreSuper>;
        // }
        // unsafe impl $crate::class::ClassVtable for vtable::$class<$crate::vtable::IgnoreSuper>
        // {
        //     type Data = data::$class<$crate::vtable::IgnoreSuper>;
        //     type Opt = vtable::opt::$class<$crate::vtable::IgnoreSuper>;
        //     type VtableWithMixinHeader = vtable::$class<$crate::vtable::MixinVtableHeader>;
        // }
        // unsafe impl<V: $crate::class::ClassVtable> $crate::class::MixinVtable for vtable::mixin::$class<V>
        // {
        //     type VtableIgnoreSuper = vtable::$class<$crate::vtable::IgnoreSuper>;
        // }

        // impl $crate::class::MixinVtable
        //     for vtable::$class<$crate::prelude::CVtable<$super>>
        // {
        //     type VtableIgnoreSuper = vtable::$class<$crate::vtable::IgnoreSuper>;
        // }

        impl self::$class<$crate::ptr::RcDyn<self::$class>> {
            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub fn mixin_try_downcast<S>(&self) -> Option<S>
            where
                S: $crate::class::IsClass + From<$crate::ptr::RcDyn<S::Class>>,
            {
                $crate::ptr::RcDyn::mixin_try_downcast::<S::Class>(self.clone()._into_inner())
                    .map(Into::into)
            }

            #[inline]
            #[track_caller]
            pub fn mixin_downcast<S>(&self) -> S
            where
                S: $crate::class::IsClass + From<$crate::ptr::RcDyn<S::Class>>,
            {
                $crate::ptr::RcDyn::mixin_downcast::<S::Class>(self.clone()._into_inner()).into()
            }

            #[inline]
            #[cfg_attr(debug_assertions, track_caller)]
            pub unsafe fn mixin_downcast_unchecked<S>(&self) -> S
            where
                S: $crate::class::IsClass + From<$crate::ptr::RcDyn<S::Class>>,
            {
                unsafe {
                    $crate::ptr::RcDyn::mixin_downcast_unchecked::<S::Class>(self.clone()._into_inner())
                } .into()
            }

            #[inline]
            pub fn mixin_to_impl<A: $crate::class::ClassImpl>(&self) -> A
            where
                Self: $crate::class::MixinHasImpl<A>,
            {
                $crate::class::MixinHasImpl::to_impl(self)
            }

            pub fn to_mixin(&self) -> Self {
                self.clone()
            }
        }

        impl self::$class<$crate::ptr::WeakDyn<self::$class>> {
            #[inline]
            pub fn mixin_to_impl<A: $crate::class::ClassImpl>(&self) -> A
            where
                Self: $crate::class::MixinHasImpl<A>,
            {
                $crate::class::MixinHasImpl::to_impl(self)
            }
        }


        $( $( $crate::_def_mixin!(@ $class : #[$kind] $ons );)* )?
        $( $( $crate::_def_mixin!(@ $class : #[class] $impls );)* )?
    };
    (@ $class:ident : #[class] $impl:ident ) => {
        $crate::_def_mixin!(@ $class : $impl );
    };
    (@ $class:ident : #[mixin] $impl:ident ) => {
        $crate::_def_mixin!(@ $class : $impl );
    };
    (@ $class:ident : $impl:ident ) => {
        unsafe impl $crate::class::MixinVtableHasImpl<$impl>
            for vtable::$class
        {
        }
        unsafe impl<V> $crate::class::MixinVtableHasImpl<$impl<$crate::ptr::RcDyn<$impl>, V>>
            for vtable::$class
        {
        }
        unsafe impl<V> $crate::class::MixinVtableHasImpl<$impl<$crate::ptr::WeakDyn<$impl>, V>>
            for vtable::$class
        {
        }

        impl $crate::class::MixinHasImpl<$impl> for self::$class {
            #[allow(unreachable_code)]
            fn to_impl(&self) -> $impl {
                match self.0 {}
            }
        }

        impl<V> $crate::class::MixinHasImpl<$impl<$crate::ptr::RcDyn<$impl>, V>>
            for self::$class<$crate::ptr::RcDyn<self::$class>, V>
        {
            fn to_impl(&self) -> $impl<$crate::ptr::RcDyn<$impl>, V> {
                $crate::ptr::RcDyn::mixin_into_impl::<$impl>(self.clone()._into_inner()).into()
            }
        }
        impl<V> $crate::class::MixinHasImpl<$impl<$crate::ptr::WeakDyn<$impl>, V>>
            for self::$class<$crate::ptr::WeakDyn<self::$class>, V>
        {
            fn to_impl(&self) -> $impl<$crate::ptr::WeakDyn<$impl>, V> {
                $crate::ptr::WeakDyn::mixin_into_impl::<$impl>(self.clone()._into_inner()).into()
            }
        }
    };
}

#[macro_export]
macro_rules! _def_mixin_instance {
    ($class:ident : $super:ident with $mixin:ident) => {
        impl<V> self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            #[cfg_attr(debug_assertions, track_caller)]
            pub fn to_mixin(&self) -> super::$mixin<$crate::ptr::RcDyn<super::$mixin>> {
                unsafe {
                    $crate::ptr::RcDyn::into_mixin_unchecked::<$mixin>(
                        self.clone()._into_inner(),
                        vtable::$class::TYPE.as_mixin_instance_unchecked(),
                    )
                }
                .into()
            }
        }
        // unsafe impl $crate::class::MixinVtableHasImpl<$crate::prelude::CVtable<self::$class>>
        //     for vtable::$class
        // {
        // }

        unsafe impl $crate::class::MixinVtableHasImpl<self::$class> for vtable::$class {}
        unsafe impl<V>
            $crate::class::MixinVtableHasImpl<self::$class<$crate::ptr::RcDyn<self::$class>, V>>
            for vtable::$class
        {
        }
        unsafe impl<V>
            $crate::class::MixinVtableHasImpl<self::$class<$crate::ptr::WeakDyn<self::$class>, V>>
            for vtable::$class
        {
        }

        impl $crate::class::MixinWith<super::$mixin> for self::$super {
            type Instance<T, V> = self::$class<T, V>;
        }
    };
}

#[macro_export]
macro_rules! mixin {
    ($t:ident $(,)?) => {
        $t<$crate::class::ClassMarker, $crate::class::Virtual>
    };
    (<$T:ident, $V:ident> $t:ident $(,)?) => {
        $t<$T, $V>
    };
    ($(<$T:ident, $V:ident>)? $t:ident $(,$ts:ident)* $(,)?) => {
        $crate::mixin!(@Rev($t,) $(<$T, $V>)? $($ts),*)
    };
    (@Rev($($rs:ident,)*) $(<$T:ident, $V:ident>)? $t:ident $(,$ts:ident)* $(,)?) => {
        $crate::mixin!(@Rev($t, $($rs,)*) $(<$T, $V>)? $($ts,)*)
    };
    (@Rev($t:ident,)) => {
        $t<$crate::class::ClassMarker, $crate::class::Virtual>
    };
    (@Rev($t:ident,) <$T:ident, $V:ident> ) => {
        $t<$T, $V>
    };
    (@Rev($m:ident, $($ts:ident,)*)) => {
        <$crate::mixin!(@Rev($($ts,)*)) as $crate::class::MixinWith<$m>>
            ::Instance<$crate::class::ClassMarker, $crate::class::Virtual>
    };
    (@Rev($m:ident, $($ts:ident,)*) <$T:ident, $V:ident> ) => {
        <$crate::mixin!(@Rev($($ts,)*)) as $crate::class::MixinWith<$m>>::Instance<$T, $V>
    };
}

/*
#[macro_export]
macro_rules! _def_class_reflexive {
    ($class:ident) => {
        impl $crate::class::VtableSubtype<vtable::$class> for vtable::$class {
            fn upcast(&self) -> &vtable::$class {
                self
            }
        }
        impl $crate::class::Subtype<$class> for $class {}
        impl<V> $crate::class::Subtype<$class<$crate::ptr::RcDyn<$class>, V>>
            for $class<$crate::ptr::RcDyn<$class>, V>
        {
        }
        impl<V> $crate::class::Subtype<$class<$crate::ptr::WeakDyn<$class>, V>>
            for $class<$crate::ptr::WeakDyn<$class>, V>
        {
        }
        impl $crate::class::VtableSupertype<vtable::$class> for vtable::$class {
            unsafe fn downcast_unchecked(&self) -> &vtable::$class {
                self
            }
        }
        impl $crate::class::Supertype<$class> for $class {}
        impl<V> $crate::class::Supertype<$class<$crate::ptr::RcDyn<$class>, V>>
            for $class<$crate::ptr::RcDyn<$class>, V>
        {
        }
        impl<V> $crate::class::Supertype<$class<$crate::ptr::WeakDyn<$class>, V>>
            for $class<$crate::ptr::WeakDyn<$class>, V>
        {
        }
    };
}
*/

#[macro_export]
macro_rules! _def_class_extends {
    /*
    // no `HasSuper`, only `Hassuperclass`
    ($class:ident : Object) => {
        unsafe impl $crate::class::Hassuperclass<$crate::object::Object> for $class {
            #[allow(unreachable_code)]
            fn into_superclass(self) -> $crate::object::Object {
                match self._into_inner() {}
            }
        }
        unsafe impl<V>
            $crate::class::Hassuperclass<
                $crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V>,
            > for $class<$crate::ptr::RcDyn<$class>, V>
        {
            fn into_superclass(
                self,
            ) -> $crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V> {
                // any class is a subclass of `Object`
                unsafe { ::core::mem::transmute(self) }
            }
        }
        unsafe impl<V>
            $crate::class::Hassuperclass<
                $crate::object::Object<$crate::ptr::WeakDyn<$crate::object::Object>, V>,
            > for $class<$crate::ptr::WeakDyn<$class>, V>
        {
            fn into_superclass(
                self,
            ) -> $crate::object::Object<$crate::ptr::WeakDyn<$crate::object::Object>, V> {
                // any class is a subclass of `Object`
                unsafe { ::core::mem::transmute(self) }
            }
        }
        unsafe impl $crate::class::DataHassuperclass<$crate::object::ObjectData> for data::$class {}
        unsafe impl $crate::class::VtableHassuperclass<$crate::object::ObjectVtable>
            for vtable::$class
        {
        }
        unsafe impl $crate::class::VtableOptHassuperclass<$crate::object::ObjectVtableOpt>
            for vtable::opt::$class
        {
        }

        impl<V> From<$class<$crate::ptr::RcDyn<$class>, V>>
            for $crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V>
        {
            fn from(
                class: $class<$crate::ptr::RcDyn<$class>, V>,
            ) -> $crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V> {
                class.into_superclass()
            }
        }

        impl<V> TryFrom<$crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V>>
            for $class<$crate::ptr::RcDyn<$class>, V>
        {
            type Error = $crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V>;

            fn try_from(
                class: $crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V>,
            ) -> ::core::result::Result<$class<$crate::ptr::RcDyn<$class>, V>, Self::Error> {
                class
                    .try_as_subclass()
                    .cloned()
                    .ok_or_else(|| class.clone())
            }
        }

        impl $class<$crate::ptr::RcDyn<$class>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &$crate::object::Object<
                $crate::ptr::RcDyn<$crate::object::Object>,
                $crate::class::NonVirtual,
            > {
                self.as_non_virtual().as_super()
            }
        }

        impl<V> $class<$crate::ptr::RcDyn<$class>, V> {
            #[inline]
            pub fn as_super(
                &self,
            ) -> &$crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V> {
                // any class is a subclass of `Object`
                unsafe { &*::core::ptr::from_ref(self).cast() }
            }
            #[inline]
            pub fn into_super(
                self,
            ) -> $crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V> {
                // any class is a subclass of `Object`
                unsafe { ::core::mem::transmute(self) }
            }
        }

        impl<V> $class<$crate::ptr::WeakDyn<$class>, V> {
            #[inline]
            pub fn as_super(
                &self,
            ) -> &$crate::object::Object<$crate::ptr::WeakDyn<$crate::object::Object>, V> {
                // any class is a subclass of `Object`
                unsafe { &*::core::ptr::from_ref(self).cast() }
            }
            #[inline]
            pub fn into_super(
                self,
            ) -> $crate::object::Object<$crate::ptr::WeakDyn<$crate::object::Object>, V> {
                // any class is a subclass of `Object`
                unsafe { ::core::mem::transmute(self) }
            }
        }

        impl $crate::class::VtableSubtype<$crate::object::ObjectVtable> for vtable::$class {
            fn upcast(&self) -> &$crate::prelude::CVtable<$crate::object::Object> {
                // any class is a subclass of `Object`
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
        impl $crate::class::Subtype<$crate::object::Object> for $class {}
        impl<V>
            $crate::class::Subtype<
                $crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V>,
            > for $class<$crate::ptr::RcDyn<$class>, V>
        {
        }
        impl<V>
            $crate::class::Subtype<
                $crate::object::Object<$crate::ptr::WeakDyn<$crate::object::Object>>,
            > for $class<$crate::ptr::WeakDyn<$class>, V>
        {
        }
        impl $crate::class::VtableSupertype<vtable::$class> for $crate::object::ObjectVtable {
            unsafe fn downcast_unchecked(&self) -> &vtable::$class {
                // any class is a subclass of `Object`
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            fn try_downcast(&self) -> Option<&vtable::$class> {
                // any class is a subclass of `Object`
                Some(unsafe { self.downcast_unchecked() })
            }
        }
        impl $crate::class::Supertype<$class> for $crate::object::Object {}
        impl<V> $crate::class::Supertype<$class<$crate::ptr::RcDyn<$class>, V>>
            for $crate::object::Object<$crate::ptr::RcDyn<$crate::object::Object>, V>
        {
        }
        impl<V> $crate::class::Supertype<$class<$crate::ptr::WeakDyn<$class>, V>>
            for $crate::object::Object<$crate::ptr::WeakDyn<$crate::object::Object>, V>
        {
        }

        impl<V> From<$class<$crate::ptr::WeakDyn<$class>, V>>
            for $crate::object::Object<$crate::ptr::WeakDyn<$crate::object::Object>, V>
        {
            fn from(
                class: $class<$crate::ptr::WeakDyn<$class>, V>,
            ) -> $crate::object::Object<$crate::ptr::WeakDyn<$crate::object::Object>, V> {
                class.into_super()
            }
        }

        impl vtable::$class {
            pub const fn as_super(&self) -> &$crate::prelude::CVtable<$crate::object::Object> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
            pub const fn as_mut_super(&mut self) -> &mut $crate::prelude::CVtable<$crate::object::Object> {
                unsafe { &mut *core::ptr::from_mut(self).cast() }
            }
        }
    };
    */
    ($class:ident : $super:ident (mixin_instance)) => {
        unsafe impl $crate::class::HasSuper for self::$class {
            type Super = $super;
            fn into_super(self) -> Self::Super {
                #[allow(unreachable_code)]
                match self._into_inner() {}
            }
        }
        unsafe impl<V> $crate::class::HasSuper
            for self::$class<$crate::ptr::RcDyn<self::$class>, V>
        {
            type Super = $super<$crate::ptr::RcDyn<$super>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        impl<V> ::core::ops::Deref for self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            type Target = $super<$crate::ptr::RcDyn<$super>, V>;
            fn deref(&self) -> &Self::Target {
                self.as_super()
            }
        }
        unsafe impl<V> $crate::class::HasSuper
            for self::$class<$crate::ptr::WeakDyn<self::$class>, V>
        {
            type Super = $super<$crate::ptr::WeakDyn<$super>, V>;
            fn into_super(self) -> Self::Super {
                self.into_super()
            }
        }
        unsafe impl $crate::class::DataHasSuper for data::$class {
            type SuperData = $crate::prelude::CData<$super>;
        }
        unsafe impl $crate::class::VtableHasSuper for vtable::$class {
            type SuperVtable = $crate::prelude::CVtable<$super>;
        }

        impl<V> self::$class<$crate::ptr::RcDyn<self::$class>, V> {
            #[inline]
            pub fn as_super(&self) -> &$super<$crate::ptr::RcDyn<$super>, V> {
                $crate::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> $super<$crate::ptr::RcDyn<$super>, V> {
                $super::_from_inner($crate::ptr::RcDyn::into_super(self.clone()._into_inner()))
            }
            #[inline]
            pub fn into_super(self) -> $super<$crate::ptr::RcDyn<$super>, V> {
                $super::_from_inner($crate::ptr::RcDyn::into_super(self._into_inner()))
            }
        }

        impl self::$class<$crate::ptr::RcDyn<self::$class>> {
            #[inline]
            pub fn delegate_super(
                &self,
            ) -> &$super<$crate::ptr::RcDyn<$super>, $crate::class::NonVirtual> {
                self.as_non_virtual().as_super()
            }
        }

        impl<V> self::$class<$crate::ptr::WeakDyn<self::$class>, V> {
            #[inline]
            pub fn as_super(&self) -> &$super<$crate::ptr::WeakDyn<$super>, V> {
                $crate::class::HasSuper::as_super(self)
            }
            #[inline]
            pub fn to_super(&self) -> $super<$crate::ptr::WeakDyn<$super>, V> {
                $super::_from_inner($crate::ptr::WeakDyn::into_super(self.clone()._into_inner()))
            }
            #[inline]
            pub fn into_super(self) -> $super<$crate::ptr::WeakDyn<$super>, V> {
                $super::_from_inner($crate::ptr::WeakDyn::into_super(self._into_inner()))
            }
        }

        impl vtable::$class {
            pub const fn as_super(&self) -> &$crate::prelude::CVtable<$super> {
                unsafe { &*core::ptr::from_ref(self).cast() }
            }
        }
    };
    ($class:ident : $super:ident) => {
        $crate::_def_class_extends!($class : $super (mixin_instance));

        // do not impl `From`/`TryFrom` for mixin classes to avoid orphan impls
        impl<V> From<self::$class<$crate::ptr::RcDyn<self::$class>, V>>
            for $super<$crate::ptr::RcDyn<$super>, V>
        {
            fn from(
                class: self::$class<$crate::ptr::RcDyn<self::$class>, V>,
            ) -> $super<$crate::ptr::RcDyn<$super>, V> {
                class.into_super()
            }
        }

        impl<V> TryFrom<$super<$crate::ptr::RcDyn<$super>, V>>
            for self::$class<$crate::ptr::RcDyn<self::$class>, V>
        {
            type Error = $super<$crate::ptr::RcDyn<$super>, V>;

            fn try_from(
                class: $super<$crate::ptr::RcDyn<$super>, V>,
            ) -> ::core::result::Result<
                self::$class<$crate::ptr::RcDyn<self::$class>, V>,
                Self::Error,
            > {
                class
                    .try_as_subclass()
                    .cloned()
                    .ok_or_else(|| class.clone())
            }
        }

        impl<V> From<self::$class<$crate::ptr::WeakDyn<self::$class>, V>>
            for $super<$crate::ptr::WeakDyn<$super>, V>
        {
            fn from(
                class: self::$class<$crate::ptr::WeakDyn<self::$class>, V>,
            ) -> $super<$crate::ptr::WeakDyn<$super>, V> {
                class.into_super()
            }
        }

    };
}

#[macro_export]
macro_rules! _def_class_impl {
    ($class:ident : $impl:ident) => {
        impl $crate::class::HasImpl<$impl> for self::$class {
            #[allow(unreachable_code)]
            fn to_impl(&self) -> $impl {
                match self.0 {}
            }
        }
        impl<V> $crate::class::HasImpl<$impl<$crate::ptr::RcDyn<$impl>, V>>
            for self::$class<$crate::ptr::RcDyn<self::$class>, V>
        {
            fn to_impl(&self) -> $impl<$crate::ptr::RcDyn<$impl>, V> {
                $crate::ptr::RcDyn::into_impl::<$impl>(self.clone()._into_inner()).into()
            }
        }
        impl<V> $crate::class::HasImpl<$impl<$crate::ptr::WeakDyn<$impl>, V>>
            for self::$class<$crate::ptr::WeakDyn<self::$class>, V>
        {
            fn to_impl(&self) -> $impl<$crate::ptr::WeakDyn<$impl>, V> {
                $crate::ptr::WeakDyn::into_impl::<$impl>(self.clone()._into_inner()).into()
            }
        }
        unsafe impl $crate::class::VtableHasImpl<$impl> for vtable::$class {
            const OFFSET: usize = ::core::mem::offset_of!(vtable::$class, $impl);
        }
        unsafe impl<V> $crate::class::VtableHasImpl<$impl<$crate::ptr::RcDyn<$impl>, V>>
            for vtable::$class
        {
            const OFFSET: usize = ::core::mem::offset_of!(vtable::$class, $impl);
        }
        unsafe impl<V> $crate::class::VtableHasImpl<$impl<$crate::ptr::WeakDyn<$impl>, V>>
            for vtable::$class
        {
            const OFFSET: usize = ::core::mem::offset_of!(vtable::$class, $impl);
        }

        impl<V> From<self::$class<$crate::ptr::RcDyn<self::$class>, V>>
            for super::$impl<$crate::ptr::RcDyn<super::$impl>, V>
        {
            fn from(
                class: self::$class<$crate::ptr::RcDyn<self::$class>, V>,
            ) -> super::$impl<$crate::ptr::RcDyn<super::$impl>, V> {
                $crate::class::HasImpl::to_impl(&class)
            }
        }

        impl TryFrom<super::$impl<$crate::ptr::RcDyn<super::$impl>>>
            for self::$class<$crate::ptr::RcDyn<self::$class>>
        {
            type Error = super::$impl<$crate::ptr::RcDyn<super::$impl>>;

            fn try_from(
                class: $impl<$crate::ptr::RcDyn<$impl>>,
            ) -> ::core::result::Result<$class<$crate::ptr::RcDyn<$class>>, Self::Error> {
                class
                    .try_downcast::<Self, Self>()
                    .ok_or_else(|| class.clone())
            }
        }

        impl<V> From<self::$class<$crate::ptr::WeakDyn<self::$class>, V>>
            for super::$impl<$crate::ptr::WeakDyn<super::$impl>, V>
        {
            fn from(
                class: self::$class<$crate::ptr::WeakDyn<self::$class>, V>,
            ) -> super::$impl<$crate::ptr::WeakDyn<super::$impl>, V> {
                $crate::class::HasImpl::to_impl(&class)
            }
        }
    };
}

#[macro_export]
macro_rules! _mod_uses {
    (mod data) => {
        use super::*;

        use $crate::get_set::{New, NewCopy};
        use $crate::prelude::*;
        use $crate::ptr::RcDyn;
    };
    (mod vtable) => {
        use super::*;
        use $crate::class::{
            ClassVtable, ClassVtableBase, NonVirtual, Virtual, VtableHasImpl, VtableHasSuper,
        };
        use $crate::prelude::*;
        use $crate::vtable::{MixinVtableHeader, TypeInfo, VtableHeader};
    };
    (mod offset) => {
        use $crate::vtable::{MixinInstanceType, MixinVtableHeader};
    };
    (mod vtable::opt) => {
        use super::*;
        use $crate::class::{ClassVtable, NonVirtual, Virtual, VtableHasImpl, VtableHasSuper};
        use $crate::prelude::*;
        use $crate::vtable::{MixinVtableHeader, VtableHeaderOpt};
    };
    (mod class $class:ident) => {
        use super::*;
        use ::core::ptr::NonNull;
        use $crate::class::{ConcreteClass, NonVirtual, Virtual};
        use $crate::get_set::{GetSet, GetSetCopy};
        use $crate::prelude::*;
        use $crate::ptr::RcDyn;
        use $crate::vtable::{
            MaybeUninitVtableWithMixinHeader, VtableHeader, VtableWithMixinHeader,
        };
    };
}

#[macro_export]
macro_rules! static_assert_subclass {
    ($sub:ty, $super:ty $(,$lit:literal)? ) => {
        // #[cfg(debug_assertions)]
        let _ = {
            use $crate::class::ClassVtable;

            struct Assert<C: ClassVtable, A: ClassVtable>(core::marker::PhantomData<(C, A)>);
            impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                const CHECK: () = if !C::TYPE.const_is_subclass_of(A::TYPE) {
                    panic!(concat!("not a subclass" $(, $lit)?))
                };
            }
            Assert::<$sub, $super>::CHECK
        };
    };
}

#[macro_export]
macro_rules! static_assert_subtype {
    ($sub:ty, $super:ty $(,$lit:literal)? ) => {
        // #[cfg(debug_assertions)]
        let _ = {
            use $crate::class::ClassVtable;

            struct Assert<C: ClassVtable, A: ClassVtable>(core::marker::PhantomData<(C, A)>);
            impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                const CHECK: () = if C::KIND.is_mixin() && A::TYPE.const_eq($crate::object::Object::TYPE) {
                    // mixins can always be upcasted to `Object`
                } else if !C::TYPE.const_is_subtype_of(A::TYPE) {
                    panic!(concat!("not a subtype" $(, $lit)?))
                };
            }
            Assert::<$sub, $super>::CHECK
        };
    };
}

#[macro_export]
macro_rules! class_const_eq {
    ($lhs:ty, $rhs:ty $(,)?) => {{
        use $crate::class::ClassVtable;

        struct Eq<C1: ClassVtable, C2: ClassVtable>(core::marker::PhantomData<(C1, C2)>);
        impl<C1: ClassVtable, C2: ClassVtable> Eq<C1, C2> {
            const EQUAL: bool = C1::TYPE.const_eq(C2::TYPE);
        }
        Eq::<$lhs, $rhs>::EQUAL
    }};
}

#[macro_export]
macro_rules! static_assert_subclass_or_mixin_instance {
    ($sub:ty, $super:ty $(,$lit:literal)? ) => {
        // #[cfg(debug_assertions)]
        let _ = {
            use $crate::class::ClassVtable;

            struct Assert<C: ClassVtable, A: ClassVtable>(core::marker::PhantomData<(C, A)>);
            impl<C: ClassVtable, A: ClassVtable> Assert<C, A> {
                const CHECK: () = if !matches!(C::TYPE.const_offset_of(A::TYPE), Some($crate::vtable::OffsetResult { offset: 0, .. })) {
                    panic!(concat!("not a subclass" $(, $lit)?))
                };
            }
            Assert::<$sub, $super>::CHECK
        };
    };
}

#[macro_export]
macro_rules! static_assert_mixin_header_entries {
    ($vtable:ty, $entries:expr) => {
        let _ = {
            use $crate::class::ClassVtable;

            struct Assert<V: ClassVtable, const N: usize>(::core::marker::PhantomData<V>);
            impl<V: ClassVtable, const N: usize> Assert<V, N> {
                const _ASSERT: () = {
                    if !V::KIND.is_mixin() {
                        panic!("not a mixin");
                    }
                    if N != V::MIXIN_HEADER_ENTRIES {
                        panic!("Invalid number of mixin headers");
                    }
                };
            }
            Assert::<$vtable, { $entries }>(::core::marker::PhantomData)
        };
    };
}

#[macro_export]
macro_rules! _match_offset {
    ($($pat:literal),* $(,)?) => {
        match C::TYPE.const_offset_of(A::TYPE) {
            None => panic!("not a subtype"),
            Some($crate::vtable::OffsetResult { offset, .. }) if offset == OFFSET => (),
            $( Some($crate::vtable::OffsetResult { offset: $pat, .. }) => {
                panic!(concat!("offset mismatched, expected: ", $pat))
            }, )*
            _ => panic!("offset mismatched"),
        }
    };
}

#[macro_export]
macro_rules! static_assert_subtype_with_offset {
    ($sub:ty, $super:ty, $offset:expr) => {
        #[cfg(debug_assertions)]
        let _ = {
            use $crate::class::ClassVtable;

            struct Assert<C: ClassVtable, A: ClassVtable, const OFFSET: usize>(
                core::marker::PhantomData<(C, A)>,
            );

            impl<C: ClassVtable, A: ClassVtable, const OFFSET: usize> Assert<C, A, OFFSET> {
                const CHECK: () = {
                    $crate::_match_offset!(
                        0x00, 0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58,
                        0x60, 0x68, 0x70, 0x78, 0x80, 0x88, 0x90, 0x98, 0xA0, 0xA8, 0xB0, 0xB8,
                        0xC0, 0xC8, 0xD0, 0xD8, 0xE0, 0xE8, 0xF0, 0xF8,
                    )
                };
            }
            Assert::<$sub, $super, { $offset }>::CHECK
        };
    };
}

#[macro_export]
macro_rules! assert_layout_eq {
    ($vtable:ty, $vtable_opt:ty, { $($fields:expr ),* $(,)? }) => {
        const _: () = { $(
            assert!(
                ::core::mem::size_of::<$vtable>() == ::core::mem::size_of::<$vtable_opt>(),
                concat!("size of ", stringify!($vtable), " != size of ", stringify!($vtable_opt)),
            );
            assert!(
                ::core::mem::offset_of!($vtable, $fields) == ::core::mem::offset_of!($vtable_opt, $fields),
                concat!(
                    "offset of ",
                    stringify!($vtable), "::", stringify!($fields),
                    " != offset of ",
                    stringify!($vtable_opt), "::", stringify!($fields),
                ),
            );
        )* };
    };
}
