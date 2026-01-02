expand_test_case! {
    class no_super {
        #[no_super]
        class Object {
            pub fn new() -> Self {
                Self {}
            }
        }
    } => {
        const MODULE_PATH: &str = ::core::module_path!();

        use ::classes::prelude::*;

        #[allow(unused_imports)]
        pub(super) use _Object::Object;

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        mod _Object {
            ::classes::_mod_uses! { mod class Object }

            ::classes::_def_class! { class Object }

            mod data {
                ::classes::_mod_uses! { mod data }

                #[repr(C)]
                pub struct Object { }

                impl Object {
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
                            let _ = |Self { } : Self| ();
                            ::classes::prelude::CRc::<Self>::_from_inner(_self.assume_init())
                        }
                    }
                }
            }
            mod vtable {
                ::classes::_mod_uses! { mod vtable }

                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct Object {
                    header: ::classes::vtable::VtableHeader,
                }

                impl Object {
                    pub const fn debug_vtable_layout(&self, offset: usize) -> self::DebugVtableLayout<'_> {
                        self::DebugVtableLayout { this: self, offset }
                    }
                }
                pub struct DebugVtableLayout <'a> {
                    this: &'a self::Object ,
                    offset: usize ,
                }
                impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                    #[allow(unused_macros)]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        macro_rules! offset_of {
                            ($field:ident) => {
                                self.offset + ::core::mem::offset_of!(Object, $field)
                            };
                        }
                        let mut dbg = f.debug_struct(stringify!(Object));
                        dbg.field("\'start" , &self.offset);
                        dbg.field("header", &self.this.header);
                        dbg.field("\'end", &(self.offset + ::core::mem::size_of::<Object>())) ;
                        dbg.finish()
                    }
                }

                pub(super) mod opt {
                    ::classes::_mod_uses! { mod vtable::opt }

                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    #[derive(Default)]
                    pub struct Object {
                        header: ::classes::vtable::VtableHeaderOpt,
                    }

                    impl Object {
                        pub const DEFAULT: Self = Self {
                            header: ::classes::vtable::VtableHeaderOpt::DEFAULT,
                        };

                        pub const fn init_mixin_header(
                            mixin_header: &mut [::core::mem::MaybeUninit<::classes::vtable::MixinVtableHeader>],
                        ) {
                            assert!(mixin_header.is_empty());
                        }
                        pub const fn init_header(&mut self, ty: ::core::option::Option<::classes::vtable::Type>, offset: usize) {
                            let ty = match ty {
                                ::core::option::Option::None => Self::TYPE,
                                ::core::option::Option::Some(ty) => ty,
                            };
                            self.header = ::classes::vtable::VtableHeaderOpt::new(ty, offset);
                        }
                        #[allow(unused_unsafe)]
                        pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) { }
                        #[track_caller]
                        pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                            ::classes::prelude::CVtable::<Self> {
                                header: self.header.assert_init(),
                            }
                        }
                    }
                }

                pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                    ::classes::vtable::TypeInfo::new_concrete_class::<super::Object>(
                        ::core::option::Option::None,
                        [],
                        // #[cfg(debug_assertions)]
                        MODULE_PATH,
                        // #[cfg(debug_assertions)]
                        stringify!(Object),
                    );
            }
            ::classes::assert_layout_eq! {
                vtable::Object,
                vtable::opt::Object,
                {}
            }
            ::classes::_def_concrete_class! { Object }
            impl Object<::classes::ptr::RcDyn<Object>> {
                #[inline]
                pub fn new() -> Self {
                    ::classes::prelude::CData::<Self>::new(::classes::prelude::CRcUninit::<Self>::new_uninit(),)
                }
            }
            impl Object<::classes::ptr::RcDyn<Object>, ::classes::class::NonVirtual> {}
            impl Object<::classes::ptr::RcDyn<Object>> {}
        }
    }
}
