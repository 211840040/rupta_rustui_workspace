use core::cell::{Cell, RefCell};

thread_local! {
    static BUF: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

use _classes::*;

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
    pub(super) use self::_BuildContext::BuildContext;
    pub(super) use self::_ComponentElement::ComponentElement;
    pub(super) use self::_Element::Element;
    pub(super) use self::_State::State;
    pub(super) use self::_StatefulElement::StatefulElement;
    pub(super) use self::_StatefulWidget::StatefulWidget;
    pub(super) use self::_Widget::Widget;
    use super::*;

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _BuildContext {
        ::classes::_mod_uses!(mod class BuildContext);

        ::classes::_def_class!(class BuildContext);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct BuildContext {}
        }
        mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct BuildContext {
                header: VtableHeader,
                pub widget: fn(&CRc<Self>) -> CRc<Widget>,
            }

            impl BuildContext {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }

            pub struct DebugVtableLayout<'a> {
                this: &'a self::BuildContext,
                offset: usize,
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
                    dbg.field("\'start", &self.offset);
                    dbg.field("header", &self.this.header);
                    dbg.field("widget", &offset_of!(widget));
                    dbg.field(
                        "\'end",
                        &(self.offset + core::mem::size_of::<BuildContext>()),
                    );
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct BuildContext {
                    header: VtableHeaderOpt,
                    pub widget: Option<fn(&CRc<Self>) -> CRc<Widget>>,
                }

                impl BuildContext {
                    pub const DEFAULT: Self = Self {
                        header: VtableHeaderOpt::DEFAULT,
                        widget: None,
                    };

                    pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => Self::TYPE,
                            Some(ty) => ty,
                        };
                        self.header = VtableHeaderOpt::new(ty, offset);
                    }
                    pub const fn init<V: ClassVtable>(&mut self) {}
                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            header: self.header.assert_init(),
                            widget: self.widget.unwrap(),
                        }
                    }
                }
            }

            pub static TYPE: TypeInfo<0> = TypeInfo::new_abstract_class::<super::BuildContext>(
                None,
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "BuildContext",
            );
        }
        impl BuildContext<RcDyn<BuildContext>> {
            #[inline]
            pub fn widget(&self) -> CRc<Widget> {
                (self.0.vtable().widget)(self)
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Element {
        ::classes::_mod_uses!(mod class Element);

        ::classes::_def_class!(class Element);
        ::classes::_def_class_impl!(Element: BuildContext);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct Element {
                pub(super) parent: Cell<Option<CWeak<Element>>>,
                pub(super) widget: Cell<Option<CWeak<Widget>>>,
                pub(super) dirty: Cell<bool>,
            }

            impl Element {
                pub fn new(mut _self: CRcUninit<Self>, widget: Option<CRc<Widget>>) -> CRc<Self> {
                    println!("Element");
                    let _ = |Self {
                                 widget: _,
                                 parent: _,
                                 dirty: _,
                             }: Self| ();
                    #[allow(unused_unsafe)]
                    unsafe {
                        core::ptr::write(
                            &raw mut (*_self.as_mut_ptr()).widget,
                            New::new_cell(widget),
                        );
                        core::ptr::write(
                            &raw mut (*_self.as_mut_ptr()).parent,
                            New::new_cell(None),
                        );
                        core::ptr::write(
                            &raw mut (*_self.as_mut_ptr()).dirty,
                            NewCopy::new_cell(true),
                        );
                    }
                    CRc::<Self>::_from_inner(unsafe { _self.assume_init() })
                }
                pub(super) fn widget(this: &CRc<Self>) -> CRc<Widget> {
                    this.get_widget().unwrap()
                }
                pub(super) fn rebuild(this: &CRc<Self>, force: bool) {
                    println!("Element::rebuild");
                    if this.get_dirty() || force {
                        this.perform_rebuild();
                    }
                }
                pub(super) fn perform_rebuild(this: &CRc<Self>) {
                    println!("Element::perform_rebuild");
                    this.set_dirty(false);
                }
                pub(super) fn mount(this: &CRc<Self>, parent: Option<CRc<Element>>) {
                    println!("Element::mount");
                    this.set_parent(parent);
                }
                pub(super) fn mark_needs_build(this: &CRc<Self>) {
                    println!("Element::mark_needs_build");
                    this.set_dirty(true);
                    this.rebuild(false);
                }
            }
            mod impls {
                pub(super) mod BuildContext {
                    use super::super::*;
                }
            }
        }

        mod vtable {
            use std::mem::offset_of;

            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Element {
                header: VtableHeader,
                pub rebuild: fn(&CRc<Self>, force: bool),
                pub perform_rebuild: fn(&CRc<Self>),
                pub mount: fn(&CRc<Self>, parent: Option<CRc<Element>>),
                pub mark_needs_build: fn(&CRc<Self>),
                pub(super) BuildContext: CVtable<BuildContext>,
            }

            impl Element {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
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
                    dbg.field("header", &self.this.header);
                    dbg.field("rebuild", &offset_of!(rebuild));
                    dbg.field("perform_rebuild", &offset_of!(perform_rebuild));
                    dbg.field("mount", &offset_of!(mount));
                    dbg.field("mark_needs_build", &offset_of!(mark_needs_build));
                    dbg.field(
                        "BuildContext",
                        &self
                            .this
                            .BuildContext
                            .debug_vtable_layout(offset_of!(BuildContext)),
                    );
                    dbg.field("\'end", &(self.offset + core::mem::size_of::<Element>()));
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct Element {
                    header: VtableHeaderOpt,
                    pub rebuild: Option<fn(&CRc<Self>, force: bool)>,
                    pub perform_rebuild: Option<fn(&CRc<Self>)>,
                    pub mount: Option<fn(&CRc<Self>, parent: Option<CRc<Element>>)>,
                    pub mark_needs_build: Option<fn(&CRc<Self>)>,
                    pub(in super::super) BuildContext: CVtableOpt<BuildContext>,
                }
                #[allow(unused_variables)]
                impl Element {
                    pub const DEFAULT: Self = Self {
                        header: VtableHeaderOpt::DEFAULT,
                        rebuild: None,
                        perform_rebuild: None,
                        mount: None,
                        mark_needs_build: None,
                        BuildContext: CVtableOpt::<BuildContext>::DEFAULT,
                    };

                    pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => Self::TYPE,
                            Some(ty) => ty,
                        };
                        self.header = VtableHeaderOpt::new(ty, offset);
                        self.BuildContext
                            .init_header(None, offset + core::mem::offset_of!(Self, BuildContext));
                    }
                    pub const fn init(&mut self) {
                        self.rebuild = Some(CData::<Self>::rebuild);
                        self.perform_rebuild = Some(CData::<Self>::perform_rebuild);
                        self.mount = Some(CData::<Self>::mount);
                        self.mark_needs_build = Some(CData::<Self>::mark_needs_build);
                        ::classes::vtable::vtable_opt_upcast_mut::<_, CVtableOpt<BuildContext>>(
                            self,
                        )
                        .0
                        .widget = Some(|this| {
                            CData::<Self>::widget(&unsafe {
                                this.try_to_subtype().unwrap_unchecked()
                            })
                        });
                    }

                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            header: self.header.assert_init(),
                            rebuild: self.rebuild.unwrap(),
                            perform_rebuild: self.perform_rebuild.unwrap(),
                            mount: self.mount.unwrap(),
                            mark_needs_build: self.mark_needs_build.unwrap(),
                            BuildContext: self.BuildContext.assert_init(),
                        }
                    }
                }
            }

            pub static TYPE: TypeInfo<1> = TypeInfo::new_abstract_class::<super::Element>(
                None,
                [(
                    CVtable::<BuildContext>::TYPE,
                    offset_of!(CVtable<Element>, BuildContext),
                )],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "Element",
            );
        }

        impl Element<RcDyn<Element>> {
            #[inline]
            pub fn widget(&self) -> CRc<Widget> {
                self.to_impl::<CRc<BuildContext>>().widget()
            }
            #[inline]
            pub fn rebuild(&self, force: bool) {
                (self.0.vtable().rebuild)(self, force)
            }
            #[inline]
            pub fn perform_rebuild(&self) {
                (self.0.vtable().perform_rebuild)(self)
            }
            #[inline]
            pub fn mount(&self, parent: Option<CRc<Element>>) {
                (self.0.vtable().mount)(self, parent)
            }
            #[inline]
            pub fn mark_needs_build(&self) {
                (self.0.vtable().mark_needs_build)(self)
            }
        }

        impl<V> Element<RcDyn<Element>, V> {
            #[inline]
            pub fn get_parent(&self) -> Option<CRc<Element>> {
                GetSet::cell_get(&self.0.parent)
            }
            #[inline]
            pub fn get_widget(&self) -> Option<CRc<Widget>> {
                GetSet::cell_get(&self.0.widget)
            }
            #[inline]
            pub fn get_dirty(&self) -> bool {
                GetSetCopy::cell_get(&self.0.dirty)
            }
            #[inline]
            pub fn set_parent(&self, parent: impl Into<Option<CRc<Element>>>) {
                GetSet::cell_set(&self.0.parent, parent.into());
            }
            #[inline]
            pub fn set_widget(&self, widget: impl Into<Option<CRc<Widget>>>) {
                GetSet::cell_set(&self.0.widget, widget.into());
            }
            #[inline]
            pub fn set_dirty(&self, dirty: impl Into<bool>) {
                GetSetCopy::cell_set(&self.0.dirty, dirty.into());
            }
        }

        impl Element<RcDyn<Element>, NonVirtual> {
            #[inline]
            pub fn widget(&self) -> CRc<Widget> {
                CData::<Self>::widget(self.as_virtual())
            }
            #[inline]
            pub fn rebuild(&self, force: bool) {
                CData::<Self>::rebuild(self.as_virtual(), force)
            }
            #[inline]
            pub fn perform_rebuild(&self) {
                CData::<Self>::perform_rebuild(self.as_virtual())
            }
            #[inline]
            pub fn mount(&self, parent: Option<CRc<Element>>) {
                CData::<Self>::mount(self.as_virtual(), parent)
            }
            #[inline]
            pub fn mark_needs_build(&self) {
                CData::<Self>::mark_needs_build(self.as_virtual())
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _ComponentElement {
        ::classes::_mod_uses!(mod class ComponentElement);

        ::classes::_def_class!(class ComponentElement);
        ::classes::_def_class_extends!(ComponentElement: Element);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct ComponentElement {
                pub(super) _super: CData<Element>,
                child: Cell<Option<CRc<Element>>>,
            }

            impl ComponentElement {
                pub fn new(mut _self: CRcUninit<Self>, widget: Option<CRc<Widget>>) -> CRc<Self> {
                    let _ = |Self {
                                 _super: _,
                                 child: _,
                             }: Self| ();
                    #[allow(unused_unsafe)]
                    unsafe {
                        core::ptr::write(&raw mut (*_self.as_mut_ptr()).child, New::new_cell(None));
                    }
                    let _super = CData::<Element>::new(_self.into_super(), widget);
                    let _self: CRc<Self> = unsafe { _super.into_subclass_unchecked() };
                    println!("ComponentElement");
                    _self
                }

                pub(super) fn first_build(this: &CRc<Self>) {
                    println!("ComponentElement::first_build");
                    this.delegate_super().rebuild(false);
                }

                pub(super) fn perform_rebuild(this: &CRc<Self>) {
                    println!("ComponentElement::perform_rebuild");
                    this.build();
                    this.delegate_super().perform_rebuild();
                }

                pub(super) fn mount(this: &CRc<Self>, parent: Option<CRc<Element>>) {
                    println!("ComponentElement::mount");
                    this.delegate_super().mount(parent);
                    this.first_build();
                }
            }
        }

        mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct ComponentElement {
                pub(super) _super: CVtable<Element>,
                pub first_build: fn(&CRc<Self>),
                pub build: fn(&CRc<Self>) -> CRc<Widget>,
            }

            impl ComponentElement {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
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
                    dbg.field(
                        "super",
                        &self.this._super.debug_vtable_layout(offset_of!(_super)),
                    );
                    dbg.field("first_build", &offset_of!(first_build));
                    dbg.field("build", &offset_of!(build));
                    dbg.field(
                        "\'end",
                        &(self.offset + core::mem::size_of::<ComponentElement>()),
                    );
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct ComponentElement {
                    pub(in super::super) _super: CVtableOpt<Element>,
                    pub first_build: Option<fn(&CRc<Self>)>,
                    pub build: Option<fn(&CRc<Self>) -> CRc<Widget>>,
                }

                #[allow(unused_variables)]
                impl ComponentElement {
                    pub const DEFAULT: Self = Self {
                        _super: CVtableOpt::<Element>::DEFAULT,
                        first_build: None,
                        build: None,
                    };

                    pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => Self::TYPE,
                            Some(ty) => ty,
                        };
                        self._super.init_header(Some(ty), offset);
                    }
                    pub const fn init(&mut self) {
                        self._super.init();
                        self.first_build = Some(CData::<ComponentElement>::first_build);
                        self._super.perform_rebuild = Some(|this| {
                            CData::<Self>::perform_rebuild(unsafe { this.as_subclass_unchecked() })
                        });
                        self._super.mount = Some(|this, parent| {
                            CData::<Self>::mount(unsafe { this.as_subclass_unchecked() }, parent)
                        });
                    }
                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            _super: self._super.assert_init(),
                            first_build: self.first_build.unwrap(),
                            build: self.build.unwrap(),
                        }
                    }
                }
            }

            pub static TYPE: TypeInfo<0> = TypeInfo::new_abstract_class::<super::ComponentElement>(
                Some(CVtable::<Element>::TYPE),
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "ComponentElement",
            );
        }

        impl ComponentElement<RcDyn<ComponentElement>> {
            #[inline]
            pub fn first_build(&self) {
                (self.0.vtable().first_build)(self)
            }
            #[inline]
            pub fn perform_rebuild(&self) {
                (self.0.vtable()._super.perform_rebuild)(self)
            }
            #[inline]
            pub fn build(&self) -> CRc<Widget> {
                (self.0.vtable().build)(self)
            }
            #[inline]
            pub fn mount(&self, parent: Option<CRc<Element>>) {
                (self.0.vtable()._super.mount)(self, parent)
            }
        }

        impl ComponentElement<RcDyn<ComponentElement>, NonVirtual> {
            #[inline]
            pub fn first_build(&self) {
                CData::<Self>::first_build(self.as_virtual())
            }
            #[inline]
            pub fn perform_rebuild(&self) {
                CData::<Self>::perform_rebuild(self.as_virtual())
            }
            #[inline]
            pub fn mount(&self, parent: Option<CRc<Element>>) {
                CData::<Self>::mount(self.as_virtual(), parent)
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _StatefulElement {
        ::classes::_mod_uses!(mod class StatefulElement);

        ::classes::_def_class!(class StatefulElement);
        ::classes::_def_class_extends!(StatefulElement: ComponentElement);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct StatefulElement {
                pub(super) _super: CData<ComponentElement>,
                pub(super) state: Cell<Option<CRc<State>>>,
            }
            impl StatefulElement {
                pub fn new(mut _self: CRcUninit<Self>, widget: CRc<StatefulWidget>) -> CRc<Self> {
                    let state = Cell::new(Some(widget.create_state()));
                    let _ = |Self {
                                 _super: _,
                                 state: _,
                             }: Self| ();
                    #[allow(unused_unsafe)]
                    unsafe {
                        core::ptr::write(&raw mut (*_self.as_mut_ptr()).state, state);
                    }
                    let _super = CData::<ComponentElement>::new(
                        _self.into_super(),
                        Some(widget.into_super()),
                    );
                    let _self: CRc<Self> = unsafe { _super.into_subclass_unchecked() };
                    println!("StatefulElement");
                    _self
                }
                pub(super) fn build(this: &CRc<Self>) -> CRc<Widget> {
                    this.state()
                        .build(&this.upcast::<CRc<Element>, CRc<BuildContext>>())
                }
                pub(super) fn first_build(this: &CRc<Self>) {
                    println!("StatefulElement::first_build");
                    this.state().init_state();
                    this.delegate_super().first_build();
                }
                pub(super) fn perform_rebuild(this: &CRc<Self>) {
                    println!("StatefulElement::perform_rebuild");
                    this.delegate_super().perform_rebuild();
                }
            }
        }

        mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct StatefulElement {
                pub(super) _super: CVtable<ComponentElement>,
            }

            impl StatefulElement {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
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
                    dbg.field(
                        "super",
                        &self.this._super.debug_vtable_layout(offset_of!(_super)),
                    );
                    dbg.field(
                        "\'end",
                        &(self.offset + core::mem::size_of::<StatefulElement>()),
                    );
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct StatefulElement {
                    pub(in super::super) _super: CVtableOpt<ComponentElement>,
                }

                impl StatefulElement {
                    pub const DEFAULT: Self = Self {
                        _super: CVtableOpt::<ComponentElement>::DEFAULT,
                    };

                    pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => Self::TYPE,
                            Some(ty) => ty,
                        };
                        self._super.init_header(Some(ty), offset);
                    }

                    pub const fn init(&mut self) {
                        self._super.init();
                        self._super.build = Some(|this| {
                            CData::<Self>::build(unsafe { this.as_subclass_unchecked() })
                        });
                        self._super.first_build = Some(|this| {
                            CData::<Self>::first_build(unsafe { this.as_subclass_unchecked() })
                        });
                        ::classes::vtable::vtable_opt_upcast_mut::<_, CVtableOpt<Element>>(self)
                            .0
                            .perform_rebuild = Some(|this| {
                            CData::<Self>::perform_rebuild(unsafe { this.as_subclass_unchecked() })
                        });
                    }
                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            _super: self._super.assert_init(),
                        }
                    }
                }
            }

            pub static TYPE: TypeInfo<0> = TypeInfo::new_concrete_class::<super::StatefulElement>(
                Some(CVtable::<ComponentElement>::TYPE),
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "StatefulElement",
            );
        }

        static VTABLE: vtable::StatefulElement = {
            let mut vtable = vtable::opt::StatefulElement::DEFAULT;
            vtable.init_header(None, 0);
            vtable.init();
            vtable.assert_init()
        };
        unsafe impl ConcreteClass for StatefulElement {
            const VTABLE: NonNull<Self::Vtable> =
                unsafe { NonNull::new_unchecked((&raw const VTABLE).cast_mut()) };
        }

        impl StatefulElement<RcDyn<StatefulElement>> {
            #[inline]
            pub fn new(widget: CRc<StatefulWidget>) -> Self {
                CData::<Self>::new(CRcUninit::<Self>::new_uninit(), widget)
            }

            #[inline]
            pub fn build(&self) -> CRc<Widget> {
                (self.0.vtable().as_super().build)(self.as_super())
            }
            #[inline]
            pub fn first_build(&self) {
                (self.0.vtable().as_super().first_build)(self.as_super())
            }
            #[inline]
            pub fn perform_rebuild(&self) {
                self.as_super().perform_rebuild()
            }
        }

        impl<V> StatefulElement<RcDyn<StatefulElement>, V> {
            pub(super) fn state(&self) -> CRc<State> {
                self.get_state().unwrap()
            }
            pub fn get_state(&self) -> Option<CRc<State>> {
                GetSet::cell_get(&self.0.state)
            }
            pub fn set_state(&self, state: impl Into<Option<CRc<State>>>) {
                GetSet::cell_set(&self.0.state, state.into())
            }
        }

        impl StatefulElement<RcDyn<StatefulElement>, NonVirtual> {
            #[inline]
            pub fn build(&self) -> CRc<Widget> {
                CData::<Self>::build(self.as_virtual())
            }
            #[inline]
            pub fn first_build(&self) {
                CData::<Self>::first_build(self.as_virtual())
            }
            #[inline]
            pub fn perform_rebuild(&self) {
                CData::<Self>::perform_rebuild(self.as_virtual())
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _State {
        ::classes::_mod_uses!(mod class State);

        ::classes::_def_class!(class State);
        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct State {
                pub(super) widget: Cell<Option<CRc<Widget>>>,
                pub(super) element: Cell<Option<CRc<Element>>>,
            }
            impl State {
                pub fn new(mut _self: CRcUninit<Self>) -> CRc<Self> {
                    let _ = |Self {
                                 widget: _,
                                 element: _,
                             }: Self| ();
                    #[allow(unused_unsafe)]
                    unsafe {
                        core::ptr::write(&raw mut (*_self.as_mut_ptr()).widget, Default::default());
                        core::ptr::write(
                            &raw mut (*_self.as_mut_ptr()).element,
                            Default::default(),
                        );
                    }
                    CRc::<Self>::_from_inner(unsafe { _self.assume_init() })
                }
                pub(super) fn init_state(_this: &CRc<Self>) {
                    println!("State::init_state");
                }
                pub(super) fn set_state(this: &CRc<Self>, f: &dyn Fn()) {
                    println!("State::set_state");
                    f();
                    this.get_element().unwrap().mark_needs_build();
                }
            }
        }
        mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct State {
                header: VtableHeader,
                pub init_state: fn(&CRc<Self>),
                pub build: fn(&CRc<Self>, cx: &CRc<BuildContext>) -> CRc<Widget>,
                pub set_state: fn(&CRc<Self>, f: &dyn Fn()),
            }

            impl State {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
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
                    dbg.field("header", &self.this.header);
                    dbg.field("init_state", &offset_of!(init_state));
                    dbg.field("build", &offset_of!(build));
                    dbg.field("set_state", &offset_of!(set_state));
                    dbg.field("\'end", &(self.offset + core::mem::size_of::<State>()));
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct State {
                    header: VtableHeaderOpt,
                    pub init_state: Option<fn(&CRc<Self>)>,
                    pub build: Option<fn(&CRc<Self>, cx: &CRc<BuildContext>) -> CRc<Widget>>,
                    pub set_state: Option<fn(&CRc<Self>, f: &dyn Fn())>,
                }

                #[allow(unused_variables)]
                impl State {
                    pub const DEFAULT: Self = Self {
                        header: VtableHeaderOpt::DEFAULT,
                        init_state: None,
                        build: None,
                        set_state: None,
                    };

                    pub const fn init(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => Self::TYPE,
                            Some(ty) => ty,
                        };
                        self.header = VtableHeaderOpt::new(ty, offset);
                        self.init_state = Some(CData::<State>::init_state);
                        self.set_state = Some(CData::<State>::set_state);
                    }

                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            header: self.header.assert_init(),
                            init_state: self.init_state.unwrap(),
                            build: self.build.unwrap(),
                            set_state: self.set_state.unwrap(),
                        }
                    }
                }
            }
            pub static TYPE: TypeInfo<0> = TypeInfo::new_abstract_class::<super::State>(
                None,
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "State",
            );
        }
        impl State<RcDyn<State>> {
            #[inline]
            pub fn init_state(&self) {
                (self.0.vtable().init_state)(self)
            }
            #[inline]
            pub fn build(&self, cx: &CRc<BuildContext>) -> CRc<Widget> {
                (self.0.vtable().build)(self, cx)
            }
            #[inline]
            pub fn set_state(&self, f: &dyn Fn()) {
                (self.0.vtable().set_state)(self, f)
            }
        }

        impl<V> State<RcDyn<State>, V> {
            pub fn get_widget(&self) -> Option<CRc<Widget>> {
                GetSet::cell_get(&self.0.widget)
            }
            pub fn get_element(&self) -> Option<CRc<Element>> {
                GetSet::cell_get(&self.0.element)
            }
            pub fn set_widget(&self, widget: impl Into<Option<CRc<Widget>>>) {
                GetSet::cell_set(&self.0.widget, widget.into())
            }
            pub fn set_element(&self, element: impl Into<Option<CRc<Element>>>) {
                GetSet::cell_set(&self.0.element, element.into())
            }
        }

        impl State<RcDyn<State>, NonVirtual> {
            #[inline]
            pub fn init_state(&self) {
                CData::<Self>::init_state(self.as_virtual())
            }
            #[inline]
            pub fn set_state(&self, f: &dyn Fn()) {
                CData::<Self>::set_state(self.as_virtual(), f)
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _Widget {
        ::classes::_mod_uses!(mod class Widget);

        ::classes::_def_class!(class Widget);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct Widget {}
            impl Widget {
                pub fn new(mut _self: CRcUninit<Self>) -> CRc<Self> {
                    let _ = |Self {}: Self| ();
                    #[allow(unused_unsafe)]
                    unsafe {}
                    CRc::<Self>::_from_inner(unsafe { _self.assume_init() })
                }
            }
        }

        mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Widget {
                header: VtableHeader,
                pub create_element: fn(&CRc<Self>) -> CRc<Element>,
            }

            impl Widget {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }

            pub struct DebugVtableLayout<'a> {
                this: &'a self::Widget,
                offset: usize,
            }

            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    macro_rules! offset_of {
                        ($field:ident) => {
                            self.offset + ::core::mem::offset_of!(Widget, $field)
                        };
                    }
                    let mut dbg = f.debug_struct(stringify!(Widget));
                    dbg.field("\'start", &self.offset);
                    dbg.field("header", &self.this.header);
                    dbg.field("create_element", &offset_of!(create_element));
                    dbg.field("\'end", &(self.offset + core::mem::size_of::<Widget>()));
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct Widget {
                    header: VtableHeaderOpt,
                    pub create_element: Option<fn(&CRc<Self>) -> CRc<Element>>,
                }

                #[allow(unused_variables)]
                impl Widget {
                    pub const DEFAULT: Self = Self {
                        header: VtableHeaderOpt::DEFAULT,
                        create_element: None,
                    };

                    pub const fn init(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => Self::TYPE,
                            Some(ty) => ty,
                        };
                        self.header = VtableHeaderOpt::new(ty, offset);
                    }

                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            header: self.header.assert_init(),
                            create_element: self.create_element.unwrap(),
                        }
                    }
                }
            }

            pub static TYPE: TypeInfo<0> = TypeInfo::new_abstract_class::<super::Widget>(
                None,
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "Widget",
            );
        }

        impl Widget<RcDyn<Widget>> {
            #[inline]
            pub fn create_element(&self) -> CRc<Element> {
                (self.0.vtable().create_element)(self)
            }
        }

        impl Widget<RcDyn<Widget>, NonVirtual> {}
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    mod _StatefulWidget {
        ::classes::_mod_uses!(mod class StatefulWidget);

        ::classes::_def_class!(class StatefulWidget);
        ::classes::_def_class_extends!(StatefulWidget: Widget);

        mod data {
            ::classes::_mod_uses!(mod data);

            #[repr(C)]
            pub struct StatefulWidget {
                pub(super) _super: CData<Widget>,
            }

            impl StatefulWidget {
                pub fn new(_self: CRcUninit<Self>) -> CRc<Self> {
                    let _ = |Self { _super: _ }: Self| ();
                    #[allow(unused_unsafe)]
                    unsafe {}
                    let _super = CData::<Widget>::new(_self.into_super());
                    unsafe { _super.into_subclass_unchecked() }
                }
                pub(super) fn create_element(this: &CRc<Self>) -> CRc<Element> {
                    StatefulElement::new(this.clone()).into_superclass()
                }
            }
        }
        mod vtable {
            ::classes::_mod_uses!(mod vtable);

            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct StatefulWidget {
                pub(super) _super: CVtable<Widget>,
                pub create_state: fn(&CRc<Self>) -> CRc<State>,
            }

            impl StatefulWidget {
                pub const fn debug_vtable_layout(
                    &self,
                    offset: usize,
                ) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }

            pub struct DebugVtableLayout<'a> {
                this: &'a self::StatefulWidget,
                offset: usize,
            }

            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    macro_rules! offset_of {
                        ($field:ident) => {
                            self.offset + ::core::mem::offset_of!(StatefulWidget, $field)
                        };
                    }
                    let mut dbg = f.debug_struct(stringify!(StatefulWidget));
                    dbg.field("\'start", &self.offset);
                    dbg.field(
                        "super",
                        &self.this._super.debug_vtable_layout(offset_of!(_super)),
                    );
                    dbg.field("create_state", &offset_of!(create_state));
                    dbg.field(
                        "\'end",
                        &(self.offset + core::mem::size_of::<StatefulWidget>()),
                    );
                    dbg.finish()
                }
            }

            pub(super) mod opt {
                ::classes::_mod_uses!(mod vtable::opt);

                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct StatefulWidget {
                    pub(in super::super) _super: CVtableOpt<Widget>,
                    pub create_state: Option<fn(&CRc<Self>) -> CRc<State>>,
                }

                impl StatefulWidget {
                    pub const DEFAULT: Self = Self {
                        _super: CVtableOpt::<Widget>::DEFAULT,
                        create_state: None,
                    };

                    pub const fn init(&mut self, ty: Option<Type>, offset: usize) {
                        let ty = match ty {
                            None => Self::TYPE,
                            Some(ty) => ty,
                        };
                        self._super.init(Some(ty), offset);
                        self._super.create_element = Some(|this| {
                            CData::<StatefulWidget>::create_element(unsafe {
                                this.as_subclass_unchecked()
                            })
                        });
                    }

                    pub const fn assert_init(self) -> CVtable<Self> {
                        CVtable::<Self> {
                            _super: self._super.assert_init(),
                            create_state: self.create_state.unwrap(),
                        }
                    }
                }
            }
            pub static TYPE: TypeInfo<0> = TypeInfo::new_abstract_class::<super::StatefulWidget>(
                Some(CVtable::<Widget>::TYPE),
                [],
                // #[cfg(debug_assertions)]
                MODULE_PATH,
                // #[cfg(debug_assertions)]
                "StatefulWidget",
            );
        }

        impl StatefulWidget<RcDyn<StatefulWidget>> {
            #[inline]
            pub fn create_state(&self) -> CRc<State> {
                (self.0.vtable().create_state)(self)
            }
            #[inline]
            // TODO: downcast `Element` to `StatefulElement`
            pub fn create_element(&self) -> CRc<Element> {
                (self.0.vtable().as_super().create_element)(self.as_super())
            }
        }

        impl StatefulWidget<RcDyn<StatefulWidget>, NonVirtual> {
            #[inline]
            // TODO: downcast `Element` to `StatefulElement`
            pub fn create_element(&self) -> CRc<Element> {
                CData::<Self>::create_element(self.as_virtual())
            }
        }
    }
}

#[test]
fn gallery_page() {
    use _classes::*;
    mod _classes {
        use super::*;
        pub(super) use _GalleryPage::GalleryPage;
        pub(super) use _GalleryPageState::GalleryPageState;
        pub(super) use _MyElement::MyElement;
        pub(super) use _MyWidget::MyWidget;

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        mod _GalleryPage {
            ::classes::_mod_uses!(mod class GalleryPage);

            ::classes::_def_class!(class GalleryPage);
            ::classes::_def_class_extends!(GalleryPage: StatefulWidget);
            mod data {
                ::classes::_mod_uses!(mod data);

                #[repr(C)]
                pub struct GalleryPage {
                    pub(super) _super: CData<StatefulWidget>,
                }
                impl GalleryPage {
                    pub fn new(mut _self: CRcUninit<Self>) -> CRc<Self> {
                        let _ = |Self { _super: _ }: Self| ();
                        #[allow(unused_unsafe)]
                        unsafe {}
                        let _super = CData::<StatefulWidget>::new(_self.into_super());
                        unsafe { _super.into_subclass_unchecked() }
                    }
                    pub(super) fn create_state(this: &CRc<Self>) -> CRc<GalleryPageState> {
                        println!("GalleryPage::create_state");
                        GalleryPageState::new()
                    }
                    pub(super) fn on_create(this: &CRc<Self>) {
                        println!("GalleryPage::on_create");
                    }
                }
            }
            mod vtable {
                ::classes::_mod_uses!(mod vtable);

                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct GalleryPage {
                    pub(super) _super: CVtable<StatefulWidget>,
                    pub on_create: fn(&CRc<Self>),
                }

                impl GalleryPage {
                    pub const fn debug_vtable_layout(
                        &self,
                        offset: usize,
                    ) -> self::DebugVtableLayout<'_> {
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
                        dbg.field(
                            "super",
                            &self.this._super.debug_vtable_layout(offset_of!(_super)),
                        );
                        dbg.field("on_create", &offset_of!(on_create));
                        dbg.field(
                            "\'end",
                            &(self.offset + core::mem::size_of::<GalleryPage>()),
                        );
                        dbg.finish()
                    }
                }

                pub(super) mod opt {
                    ::classes::_mod_uses!(mod vtable::opt);

                    #[repr(C)]
                    #[derive(Clone, Copy, Default)]
                    pub struct GalleryPage {
                        pub(in super::super) _super: CVtableOpt<StatefulWidget>,
                        pub on_create: Option<fn(&CRc<Self>)>,
                    }

                    #[allow(unused_variables)]
                    impl GalleryPage {
                        pub const DEFAULT: Self = Self {
                            _super: CVtableOpt::<StatefulWidget>::DEFAULT,
                            on_create: None,
                        };

                        pub const fn init(&mut self, ty: Option<Type>, offset: usize) {
                            let ty = match ty {
                                None => Self::TYPE,
                                Some(ty) => ty,
                            };
                            self._super.init(Some(ty), offset);
                            self.on_create = Some(CData::<Self>::on_create);
                            self._super.create_state = Some(|this| {
                                CData::<Self>::create_state(unsafe { this.as_subclass_unchecked() })
                                    .into()
                            });
                        }

                        pub const fn assert_init(self) -> CVtable<Self> {
                            CVtable::<Self> {
                                _super: self._super.assert_init(),
                                on_create: self.on_create.unwrap(),
                            }
                        }
                    }
                }

                pub static TYPE: TypeInfo<0> = TypeInfo::new_concrete_class::<super::GalleryPage>(
                    Some(CVtable::<StatefulWidget>::TYPE),
                    [],
                    // #[cfg(debug_assertions)]
                    MODULE_PATH,
                    // #[cfg(debug_assertions)]
                    "GalleryPage",
                );
            }

            static VTABLE: vtable::GalleryPage = {
                let mut vtable = vtable::opt::GalleryPage::DEFAULT;
                vtable.init(None, 0);
                vtable.assert_init()
            };
            unsafe impl ConcreteClass for GalleryPage {
                const VTABLE: NonNull<Self::Vtable> =
                    unsafe { NonNull::new_unchecked((&raw const VTABLE).cast_mut()) };
            }

            impl GalleryPage<RcDyn<GalleryPage>> {
                #[inline]
                pub fn new() -> Self {
                    CData::<Self>::new(CRcUninit::<Self>::new_uninit())
                }
                #[inline]
                pub fn on_create(&self) {
                    (self.0.vtable().on_create)(self)
                }
                #[inline]
                pub fn create_state(&self) -> CRc<GalleryPageState> {
                    self.as_super().create_state().try_into().unwrap()
                }
            }

            impl GalleryPage<RcDyn<GalleryPage>, NonVirtual> {
                #[inline]
                pub fn create_state(&self) -> CRc<GalleryPageState> {
                    CData::<Self>::create_state(self.as_virtual())
                }
            }
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub(super) mod _GalleryPageState {
            ::classes::_mod_uses!(mod class GalleryPageState);

            ::classes::_def_class!(class GalleryPageState);
            ::classes::_def_class_extends!(GalleryPageState: State);

            mod data {
                ::classes::_mod_uses!(mod data);

                #[repr(C)]
                pub struct GalleryPageState {
                    pub(super) _super: CData<State>,
                }
                impl GalleryPageState {
                    pub fn new(_self: CRcUninit<Self>) -> CRc<Self> {
                        let _ = |Self { _super: _ }: Self| ();
                        #[allow(unused_unsafe)]
                        unsafe {}
                        let _super = CData::<State>::new(_self.into_super());
                        unsafe { _super.into_subclass_unchecked() }
                    }
                    pub(super) fn init_state(this: &CRc<Self>) {
                        this.delegate_super().init_state();
                        println!("GalleryPageState::init_state");
                    }
                    pub(super) fn build(this: &CRc<Self>, cx: &CRc<BuildContext>) -> CRc<Widget> {
                        println!("GalleryPageState::build");
                        cx.widget().as_subclass::<CRc<GalleryPage>>().on_create();
                        MyWidget::new().into_super()
                    }
                }
            }
            mod vtable {
                ::classes::_mod_uses!(mod vtable);

                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct GalleryPageState {
                    pub(super) _super: CVtable<State>,
                }

                impl GalleryPageState {
                    pub const fn debug_vtable_layout(
                        &self,
                        offset: usize,
                    ) -> self::DebugVtableLayout<'_> {
                        self::DebugVtableLayout { this: self, offset }
                    }
                }

                pub struct DebugVtableLayout<'a> {
                    this: &'a self::GalleryPageState,
                    offset: usize,
                }

                impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                    #[allow(unused_macros)]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        macro_rules! offset_of {
                            ($field:ident) => {
                                self.offset + ::core::mem::offset_of!(GalleryPageState, $field)
                            };
                        }
                        let mut dbg = f.debug_struct(stringify!(GalleryPageState));
                        dbg.field("\'start", &self.offset);
                        dbg.field(
                            "super",
                            &self.this._super.debug_vtable_layout(offset_of!(_super)),
                        );
                        dbg.field(
                            "\'end",
                            &(self.offset + core::mem::size_of::<GalleryPageState>()),
                        );
                        dbg.finish()
                    }
                }

                pub(super) mod opt {
                    ::classes::_mod_uses!(mod vtable::opt);

                    #[repr(C)]
                    #[derive(Clone, Copy, Default)]
                    pub struct GalleryPageState {
                        pub(in super::super) _super: CVtableOpt<State>,
                    }

                    #[allow(unused_variables)]
                    impl GalleryPageState {
                        pub const DEFAULT: Self = Self {
                            _super: CVtableOpt::<State>::DEFAULT,
                        };

                        pub const fn init(&mut self, ty: Option<Type>, offset: usize) {
                            let ty = match ty {
                                None => Self::TYPE,
                                Some(ty) => ty,
                            };
                            self._super.init(Some(ty), offset);
                            self._super.init_state = Some(|this| {
                                CData::<GalleryPageState>::init_state(unsafe {
                                    this.as_subclass_unchecked()
                                })
                            });
                            self._super.build = Some(|this, cx| {
                                CData::<GalleryPageState>::build(
                                    unsafe { this.as_subclass_unchecked() },
                                    cx,
                                )
                            });
                        }

                        pub const fn assert_init(self) -> CVtable<Self> {
                            CVtable::<Self> {
                                _super: self._super.assert_init(),
                            }
                        }
                    }
                }
                pub static TYPE: TypeInfo<0> =
                    TypeInfo::new_concrete_class::<super::GalleryPageState>(
                        Some(CVtable::<State>::TYPE),
                        [],
                        // #[cfg(debug_assertions)]
                        MODULE_PATH,
                        // #[cfg(debug_assertions)]
                        "GalleryPageState",
                    );
            }

            static VTABLE: vtable::GalleryPageState = {
                let mut vtable = vtable::opt::GalleryPageState::DEFAULT;
                vtable.init(None, 0);
                vtable.assert_init()
            };
            unsafe impl ConcreteClass for GalleryPageState {
                const VTABLE: NonNull<Self::Vtable> =
                    unsafe { NonNull::new_unchecked((&raw const VTABLE).cast_mut()) };
            }

            impl GalleryPageState<RcDyn<GalleryPageState>> {
                #[inline]
                pub fn new() -> Self {
                    CData::<Self>::new(CRcUninit::<Self>::new_uninit())
                }
                #[inline]
                pub fn init_state(&self) {
                    (self.0.vtable().as_super().init_state)(self.as_super())
                }
                #[inline]
                pub fn build(&self, cx: &CRc<BuildContext>) -> CRc<Widget> {
                    (self.0.vtable().as_super().build)(self.as_super(), cx)
                }
            }

            impl GalleryPageState<RcDyn<GalleryPageState>, NonVirtual> {
                #[inline]
                pub fn init_state(&self) {
                    CData::<Self>::init_state(self.as_virtual())
                }
                #[inline]
                pub fn build(&self, cx: &CRc<BuildContext>) -> CRc<Widget> {
                    CData::<Self>::build(self.as_virtual(), cx)
                }
            }
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub(super) mod _MyWidget {
            ::classes::_mod_uses!(mod class MyWidget);

            ::classes::_def_class!(class MyWidget);
            ::classes::_def_class_extends!(MyWidget: Widget);
            mod data {
                ::classes::_mod_uses!(mod data);

                #[repr(C)]
                pub struct MyWidget {
                    pub(super) _super: CData<Widget>,
                }
                impl MyWidget {
                    pub fn new(_self: CRcUninit<Self>) -> CRc<Self> {
                        let _ = |Self { _super: _ }: Self| ();
                        #[allow(unused_unsafe)]
                        unsafe {}
                        let _super = CData::<Widget>::new(_self.into_super());
                        unsafe { _super.into_subclass_unchecked() }
                    }
                    pub(super) fn create_element(this: &CRc<Self>) -> CRc<Element> {
                        MyElement::new(Some(this.as_super().clone())).into_super()
                    }
                }
            }
            mod vtable {
                ::classes::_mod_uses!(mod vtable);

                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct MyWidget {
                    pub(super) _super: CVtable<Widget>,
                }

                impl MyWidget {
                    pub const fn debug_vtable_layout(
                        &self,
                        offset: usize,
                    ) -> self::DebugVtableLayout<'_> {
                        self::DebugVtableLayout { this: self, offset }
                    }
                }

                pub struct DebugVtableLayout<'a> {
                    this: &'a self::MyWidget,
                    offset: usize,
                }

                impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                    #[allow(unused_macros)]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        macro_rules! offset_of {
                            ($field:ident) => {
                                self.offset + ::core::mem::offset_of!(MyWidget, $field)
                            };
                        }
                        let mut dbg = f.debug_struct(stringify!(#MyWidget));
                        dbg.field("\'start", &self.offset);
                        dbg.field(
                            "super",
                            &self.this._super.debug_vtable_layout(offset_of!(_super)),
                        );
                        dbg.field("\'end", &(self.offset + core::mem::size_of::<MyWidget>()));
                        dbg.finish()
                    }
                }

                pub(super) mod opt {
                    ::classes::_mod_uses!(mod vtable::opt);

                    #[repr(C)]
                    #[derive(Clone, Copy, Default)]
                    pub struct MyWidget {
                        pub(in super::super) _super: CVtableOpt<Widget>,
                    }

                    #[allow(unused_variables)]
                    impl MyWidget {
                        pub const DEFAULT: Self = Self {
                            _super: CVtableOpt::<Widget>::DEFAULT,
                        };

                        pub const fn init(&mut self, ty: Option<Type>, offset: usize) {
                            let ty = match ty {
                                None => Self::TYPE,
                                Some(ty) => ty,
                            };
                            self._super.init(Some(ty), offset);
                            self._super.create_element = Some(|this| {
                                CData::<MyWidget>::create_element(unsafe {
                                    this.as_subclass_unchecked()
                                })
                            });
                        }

                        pub const fn assert_init(self) -> CVtable<Self> {
                            CVtable::<Self> {
                                _super: self._super.assert_init(),
                            }
                        }
                    }
                }

                pub static TYPE: TypeInfo<0> = TypeInfo::new_concrete_class::<super::MyWidget>(
                    Some(CVtable::<Widget>::TYPE),
                    [],
                    // #[cfg(debug_assertions)]
                    MODULE_PATH,
                    // #[cfg(debug_assertions)]
                    "MyWidget",
                );
            }

            static VTABLE: vtable::MyWidget = {
                let mut vtable = vtable::opt::MyWidget::DEFAULT;
                vtable.init(None, 0);
                vtable.assert_init()
            };
            unsafe impl ConcreteClass for MyWidget {
                const VTABLE: NonNull<Self::Vtable> =
                    unsafe { NonNull::new_unchecked((&raw const VTABLE).cast_mut()) };
            }

            impl MyWidget<RcDyn<MyWidget>> {
                #[inline]
                pub fn new() -> Self {
                    CData::<Self>::new(CRcUninit::<Self>::new_uninit())
                }
                #[inline]
                pub fn create_element(&self) -> CRc<Element> {
                    (self.0.vtable().as_super().create_element)(self.as_super())
                }
            }

            impl MyWidget<RcDyn<MyWidget>, NonVirtual> {
                #[inline]
                pub fn create_element(&self) -> CRc<Element> {
                    CData::<Self>::create_element(self.as_virtual())
                }
            }
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub(super) mod _MyElement {
            ::classes::_mod_uses!(mod class MyElement);

            ::classes::_def_class!(class MyElement);
            ::classes::_def_class_extends!(MyElement: Element);
            mod data {
                ::classes::_mod_uses!(mod data);

                #[repr(C)]
                pub struct MyElement {
                    pub(super) _super: CData<Element>,
                }
                impl MyElement {
                    pub fn new(_self: CRcUninit<Self>, widget: Option<CRc<Widget>>) -> CRc<Self> {
                        let _ = |Self { _super: _ }: Self| ();
                        #[allow(unused_unsafe)]
                        unsafe {}
                        let _super = CData::<Element>::new(_self.into_super(), widget);
                        unsafe { _super.into_subclass_unchecked() }
                    }
                }
            }
            mod vtable {
                ::classes::_mod_uses!(mod vtable);

                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct MyElement {
                    pub(super) _super: CVtable<Element>,
                }

                impl MyElement {
                    pub const fn debug_vtable_layout(
                        &self,
                        offset: usize,
                    ) -> self::DebugVtableLayout<'_> {
                        self::DebugVtableLayout { this: self, offset }
                    }
                }

                pub struct DebugVtableLayout<'a> {
                    this: &'a self::MyElement,
                    offset: usize,
                }

                impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                    #[allow(unused_macros)]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        macro_rules! offset_of {
                            ($field:ident) => {
                                self.offset + ::core::mem::offset_of!(MyElement, $field)
                            };
                        }
                        let mut dbg = f.debug_struct(stringify!(MyElement));
                        dbg.field("\'start", &self.offset);
                        dbg.field(
                            "super",
                            &self.this._super.debug_vtable_layout(offset_of!(_super)),
                        );
                        dbg.field("\'end", &(self.offset + core::mem::size_of::<MyElement>()));
                        dbg.finish()
                    }
                }

                pub(super) mod opt {
                    ::classes::_mod_uses!(mod vtable::opt);

                    #[repr(C)]
                    #[derive(Clone, Copy, Default)]
                    pub struct MyElement {
                        pub(in super::super) _super: CVtableOpt<Element>,
                    }

                    #[allow(unused_variables)]
                    impl MyElement {
                        pub const DEFAULT: Self = Self {
                            _super: CVtableOpt::<Element>::DEFAULT,
                        };

                        pub const fn init_header(&mut self, ty: Option<Type>, offset: usize) {
                            let ty = match ty {
                                None => Self::TYPE,
                                Some(ty) => ty,
                            };
                            self._super.init_header(Some(ty), offset);
                        }
                        pub const fn init(&mut self) {
                            self._super.init();
                        }

                        pub const fn assert_init(self) -> CVtable<Self> {
                            CVtable::<Self> {
                                _super: self._super.assert_init(),
                            }
                        }
                    }
                }

                pub static TYPE: TypeInfo<0> = TypeInfo::new_concrete_class::<super::MyElement>(
                    Some(CVtable::<Element>::TYPE),
                    [],
                    // #[cfg(debug_assertions)]
                    MODULE_PATH,
                    // #[cfg(debug_assertions)]
                    "MyElement",
                );
            }

            static VTABLE: vtable::MyElement = {
                let mut vtable = vtable::opt::MyElement::DEFAULT;
                vtable.init_header(None, 0);
                vtable.init();
                vtable.assert_init()
            };
            unsafe impl ConcreteClass for MyElement {
                const VTABLE: NonNull<Self::Vtable> =
                    unsafe { NonNull::new_unchecked((&raw const VTABLE).cast_mut()) };
            }

            impl MyElement<RcDyn<MyElement>> {
                pub fn new(widget: Option<CRc<Widget>>) -> Self {
                    CData::<Self>::new(CRcUninit::<Self>::new_uninit(), widget)
                }
            }
        }
    }

    BUF.take();
    GalleryPage::new().create_element().mount(None);
    assert_eq!(BUF.take(), super::EXPECTED_OUTPUT);
}
