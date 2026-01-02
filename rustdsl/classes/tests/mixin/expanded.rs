use core::cell::RefCell;

thread_local! {
    static BUF: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

use classes::prelude::*;

use _classes::*;
use mixins::*;

macro_rules! println {
    ($($args:tt)*) => {
        BUF.with_borrow_mut(|buf| {
            buf.push(format!($($args)*));
        })
    };
}

// #[cfg(debug_assertions)]
const MODULE_PATH: &str = module_path!();

mod _classes {
    pub(super) use self::_A::A;
    pub(super) use self::_B::B;
    pub(super) use self::_C1::C1;
    pub(super) use self::_C2::C2;
    pub(super) use self::_I::I;
    // pub(super) use self::_M::{A_M, B_M, M};
    pub(super) use self::_M::M;
    use super::*;

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _I {
        ::classes::_mod_uses!(mod class I);

        ::classes::_def_class!(class I);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct I {}
        }
        mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            pub struct I {
                header: VtableHeader,
                pub i: fn(&CRc<Self>),
            }

            impl I {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }

            pub struct DebugVtableLayout<'a> {
                this: &'a self::I,
                offset: usize,
            }

            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    macro_rules! offset_of {
                        ($field:ident) => {
                            self.offset + ::core::mem::offset_of!(I, $field)
                        };
                    }
                    let mut dbg = f.debug_struct(stringify!(I));
                    dbg.field("\'start", &self.offset);
                    dbg.field("header", &self.this.header);
                    dbg.field("i", &offset_of!(i));
                    dbg.field("\'end", &(self.offset + core::mem::size_of::<I>()));
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct I {
                    header: VtableHeaderOpt,
                    pub i: Option<fn(&CRc<Self>)>,
                }

                impl I {
                    pub const DEFAULT: Self = Self {
                        header: VtableHeaderOpt::DEFAULT,
                        i: None,
                    };

                    pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => super::TYPE.as_type(),
                            Some(ty) => ty,
                        };
                        self.header = VtableHeaderOpt::new(ty, offset);
                    }
                    pub const fn init(&mut self) {}
                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            header: self.header.assert_init(),
                            i: self.i.unwrap(),
                        }
                    }
                }
            }

            pub static TYPE: TypeInfo<0> = TypeInfo::new_abstract_class::<super::I>(
                None,
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "I",
            );
        }
        impl I<RcDyn<I>> {
            #[inline]
            pub fn i(&self) {
                (self.0.vtable().i)(self)
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _A {
        ::classes::_mod_uses!(mod class A);

        ::classes::_def_class!(class A);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct A {
                pub(super) x: usize,
            }

            impl A {
                pub fn new(mut _self: CRcUninit<Self>) -> CRc<Self> {
                    let _ = |Self { x: _ }: Self| ();
                    #[allow(unused_unsafe)]
                    unsafe {
                        core::ptr::write(&raw mut (*_self.as_mut_ptr()).x, 0);
                    }
                    CRc::<Self>::_from_inner(unsafe { _self.assume_init() })
                }

                pub(super) fn f(this: &CRc<Self>) {
                    println!("A::f, x = {}", this.get_x());
                }
            }
        }
        mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            pub struct A {
                header: VtableHeader,
                pub f: fn(&CRc<Self>),
            }

            impl A {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }

            pub struct DebugVtableLayout<'a> {
                this: &'a self::A,
                offset: usize,
            }

            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    macro_rules! offset_of {
                        ($field:ident) => {
                            self.offset + ::core::mem::offset_of!(A, $field)
                        };
                    }
                    let mut dbg = f.debug_struct(stringify!(A));
                    dbg.field("\'start", &self.offset);
                    dbg.field("header", &self.this.header);
                    dbg.field("f", &offset_of!(f));
                    dbg.field("\'end", &(self.offset + core::mem::size_of::<A>()));
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct A {
                    header: VtableHeaderOpt,
                    pub f: Option<fn(&CRc<Self>)>,
                }

                impl A {
                    pub const DEFAULT: Self = Self {
                        header: VtableHeaderOpt::DEFAULT,
                        f: None,
                    };

                    pub const fn init_mixin_header(
                        mixin_header: &mut [core::mem::MaybeUninit<MixinVtableHeader>],
                    ) {
                        assert!(mixin_header.is_empty());
                    }
                    pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => super::TYPE.as_type(),
                            Some(ty) => ty,
                        };
                        self.header = VtableHeaderOpt::new(ty, offset);
                    }
                    pub const fn init(&mut self) {
                        self.f = Some(CData::<Self>::f);
                    }
                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            header: self.header.assert_init(),
                            f: self.f.unwrap(),
                        }
                    }
                }
            }

            pub static TYPE: TypeInfo<0> = TypeInfo::new_abstract_class::<super::A>(
                None,
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "A",
            );
        }
        impl A<RcDyn<A>> {
            #[inline]
            pub fn f(&self) {
                (self.0.vtable().f)(self)
            }
        }

        impl A<RcDyn<A>, NonVirtual> {
            #[inline]
            pub fn f(&self) {
                CData::<Self>::f(self.as_virtual())
            }
        }

        impl<V> A<RcDyn<A>, V> {
            pub fn get_x(&self) -> usize {
                self.0.x
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _B {
        ::classes::_mod_uses!(mod class B);

        ::classes::_def_class!(class B);
        ::classes::_def_class_extends!(B: A);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct B {
                _super: CData<A>,
                pub(super) y: usize,
            }

            impl B {
                pub fn new(mut _self: CRcUninit<Self>) -> CRc<Self> {
                    let _ = |Self { _super: _, y: _ }: Self| ();
                    #[allow(unused_unsafe)]
                    unsafe {
                        core::ptr::write(&raw mut (*_self.as_mut_ptr()).y, 1);
                    }
                    let _super = CData::<A>::new(_self.into_super());
                    unsafe { _super.into_subclass_unchecked() }
                }
                pub(super) fn f(this: &CRc<Self>) {
                    this.delegate_super().f();
                    println!("B::f, y = {}", this.get_y());
                }
                pub(super) fn g(this: &CRc<Self>) {
                    println!("B::g, y = {}", this.get_y());
                }
            }
        }

        pub mod vtable {
            use std::mem::offset_of;

            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            pub struct B {
                pub(super) _super: CVtable<A>,
                pub g: fn(&CRc<Self>),
            }

            impl B {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }

            pub struct DebugVtableLayout<'a> {
                this: &'a self::B,
                offset: usize,
            }

            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    macro_rules! offset_of {
                        ($field:ident) => {
                            self.offset + ::core::mem::offset_of!(B, $field)
                        };
                    }
                    let mut dbg = f.debug_struct(stringify!(B));
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self.this._super.debug_vtable_layout(offset_of!(_super)),
                    );
                    dbg.field("g", &offset_of!(g));
                    dbg.field("\'end", &(self.offset + core::mem::size_of::<B>()));
                    dbg.finish()
                }
            }

            pub mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct B {
                    pub(in super::super) _super: CVtableOpt<A>,
                    pub g: Option<fn(&CRc<Self>)>,
                }
                #[allow(unused_variables)]
                impl B {
                    pub const DEFAULT: Self = Self {
                        _super: CVtableOpt::<A>::DEFAULT,
                        g: None,
                    };

                    pub const fn init_mixin_header(
                        mixin_header: &mut [core::mem::MaybeUninit<MixinVtableHeader>],
                    ) {
                        CVtableOpt::<A>::init_mixin_header(mixin_header);
                    }
                    pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => super::TYPE.as_type(),
                            Some(ty) => ty,
                        };
                        self._super.init_header(Some(ty), offset);
                    }
                    pub const fn init(&mut self) {
                        self._super.init();
                        self._super.f =
                            Some(|this| CData::<Self>::f(unsafe { this.as_subclass_unchecked() }));
                        self.g = Some(CData::<Self>::g);
                    }
                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            _super: self._super.assert_init(),
                            g: self.g.unwrap(),
                        }
                    }
                }
            }

            pub static TYPE: TypeInfo<0> = TypeInfo::new_abstract_class::<super::B>(
                Some(CVtable::<A>::TYPE),
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "B",
            );
        }

        impl B<RcDyn<B>> {
            pub fn f(&self) {
                self.as_super().f()
            }
            #[inline]
            pub fn g(&self) {
                (self.0.vtable().g)(self)
            }
        }

        impl B<RcDyn<B>, NonVirtual> {
            #[inline]
            pub fn f(&self) {
                CData::<Self>::f(self.as_virtual())
            }
            #[inline]
            pub fn g(&self) {
                CData::<Self>::g(self.as_virtual())
            }
        }

        impl<V> B<RcDyn<B>, V> {
            pub fn get_y(&self) -> usize {
                self.0.y
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _M {
        ::classes::_mod_uses!(mod class M);

        ::classes::_def_class!(mixin M);
        ::classes::_def_mixin!(M on #[class] A implements I);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct M {
                pub(super) z: usize,
            }
        }

        pub mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct M {
                pub h: fn(&CRc<super::M>),
                pub I: CVtable<I>,
            }

            impl M {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }

            pub struct DebugVtableLayout<'a> {
                this: &'a self::M,
                offset: usize,
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
                    dbg.field("\'start", &self.offset);
                    dbg.field("h", &offset_of!(h));
                    dbg.field("I", &self.this.I.debug_vtable_layout(offset_of!(I)));
                    dbg.field("\'end", &(self.offset + core::mem::size_of::<M>()));
                    dbg.finish()
                }
            }

            // unused
            pub mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct M {
                    pub h: Option<fn(&CRc<super::super::M>)>,
                    pub I: CVtableOpt<I>,
                }
            }

            pub static TYPE: TypeInfo<2> = TypeInfo::new_mixin::<super::M>(
                [CVtable::<A>::TYPE, CVtable::<I>::TYPE],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "M",
            );
        }

        impl M<RcDyn<M>> {
            #[inline]
            pub fn f(&self) {
                self.to_supertype::<CRc<A>>().f()
            }
            #[inline]
            pub fn h(&self) {
                (self.0.vtable().vtable_without_super().h)(self)
            }
        }

        impl<V> M<RcDyn<M>, V> {
            pub fn get_z(&self) -> usize {
                self.0.vtable().data_without_super(&self.0).z
            }
        }

        #[macro_export]
        macro_rules! _M {
            ($mod:ident, $class:ident, $super_ty:ty) => {
                #[allow(non_snake_case)]
                #[allow(non_camel_case_types)]
                pub(super) mod $mod {
                    ::classes::_mod_uses!(mod class $class);

                    ::classes::_def_class!(class $class);
                    type Super<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual> = $super_ty;
                    ::classes::_def_mixin_instance!($class: Super with M);
                    ::classes::_def_class_extends!($class: Super);
                    ::classes::_def_class_impl!($class: I);

                    mod data {
                        ::classes::_mod_uses!(mod data);

                        #[repr(C)]
                        pub struct $class {
                            pub(super) _super: CData<Super>,
                            pub(super) z: usize,
                        }
                        impl $class {
                            pub fn new(mut _self: CRcUninit<Self>) -> CRc<Self> {
                                let _ = |Self { _super: _, z: _ }: Self| ();
                                #[allow(unused_unsafe)]
                                unsafe {
                                    core::ptr::write(&raw mut (*_self.as_mut_ptr()).z, 2);
                                }
                                let _super = CData::<Super>::new(_self.into_super());
                                unsafe { _super.into_subclass_unchecked() }
                            }
                            pub(super) fn f(this: &CRc<Self>) {
                                this.delegate_super().f();
                                println!("M::f, z = {}", this.get_z());
                            }
                            pub(super) fn h(this: &CRc<Self>) {
                                println!("M::h, z = {}", this.get_z());
                            }
                        }
                    }

                    pub mod vtable {
                        ::classes::_mod_uses!(mod vtable);

                        #[repr(C)]
                        #[derive(Debug, Clone, Copy)]
                        pub struct $class {
                            pub(super) _super: CVtable<Super>,
                            pub h: fn(&CRc<M>),
                            pub I: CVtable<I>,
                            // $( pub $impl_name: CVtable<$impl_name>, )*
                        }

                        impl $class {
                            pub const fn debug_vtable_layout(
                                &self,
                                offset: usize,
                            ) -> self::DebugVtableLayout<'_> {
                                self::DebugVtableLayout { this: self, offset }
                            }
                        }

                        pub struct DebugVtableLayout<'a> {
                            this: &'a self::$class,
                            offset: usize,
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
                                dbg.field("\'start", &self.offset);
                                dbg.field(
                                    "super",
                                    &self.this._super.debug_vtable_layout(offset_of!(_super)),
                                );
                                dbg.field("h", &offset_of!(h));
                                dbg.field("I", &self.this.I.debug_vtable_layout(offset_of!(I)));
                                dbg.field("\'end", &(self.offset + core::mem::size_of::<$class>()));
                                dbg.finish()
                            }
                        }

                        pub mod opt {
                            ::classes::_mod_uses!(mod vtable::opt);

                            #[repr(C)]
                            #[derive(Clone, Copy, Default)]
                            pub struct $class {
                                pub(super) _super: CVtableOpt<Super>,
                                pub h: Option<fn(&CRc<super::super::M>)>,
                                pub I: CVtableOpt<I>,
                            }

                            impl $class {
                                pub const DEFAULT: Self = Self {
                                    _super: CVtableOpt::<Super>::DEFAULT,
                                    I: CVtableOpt::<I>::DEFAULT,
                                    h: None,
                                };

                                pub const fn init_mixin_header(
                                    mixin_header: &mut [core::mem::MaybeUninit<MixinVtableHeader>],
                                ) {
                                    let (first, rest) = mixin_header.split_first_mut().unwrap();
                                    CVtableOpt::<Super>::init_mixin_header(rest);
                                    first.write(MixinVtableHeader::new::<super::$class>(
                                        core::mem::size_of::<CData<Super>>(),
                                        super::$class::MIXIN_HEADER_ENTRIES
                                            * core::mem::size_of::<MixinVtableHeader>()
                                            + core::mem::size_of::<CVtable<Super>>(),
                                    ));
                                }
                                pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                                    let ty = match ty {
                                        None => super::TYPE.as_type(),
                                        Some(ty) => ty,
                                    };
                                    self._super.init_header(Some(ty), offset);
                                    self.I
                                        .init_header(None, offset + core::mem::offset_of!($class, I));
                                }
                                pub const fn init(&mut self) {
                                    self._super.init();
                                    ::classes::vtable::vtable_opt_upcast_mut::<_, CVtableOpt<A>>(self)
                                        .0
                                        .f = Some(|this| {
                                        CData::<Self>::f(&unsafe {
                                            this.try_to_subtype().unwrap_unchecked()
                                        })
                                    });
                                    self.h = Some(|this| {
                                        CData::<Self>::h(&unsafe {
                                            this.try_to_subtype().unwrap_unchecked()
                                        })
                                    });
                                }
                                pub const fn assert_init(self) -> super::$class {
                                    super::$class {
                                        _super: self._super.assert_init(),
                                        I: self.I.assert_init(),
                                        h: self.h.unwrap(),
                                    }
                                }
                            }
                        }

                        pub static TYPE: TypeInfo<2> = TypeInfo::new_mixin_instance::<super::$class>(
                            CVtable::<Super>::TYPE,
                            unsafe { CVtable::<M>::TYPE.as_mixin_unchecked() },
                            [0, core::mem::offset_of!($class, I)],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!($class),
                        );
                    }

                    impl $class<RcDyn<$class>> {
                        #[inline]
                        pub fn f(&self) {
                            self.to_supertype::<CRc<A>>().f()
                        }
                        #[inline]
                        pub fn h(&self) {
                            self.to_mixin().h()
                        }
                    }
                    impl $class<RcDyn<$class>, NonVirtual> {
                        #[inline]
                        pub fn f(&self) {
                            CData::<Self>::f(self.as_virtual())
                        }
                        #[inline]
                        pub fn h(&self) {
                            CData::<Self>::h(self.as_virtual())
                        }
                    }
                    impl<V> $class<RcDyn<$class>, V> {
                        pub fn get_z(&self) -> usize {
                            self.0.z
                        }
                    }
                }
            };
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _C1 {
        ::classes::_mod_uses!(mod class C1);

        ::classes::_def_class!(class C1);
        ::classes::_def_class_extends!(C1: A_M);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct C1 {
                pub(super) _super: CData<A_M>,
                pub(super) w: usize,
            }
            impl C1 {
                pub fn new(mut _self: CRcUninit<Self>) -> CRc<Self> {
                    let _ = |Self { _super: _, w: _ }: Self| ();
                    #[allow(unused_unsafe)]
                    unsafe {
                        core::ptr::write(&raw mut (*_self.as_mut_ptr()).w, 3);
                    }
                    let _super = CData::<A_M>::new(_self.into_super());
                    unsafe { _super.into_subclass_unchecked() }
                }
                pub(super) fn f(this: &CRc<Self>) {
                    this.delegate_super().f();
                    println!("C1::f, w = {}", this.get_w());
                }
                pub(super) fn h(this: &CRc<Self>) {
                    this.delegate_super().h();
                    println!("C1::h, w = {}", this.get_w());
                }
                pub(super) fn i(this: &CRc<Self>) {
                    println!("C1::i, w = {}", this.get_w());
                }
                pub(super) fn j(this: &CRc<Self>) {
                    println!("C1::j, w = {}", this.get_w());
                }
            }
        }

        mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            pub struct C1 {
                pub(super) _super: CVtable<A_M>,
                j: fn(&CRc<Self>),
            }

            impl C1 {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }

            pub struct DebugVtableLayout<'a> {
                this: &'a self::C1,
                offset: usize,
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
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self.this._super.debug_vtable_layout(offset_of!(_super)),
                    );
                    dbg.field("j", &offset_of!(j));
                    dbg.field("\'end", &(self.offset + core::mem::size_of::<C1>()));
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct C1 {
                    pub(in super::super) _super: CVtableOpt<A_M>,
                    j: Option<fn(&CRc<Self>)>,
                }

                impl C1 {
                    pub const DEFAULT: Self = Self {
                        _super: CVtableOpt::<A_M>::DEFAULT,
                        j: None,
                    };

                    pub const fn init_mixin_header(
                        mixin_header: &mut [core::mem::MaybeUninit<MixinVtableHeader>],
                    ) {
                        CVtableOpt::<A_M>::init_mixin_header(mixin_header);
                    }

                    pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => super::TYPE.as_type(),
                            Some(ty) => ty,
                        };
                        self._super.init_header(Some(ty), offset);
                    }

                    pub const fn init(&mut self) {
                        self._super.init();
                        ::classes::vtable::vtable_opt_upcast_mut::<_, CVtableOpt<A>>(self)
                            .0
                            .f = Some(|this| {
                            CData::<Self>::f(&unsafe { this.try_to_subtype().unwrap_unchecked() })
                        });
                        ::classes::vtable::vtable_opt_upcast_mut::<_, CVtableOpt<M>>(self)
                            .0
                            .h = Some(|this| {
                            CData::<Self>::h(&unsafe { this.try_to_subtype().unwrap_unchecked() })
                        });
                        ::classes::vtable::vtable_opt_upcast_mut::<_, CVtableOpt<I>>(self)
                            .0
                            .i = Some(|this| {
                            CData::<Self>::i(&unsafe { this.try_to_subtype().unwrap_unchecked() })
                        });
                        self.j = Some(CData::<Self>::j);
                    }
                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            _super: self._super.assert_init(),
                            j: self.j.unwrap(),
                        }
                    }
                }
            }

            pub static TYPE: TypeInfo<0> = TypeInfo::new_concrete_class::<super::C1>(
                Some(CVtable::<A_M>::TYPE),
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "C1",
            );
        }

        static VTABLE: VtableWithMixinHeader<vtable::C1, { vtable::C1::MIXIN_HEADER_ENTRIES }> = {
            let mut vtable = MaybeUninitVtableWithMixinHeader::new(vtable::opt::C1::DEFAULT);
            vtable::opt::C1::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(None, 0);
            vtable_opt.init();
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            VtableWithMixinHeader::new(headers, vtable_opt.assert_init())
        };
        unsafe impl ConcreteClass for C1 {
            const VTABLE: NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }

        impl C1<RcDyn<C1>> {
            #[inline]
            pub fn new() -> Self {
                CData::<Self>::new(CRcUninit::<Self>::new_uninit())
            }

            #[inline]
            pub fn f(&self) {
                self.to_supertype::<CRc<A>>().f();
            }
            #[inline]
            pub fn h(&self) {
                self.to_supertype::<CRc<M>>().h();
            }
            #[inline]
            pub fn i(&self) {
                self.to_supertype::<CRc<I>>().i();
            }
            #[inline]
            pub fn j(&self) {
                CData::<Self>::j(self)
            }
        }

        impl C1<RcDyn<C1>, NonVirtual> {
            #[inline]
            pub fn f(&self) {
                CData::<Self>::f(self.as_virtual());
            }
            #[inline]
            pub fn h(&self) {
                CData::<Self>::h(self.as_virtual());
            }
            #[inline]
            pub fn i(&self) {
                CData::<Self>::i(self.as_virtual());
            }
            #[inline]
            pub fn j(&self) {
                CData::<Self>::j(self.as_virtual());
            }
        }

        impl<V> C1<RcDyn<C1>, V> {
            pub fn get_w(&self) -> usize {
                self.0.w
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _C2 {
        ::classes::_mod_uses!(mod class C2);

        ::classes::_def_class!(class C2);
        ::classes::_def_class_extends!(C2: B_M);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct C2 {
                pub(super) _super: CData<B_M>,
                pub(super) v: usize,
            }
            impl C2 {
                pub fn new(mut _self: CRcUninit<Self>) -> CRc<Self> {
                    let _ = |Self { _super: _, v: _ }: Self| ();
                    #[allow(unused_unsafe)]
                    unsafe {
                        core::ptr::write(&raw mut (*_self.as_mut_ptr()).v, 4);
                    }
                    let _super = CData::<B_M>::new(_self.into_super());
                    unsafe { _super.into_subclass_unchecked() }
                }
                pub(super) fn f(this: &CRc<Self>) {
                    this.delegate_super().f();
                    println!("C2::f, v = {}", this.get_v());
                }
                pub(super) fn g(this: &CRc<Self>) {
                    this.delegate_super().g();
                    println!("C2::g, v = {}", this.get_v());
                }
                pub(super) fn h(this: &CRc<Self>) {
                    this.delegate_super().h();
                    println!("C2::h, v = {}", this.get_v());
                }
                pub(super) fn i(this: &CRc<Self>) {
                    println!("C2::i, v = {}", this.get_v());
                }
                pub(super) fn j(this: &CRc<Self>) {
                    println!("C2::j, v = {}", this.get_v());
                }
            }
        }

        mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            pub struct C2 {
                pub(super) _super: CVtable<B_M>,
                j: fn(&CRc<Self>),
            }

            impl C2 {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }

            pub struct DebugVtableLayout<'a> {
                this: &'a self::C2,
                offset: usize,
            }

            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    macro_rules! offset_of {
                        ($field:ident) => {
                            self.offset + ::core::mem::offset_of!(C2, $field)
                        };
                    }
                    let mut dbg = f.debug_struct(stringify!(C2));
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self.this._super.debug_vtable_layout(offset_of!(_super)),
                    );
                    dbg.field("j", &offset_of!(j));
                    dbg.field("\'end", &(self.offset + core::mem::size_of::<C2>()));
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct C2 {
                    pub(in super::super) _super: CVtableOpt<B_M>,
                    j: Option<fn(&CRc<Self>)>,
                }

                impl C2 {
                    pub const DEFAULT: Self = Self {
                        _super: CVtableOpt::<B_M>::DEFAULT,
                        j: None,
                    };

                    pub const fn init_mixin_header(
                        mixin_header: &mut [core::mem::MaybeUninit<MixinVtableHeader>],
                    ) {
                        CVtableOpt::<B_M>::init_mixin_header(mixin_header);
                    }

                    pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => super::TYPE.as_type(),
                            Some(ty) => ty,
                        };
                        self._super.init_header(Some(ty), offset);
                    }

                    pub const fn init(&mut self) {
                        self._super.init();
                        ::classes::vtable::vtable_opt_upcast_mut::<_, CVtableOpt<A>>(self)
                            .0
                            .f = Some(|this| {
                            CData::<Self>::f(&unsafe { this.try_to_subtype().unwrap_unchecked() })
                        });
                        ::classes::vtable::vtable_opt_upcast_mut::<_, CVtableOpt<B>>(self)
                            .0
                            .g = Some(|this| {
                            CData::<Self>::g(&unsafe { this.try_to_subtype().unwrap_unchecked() })
                        });
                        ::classes::vtable::vtable_opt_upcast_mut::<_, CVtableOpt<M>>(self)
                            .0
                            .h = Some(|this| {
                            CData::<Self>::h(&unsafe { this.try_to_subtype().unwrap_unchecked() })
                        });
                        ::classes::vtable::vtable_opt_upcast_mut::<_, CVtableOpt<I>>(self)
                            .0
                            .i = Some(|this| {
                            CData::<Self>::i(&unsafe { this.try_to_subtype().unwrap_unchecked() })
                        });
                        self.j = Some(CData::<Self>::j);
                    }
                    pub const fn assert_init(self) -> super::C2 {
                        super::C2 {
                            _super: self._super.assert_init(),
                            j: self.j.unwrap(),
                        }
                    }
                }
            }

            pub static TYPE: TypeInfo<0> = TypeInfo::new_concrete_class::<super::C2>(
                Some(CVtable::<B_M>::TYPE),
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "C2",
            );
        }

        static VTABLE: VtableWithMixinHeader<vtable::C2, { CVtable::<C2>::MIXIN_HEADER_ENTRIES }> = {
            let mut vtable = MaybeUninitVtableWithMixinHeader::new(vtable::opt::C2::DEFAULT);
            vtable::opt::C2::init_mixin_header(vtable.headers_mut());
            let vtable_opt = vtable.vtable_opt_mut();
            vtable_opt.init_header(None, 0);
            vtable_opt.init();
            let (headers, vtable_opt) = unsafe { vtable.headers_assume_init() };
            VtableWithMixinHeader::new(headers, vtable_opt.assert_init())
        };
        unsafe impl ConcreteClass for C2 {
            const VTABLE: NonNull<Self::Vtable> = VTABLE.vtable_ptr();
        }

        impl C2<RcDyn<C2>> {
            #[inline]
            pub fn new() -> Self {
                CData::<Self>::new(CRcUninit::<Self>::new_uninit())
            }

            #[inline]
            pub fn f(&self) {
                self.to_supertype::<CRc<A>>().f();
            }
            #[inline]
            pub fn g(&self) {
                self.to_supertype::<CRc<B>>().g();
            }
            #[inline]
            pub fn h(&self) {
                self.to_supertype::<CRc<M>>().h();
            }
            #[inline]
            pub fn i(&self) {
                self.to_supertype::<CRc<I>>().i();
            }
            #[inline]
            pub fn j(&self) {
                CData::<Self>::j(self)
            }
        }

        impl C2<RcDyn<C2>, NonVirtual> {
            #[inline]
            pub fn f(&self) {
                CData::<Self>::f(self.as_virtual());
            }
            #[inline]
            pub fn g(&self) {
                CData::<Self>::g(self.as_virtual());
            }
            #[inline]
            pub fn h(&self) {
                CData::<Self>::h(self.as_virtual());
            }
            #[inline]
            pub fn i(&self) {
                CData::<Self>::i(self.as_virtual());
            }
            #[inline]
            pub fn j(&self) {
                CData::<Self>::j(self.as_virtual());
            }
        }

        impl<V> C2<RcDyn<C2>, V> {
            pub fn get_v(&self) -> usize {
                self.0.v
            }
        }
    }
}

mod mixins {
    pub use _classes::{_A_M::A_M, _B_M::B_M};

    pub mod _classes {
        use crate::expanded::{A, B, BUF, I, M, MODULE_PATH};

        crate::_M!(_A_M, A_M, A<T, V>);
        crate::_M!(_B_M, B_M, B<T, V>);
    }
}

#[test]
fn mixin() {
    BUF.take();
    let c1 = C1::new();
    let c2 = C2::new();

    c1.f();
    c1.h();
    c1.i();
    c1.j();

    let c1: CRc<M> = c1.to_mixin();
    c1.f();
    c1.h();
    println!("z = {}", c1.get_z());

    c2.f();
    c2.g();
    c2.h();
    c2.i();
    c2.j();

    let c2: CRc<M> = c2.to_mixin();
    c2.f();
    c2.h();
    println!("z = {}", c2.get_z());
    assert_eq!(BUF.take(), super::EXPECTED_OUTPUT);
}
