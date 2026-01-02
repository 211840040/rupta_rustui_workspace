expand_test_case! {
    mixin {
        #[with(A, B)]
        mixin M on A implements I {
            struct {
                final z: usize = 2,
            }
            pub override fn A::f(&self) {
                super.f();
                println!("M::f, z = {}", self.get_z());
            }
            pub fn g() {
                println!("M::g");
            }
            pub fn h(&self) {
                println!("M::h, z = {}", self.get_z());
            }
        }
    } => {
        #[allow(unused_imports)]
        use _classes::M;
        #[allow(unused_imports)]
        use _classes::{A_M, B_M};

        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();
        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;

            #[allow(unused_imports)]
            pub(super) use _M::M;
            #[allow(unused_imports)]
            pub(super) use _M::{A_M, B_M};

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod _M {
                ::classes::_mod_uses! { mod class M }
                ::classes::_def_class! { mixin M }
                ::classes::_def_mixin! { M on #[class] A implements I }

                mod data {
                    ::classes::_mod_uses! { mod data }
                    #[repr(C)]
                    pub struct M {
                        pub(super) z: usize,
                    }
                    impl M {
                        pub(super) fn g() {
                            println!("M::g");
                        }
                    }
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct M {
                        pub h: fn(&::classes::prelude::CRc<Self>,),
                        pub I: ::classes::prelude::CVtable<I>,
                    }
                    impl M {
                        pub const fn debug_vtable_layout (&self , offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }
                    pub struct DebugVtableLayout <'a> {
                        this: &'a self::M ,
                        offset: usize ,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(M, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(M));
                            dbg.field("\'start" , &self.offset);
                            dbg.field(stringify!(h), &offset_of!(h));
                            dbg.field(stringify!(I), &self.this.I.debug_vtable_layout(offset_of!(I)));
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<M>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable::opt }
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct M {
                            pub h: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,)>,
                            pub I: ::classes::prelude::CVtableOpt<I>,
                        }
                    }
                    pub static TYPE: ::classes::vtable::TypeInfo<2usize> =
                        ::classes::vtable::TypeInfo::new_mixin::<super::M>(
                            [
                                ::classes::prelude::CVtable::<A>::TYPE,
                                ::classes::prelude::CVtable::<I>::TYPE,
                            ],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(M),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::M,
                    vtable::opt::M,
                    {h, I}
                }
                impl M<::classes::ptr::RcDyn<M>> {
                    #[inline]
                    pub fn f(&self,) {
                        { self.to_supertype::<::classes::prelude::CRc<A>>().f() }
                            .try_into()
                            .unwrap()
                    }
                    #[inline]
                    pub fn g() {
                        { data::M::g() }
                    }
                    #[inline]
                    pub fn h(&self,) {
                        { (self.0.vtable().vtable_without_super().h)(self,) }
                    }
                }
                impl M<::classes::ptr::RcDyn<M>> {
                    #[inline]
                    pub(in super::super) fn get_z(&self) -> &usize {
                        &self.0.vtable().data_without_super(&self.0).z
                    }
                    #[inline]
                    pub(in super::super) fn raw_get_z(&self) -> &usize {
                        &self.0.vtable().data_without_super(&self.0).z
                    }
                }
                macro_rules! M {
                    ($mod_name:ident, $class:ident, $super_ty:ty) => {
                        #[allow(unused_imports)]
                        pub use $mod_name::$class;

                        #[allow(non_snake_case)]
                        #[allow(non_camel_case_types)]
                        #[allow(unused_attributes)]
                        mod $mod_name {
                            ::classes::_mod_uses! { mod class $class }
                            ::classes::_def_class! { class $class }
                            type Super<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual> = $super_ty;
                            ::classes::_def_mixin_instance! { $class : Super with M }
                            ::classes::_def_class_extends! { $class : Super (mixin_instance) }
                            ::classes::_def_class_impl! { $class : I }
                            mod data {
                                ::classes::_mod_uses! { mod data }
                                pub(super) type Super = ::classes::prelude::CData<super::Super>;
                                #[repr(C)]
                                pub struct $class {
                                    pub(super) _super: Super,
                                    pub(super) z: usize,
                                }
                                impl $class {
                                    #[inline]
                                    pub fn _delegate_ctor<
                                        _S : ::classes::class::IsClass,
                                        _F : FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
                                    >(mut _self : ::classes::prelude::CRcUninit<Self>, new : _F) -> ::classes::prelude::CRc<Self>
                                    where
                                        ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                                        for <'a> &'a ::classes::prelude::CRc<_S>: From<
                                            &'a ::classes::ptr::RcDyn<
                                                <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
                                            >,
                                        >,
                                    {
                                        #[allow(unused_unsafe)] unsafe {
                                            ::core::ptr::write(&raw mut (*_self.as_mut_ptr()).z, 2,);
                                        }
                                        let _ = |Self { _super, z: _, } : Self| ();
                                        let should_delegate = !::classes::class_const_eq!(
                                            <_S::Class as ::classes::class::ClassImpl>::Vtable,
                                            ::classes::prelude::CVtable<Super>,
                                        );
                                        if should_delegate {
                                            let _self = Super::_delegate_ctor::<_S, _F>(_self.into_super(), new);
                                            unsafe { _self.into_subclass_unchecked::<::classes::prelude::CRc<Self>>() }
                                        } else {
                                            let _self = new(_self.into_superclass());
                                            unsafe {
                                                ::classes::class::ClassRc::into_subclass_unchecked::<::classes::prelude::CRc<Self>>(_self)
                                            }
                                        }
                                    }
                                    pub(super) fn f(_self: &::classes::prelude::CRc<Self>,) {
                                        { _self.delegate_super() }.f();
                                        println!("M::f, z = {}", _self.get_z());
                                    }
                                    pub(super) fn g() {
                                        println!("M::g");
                                    }
                                    pub(super) fn h(_self: &::classes::prelude::CRc<Self>,) {
                                        println!("M::h, z = {}", _self.get_z());
                                    }
                                }
                            }
                            mod vtable {
                                ::classes::_mod_uses! { mod vtable }
                                pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                                #[repr(C)]
                                #[derive(Clone, Copy)]
                                pub struct $class {
                                    pub(super) _super: Super,
                                    pub h: fn(&::classes::prelude::CRc<super::super::M>,),
                                    pub I: ::classes::prelude::CVtable<I>,
                                }
                                impl $class {
                                    pub const fn debug_vtable_layout (&self , offset: usize) -> self::DebugVtableLayout<'_> {
                                        self::DebugVtableLayout { this: self, offset }
                                    }
                                }

                                pub struct DebugVtableLayout <'a> {
                                    this: &'a self::$class ,
                                    offset: usize ,
                                }
                                impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                                    #[allow(unused_macros)]
                                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                        macro_rules! offset_of {
                                            ($field:ident) => {
                                                self.offset + ::core::mem::offset_of!($class, $field)
                                            };
                                        }
                                        let mut dbg = f.debug_struct(stringify!($class));
                                        dbg.field("\'start" , &self.offset);
                                        dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                                        dbg.field(stringify!(h), &offset_of!(h));
                                        dbg.field(stringify!(I), &self.this.I.debug_vtable_layout(offset_of!(I)));
                                        dbg.field("\'end", &(self.offset + ::core::mem::size_of::<$class>())) ;
                                        dbg.finish()
                                    }
                                }
                                pub(super) mod opt {
                                    ::classes::_mod_uses! { mod vtable :: opt }
                                    pub(in super::super) type Super = ::classes::prelude::CVtableOpt<super::super::Super>;
                                    #[repr(C)]
                                    #[derive(Clone, Copy)]
                                    #[derive(Default)]
                                    pub struct $class {
                                        pub(in super::super) _super: Super,
                                        pub h: ::core::option::Option<fn(&::classes::prelude::CRc<super::super::super::M>,)>,
                                        pub I: ::classes::prelude::CVtableOpt<I>,
                                    }
                                    impl $class {
                                        pub const DEFAULT: Self = Self {
                                            _super: Super::DEFAULT,
                                            h: ::core::option::Option::None,
                                            I: ::classes::prelude::CVtableOpt::<I>::DEFAULT,
                                        };
                                        pub const fn init_mixin_header(
                                            mixin_header: &mut [::core::mem::MaybeUninit<::classes::vtable::MixinVtableHeader>],
                                        ) {
                                            let (first, rest) = mixin_header
                                                .split_first_mut()
                                                .expect("mixin header is empty");
                                            Super::init_mixin_header(rest);
                                            first.write(
                                                ::classes::vtable::MixinVtableHeader::new::<super::$class>(
                                                    ::core::mem::size_of::<data::$class>()
                                                        - ::core::mem::size_of::<::classes::prelude::CDataBase<M>>(),
                                                    super::$class::MIXIN_HEADER_ENTRIES
                                                        * ::core::mem::size_of::<::classes::vtable::MixinVtableHeader>()
                                                        + ::core::mem::size_of::<vtable::$class>()
                                                        - ::core::mem::size_of::<::classes::prelude::CVtableBase<M>>(),
                                                )
                                            );
                                        }
                                        pub const fn init_header(&mut self, ty: ::core::option::Option<::classes::vtable::Type>, offset: usize) {
                                            let ty = match ty {
                                                ::core::option::Option::None => Self::TYPE,
                                                ::core::option::Option::Some(ty) => ty,
                                            };
                                            self._super.init_header(::core::option::Option::Some(ty), offset);
                                            self.I.init_header(::core::option::Option::None, offset + ::core::mem::offset_of!(::classes::prelude::CVtable<Self>, I),);
                                        }
                                        #[allow(unused_unsafe)]
                                        pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                                            ::classes::static_assert_subclass_or_mixin_instance!(
                                                super::$class,
                                                ::classes::prelude::CVtable<A>,
                                                ", we only support `mixin` to `on` a superclass yet"
                                            );
                                            Super::init(_self);
                                            {
                                                let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<A>>(_self);
                                                ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                                    &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                ).into());
                                                while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<A>>(_self, &mut offset) {
                                                    ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                    ).into());
                                                }
                                            }
                                            {
                                                let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<M>>(_self);
                                                ptr.h = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::h(
                                                    &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                ).into());
                                                while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<M>>(_self, &mut offset) {
                                                    ptr.h = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::h(
                                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                    ).into());
                                                }
                                            }
                                        }
                                        #[track_caller]
                                        pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                                            ::classes::prelude::CVtable::<Self> {
                                                _super: self._super.assert_init(),
                                                h: self.h.expect(concat!(
                                                    "cannot instantiate because method `",
                                                    stringify!(M),
                                                    "::",
                                                    stringify!(h),
                                                    "` is not implemented",
                                                )),
                                                I: self.I.assert_init(),
                                            }
                                        }
                                    }
                                }
                                pub static TYPE: ::classes::vtable::TypeInfo<2usize> =
                                    ::classes::vtable::TypeInfo::new_mixin_instance::<super::$class>(
                                        Super::TYPE,
                                        unsafe { ::classes::prelude::CVtable::<M>::TYPE.as_mixin_unchecked() },
                                        [
                                            #[cfg(not(debug_assertions))]
                                            0,
                                            #[cfg(debug_assertions)]
                                            Super::TYPE
                                                .const_offset_of(::classes::prelude::CVtable::<A>::TYPE)
                                                .expect(concat!(
                                                    stringify!($super_ty),
                                                    " is not a subclass of ",
                                                    stringify!(A),
                                                    ", we only support `mixin` to `on` a superclass yet"
                                                ))
                                                .offset,
                                            ::core::mem::offset_of!(vtable::$class, I),
                                        ],
                                        // #[cfg(debug_assertions)]
                                        MODULE_PATH,
                                        // #[cfg(debug_assertions)]
                                        stringify!($class),
                                    );
                            }
                            ::classes::assert_layout_eq! {
                                vtable::$class,
                                vtable::opt::$class,
                                {h, I}
                            }
                            impl $class<::classes::ptr::RcDyn<$class>> {
                                #[inline]
                                pub fn f(&self,) {
                                    { self.as_super().f() }.try_into().unwrap()
                                }
                                #[inline]
                                pub fn h(&self,) {
                                    { self.to_supertype::<::classes::prelude::CRc<M>>().h() }.try_into().unwrap()
                                }
                            }
                            impl $class<::classes::ptr::RcDyn<$class>, ::classes::class::NonVirtual> {
                                #[inline]
                                pub fn f(&self,) {
                                    { ::classes::prelude::CData::<Self>::f(self.as_virtual(),) }
                                }
                                #[inline]
                                pub fn h(&self,) {
                                    { ::classes::prelude::CData::<Self>::h(self.as_virtual(),) }
                                }
                            }
                            impl $class<::classes::ptr::RcDyn<$class>> {
                                #[inline]
                                pub(in super::super) fn get_z(&self) -> &usize {
                                    &self.0.z
                                }
                                #[inline]
                                pub(in super::super) fn raw_get_z(&self) -> &usize {
                                    &self.0.z
                                }
                            }
                        }
                    };
                }
                M! { _A_M, A_M, A<T, V> }
                M! { _B_M, B_M, B<T, V> }
            }
        }
    }
}

expand_test_case! {
    mixin_multi_ons {
        #[with(B)]
        mixin M on A, B {
            pub override fn A::f(&self) { super.f(); printlntb!("M1::f"); }
            pub override fn B::g(&self) { super.g(); printlntb!("M1::g"); }
        }
    } => {
        #[allow(unused_imports)]
        use _classes::M;
        #[allow(unused_imports)]
        use _classes::{B_M};
        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();
        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;
            #[allow(unused_imports)]
            pub(super) use _M::M;
            #[allow(unused_imports)]
            pub(super) use _M::{B_M};

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod _M {
                ::classes::_mod_uses! { mod class M }
                ::classes::_def_class! { mixin M }
                ::classes::_def_mixin! { M on #[class] A, #[class] B }

                mod data {
                    ::classes::_mod_uses! { mod data }
                    #[repr(C)]
                    pub struct M {}
                    impl M {}
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct M {}

                    impl M {
                        pub const fn debug_vtable_layout (&self , offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }

                    pub struct DebugVtableLayout <'a> {
                        this: &'a self::M ,
                        offset: usize ,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(M, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(M));
                            dbg.field("\'start" , &self.offset);
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<M>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable :: opt }
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct M {}
                    }
                    pub static TYPE: ::classes::vtable::TypeInfo<2usize> =
                        ::classes::vtable::TypeInfo::new_mixin::<super::M>(
                            [
                                ::classes::prelude::CVtable::<A>::TYPE,
                                ::classes::prelude::CVtable::<B>::TYPE,
                            ],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(M),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::M,
                    vtable::opt::M,
                    {}
                }
                impl M<::classes::ptr::RcDyn<M>> {
                    #[inline]
                    pub fn f(&self,) {
                        { self.to_supertype::<::classes::prelude::CRc<A>>().f() }
                            .try_into()
                            .unwrap()
                    }
                    #[inline]
                    pub fn g(&self,) {
                        { self.to_supertype::<::classes::prelude::CRc<B>>().g() }
                            .try_into()
                            .unwrap()
                    }
                }
                impl M<::classes::ptr::RcDyn<M>> {}
                macro_rules! M {
                    ($mod_name:ident, $class:ident, $super_ty:ty) => {
                        #[allow(unused_imports)]
                        pub use $mod_name::$class;

                        #[allow(non_snake_case)]
                        #[allow(non_camel_case_types)]
                        #[allow(unused_attributes)]
                        mod $mod_name {
                            ::classes::_mod_uses! { mod class $class }
                            ::classes::_def_class! { class $class }
                            type Super<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual> = $super_ty;
                            ::classes::_def_mixin_instance! { $class : Super with M }
                            ::classes::_def_class_extends! { $class : Super (mixin_instance) }
                            mod data {
                                ::classes::_mod_uses! {
                                mod data }
                                pub(super) type Super = ::classes::prelude::CData<super::Super>;
                                #[repr(C)]
                                pub struct $class {
                                    pub(super) _super: Super,
                                }
                                impl $class {
                                    #[inline]
                                    pub fn _delegate_ctor<
                                        _S : ::classes::class::IsClass,
                                        _F : FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
                                    >(mut _self : ::classes::prelude::CRcUninit<Self>, new : _F) -> ::classes::prelude::CRc<Self>
                                    where
                                        ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                                        for <'a> &'a ::classes::prelude::CRc<_S>: From<
                                            &'a ::classes::ptr::RcDyn<
                                                <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
                                            >,
                                        >,
                                    {
                                        #[allow(unused_unsafe)] unsafe {}
                                        let _ = |Self { _super, } : Self| ();
                                        let should_delegate = !::classes::class_const_eq!(
                                            <_S::Class as ::classes::class::ClassImpl>::Vtable,
                                            ::classes::prelude::CVtable<Super>,
                                        );
                                        if should_delegate {
                                            let _self = Super::_delegate_ctor::<_S, _F>(_self.into_super(), new);
                                            unsafe { _self.into_subclass_unchecked::<::classes::prelude::CRc<Self>>() }
                                        } else {
                                            let _self = new(_self.into_superclass());
                                            unsafe {
                                                ::classes::class::ClassRc::into_subclass_unchecked::<::classes::prelude::CRc<Self>>(_self)
                                            }
                                        }
                                    }
                                    pub(super) fn f(_self: &::classes::prelude::CRc<Self>,) {
                                        { _self.delegate_super() }.f();
                                        printlntb!("M1::f");
                                    }
                                    pub(super) fn g(_self: &::classes::prelude::CRc<Self>,) {
                                        { _self.delegate_super() }.g();
                                        printlntb!("M1::g");
                                    }
                                }
                            }
                            mod vtable {
                                ::classes::_mod_uses! { mod vtable }
                                pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                                #[repr(C)]
                                #[derive(Clone, Copy)]
                                pub struct $class {
                                    pub(super) _super: Super,
                                }
                                impl $class {
                                    pub const fn debug_vtable_layout (&self , offset: usize) -> self::DebugVtableLayout<'_> {
                                        self::DebugVtableLayout { this: self, offset }
                                    }
                                }

                                pub struct DebugVtableLayout <'a> {
                                    this: &'a self::$class ,
                                    offset: usize ,
                                }
                                impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                                    #[allow(unused_macros)]
                                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                        macro_rules! offset_of {
                                            ($field:ident) => {
                                                self.offset + ::core::mem::offset_of!($class, $field)
                                            };
                                        }
                                        let mut dbg = f.debug_struct(stringify!($class));
                                        dbg.field("\'start" , &self.offset);
                                        dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                                        dbg.field("\'end", &(self.offset + ::core::mem::size_of::<$class>())) ;
                                        dbg.finish()
                                    }
                                }
                                pub(super) mod opt {
                                    ::classes::_mod_uses! {
                                    mod vtable :: opt }
                                    pub(in super::super) type Super =
                                        ::classes::prelude::CVtableOpt<super::super::Super>;
                                    #[repr(C)]
                                    #[derive(Clone, Copy)]
                                    #[derive(Default)]
                                    pub struct $class {
                                        pub(in super::super) _super: Super,
                                    }
                                    impl $class {
                                        pub const DEFAULT: Self = Self {
                                            _super: Super::DEFAULT,
                                        };
                                        pub const fn init_mixin_header(
                                            mixin_header: &mut [::core::mem::MaybeUninit<::classes::vtable::MixinVtableHeader>],
                                        ) {
                                            let (first, rest) = mixin_header
                                                .split_first_mut()
                                                .expect("mixin header is empty");
                                            Super::init_mixin_header(rest);
                                            first.write(
                                                ::classes::vtable::MixinVtableHeader::new::<super::$class>(
                                                    ::core::mem::size_of::<data::$class>()
                                                        - ::core::mem::size_of::<::classes::prelude::CDataBase<M>>(),
                                                    super::$class::MIXIN_HEADER_ENTRIES
                                                        * ::core::mem::size_of::<::classes::vtable::MixinVtableHeader>()
                                                        + ::core::mem::size_of::<vtable::$class>()
                                                        - ::core::mem::size_of::<::classes::prelude::CVtableBase<M>>(),
                                                )
                                            );
                                        }
                                        pub const fn init_header(&mut self, ty: ::core::option::Option<::classes::vtable::Type>, offset: usize) {
                                            let ty = match ty {
                                                ::core::option::Option::None => Self::TYPE,
                                                ::core::option::Option::Some(ty) => ty,
                                            };
                                            self._super.init_header(::core::option::Option::Some(ty), offset);
                                        }
                                        #[allow(unused_unsafe)]
                                        pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                                            ::classes::static_assert_subclass_or_mixin_instance!(
                                                super::$class,
                                                ::classes::prelude::CVtable<A>,
                                                ", we only support `mixin` to `on` a superclass yet"
                                            );
                                            ::classes::static_assert_subclass_or_mixin_instance!(
                                                super::$class,
                                                ::classes::prelude::CVtable<B>,
                                                ", we only support `mixin` to `on` a superclass yet"
                                            );
                                            Super::init(_self);
                                            {
                                                let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<A>>(_self);
                                                ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                                    &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                ).into());
                                                while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<A>>(_self, &mut offset) {
                                                    ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                    ).into());
                                                }
                                            }
                                            {
                                                let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<B>>(_self);
                                                ptr.g = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::g(
                                                    &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                ).into());
                                                while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<B>>(_self, &mut offset) {
                                                    ptr.g = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::g(
                                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                    ).into());
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
                                pub static TYPE: ::classes::vtable::TypeInfo<2usize> =
                                    ::classes::vtable::TypeInfo::new_mixin_instance::<super::$class>(
                                        Super::TYPE,
                                        unsafe { ::classes::prelude::CVtable::<M>::TYPE.as_mixin_unchecked() },
                                        [
                                            #[cfg(not(debug_assertions))]
                                            0,
                                            #[cfg(debug_assertions)]
                                            Super::TYPE
                                                .const_offset_of(::classes::prelude::CVtable::<A>::TYPE)
                                                .expect(concat!(
                                                    stringify!($super_ty),
                                                    " is not a subclass of ",
                                                    stringify!(A),
                                                    ", we only support `mixin` to `on` a superclass yet"
                                                ))
                                                .offset,
                                            #[cfg(not(debug_assertions))]
                                            0,
                                            #[cfg(debug_assertions)]
                                            Super::TYPE
                                                .const_offset_of(::classes::prelude::CVtable::<B>::TYPE)
                                                .expect(concat!(
                                                    stringify!($super_ty),
                                                    " is not a subclass of ",
                                                    stringify!(B),
                                                    ", we only support `mixin` to `on` a superclass yet"
                                                ))
                                                .offset,
                                        ],
                                        // #[cfg(debug_assertions)]
                                        MODULE_PATH,
                                        // #[cfg(debug_assertions)]
                                        stringify!($class),
                                    );
                            }
                            ::classes::assert_layout_eq! {
                                vtable::$class,
                                vtable::opt::$class,
                                {}
                            }
                            impl $class<::classes::ptr::RcDyn<$class>> {
                                #[inline]
                                pub fn f(&self,) {
                                    { self.as_super().f() }.try_into().unwrap()
                                }
                                #[inline]
                                pub fn g(&self,) {
                                    { self.as_super().g() }.try_into().unwrap()
                                }
                            }
                            impl $class<::classes::ptr::RcDyn<$class>, ::classes::class::NonVirtual> {
                                #[inline]
                                pub fn f(&self,) {
                                    { ::classes::prelude::CData::<Self>::f(self.as_virtual(),) }
                                }
                                #[inline]
                                pub fn g(&self,) {
                                    { ::classes::prelude::CData::<Self>::g(self.as_virtual(),) }
                                }
                            }
                            impl $class<::classes::ptr::RcDyn<$class>> {}
                        }
                    };
                }
                M! { _B_M, B_M, B<T, V> }
            }
        }

    }
}

expand_test_case! {
    mixin_multi_withs {
        #[with(A, A/M2)]
        mixin M1 on A {
            pub override fn A::f(&self) { super.f(); println!("M1::f"); }
        }
    } => {
        #[allow(unused_imports)]
        use _classes::M1;
        #[allow(unused_imports)]
        use _classes::{A_M1, A_M2_M1};

        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();
        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;

            #[allow(unused_imports)]
            pub(super) use _M1::M1;
            #[allow(unused_imports)]
            pub(super) use _M1::{A_M1, A_M2_M1};

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod _M1 {
                ::classes::_mod_uses! { mod class M1 }
                ::classes::_def_class! { mixin M1 }
                ::classes::_def_mixin! { M1 on #[class] A }

                mod data {
                    ::classes::_mod_uses! { mod data }
                    #[repr(C)]
                    pub struct M1 {}
                    impl M1 {}
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct M1 {}

                    impl M1 {
                        pub const fn debug_vtable_layout (&self , offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }

                    pub struct DebugVtableLayout <'a> {
                        this: &'a self::M1 ,
                        offset: usize ,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(M1, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(M1));
                            dbg.field("\'start" , &self.offset);
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<M1>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable :: opt }
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct M1 {}
                    }
                    pub static TYPE: ::classes::vtable::TypeInfo<1usize> =
                        ::classes::vtable::TypeInfo::new_mixin::<super::M1>(
                            [::classes::prelude::CVtable::<A>::TYPE,],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(M1),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::M1,
                    vtable::opt::M1,
                    {}
                }
                impl M1<::classes::ptr::RcDyn<M1>> {
                    #[inline]
                    pub fn f(&self,) {
                        { self.to_supertype::<::classes::prelude::CRc<A>>().f() }
                            .try_into()
                            .unwrap()
                    }
                }
                impl M1<::classes::ptr::RcDyn<M1>> {}
                macro_rules! M1 {
                    ($mod_name:ident, $class:ident, $super_ty:ty) => {
                        #[allow(unused_imports)]
                        pub use $mod_name::$class;

                        #[allow(non_snake_case)]
                        #[allow(non_camel_case_types)]
                        #[allow(unused_attributes)]
                        mod $mod_name {
                            ::classes::_mod_uses! { mod class $class }
                            ::classes::_def_class! { class $class }
                            type Super<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual> = $super_ty;
                            ::classes::_def_mixin_instance! { $class : Super with M1 }
                            ::classes::_def_class_extends! { $class : Super (mixin_instance) }
                            mod data {
                                ::classes::_mod_uses! { mod data }
                                pub(super) type Super = ::classes::prelude::CData<super::Super>;
                                #[repr(C)]
                                pub struct $class {
                                    pub(super) _super: Super,
                                }
                                impl $class {
                                    #[inline]
                                    pub fn _delegate_ctor<
                                        _S : ::classes::class::IsClass,
                                        _F : FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
                                    >(mut _self : ::classes::prelude::CRcUninit<Self>, new : _F) -> ::classes::prelude::CRc<Self>
                                    where
                                        ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                                        for <'a> &'a ::classes::prelude::CRc<_S>: From<
                                            &'a ::classes::ptr::RcDyn<
                                                <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
                                            >,
                                        >,
                                    {
                                        #[allow(unused_unsafe)] unsafe {}
                                        let _ = |Self { _super, } : Self| ();
                                        let should_delegate = !::classes::class_const_eq!(
                                            <_S::Class as ::classes::class::ClassImpl>::Vtable,
                                            ::classes::prelude::CVtable<Super>,
                                        );
                                        if should_delegate {
                                            let _self = Super::_delegate_ctor::<_S, _F>(_self.into_super(), new);
                                            unsafe { _self.into_subclass_unchecked::<::classes::prelude::CRc<Self>>() }
                                        } else {
                                            let _self = new(_self.into_superclass());
                                            unsafe {
                                                ::classes::class::ClassRc::into_subclass_unchecked::<::classes::prelude::CRc<Self>>(_self)
                                            }
                                        }
                                    }
                                    pub(super) fn f(_self: &::classes::prelude::CRc<Self>,) {
                                        { _self.delegate_super() }.f();
                                        println!("M1::f");
                                    }
                                }
                            }
                            mod vtable {
                                ::classes::_mod_uses! { mod vtable }
                                pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                                #[repr(C)]
                                #[derive(Clone, Copy)]
                                pub struct $class {
                                    pub(super) _super: Super,
                                }
                                impl $class {
                                    pub const fn debug_vtable_layout (&self , offset: usize) -> self::DebugVtableLayout<'_> {
                                        self::DebugVtableLayout { this: self, offset }
                                    }
                                }

                                pub struct DebugVtableLayout <'a> {
                                    this: &'a self::$class ,
                                    offset: usize ,
                                }
                                impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                                    #[allow(unused_macros)]
                                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                        macro_rules! offset_of {
                                            ($field:ident) => {
                                                self.offset + ::core::mem::offset_of!($class, $field)
                                            };
                                        }
                                        let mut dbg = f.debug_struct(stringify!($class));
                                        dbg.field("\'start" , &self.offset);
                                        dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                                        dbg.field("\'end", &(self.offset + ::core::mem::size_of::<$class>())) ;
                                        dbg.finish()
                                    }
                                }
                                pub(super) mod opt {
                                    ::classes::_mod_uses! { mod vtable :: opt }
                                    pub(in super::super) type Super =
                                        ::classes::prelude::CVtableOpt<super::super::Super>;
                                    #[repr(C)]
                                    #[derive(Clone, Copy)]
                                    #[derive(Default)]
                                    pub struct $class {
                                        pub(in super::super) _super: Super,
                                    }
                                    impl $class {
                                        pub const DEFAULT: Self = Self {
                                            _super: Super::DEFAULT,
                                        };
                                        pub const fn init_mixin_header(
                                            mixin_header: &mut [::core::mem::MaybeUninit<::classes::vtable::MixinVtableHeader>],
                                        ) {
                                            let (first, rest) = mixin_header
                                                .split_first_mut()
                                                .expect("mixin header is empty");
                                            Super::init_mixin_header(rest);
                                            first.write(
                                                ::classes::vtable::MixinVtableHeader::new::<super::$class>(
                                                    ::core::mem::size_of::<data::$class>()
                                                        - ::core::mem::size_of::<::classes::prelude::CDataBase<M1>>(),
                                                    super::$class::MIXIN_HEADER_ENTRIES
                                                        * ::core::mem::size_of::<::classes::vtable::MixinVtableHeader>()
                                                        + ::core::mem::size_of::<vtable::$class>()
                                                        - ::core::mem::size_of::<::classes::prelude::CVtableBase<M1>>(),
                                                )
                                            );
                                        }
                                        pub const fn init_header(&mut self, ty: ::core::option::Option<::classes::vtable::Type>, offset: usize) {
                                            let ty = match ty {
                                                ::core::option::Option::None => Self::TYPE,
                                                ::core::option::Option::Some(ty) => ty,
                                            };
                                            self._super.init_header(::core::option::Option::Some(ty), offset);
                                        }
                                        #[allow(unused_unsafe)]
                                        pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                                            ::classes::static_assert_subclass_or_mixin_instance!(
                                                super::$class,
                                                ::classes::prelude::CVtable<A>,
                                                ", we only support `mixin` to `on` a superclass yet"
                                            );
                                            Super::init(_self);
                                            {
                                                let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<A>>(_self);
                                                ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                                    &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                ).into());
                                                while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<A>>(_self, &mut offset) {
                                                    ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                    ).into());
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
                                pub static TYPE: ::classes::vtable::TypeInfo<1usize> =
                                    ::classes::vtable::TypeInfo::new_mixin_instance::<super::$class>(
                                        Super::TYPE,
                                        unsafe { ::classes::prelude::CVtable::<M1>::TYPE.as_mixin_unchecked() },
                                        [
                                            #[cfg(not(debug_assertions))]
                                            0,
                                            #[cfg(debug_assertions)]
                                            Super::TYPE
                                                .const_offset_of(::classes::prelude::CVtable::<A>::TYPE)
                                                .expect(concat!(
                                                    stringify!($super_ty),
                                                    " is not a subclass of ",
                                                    stringify!(A),
                                                    ", we only support `mixin` to `on` a superclass yet"
                                                ))
                                                .offset,
                                        ],
                                        // #[cfg(debug_assertions)]
                                        MODULE_PATH,
                                        // #[cfg(debug_assertions)]
                                        stringify!($class),
                                    );
                            }
                            ::classes::assert_layout_eq! {
                                vtable::$class,
                                vtable::opt::$class,
                                {}
                            }
                            impl $class<::classes::ptr::RcDyn<$class>> {
                                #[inline]
                                pub fn f(&self,) {
                                    { self.as_super().f() }.try_into().unwrap()
                                }
                            }
                            impl $class<::classes::ptr::RcDyn<$class>, ::classes::class::NonVirtual> {
                                #[inline]
                                pub fn f(&self,) {
                                    { ::classes::prelude::CData::<Self>::f(self.as_virtual(),) }
                                }
                            }
                            impl $class<::classes::ptr::RcDyn<$class>> {}
                        }
                    };
                }
                M1! { _A_M1, A_M1, A<T, V> }
                M1! { _A_M2_M1, A_M2_M1, ::classes::mixin!(<T, V> A, M2,) }
            }
        }

    }
}

expand_test_case! {
    mixin_on_mixin {
        #[with(A/M1)]
        mixin M2 on M1 {
            pub override fn M1::f(&self) { super.f(); printlntb!("M2::f"); }
            pub override fn <M1 as I>::g(&self) { printlntb!("M2::g"); }
        }
    } => {
        #[allow(unused_imports)]
        use _classes::M2;
        #[allow(unused_imports)]
        use _classes::{A_M1_M2};
        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();
        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;
            #[allow(unused_imports)]
            pub(super) use _M2::M2;
            #[allow(unused_imports)]
            pub(super) use _M2::{A_M1_M2};
            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod _M2 {
                ::classes::_mod_uses! { mod class M2 }
                ::classes::_def_class! { mixin M2 }
                ::classes::_def_mixin! { M2 on #[mixin] M1 }

                mod data {
                    ::classes::_mod_uses! { mod data }
                    #[repr(C)]
                    pub struct M2 {}
                    impl M2 {}
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct M2 {}
                    impl M2 {
                        pub const fn debug_vtable_layout (&self , offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }

                    pub struct DebugVtableLayout <'a> {
                        this: &'a self::M2 ,
                        offset: usize ,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(M2, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(M2));
                            dbg.field("\'start" , &self.offset);
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<M2>())) ;
                            dbg.finish()
                        }
                    }
                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable :: opt }
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct M2 {}
                    }
                    pub static TYPE: ::classes::vtable::TypeInfo<1usize> =
                        ::classes::vtable::TypeInfo::new_mixin::<super::M2>(
                            [ ::classes::prelude::CVtable::<M1>::TYPE, ],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(M2),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::M2,
                    vtable::opt::M2,
                    {}
                }
                impl M2<::classes::ptr::RcDyn<M2>> {
                    #[inline]
                    pub fn f(&self,) {
                        { self.to_supertype::<::classes::prelude::CRc<M1>>().f() }
                            .try_into()
                            .unwrap()
                    }
                    #[inline]
                    pub fn g(&self,) {
                        { self.to_supertype::<::classes::prelude::CRc<I>>().g() }
                            .try_into()
                            .unwrap()
                    }
                }
                impl M2<::classes::ptr::RcDyn<M2>> {}
                macro_rules! M2 {
                    ($mod_name:ident, $class:ident, $super_ty:ty) => {
                        #[allow(unused_imports)]
                        pub use $mod_name::$class;

                        #[allow(non_snake_case)]
                        #[allow(non_camel_case_types)]
                        #[allow(unused_attributes)]
                        mod $mod_name {
                            ::classes::_mod_uses! { mod class $class }
                            ::classes::_def_class! { class $class }
                            type Super<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual> = $super_ty;
                            ::classes::_def_mixin_instance! { $class : Super with M2 }
                            ::classes::_def_class_extends! { $class : Super (mixin_instance) }
                            mod data {
                                ::classes::_mod_uses! { mod data }
                                pub(super) type Super = ::classes::prelude::CData<super::Super>;
                                #[repr(C)]
                                pub struct $class {
                                    pub(super) _super: Super,
                                }
                                impl $class {
                                    #[inline]
                                    pub fn _delegate_ctor<
                                        _S : ::classes::class::IsClass,
                                        _F : FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
                                    >(mut _self : ::classes::prelude::CRcUninit<Self>, new : _F) -> ::classes::prelude::CRc<Self>
                                    where
                                        ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                                        for <'a> &'a ::classes::prelude::CRc<_S>: From<
                                            &'a ::classes::ptr::RcDyn<
                                                <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
                                            >,
                                        >,
                                    {
                                        #[allow(unused_unsafe)] unsafe {}
                                        let _ = |Self { _super, } : Self| ();
                                        let should_delegate = !::classes::class_const_eq!(
                                            <_S::Class as ::classes::class::ClassImpl>::Vtable,
                                            ::classes::prelude::CVtable<Super>,
                                        );
                                        if should_delegate {
                                            let _self = Super::_delegate_ctor::<_S, _F>(_self.into_super(), new);
                                            unsafe { _self.into_subclass_unchecked::<::classes::prelude::CRc<Self>>() }
                                        } else {
                                            let _self = new(_self.into_superclass());
                                            unsafe {
                                                ::classes::class::ClassRc::into_subclass_unchecked::<::classes::prelude::CRc<Self>>(_self)
                                            }
                                        }
                                    }
                                    pub(super) fn f(_self: &::classes::prelude::CRc<Self>,) {
                                        { _self.delegate_super() }.f();
                                        printlntb!("M2::f");
                                    }
                                    pub(super) fn g(_self: &::classes::prelude::CRc<Self>,) {
                                        printlntb!("M2::g");
                                    }
                                }
                            }
                            mod vtable {
                                ::classes::_mod_uses! { mod vtable }
                                pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                                #[repr(C)]
                                #[derive(Clone, Copy)]
                                pub struct $class {
                                    pub(super) _super: Super,
                                }
                                impl $class {
                                    pub const fn debug_vtable_layout (&self , offset: usize) -> self::DebugVtableLayout<'_> {
                                        self::DebugVtableLayout { this: self, offset }
                                    }
                                }

                                pub struct DebugVtableLayout <'a> {
                                    this: &'a self::$class ,
                                    offset: usize ,
                                }
                                impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                                    #[allow(unused_macros)]
                                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                        macro_rules! offset_of {
                                            ($field:ident) => {
                                                self.offset + ::core::mem::offset_of!($class, $field)
                                            };
                                        }
                                        let mut dbg = f.debug_struct(stringify!($class));
                                        dbg.field("\'start" , &self.offset);
                                        dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                                        dbg.field("\'end", &(self.offset + ::core::mem::size_of::<$class>())) ;
                                        dbg.finish()
                                    }
                                }
                                pub(super) mod opt {
                                    ::classes::_mod_uses! {
                                    mod vtable :: opt }
                                    pub(in super::super) type Super =
                                        ::classes::prelude::CVtableOpt<super::super::Super>;
                                    #[repr(C)]
                                    #[derive(Clone, Copy)]
                                    #[derive(Default)]
                                    pub struct $class {
                                        pub(in super::super) _super: Super,
                                    }
                                    impl $class {
                                        pub const DEFAULT: Self = Self {
                                            _super: Super::DEFAULT,
                                        };
                                        pub const fn init_mixin_header(
                                            mixin_header: &mut [::core::mem::MaybeUninit<::classes::vtable::MixinVtableHeader>],
                                        ) {
                                            let (first, rest) = mixin_header
                                                .split_first_mut()
                                                .expect("mixin header is empty");
                                            Super::init_mixin_header(rest);
                                            first.write(
                                                ::classes::vtable::MixinVtableHeader::new::<super::$class>(
                                                    ::core::mem::size_of::<data::$class>()
                                                        - ::core::mem::size_of::<::classes::prelude::CDataBase<M2>>(),
                                                    super::$class::MIXIN_HEADER_ENTRIES
                                                        * ::core::mem::size_of::<::classes::vtable::MixinVtableHeader>()
                                                        + ::core::mem::size_of::<vtable::$class>()
                                                        - ::core::mem::size_of::<::classes::prelude::CVtableBase<M2>>(),
                                                )
                                            );
                                        }
                                        pub const fn init_header(&mut self, ty: ::core::option::Option<::classes::vtable::Type>, offset: usize) {
                                            let ty = match ty {
                                                ::core::option::Option::None => Self::TYPE,
                                                ::core::option::Option::Some(ty) => ty,
                                            };
                                            self._super.init_header(::core::option::Option::Some(ty), offset);
                                        }
                                        #[allow(unused_unsafe)]
                                        pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                                            ::classes::static_assert_subclass_or_mixin_instance!(
                                                super::$class,
                                                ::classes::prelude::CVtable<M1>,
                                                ", we only support `mixin` to `on` a superclass yet"
                                            );
                                            Super::init(_self);
                                            {
                                                let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<M1>>(_self);
                                                ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                                    &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                ).into());
                                                while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<M1>>(_self, &mut offset) {
                                                    ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                    ).into());
                                                }
                                            }
                                            {
                                                let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<I>>(_self);
                                                ptr.g = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::g(
                                                    &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                ).into());
                                                while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<I>>(_self, &mut offset) {
                                                    ptr.g = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::g(
                                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                                    ).into());
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
                                pub static TYPE: ::classes::vtable::TypeInfo<1usize> =
                                    ::classes::vtable::TypeInfo::new_mixin_instance::<super::$class>(
                                        Super::TYPE,
                                        unsafe { ::classes::prelude::CVtable::<M2>::TYPE.as_mixin_unchecked() },
                                        [
                                            #[cfg(not(debug_assertions))]
                                            0,
                                            #[cfg(debug_assertions)]
                                            Super::TYPE
                                                .const_offset_of(::classes::prelude::CVtable::<M1>::TYPE)
                                                .expect(concat!(
                                                    stringify!($super_ty),
                                                    " is not a subclass of ",
                                                    stringify!(M1),
                                                    ", we only support `mixin` to `on` a superclass yet"
                                                ))
                                                .offset,
                                        ],
                                        // #[cfg(debug_assertions)]
                                        MODULE_PATH,
                                        // #[cfg(debug_assertions)]
                                        stringify!($class),
                                    );
                            }
                            ::classes::assert_layout_eq! {
                                vtable::$class,
                                vtable::opt::$class,
                                {}
                            }
                            impl $class<::classes::ptr::RcDyn<$class>> {
                                #[inline]
                                pub fn f(&self,) {
                                    { self.as_super().f() }.try_into().unwrap()
                                }
                                #[inline]
                                pub fn g(&self,) {
                                    { self.to_supertype::<::classes::prelude::CRc<I>>().g() }
                                        .try_into()
                                        .unwrap()
                                }
                            }
                            impl $class<::classes::ptr::RcDyn<$class>, ::classes::class::NonVirtual> {
                                #[inline]
                                pub fn f(&self,) {
                                    { ::classes::prelude::CData::<Self>::f(self.as_virtual(),) }
                                }
                                #[inline]
                                pub fn g(&self,) {
                                    { ::classes::prelude::CData::<Self>::g(self.as_virtual(),) }
                                }
                            }
                            impl $class<::classes::ptr::RcDyn<$class>> {}
                        }
                    };
                }
                M2! { _A_M1_M2, A_M1_M2, ::classes::mixin!(<T, V> A, M1,) }
            }
        }
    }
}

expand_test_case! {
    mixin_instance {
        class C1 extends A with M {
            struct { final w: usize = 3 }
            pub fn new() -> Self { Self { super: Super::new(), .. } }
            pub override fn A       ::f(&self) { super.f(); println!("C1::f, w = {}", self.get_w()); }
            pub override fn M       ::h(&self) { super.h(); println!("C1::h, w = {}", self.get_w()); }
            pub override fn <M as I>::i(&self) {            println!("C1::i, w = {}", self.get_w()); }
            pub          fn           j(&self) {            println!("C1::j, w = {}", self.get_w()); }
        }
    } => {
        #[allow(unused_imports)]
        use _classes::C1;
        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();
        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;

            #[allow(unused_imports)]
            pub(super) use _C1::C1;

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod _C1 {
                ::classes::_mod_uses! { mod class C1 }
                ::classes::_def_class! { class C1 }
                type Super<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual> = ::classes::mixin!(<T, V> A, M,);
                ::classes::_def_class_extends! { C1 : Super (mixin_instance) }

                mod data {
                    ::classes::_mod_uses! { mod data }
                    pub(super) type Super = ::classes::prelude::CData<super::Super>;
                    #[repr(C)]
                    pub struct C1 {
                        pub(super) _super: Super,
                        pub(super) w: usize,
                    }
                    impl C1 {
                        #[cold]
                        #[inline(never)]
                        pub fn _delegate_ctor<
                            _S : ::classes::class::IsClass,
                            _F : FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
                        >(mut _self : ::classes::prelude::CRcUninit<Self>, new : _F) -> ::classes::prelude::CRc<Self>
                        where
                            ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                            for <'a> &'a ::classes::prelude::CRc<_S>: From<
                                &'a ::classes::ptr::RcDyn<
                                    <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
                                >,
                            >,
                        {
                            let _ = new;
                            panic!("unsupported")
                        }
                        pub fn new(mut _self: ::classes::prelude::CRcUninit<Self>,) -> ::classes::prelude::CRc<Self> {
                            unsafe {
                                ::core::ptr::write(&raw mut (*_self.as_mut_ptr()).w, 3,);
                                let _ = |Self { _super, w: _, } : Self| ();
                                Super::_delegate_ctor::<A, _>(_self.into_super(), |_self| {
                                    ::classes::prelude::CData::<A>::new(_self,)
                                })
                                .into_subclass_unchecked()
                            }
                        }
                        pub(super) fn f(_self: &::classes::prelude::CRc<Self>,) {
                            { _self.delegate_super() }.f();
                            println!("C1::f, w = {}", _self.get_w());
                        }
                        pub(super) fn h(_self: &::classes::prelude::CRc<Self>,) {
                            { _self.delegate_super() }.h();
                            println!("C1::h, w = {}", _self.get_w());
                        }
                        pub(super) fn i(_self: &::classes::prelude::CRc<Self>,) {
                            println!("C1::i, w = {}", _self.get_w());
                        }
                        pub(super) fn j(_self: &::classes::prelude::CRc<Self>,) {
                            println!("C1::j, w = {}", _self.get_w());
                        }
                    }
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }
                    pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct C1 {
                        pub(super) _super: Super,
                        pub j: fn(&::classes::prelude::CRc<Self>,),
                    }

                    impl C1 {
                        pub const fn debug_vtable_layout (&self , offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }

                    pub struct DebugVtableLayout <'a> {
                        this: &'a self::C1 ,
                        offset: usize ,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(C1, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(C1));
                            dbg.field("\'start" , &self.offset);
                            dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                            dbg.field(stringify!(j), &offset_of!(j));
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<C1>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable :: opt }
                        pub(in super::super) type Super = ::classes::prelude::CVtableOpt<super::super::Super>;
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct C1 {
                            pub(in super::super) _super: Super,
                            pub j: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,)>,
                        }
                        impl C1 {
                            pub const DEFAULT: Self = Self {
                                _super: Super::DEFAULT,
                                j: ::core::option::Option::None,
                            };
                            pub const fn init_mixin_header(
                                mixin_header: &mut [::core::mem::MaybeUninit<::classes::vtable::MixinVtableHeader>],
                            ) {
                                Super::init_mixin_header(mixin_header);
                            }
                            pub const fn init_header(&mut self, ty: ::core::option::Option<::classes::vtable::Type>, offset: usize) {
                                let ty = match ty {
                                    ::core::option::Option::None => Self::TYPE,
                                    ::core::option::Option::Some(ty) => ty,
                                };
                                self._super.init_header(::core::option::Option::Some(ty), offset);
                            }
                            #[allow(unused_unsafe)]
                            pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                                Super::init(_self);
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<A>>(_self);
                                    ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<A>>(_self, &mut offset) {
                                        ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<M>>(_self);
                                    ptr.h = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::h(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<M>>(_self, &mut offset) {
                                        ptr.h = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::h(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<I>>(_self);
                                    ptr.i = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::i(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<I>>(_self, &mut offset) {
                                        ptr.i = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::i(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.j = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::j(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.j = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::j(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                            }
                            #[track_caller]
                            pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                                ::classes::prelude::CVtable::<Self> {
                                    _super: self._super.assert_init(),
                                    j: self.j.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(C1),
                                        "::",
                                        stringify!(j),
                                        "` is not implemented",
                                    )),
                                }
                            }
                        }
                    }
                    pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                        ::classes::vtable::TypeInfo::new_concrete_class::<super::C1>(
                            ::core::option::Option::Some(Super::TYPE),
                            [],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(C1),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::C1,
                    vtable::opt::C1,
                    {j}
                }
                ::classes::_def_concrete_class! { C1 }
                impl C1<::classes::ptr::RcDyn<C1>> {
                    #[inline]
                    pub fn new() -> Self {
                        ::classes::prelude::CData::<Self>::new(
                            ::classes::prelude::CRcUninit::<Self>::new_uninit(),
                        )
                    }
                    #[inline]
                    pub fn f(&self,) {
                        { self.to_supertype::<::classes::prelude::CRc<A>>().f() }
                            .try_into()
                            .unwrap()
                    }
                    #[inline]
                    pub fn h(&self,) {
                        { self.as_super().h() }.try_into().unwrap()
                    }
                    #[inline]
                    pub fn i(&self,) {
                        { self.to_supertype::<::classes::prelude::CRc<I>>().i() }
                        .try_into()
                        .unwrap()
                    }
                    #[inline]
                    pub fn j(&self,) {
                        { (self.0.vtable().j)(self,) }
                    }
                }
                impl C1<::classes::ptr::RcDyn<C1>, ::classes::class::NonVirtual> {
                    #[inline]
                    pub fn f(&self,) {
                        { ::classes::prelude::CData::<Self>::f(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn h(&self,) {
                        { ::classes::prelude::CData::<Self>::h(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn i(&self,) {
                        { ::classes::prelude::CData::<Self>::i(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn j(&self,) {
                        { ::classes::prelude::CData::<Self>::j(self.as_virtual(),) }
                    }
                }
                impl C1<::classes::ptr::RcDyn<C1>> {
                    #[inline]
                    pub(in super::super) fn get_w(&self) -> &usize {
                        &self.0.w
                    }
                    #[inline]
                    pub(in super::super) fn raw_get_w(&self) -> &usize {
                        &self.0.w
                    }
                }
            }
        }
    }
}

expand_test_case! {
    mixin_instance_multi_with {
        class C extends A with M2, M1 {
            pub fn new() -> Self { Self { super: Super::new(), .. } }
            pub override fn A::f(&self) { super.f(); println!("C::f"); }
        }
    } => {
        #[allow(unused_imports)]
        use _classes::C;
        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();
        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;
            #[allow(unused_imports)]
            pub(super) use _C::C;
            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod _C {
                ::classes::_mod_uses! { mod class C }
                ::classes::_def_class! { class C }
                type Super<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual> =
                    ::classes::mixin!(<T, V> A, M2, M1,);
                ::classes::_def_class_extends! { C : Super (mixin_instance) }

                mod data {
                    ::classes::_mod_uses! { mod data }
                    pub(super) type Super = ::classes::prelude::CData<super::Super>;
                    #[repr(C)]
                    pub struct C {
                        pub(super) _super: Super,
                    }
                    impl C {
                        #[cold]
                        #[inline(never)]
                        pub fn _delegate_ctor<
                            _S : ::classes::class::IsClass,
                            _F : FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
                        >(mut _self : ::classes::prelude::CRcUninit<Self>, new : _F) -> ::classes::prelude::CRc<Self>
                        where
                            ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                            for <'a> &'a ::classes::prelude::CRc<_S>: From<
                                &'a ::classes::ptr::RcDyn<
                                    <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
                                >,
                            >,
                        {
                            let _ = new;
                            panic!("unsupported")
                        }
                        pub fn new(mut _self: ::classes::prelude::CRcUninit<Self>,) -> ::classes::prelude::CRc<Self> {
                            unsafe {
                                let _ = |Self { _super, } : Self| ();
                                Super::_delegate_ctor::<A, _>(_self.into_super(), |_self| {
                                    ::classes::prelude::CData::<A>::new(_self,)
                                })
                                .into_subclass_unchecked()
                            }
                        }
                        pub(super) fn f(_self: &::classes::prelude::CRc<Self>,) {
                            { _self.delegate_super() }.f();
                            println!("C::f");
                        }
                    }
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }
                    pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct C {
                        pub(super) _super: Super,
                    }

                    impl C {
                        pub const fn debug_vtable_layout (&self , offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }

                    pub struct DebugVtableLayout <'a> {
                        this: &'a self::C ,
                        offset: usize ,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(C, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(C));
                            dbg.field("\'start" , &self.offset);
                            dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<C>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable :: opt }
                        pub(in super::super) type Super =
                            ::classes::prelude::CVtableOpt<super::super::Super>;
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct C {
                            pub(in super::super) _super: Super,
                        }
                        impl C {
                            pub const DEFAULT: Self = Self {
                                _super: Super::DEFAULT,
                            };
                            pub const fn init_mixin_header(
                                mixin_header: &mut [::core::mem::MaybeUninit<::classes::vtable::MixinVtableHeader>],
                            ) {
                                Super::init_mixin_header(mixin_header);
                            }
                            pub const fn init_header(&mut self, ty: ::core::option::Option<::classes::vtable::Type>, offset: usize) {
                                let ty = match ty {
                                    ::core::option::Option::None => Self::TYPE,
                                    ::core::option::Option::Some(ty) => ty,
                                };
                                self._super.init_header(::core::option::Option::Some(ty), offset);
                            }
                            #[allow(unused_unsafe)]
                            pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                                Super::init(_self);
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<A>>(_self);
                                    ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<A>>(_self, &mut offset) {
                                        ptr.f = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::f(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
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
                        ::classes::vtable::TypeInfo::new_concrete_class::<super::C>(
                            ::core::option::Option::Some(Super::TYPE),
                            [],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(C),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::C,
                    vtable::opt::C,
                    {}
                }
                ::classes::_def_concrete_class! { C }
                impl C<::classes::ptr::RcDyn<C>> {
                    #[inline]
                    pub fn new() -> Self {
                        ::classes::prelude::CData::<Self>::new(
                            ::classes::prelude::CRcUninit::<Self>::new_uninit(),
                        )
                    }
                    #[inline]
                    pub fn f(&self,) {
                        { self.to_supertype::<::classes::prelude::CRc<A>>().f() }
                            .try_into()
                            .unwrap()
                    }
                }
                impl C<::classes::ptr::RcDyn<C>, ::classes::class::NonVirtual> {
                    #[inline]
                    pub fn f(&self,) {
                        { ::classes::prelude::CData::<Self>::f(self.as_virtual(),) }
                    }
                }
                impl C<::classes::ptr::RcDyn<C>> {}
            }
        }
    }
}

expand_test_case! {
    extern_mixin {
        #[with(A, B/M2)]
        extern mixin krate::M;
    } => {
        #[allow(unused_imports)]
        pub use _classes::{A_M, B_M2_M};

        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();
        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            #[allow(unused_imports)]
            use {A as _, B as _, M2 as _,};
            krate::M! { _A_M, A_M, A<T, V> }
            krate::M! { _B_M2_M, B_M2_M, ::classes::mixin!(<T, V> B, M2,) }
        }
    }
}
