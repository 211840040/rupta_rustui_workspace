expand_test_case! {
    build_context {
        abstract class BuildContext {
            pub fn widget(&self) -> CRc<Widget>;
        }
    } => {
        #[allow(unused_imports)]
        use _classes::BuildContext;
        use ::classes::prelude::*;

        const MODULE_PATH: &str = ::core::module_path!();

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
                ::classes::_mod_uses! { mod class BuildContext }

                ::classes::_def_class! { class BuildContext }
                type Super = ::classes::object::Object;
                ::classes::_def_class_extends! { BuildContext : Object }

                mod data {
                    ::classes::_mod_uses! { mod data }

                    pub(super) type Super = ::classes::prelude::CData<super::Super>;
                    #[repr(C)]
                    pub struct BuildContext {
                        pub(super) _super: Super,
                    }

                    impl BuildContext {
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
                    }
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }

                    pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct BuildContext {
                        pub(super) _super: Super,
                        pub widget: fn(&::classes::prelude::CRc<Self>,) -> CRc<Widget>,
                    }

                    impl BuildContext {
                        pub const fn debug_vtable_layout(&self, offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }
                    pub struct DebugVtableLayout <'a> {
                        this: &'a self::BuildContext ,
                        offset: usize ,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(BuildContext, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(BuildContext));
                            dbg.field("\'start" , &self.offset);
                            dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                            dbg.field(stringify!(widget), &offset_of!(widget));
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<BuildContext>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable::opt }

                        pub(in super::super) type Super = ::classes::prelude::CVtableOpt<super::super::Super>;
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct BuildContext {
                            pub(in super::super) _super: Super,
                            pub widget: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,) -> CRc<Widget> >,
                        }

                        impl BuildContext {
                            pub const DEFAULT: Self = Self {
                                _super: Super::DEFAULT,
                                widget: ::core::option::Option::None,
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
                            }
                            #[track_caller]
                            pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                                ::classes::prelude::CVtable::<Self> {
                                    _super: self._super.assert_init(),
                                    widget: self.widget.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(BuildContext),
                                        "::",
                                        stringify!(widget),
                                        "` is not implemented",
                                    )),
                                }
                            }
                        }
                    }

                    pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                        ::classes::vtable::TypeInfo::new_abstract_class::<super::BuildContext>(
                            ::core::option::Option::Some(Super::TYPE),
                            [],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(BuildContext),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::BuildContext,
                    vtable::opt::BuildContext,
                    {widget}
                }
                impl BuildContext<::classes::ptr::RcDyn<BuildContext>> {
                    #[inline]
                    pub fn widget(&self,) -> CRc<Widget> {
                        { (self.0.vtable().widget)(self, ) }
                    }
                }
                impl BuildContext<::classes::ptr::RcDyn<BuildContext>, ::classes::class::NonVirtual> {}
                impl BuildContext<::classes::ptr::RcDyn<BuildContext>> {}
            }
        }
    }
}

expand_test_case! {
    element {
        abstract class Element implements BuildContext {
            struct {
                parent: Option<CWeak<Element>> = None,
                widget: Option<CWeak<Widget>>,
                dirty: bool = true,
            }
            pub fn new(widget: Option<CRc<Widget>>) -> Self {
                println!("Element");
                Self { widget, .. }
            }
            pub override fn BuildContext::widget(&self) -> CRc<Widget> {
                self.0.widget.cloned().unwrap().upgrade().unwrap()
            }
            pub fn rebuild(&self, force: bool) {
                println!("Element::rebuild");
                if self.0.dirty.get() || force {
                    self.perform_rebuild();
                }
            }
            pub fn perform_rebuild(&self) {
                println!("Element::perform_rebuild");
                self.0.dirty.set(false);
            }
            pub fn mount(&self, parent: Option<CWeak<Element>>) {
                println!("Element::mount");
                self.0.parent.set(parent);
            }
            pub fn mark_needs_build(&self) {
                println!("Element::mark_needs_build");
                self.0.dirty.set(true);
                self.rebuild(false);
            }
        }
    } => {
        #[allow(unused_imports)]
        use _classes::Element;

        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();

        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;

            #[allow(unused_imports)]
            pub(super) use _Element::Element;

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod _Element {
                ::classes::_mod_uses! { mod class Element }

                ::classes::_def_class! { class Element }
                type Super = ::classes::object::Object;
                ::classes::_def_class_extends! { Element : Object }
                ::classes::_def_class_impl! { Element: BuildContext }

                mod data {
                    ::classes::_mod_uses! { mod data }

                    pub(super) type Super = ::classes::prelude::CData<super::Super>;
                    #[repr(C)]
                    pub struct Element {
                        pub(super) _super: Super,
                        pub(super) parent: ::core::cell::Cell<Option<CWeak<Element> > >,
                        pub(super) widget: ::core::cell::Cell<Option<CWeak<Widget> > >,
                        pub(super) dirty: ::core::cell::Cell<bool>,
                    }

                    impl Element {
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
                        pub fn new(mut _self: ::classes::prelude::CRcUninit<Self>, widget: Option<CRc<Widget> >) -> ::classes::prelude::CRc<Self> {
                            println!("Element");
                            unsafe {
                                ::core::ptr::write(
                                    &raw mut (*_self.as_mut_ptr()).widget,
                                    ::classes::get_set::New::new_cell(widget),
                                );
                                ::core::ptr::write(
                                    &raw mut (*_self.as_mut_ptr()).parent,
                                    ::classes::get_set::New::new_cell(None),
                                );
                                ::core::ptr::write(
                                    &raw mut (*_self.as_mut_ptr()).dirty,
                                    ::classes::get_set::NewCopy::new_cell(true),
                                );
                                let _ = |Self { _super, widget: _, parent: _, dirty: _, } : Self| ();
                                ::classes::prelude::CData::<::classes::object::Object>::new(_self.into_super())
                                    .into_subclass_unchecked()
                            }
                        }
                        pub(super) fn widget(_self: &::classes::prelude::CRc<Self>,) -> CRc<Widget> {
                            _self.0.widget.cloned().unwrap().upgrade().unwrap()
                        }
                        pub(super) fn rebuild(_self: &::classes::prelude::CRc<Self>, force: bool) {
                            println!("Element::rebuild");
                            if _self.0.dirty.get() || force {
                                _self.perform_rebuild();
                            }
                        }
                        pub(super) fn perform_rebuild(_self: &::classes::prelude::CRc<Self>,) {
                            println!("Element::perform_rebuild");
                            _self.0.dirty.set(false);
                        }
                        pub(super) fn mount(_self: &::classes::prelude::CRc<Self>, parent: Option<CWeak<Element> >) {
                            println!("Element::mount");
                            _self.0.parent.set(parent);
                        }
                        pub(super) fn mark_needs_build(_self: &::classes::prelude::CRc<Self>,) {
                            println!("Element::mark_needs_build");
                            _self.0.dirty.set(true);
                            _self.rebuild(false);
                        }
                    }
                }

                mod vtable {
                    ::classes::_mod_uses! { mod vtable }

                    pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct Element {
                        pub(super) _super: Super,
                        pub rebuild: fn(&::classes::prelude::CRc<Self>, force: bool,),
                        pub perform_rebuild: fn(&::classes::prelude::CRc<Self>,),
                        pub mount: fn(&::classes::prelude::CRc<Self>, parent: Option<CWeak<Element> >,),
                        pub mark_needs_build: fn(&::classes::prelude::CRc<Self>,),
                        pub BuildContext: ::classes::prelude::CVtable<BuildContext>,
                    }

                    impl Element {
                        pub const fn debug_vtable_layout(&self, offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }
                    pub struct DebugVtableLayout<'a> {
                        this: &'a self::Element,
                        offset: usize,
                    }

                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(Element, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(Element));
                            dbg.field("\'start", &self.offset);
                            dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                            dbg.field(stringify!(rebuild), &offset_of!(rebuild));
                            dbg.field(stringify!(perform_rebuild), &offset_of!(perform_rebuild));
                            dbg.field(stringify!(mount), &offset_of!(mount));
                            dbg.field(stringify!(mark_needs_build), &offset_of!(mark_needs_build));
                            dbg.field(stringify!(BuildContext), &self.this.BuildContext.debug_vtable_layout(offset_of!(BuildContext)));
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<Element>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable::opt }

                        pub(in super::super) type Super = ::classes::prelude::CVtableOpt<super::super::Super>;
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct Element {
                            pub(in super::super) _super: Super,
                            pub rebuild: ::core::option::Option<fn(&::classes::prelude::CRc<Self>, force: bool,)>,
                            pub perform_rebuild: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,)>,
                            pub mount: ::core::option::Option<fn(&::classes::prelude::CRc<Self>, parent: Option<CWeak<Element> >,)>,
                            pub mark_needs_build: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,)>,
                            pub BuildContext: ::classes::prelude::CVtableOpt<BuildContext>,
                        }
                        impl Element {
                            pub const DEFAULT: Self = Self {
                                _super: Super::DEFAULT,
                                rebuild: ::core::option::Option::None,
                                perform_rebuild: ::core::option::Option::None,
                                mount: ::core::option::Option::None,
                                mark_needs_build: ::core::option::Option::None,
                                BuildContext: ::classes::prelude::CVtableOpt::<BuildContext>::DEFAULT,
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
                                self.BuildContext.init_header(::core::option::Option::None, offset + ::core::mem::offset_of!(::classes::prelude::CVtable<Self>, BuildContext),);
                            }
                            #[allow(unused_unsafe)]
                            pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                                Super::init(_self);
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<BuildContext>>(_self);
                                    ptr.widget = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::widget(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<BuildContext>>(_self, &mut offset) {
                                        ptr.widget = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::widget(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.rebuild = ::core::option::Option::Some(|this, force,| ::classes::prelude::CData::<Self>::rebuild(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        force,
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.rebuild = ::core::option::Option::Some(|this, force,| ::classes::prelude::CData::<Self>::rebuild(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                            force,
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.perform_rebuild = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::perform_rebuild(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.perform_rebuild = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::perform_rebuild(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.mount = ::core::option::Option::Some(|this, parent,| ::classes::prelude::CData::<Self>::mount(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        parent,
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.mount = ::core::option::Option::Some(|this, parent,| ::classes::prelude::CData::<Self>::mount(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                            parent,
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.mark_needs_build = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::mark_needs_build(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.mark_needs_build = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::mark_needs_build(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                            }

                            #[track_caller]
                            pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                                ::classes::prelude::CVtable::<Self> {
                                    _super: self._super.assert_init(),
                                    rebuild: self.rebuild.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(Element),
                                        "::",
                                        stringify!(rebuild),
                                        "` is not implemented",
                                    )),
                                    perform_rebuild: self.perform_rebuild.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(Element),
                                        "::",
                                        stringify!(perform_rebuild),
                                        "` is not implemented",
                                    )),
                                    mount: self.mount.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(Element),
                                        "::",
                                        stringify!(mount),
                                        "` is not implemented",
                                    )),
                                    mark_needs_build: self.mark_needs_build.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(Element),
                                        "::",
                                        stringify!(mark_needs_build),
                                        "` is not implemented",
                                    )),
                                    BuildContext: self.BuildContext.assert_init(),
                                }
                            }
                        }
                    }

                    pub static TYPE: ::classes::vtable::TypeInfo<1usize> =
                        ::classes::vtable::TypeInfo::new_abstract_class::<super::Element>(
                            ::core::option::Option::Some(Super::TYPE),
                            [(
                                ::classes::prelude::CVtable::<BuildContext>::TYPE,
                                ::core::mem::offset_of!(vtable::Element, BuildContext),
                            ),],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(Element),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::Element,
                    vtable::opt::Element,
                    {rebuild, perform_rebuild, mount, mark_needs_build, BuildContext}
                }

                impl Element<::classes::ptr::RcDyn<Element>> {
                    #[inline]
                    pub fn widget(&self,) -> CRc<Widget> {
                        { self.to_supertype::<::classes::prelude::CRc<BuildContext>>().widget() }.try_into().unwrap()
                    }
                    #[inline]
                    pub fn rebuild(&self, force: bool) {
                        { (self.0.vtable().rebuild)(self, force,) }
                    }
                    #[inline]
                    pub fn perform_rebuild(&self,) {
                        { (self.0.vtable().perform_rebuild)(self,) }
                    }
                    #[inline]
                    pub fn mount(&self, parent: Option<CWeak<Element> >) {
                        { (self.0.vtable().mount)(self, parent,) }
                    }
                    #[inline]
                    pub fn mark_needs_build(&self,) {
                        { (self.0.vtable().mark_needs_build)(self,) }
                    }
                }

                impl Element<::classes::ptr::RcDyn<Element>, ::classes::class::NonVirtual> {
                    #[inline]
                    pub fn widget(&self,) -> CRc<Widget> {
                        { ::classes::prelude::CData::<Self>::widget(self.as_virtual(), ) }
                    }
                    #[inline]
                    pub fn rebuild(&self, force: bool) {
                        { ::classes::prelude::CData::<Self>::rebuild(self.as_virtual(), force,) }
                    }
                    #[inline]
                    pub fn perform_rebuild(&self,) {
                        { ::classes::prelude::CData::<Self>::perform_rebuild(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn mount(&self, parent: Option<CWeak<Element> >) {
                        { ::classes::prelude::CData::<Self>::mount(self.as_virtual(), parent,) }
                    }
                    #[inline]
                    pub fn mark_needs_build(&self,) {
                        { ::classes::prelude::CData::<Self>::mark_needs_build(self.as_virtual(),) }
                    }
                }
                impl Element<::classes::ptr::RcDyn<Element>> {
                    #[inline]
                    pub(in super::super) fn get_parent(&self) -> <Option<CWeak<Element> > as ::classes::get_set::GetSet>::Get {
                        ::classes::get_set::GetSet::cell_get(&self.0.parent)
                    }
                    #[inline]
                    pub(in super::super) fn set_parent<_T: ::core::convert::Into<<Option<CWeak<Element> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        parent: _T
                    ) {
                        ::classes::get_set::GetSet::cell_set(&self.0.parent, parent.into());
                    }
                    #[inline]
                    #[must_use]
                    pub(in super::super) fn replace_parent<_T: ::core::convert::Into<<Option<CWeak<Element> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        parent: _T
                    ) -> <Option<CWeak<Element> > as ::classes::get_set::GetSet>::Get {
                        let old = self.get_parent();
                        self.set_parent(parent);
                        old
                    }
                    #[inline]
                    pub(in super::super) fn update_parent_with<
                        _T: ::core::convert::Into<<Option<CWeak<Element> > as ::classes::get_set::GetSet>::Set>,
                        _F: ::core::ops::FnOnce(<Option<CWeak<Element> > as ::classes::get_set::GetSet>::Get) -> _T
                    >(&self, f: _F) {
                        self.set_parent(f(self.get_parent()));
                    }
                    #[inline]
                    pub(in super::super) fn raw_get_parent(&self) -> &::core::cell::Cell<Option<CWeak<Element> > > {
                        &self.0.parent
                    }

                    #[inline]
                    pub(in super::super) fn get_widget(&self) -> <Option<CWeak<Widget> > as ::classes::get_set::GetSet>::Get {
                        ::classes::get_set::GetSet::cell_get(&self.0.widget)
                    }

                    #[inline]
                    pub(in super::super) fn set_widget<_T: ::core::convert::Into<<Option<CWeak<Widget> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        widget: _T
                    ) {
                        ::classes::get_set::GetSet::cell_set(&self.0.widget, widget.into());
                    }
                    #[inline]
                    #[must_use]
                    pub(in super::super) fn replace_widget<_T: ::core::convert::Into<<Option<CWeak<Widget> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        widget: _T
                    ) -> <Option<CWeak<Widget> > as ::classes::get_set::GetSet>::Get {
                        let old = self.get_widget();
                        self.set_widget(widget);
                        old
                    }
                    #[inline]
                    pub(in super::super) fn update_widget_with<
                        _T: ::core::convert::Into<<Option<CWeak<Widget> > as ::classes::get_set::GetSet>::Set>,
                        _F: ::core::ops::FnOnce(<Option<CWeak<Widget> > as ::classes::get_set::GetSet>::Get) -> _T
                    >(&self, f: _F) {
                        self.set_widget(f(self.get_widget()));
                    }
                    #[inline]
                    pub(in super::super) fn raw_get_widget(&self) -> &::core::cell::Cell<Option<CWeak<Widget> > > {
                        &self.0.widget
                    }

                    #[inline]
                    pub(in super::super) fn get_dirty(&self) -> bool {
                        ::classes::get_set::GetSetCopy::cell_get(&self.0.dirty)
                    }
                    #[inline]
                    pub(in super::super) fn set_dirty<_T: ::core::convert::Into<bool> >(
                        &self,
                        dirty: _T
                    ) {
                        ::classes::get_set::GetSetCopy::cell_set(&self.0.dirty, dirty.into());
                    }
                    #[inline]
                    #[must_use]
                    pub(in super::super) fn replace_dirty<_T: ::core::convert::Into<bool> >(
                        &self,
                        dirty: _T
                    ) -> bool {
                        let old = self.get_dirty();
                        self.set_dirty(dirty);
                        old
                    }
                    #[inline]
                    pub(in super::super) fn update_dirty_with<
                        _T: ::core::convert::Into<bool>,
                        _F: ::core::ops::FnOnce(bool) -> _T
                    >(&self, f: _F) {
                        self.set_dirty(f(self.get_dirty()));
                    }
                    #[inline]
                    pub(in super::super) fn raw_get_dirty(&self) -> &::core::cell::Cell<bool> {
                        &self.0.dirty
                    }
                }
            }
        }
    }
}

expand_test_case! {
    component_element {
        abstract class ComponentElement extends Element {
            struct {
                child: Option<CRc<Element>>,
            }
            pub fn new(widget: Option<CRc<Widget>>) -> Self {
                let self = Self {
                    super: Super::new(widget),
                    child: None,
                };
                println!("ComponentElement");
                self
            }

            pub fn first_build(&self) {
                println!("ComponentElement::first_build");
                super.rebuild(false);
            }

            pub fn build(&self) -> CRc<Widget>;

            pub override fn perform_rebuild(&self) {
                println!("ComponentElement::perform_rebuild");
                super.perform_rebuild();
            }

            pub override fn mount(&self, parent: Option<CWeak<Element>>) {
                println!("ComponentElement::mount");
                super.mount(parent);
                self.first_build();
            }
        }
    } => {
        #[allow(unused_imports)]
        use _classes::ComponentElement;

        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();

        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;
            #[allow(unused_imports)]
            pub(super) use _ComponentElement::ComponentElement;

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]

            mod _ComponentElement {
                ::classes::_mod_uses! { mod class ComponentElement }
                ::classes::_def_class! { class ComponentElement }
                type Super = Element;
                ::classes::_def_class_extends! { ComponentElement : Element }

                mod data {
                    ::classes::_mod_uses! { mod data }
                    pub(super) type Super = ::classes::prelude::CData<super::Super>;
                    #[repr(C)]
                    pub struct ComponentElement {
                        pub(super) _super: Super,
                        pub(super) child: ::core::cell::Cell<Option<CRc<Element> > >,
                    }
                    impl ComponentElement {
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
                        pub fn new(mut _self: ::classes::prelude::CRcUninit<Self>, widget: Option<CRc<Widget> >) -> ::classes::prelude::CRc<Self> {
                            let _self: ::classes::prelude::CRc<Self> = unsafe {
                                ::core::ptr::write(
                                    &raw mut (*_self.as_mut_ptr()).child,
                                    ::classes::get_set::New::new_cell(None),
                                );
                                let _ = |Self { _super, child: _, } : Self| ();
                                Super::new(_self.into_super(), widget)
                                    .into_subclass_unchecked()
                            };
                            println!("ComponentElement");
                            _self
                        }
                        pub(super) fn first_build(_self: &::classes::prelude::CRc<Self>,) {
                            println!("ComponentElement::first_build");
                            { _self.delegate_super() }.rebuild(false);
                        }
                        pub(super) fn perform_rebuild(_self: &::classes::prelude::CRc<Self>,) {
                            println!("ComponentElement::perform_rebuild");
                            { _self.delegate_super() }.perform_rebuild();
                        }
                        pub(super) fn mount(_self: &::classes::prelude::CRc<Self>, parent: Option<CWeak<Element> >) {
                            println!("ComponentElement::mount");
                            { _self.delegate_super() }.mount(parent);
                            _self.first_build();
                        }
                    }
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }
                    pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct ComponentElement {
                        pub(super) _super: Super,
                        pub first_build: fn(&::classes::prelude::CRc<Self>,),
                        pub build: fn(&::classes::prelude::CRc<Self>,) -> CRc<Widget>,
                    }

                    impl ComponentElement {
                        pub const fn debug_vtable_layout(&self, offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }
                    pub struct DebugVtableLayout<'a> {
                        this: &'a self::ComponentElement,
                        offset: usize,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(ComponentElement, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(ComponentElement));
                            dbg.field("\'start", &self.offset);
                            dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                            dbg.field(stringify!(first_build), &offset_of!(first_build));
                            dbg.field(stringify!(build), &offset_of!(build));
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<ComponentElement>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable :: opt }
                        pub(in super::super) type Super = ::classes::prelude::CVtableOpt<super::super::Super>;
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct ComponentElement {
                            pub(in super::super) _super: Super,
                            pub first_build: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,)>,
                            pub build: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,) -> CRc<Widget> >,
                        }
                        impl ComponentElement {
                            pub const DEFAULT: Self = Self {
                                _super: Super::DEFAULT,
                                first_build: ::core::option::Option::None,
                                build: ::core::option::Option::None,
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
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.first_build = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::first_build(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.first_build = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::first_build(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Super>>(_self);
                                    ptr.perform_rebuild = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::perform_rebuild(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Super>>(_self, &mut offset) {
                                        ptr.perform_rebuild = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::perform_rebuild(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Super>>(_self);
                                    ptr.mount = ::core::option::Option::Some(|this, parent,| ::classes::prelude::CData::<Self>::mount(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        parent,
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Super>>(_self, &mut offset) {
                                        ptr.mount = ::core::option::Option::Some(|this, parent,| ::classes::prelude::CData::<Self>::mount(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                            parent,
                                        ).into());
                                    }
                                }
                            }
                            #[track_caller]
                            pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                                ::classes::prelude::CVtable::<Self> {
                                    _super: self._super.assert_init(),
                                    first_build: self.first_build.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(ComponentElement),
                                        "::",
                                        stringify!(first_build),
                                        "` is not implemented",
                                    )),
                                    build: self.build.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(ComponentElement),
                                        "::",
                                        stringify!(build),
                                        "` is not implemented",
                                    )),
                                }
                            }
                        }
                    }
                    pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                        ::classes::vtable::TypeInfo::new_abstract_class::<super::ComponentElement>(
                            ::core::option::Option::Some(Super::TYPE),
                            [],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(ComponentElement),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::ComponentElement,
                    vtable::opt::ComponentElement,
                    {first_build, build}
                }
                impl ComponentElement<::classes::ptr::RcDyn<ComponentElement>> {
                    #[inline]
                    pub fn first_build(&self,) {
                        { (self.0.vtable().first_build)(self,) }
                    }
                    #[inline]
                    pub fn build(&self,) -> CRc<Widget> {
                        { (self.0.vtable().build)(self,) }
                    }
                    #[inline]
                    pub fn perform_rebuild(&self,) {
                        { self.as_super().perform_rebuild() }.try_into().unwrap()
                    }
                    #[inline]
                    pub fn mount(&self, parent: Option<CWeak<Element> >) {
                        { self.as_super().mount(parent,) }.try_into().unwrap()
                    }
                }
                impl ComponentElement<::classes::ptr::RcDyn<ComponentElement>, ::classes::class::NonVirtual> {
                    #[inline]
                    pub fn first_build(&self,) {
                        { ::classes::prelude::CData::<Self>::first_build(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn perform_rebuild(&self,) {
                        { ::classes::prelude::CData::<Self>::perform_rebuild(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn mount(&self, parent: Option<CWeak<Element> >) {
                        { ::classes::prelude::CData::<Self>::mount(self.as_virtual(), parent,) }
                    }
                }
                impl ComponentElement<::classes::ptr::RcDyn<ComponentElement>> {
                    #[inline]
                    pub(in super::super) fn get_child(&self) -> <Option<CRc<Element> > as ::classes::get_set::GetSet>::Get {
                        ::classes::get_set::GetSet::cell_get(&self.0.child)
                    }
                    #[inline]
                    pub(in super::super) fn set_child<_T: ::core::convert::Into<<Option<CRc<Element> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        child: _T
                    ) {
                        ::classes::get_set::GetSet::cell_set(&self.0.child, child.into());
                    }
                    #[inline]
                    #[must_use]
                    pub(in super::super) fn replace_child<_T: ::core::convert::Into<<Option<CRc<Element> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        child: _T
                    ) -> <Option<CRc<Element> > as ::classes::get_set::GetSet>::Get {
                        let old = self.get_child();
                        self.set_child(child);
                        old
                    }
                    #[inline]
                    pub(in super::super) fn update_child_with<
                        _T: ::core::convert::Into<<Option<CRc<Element> > as ::classes::get_set::GetSet>::Set>,
                        _F: ::core::ops::FnOnce(<Option<CRc<Element> > as ::classes::get_set::GetSet>::Get) -> _T
                    >(&self, f: _F) {
                        self.set_child(f(self.get_child()));
                    }
                    #[inline]
                    pub(in super::super) fn raw_get_child(&self) -> &::core::cell::Cell<Option<CRc<Element> > > {
                        &self.0.child
                    }
                }
            }
        }
    }
}

expand_test_case! {
    stateful_element {
        class StatefulElement extends ComponentElement {
            struct {
                state: Option<CRc<State>>,
            }
            pub fn new(widget: CRc<StatefulWidget>) -> Self {
                let state = Some(widget.create_state());
                let self = Self { super: Super::new(Some(widget.into_super())), state };
                println!("StatefulElement");
                self
            }
            pub fn state(&self) -> CRc<State> {
                self.0.state.cloned().unwrap()
            }
            pub fn build(&self) -> CRc<Widget> {
                self.state()
                    .build(/* self.into_ancestor::<Element>().into_impl() */)
            }
            pub override fn first_build(&self) {
                println!("StatefulElement::first_build");
                self.state().init_state();
                super.first_build();
            }
            pub override fn Element::perform_rebuild(&self) {
                println!("StatefulElement::perform_rebuild");
                super.perform_rebuild();
            }
        }
    } => {
        #[allow(unused_imports)]
        use _classes::StatefulElement;

        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();

        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;
            #[allow(unused_imports)]
            pub(super) use _StatefulElement::StatefulElement;

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod _StatefulElement {
                ::classes::_mod_uses! { mod class StatefulElement }
                ::classes::_def_class! { class StatefulElement }
                type Super = ComponentElement;
                ::classes::_def_class_extends! { StatefulElement : ComponentElement }

                mod data {
                    ::classes::_mod_uses! { mod data }

                    pub(super) type Super = ::classes::prelude::CData<super::Super>;
                    #[repr(C)]
                    pub struct StatefulElement {
                        pub(super) _super: Super,
                        pub(super) state: ::core::cell::Cell<Option<CRc<State> > >,
                    }
                    impl StatefulElement {
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
                        pub fn new(mut _self: ::classes::prelude::CRcUninit<Self>, widget: CRc<StatefulWidget>) -> ::classes::prelude::CRc<Self> {
                            let state = Some(widget.create_state());
                            let _self: ::classes::prelude::CRc<Self> = unsafe {
                                ::core::ptr::write(
                                    &raw mut (*_self.as_mut_ptr()).state,
                                    ::classes::get_set::New::new_cell(state),
                                );
                                let _ = |Self { _super, state: _, } : Self| ();
                                Super::new(_self.into_super(), Some(widget.into_super()))
                                    .into_subclass_unchecked()
                            };
                            println!("StatefulElement");
                            _self
                        }
                        pub(super) fn state(_self: &::classes::prelude::CRc<Self>,) -> CRc<State> {
                            _self.0.state.cloned().unwrap()
                        }
                        pub(super) fn build(_self: &::classes::prelude::CRc<Self>,) -> CRc<Widget> {
                            _self.state().build()
                        }
                        pub(super) fn first_build(_self: &::classes::prelude::CRc<Self>,) {
                            println!("StatefulElement::first_build");
                            _self.state().init_state();
                            { _self.delegate_super() }.first_build();
                        }
                        pub(super) fn perform_rebuild(_self: &::classes::prelude::CRc<Self>,) {
                            println!("StatefulElement::perform_rebuild");
                            { _self.delegate_super() }.perform_rebuild();
                        }
                    }
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }
                    pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct StatefulElement {
                        pub(super) _super: Super,
                        pub state: fn(&::classes::prelude::CRc<Self>,) -> CRc<State>,
                        pub build: fn(&::classes::prelude::CRc<Self>,) -> CRc<Widget>,
                    }
                    impl StatefulElement {
                        pub const fn debug_vtable_layout(&self, offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }
                    pub struct DebugVtableLayout<'a> {
                        this: &'a self::StatefulElement,
                        offset: usize,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(StatefulElement, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(StatefulElement));
                            dbg.field("\'start", &self.offset);
                            dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                            dbg.field(stringify!(state), &offset_of!(state));
                            dbg.field(stringify!(build), &offset_of!(build));
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<StatefulElement>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable :: opt }
                        pub(in super::super) type Super = ::classes::prelude::CVtableOpt<super::super::Super>;
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct StatefulElement {
                            pub(in super::super) _super: Super,
                            pub state: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,) -> CRc<State> >,
                            pub build: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,) -> CRc<Widget> >,
                        }
                        impl StatefulElement {
                            pub const DEFAULT: Self = Self {
                                _super: Super::DEFAULT,
                                state: ::core::option::Option::None,
                                build: ::core::option::Option::None,
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
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.state = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::state(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.state = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::state(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.build = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::build(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.build = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::build(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Super>>(_self);
                                    ptr.first_build = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::first_build(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Super>>(_self, &mut offset) {
                                        ptr.first_build = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::first_build(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Element>>(_self);
                                    ptr.perform_rebuild = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::perform_rebuild(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Element>>(_self, &mut offset) {
                                        ptr.perform_rebuild = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::perform_rebuild(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                            }
                            #[track_caller]
                            pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                                ::classes::prelude::CVtable::<Self> {
                                    _super: self._super.assert_init(),
                                    state: self.state.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(StatefulElement),
                                        "::",
                                        stringify!(state),
                                        "` is not implemented",
                                    )),
                                    build: self.build.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(StatefulElement),
                                        "::",
                                        stringify!(build),
                                        "` is not implemented",
                                    )),
                                }
                            }
                        }
                    }
                    pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                        ::classes::vtable::TypeInfo::new_concrete_class::<super::StatefulElement>(
                            ::core::option::Option::Some(Super::TYPE),
                            [],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(StatefulElement),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::StatefulElement,
                    vtable::opt::StatefulElement,
                    {state, build}
                }
                ::classes::_def_concrete_class! { StatefulElement }
                impl StatefulElement<::classes::ptr::RcDyn<StatefulElement>> {
                    #[inline]
                    pub fn new(widget: CRc<StatefulWidget>) -> Self {
                        ::classes::prelude::CData::<Self>::new(
                            ::classes::prelude::CRcUninit::<Self>::new_uninit(),
                            widget,
                        )
                    }
                    #[inline]
                    pub fn state(&self,) -> CRc<State> {
                        { (self.0.vtable().state)(self,) }
                    }
                    #[inline]
                    pub fn build(&self,) -> CRc<Widget> {
                        { (self.0.vtable().build)(self,) }
                    }
                    #[inline]
                    pub fn first_build(&self,) {
                        { self.as_super().first_build() }.try_into().unwrap()
                    }
                    #[inline]
                    pub fn perform_rebuild(&self,) {
                        { self.to_supertype::<::classes::prelude::CRc<Element>>().perform_rebuild() }.try_into().unwrap()
                    }
                }
                impl StatefulElement<::classes::ptr::RcDyn<StatefulElement>, ::classes::class::NonVirtual> {
                    #[inline]
                    pub fn state(&self,) -> CRc<State> {
                        { ::classes::prelude::CData::<Self>::state(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn build(&self,) -> CRc<Widget> {
                        { ::classes::prelude::CData::<Self>::build(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn first_build(&self,) {
                        { ::classes::prelude::CData::<Self>::first_build(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn perform_rebuild(&self,) {
                        { ::classes::prelude::CData::<Self>::perform_rebuild(self.as_virtual(),) }
                    }
                }
                impl StatefulElement<::classes::ptr::RcDyn<StatefulElement>> {
                    #[inline]
                    pub(in super::super) fn get_state(&self) -> <Option<CRc<State> > as ::classes::get_set::GetSet>::Get {
                        ::classes::get_set::GetSet::cell_get(&self.0.state)
                    }
                    #[inline]
                    pub(in super::super) fn set_state<_T: ::core::convert::Into<<Option<CRc<State> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        state: _T
                    ) {
                        ::classes::get_set::GetSet::cell_set(&self.0.state, state.into());
                    }
                    #[inline]
                    #[must_use]
                    pub(in super::super) fn replace_state<_T: ::core::convert::Into<<Option<CRc<State> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        state: _T
                    ) -> <Option<CRc<State> > as ::classes::get_set::GetSet>::Get {
                        let old = self.get_state();
                        self.set_state(state);
                        old
                    }
                    #[inline]
                    pub(in super::super) fn update_state_with<
                        _T: ::core::convert::Into<<Option<CRc<State> > as ::classes::get_set::GetSet>::Set>,
                        _F: ::core::ops::FnOnce(<Option<CRc<State> > as ::classes::get_set::GetSet>::Get) -> _T
                    >(&self, f: _F) {
                        self.set_state(f(self.get_state()));
                    }
                    #[inline]
                    pub(in super::super) fn raw_get_state(&self) -> &::core::cell::Cell<Option<CRc<State> > > {
                        &self.0.state
                    }
                }
            }
        }
    }
}

expand_test_case! {
    state {
        abstract class State {
            struct {
                widget: Option<CRc<Widget>> = None,
                element: Option<CRc<Element>> = None,
            }
            pub fn new() -> Self {
                Self { .. }
            }
            pub fn init_state(&self) {
                println!("State::init_state");
            }
            pub fn build(&self, /* cx: Ref<'_, BuildContext> */) -> CRc<Widget>;
            pub fn set_state(&self, f: &dyn Fn()) {
                println!("State::set_state");
                f();
                self.get_element().unwrap().mark_needs_build();
            }
        }
    } => {
        #[allow(unused_imports)]
        use _classes::State;

        use ::classes::prelude::*;

        const MODULE_PATH: &str = ::core::module_path!();

        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;
            #[allow(unused_imports)]
            pub(super) use _State::State;

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod _State {
                ::classes::_mod_uses! { mod class State }
                ::classes::_def_class! { class State }
                type Super = ::classes::object::Object;
                ::classes::_def_class_extends! { State : Object }

                mod data {
                    ::classes::_mod_uses! { mod data }
                    pub(super) type Super = ::classes::prelude::CData<super::Super>;
                    #[repr(C)]
                    pub struct State {
                        pub(super) _super: Super,
                        pub(super) widget: ::core::cell::Cell<Option<CRc<Widget> > >,
                        pub(super) element: ::core::cell::Cell<Option<CRc<Element> > >,
                    }
                    impl State {
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
                                ::core::ptr::write(
                                    &raw mut (*_self.as_mut_ptr()).widget,
                                    ::classes::get_set::New::new_cell(None),
                                );
                                ::core::ptr::write(
                                    &raw mut (*_self.as_mut_ptr()).element,
                                    ::classes::get_set::New::new_cell(None),
                                );
                                let _ = |Self { _super, widget: _, element: _, } : Self| ();
                                ::classes::prelude::CData::<::classes::object::Object>::new(_self.into_super())
                                    .into_subclass_unchecked()
                            }
                        }
                        pub(super) fn init_state(_self: &::classes::prelude::CRc<Self>,) {
                            println!("State::init_state");
                        }
                        pub(super) fn set_state(_self: &::classes::prelude::CRc<Self>, f: &dyn Fn()) {
                            println!("State::set_state");
                            f();
                            _self.get_element().unwrap().mark_needs_build();
                        }
                    }
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }
                    pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct State {
                        pub(super) _super: Super,
                        pub init_state: fn(&::classes::prelude::CRc<Self>,),
                        pub build: fn(&::classes::prelude::CRc<Self>,) -> CRc<Widget>,
                        pub set_state: fn(&::classes::prelude::CRc<Self>, f: &dyn Fn(),),
                    }
                    impl State {
                        pub const fn debug_vtable_layout(&self, offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }
                    pub struct DebugVtableLayout<'a> {
                        this: &'a self::State,
                        offset: usize,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(State, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(State));
                            dbg.field("\'start", &self.offset);
                            dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                            dbg.field(stringify!(init_state), &offset_of!(init_state));
                            dbg.field(stringify!(build), &offset_of!(build));
                            dbg.field(stringify!(set_state), &offset_of!(set_state));
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<State>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable :: opt }
                        pub(in super::super) type Super = ::classes::prelude::CVtableOpt<super::super::Super>;
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct State {
                            pub(in super::super) _super: Super,
                            pub init_state: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,)>,
                            pub build: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,) -> CRc<Widget> >,
                            pub set_state:
                                ::core::option::Option<fn(&::classes::prelude::CRc<Self>, f: &dyn Fn(),)>,
                        }
                        impl State {
                            pub const DEFAULT: Self = Self {
                                _super: Super::DEFAULT,
                                init_state: ::core::option::Option::None,
                                build: ::core::option::Option::None,
                                set_state: ::core::option::Option::None,
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
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.init_state = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::init_state(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.init_state = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::init_state(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.set_state = ::core::option::Option::Some(|this, f,| ::classes::prelude::CData::<Self>::set_state(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        f,
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.set_state = ::core::option::Option::Some(|this, f,| ::classes::prelude::CData::<Self>::set_state(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                            f,
                                        ).into());
                                    }
                                }
                            }
                            #[track_caller]
                            pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                                ::classes::prelude::CVtable::<Self> {
                                    _super: self._super.assert_init(),
                                    init_state: self.init_state.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(State),
                                        "::",
                                        stringify!(init_state),
                                        "` is not implemented",
                                    )),
                                    build: self.build.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(State),
                                        "::",
                                        stringify!(build),
                                        "` is not implemented",
                                    )),
                                    set_state: self.set_state.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(State),
                                        "::",
                                        stringify!(set_state),
                                        "` is not implemented",
                                    )),
                                }
                            }
                        }
                    }
                    pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                        ::classes::vtable::TypeInfo::new_abstract_class::<super::State>(
                            ::core::option::Option::Some(Super::TYPE),
                            [],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(State),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::State,
                    vtable::opt::State,
                    {init_state, build, set_state}
                }
                impl State<::classes::ptr::RcDyn<State>> {
                    #[inline]
                    pub fn init_state(&self,) {
                        { (self.0.vtable().init_state)(self,) }
                    }
                    #[inline]
                    pub fn build(&self,) -> CRc<Widget> {
                        { (self.0.vtable().build)(self,) }
                    }
                    #[inline]
                    pub fn set_state(&self, f: &dyn Fn()) {
                        { (self.0.vtable().set_state)(self, f,) }
                    }
                }
                impl State<::classes::ptr::RcDyn<State>, ::classes::class::NonVirtual> {
                    #[inline]
                    pub fn init_state(&self,) {
                        { ::classes::prelude::CData::<Self>::init_state(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn set_state(&self, f: &dyn Fn()) {
                        { ::classes::prelude::CData::<Self>::set_state(self.as_virtual(), f,) }
                    }
                }
                impl State<::classes::ptr::RcDyn<State>> {
                    #[inline]
                    pub(in super::super) fn get_widget(&self) -> <Option<CRc<Widget> > as ::classes::get_set::GetSet>::Get {
                        ::classes::get_set::GetSet::cell_get(&self.0.widget)
                    }

                    #[inline]
                    pub(in super::super) fn set_widget<_T: ::core::convert::Into<<Option<CRc<Widget> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        widget: _T
                    ) {
                        ::classes::get_set::GetSet::cell_set(&self.0.widget, widget.into());
                    }
                    #[inline]
                    #[must_use]
                    pub(in super::super) fn replace_widget<_T: ::core::convert::Into<<Option<CRc<Widget> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        widget: _T
                    ) -> <Option<CRc<Widget> > as ::classes::get_set::GetSet>::Get {
                        let old = self.get_widget();
                        self.set_widget(widget);
                        old
                    }
                    #[inline]
                    pub(in super::super) fn update_widget_with<
                        _T: ::core::convert::Into<<Option<CRc<Widget> > as ::classes::get_set::GetSet>::Set>,
                        _F: ::core::ops::FnOnce(<Option<CRc<Widget> > as ::classes::get_set::GetSet>::Get) -> _T
                    >(&self, f: _F) {
                        self.set_widget(f(self.get_widget()));
                    }
                    #[inline]
                    pub(in super::super) fn raw_get_widget(&self) -> &::core::cell::Cell<Option<CRc<Widget> > > {
                        &self.0.widget
                    }

                    #[inline]
                    pub(in super::super) fn get_element(&self) -> <Option<CRc<Element> > as ::classes::get_set::GetSet>::Get {
                        ::classes::get_set::GetSet::cell_get(&self.0.element)
                    }
                    #[inline]
                    pub(in super::super) fn set_element<_T: ::core::convert::Into<<Option<CRc<Element> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        element: _T
                    ) {
                        ::classes::get_set::GetSet::cell_set(&self.0.element, element.into());
                    }
                    #[inline]
                    #[must_use]
                    pub(in super::super) fn replace_element<_T: ::core::convert::Into<<Option<CRc<Element> > as ::classes::get_set::GetSet>::Set> >(
                        &self,
                        element: _T
                    ) -> <Option<CRc<Element> > as ::classes::get_set::GetSet>::Get {
                        let old = self.get_element();
                        self.set_element(element);
                        old
                    }
                    #[inline]
                    pub(in super::super) fn update_element_with<
                        _T: ::core::convert::Into<<Option<CRc<Element> > as ::classes::get_set::GetSet>::Set>,
                        _F: ::core::ops::FnOnce(<Option<CRc<Element> > as ::classes::get_set::GetSet>::Get) -> _T
                    >(&self, f: _F) {
                        self.set_element(f(self.get_element()));
                    }
                    #[inline]
                    pub(in super::super) fn raw_get_element(&self) -> &::core::cell::Cell<Option<CRc<Element> > > {
                        &self.0.element
                    }
                }
            }
        }
    }
}

expand_test_case! {
    gallery_page {
        class GalleryPage extends StatefulWidget {
            struct {}

            pub fn new() -> Self {
                Self { super: Super::new() }
            }

            pub override fn create_state(&self) -> CRc<GalleryPageState> {
                println!("GalleryPage::create_state");
                GalleryPageState::new()
            }

            pub fn on_create(&self) {
                println!("GalleryPage::on_create");
            }
        }

    } => {
        #[allow(unused_imports)]
        use _classes::GalleryPage;
        use ::classes::prelude::*;
        const MODULE_PATH: &str = ::core::module_path!();
        #[allow(unused_macros)]
        mod _classes {
            use super::*;
            use ::classes::prelude::*;
            #[allow(unused_imports)]
            pub(super) use _GalleryPage::GalleryPage;
            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod _GalleryPage {
                ::classes::_mod_uses! { mod class GalleryPage }
                ::classes::_def_class! { class GalleryPage }
                type Super = StatefulWidget;
                ::classes::_def_class_extends! { GalleryPage : StatefulWidget }

                mod data {
                    ::classes::_mod_uses! { mod data }
                    pub(super) type Super = ::classes::prelude::CData<super::Super>;
                    #[repr(C)]
                    pub struct GalleryPage {
                        pub(super) _super: Super,
                    }
                    impl GalleryPage {
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
                                Super::new(_self.into_super(),)
                                    .into_subclass_unchecked()
                            }
                        }
                        pub(super) fn create_state(_self: &::classes::prelude::CRc<Self>,) -> CRc<GalleryPageState> {
                            println!("GalleryPage::create_state");
                            GalleryPageState::new()
                        }
                        pub(super) fn on_create(_self: &::classes::prelude::CRc<Self>,) {
                            println!("GalleryPage::on_create");
                        }
                    }
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }
                    pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                    #[repr(C)]
                    #[derive(Clone, Copy)]
                    pub struct GalleryPage {
                        pub(super) _super: Super,
                        pub on_create: fn(&::classes::prelude::CRc<Self>,),
                    }

                    impl GalleryPage {
                        pub const fn debug_vtable_layout(&self, offset: usize) -> self::DebugVtableLayout<'_> {
                            self::DebugVtableLayout { this: self, offset }
                        }
                    }
                    pub struct DebugVtableLayout<'a> {
                        this: &'a self::GalleryPage,
                        offset: usize,
                    }
                    impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                        #[allow(unused_macros)]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            macro_rules! offset_of {
                                ($field:ident) => {
                                    self.offset + ::core::mem::offset_of!(GalleryPage, $field)
                                };
                            }
                            let mut dbg = f.debug_struct(stringify!(GalleryPage));
                            dbg.field("\'start", &self.offset);
                            dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));
                            dbg.field(stringify!(on_create), &offset_of!(on_create));
                            dbg.field("\'end", &(self.offset + ::core::mem::size_of::<GalleryPage>())) ;
                            dbg.finish()
                        }
                    }

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable :: opt }
                        pub(in super::super) type Super = ::classes::prelude::CVtableOpt<super::super::Super>;
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        #[derive(Default)]
                        pub struct GalleryPage {
                            pub(in super::super) _super: Super,
                            pub on_create: ::core::option::Option<fn(&::classes::prelude::CRc<Self>,)>,
                        }
                        impl GalleryPage {
                            pub const DEFAULT: Self = Self {
                                _super: Super::DEFAULT,
                                on_create: ::core::option::Option::None,
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
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Super>>(_self);
                                    ptr.create_state = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::create_state(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Super>>(_self, &mut offset) {
                                        ptr.create_state = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::create_state(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                                {
                                    let (ptr, mut offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                                    ptr.on_create = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::on_create(
                                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                    ).into());
                                    while let Some(ptr) = ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<Self>>(_self, &mut offset) {
                                        ptr.on_create = ::core::option::Option::Some(|this,| ::classes::prelude::CData::<Self>::on_create(
                                            &unsafe { this.try_to_subtype().unwrap_unchecked() },
                                        ).into());
                                    }
                                }
                            }
                            #[track_caller]
                            pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                                ::classes::prelude::CVtable::<Self> {
                                    _super: self._super.assert_init(),
                                    on_create: self.on_create.expect(concat!(
                                        "cannot instantiate because method `",
                                        stringify!(GalleryPage),
                                        "::",
                                        stringify!(on_create),
                                        "` is not implemented",
                                    )),
                                }
                            }
                        }
                    }
                    pub static TYPE: ::classes::vtable::TypeInfo<0usize> =
                        ::classes::vtable::TypeInfo::new_concrete_class::<super::GalleryPage>(
                            ::core::option::Option::Some(Super::TYPE),
                            [],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(GalleryPage),
                        );
                }
                ::classes::assert_layout_eq! {
                    vtable::GalleryPage,
                    vtable::opt::GalleryPage,
                    {on_create}
                }
                ::classes::_def_concrete_class! { GalleryPage }
                impl GalleryPage<::classes::ptr::RcDyn<GalleryPage>> {
                    #[inline]
                    pub fn new() -> Self {
                        ::classes::prelude::CData::<Self>::new(
                            ::classes::prelude::CRcUninit::<Self>::new_uninit(),
                        )
                    }
                    #[inline]
                    pub fn create_state(&self,) -> CRc<GalleryPageState> {
                        { self.as_super().create_state() }.try_into().unwrap()
                    }
                    #[inline]
                    pub fn on_create(&self,) {
                        { (self.0.vtable().on_create)(self,) }
                    }
                }
                impl GalleryPage<::classes::ptr::RcDyn<GalleryPage>, ::classes::class::NonVirtual> {
                    #[inline]
                    pub fn create_state(&self,) -> CRc<GalleryPageState> {
                        { ::classes::prelude::CData::<Self>::create_state(self.as_virtual(),) }
                    }
                    #[inline]
                    pub fn on_create(&self,) {
                        { ::classes::prelude::CData::<Self>::on_create(self.as_virtual(),) }
                    }
                }
                impl GalleryPage<::classes::ptr::RcDyn<GalleryPage>> {}
            }
        }
    }
}
