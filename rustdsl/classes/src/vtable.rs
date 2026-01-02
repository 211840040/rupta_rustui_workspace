use core::{
    fmt,
    hash::{Hash, Hasher},
    mem::MaybeUninit,
    num::NonZero,
    ops::Deref,
    ptr::NonNull,
};

use crate::class::{
    ClassData, ClassDataBase, ClassImpl, ClassVtable, ClassVtableBase, ClassVtableOpt, IsClass,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MixinVtableHeader {
    header: MixinHeader,
    data_offset: usize,
}

impl MixinVtableHeader {
    pub const fn new<M: ClassVtable>(data_offset: usize, vtable_offset: usize) -> Self {
        Self {
            header: MixinHeader {
                instance: M::TYPE.as_mixin_instance().expect("expect mixin instance"),
                vtable_offset: NonZero::new(vtable_offset).expect("expect non-zero vtable offset"),
            },
            data_offset,
        }
    }
    pub const fn instance(&self) -> MixinInstanceType {
        self.header.instance
    }
    pub const fn data_offset(&self) -> usize {
        self.data_offset
    }
    pub const fn vtable_offset(&self) -> usize {
        self.header.vtable_offset.get()
    }
}

impl fmt::Debug for MixinVtableHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MixinVtableHeader")
            .field("instance", &self.instance())
            .field("mixin_offset", &self.instance().mixin_offset())
            .field(
                "super_offset",
                &(self.vtable_offset() - self.instance().mixin_offset()),
            )
            .field("data_offset", &self.data_offset())
            .finish()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MixinData<D> {
    data: core::marker::PhantomData<D>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MixinVtable<V> {
    header: MixinVtableHeader,
    vtable: core::marker::PhantomData<V>,
}

impl<V: ClassVtableBase> MixinVtable<V> {
    pub fn debug_vtable_layout(&self) -> MixinDebugVtableLayout<'_, V, true> {
        ClassVtableBase::debug_vtable_layout(self, 0)
    }
}

impl<D: IsClass> IsClass for MixinData<D> {
    type Class = D::Class;
}

impl<D: ClassDataBase> ClassDataBase for MixinData<D> {
    type Vtable = MixinVtable<D::Vtable>;
}

impl<D: ClassDataBase> ClassData for MixinData<D> {}

impl<D: IsClass> IsClass for MixinVtable<D> {
    type Class = D::Class;
}

impl<V: ClassVtableBase> ClassVtableBase for MixinVtable<V> {
    type Data = MixinData<V::Data>;
    type Opt = V::Opt;

    const TYPE: Type = V::TYPE;

    type DebugVtableLayout<'a> = MixinDebugVtableLayout<'a, V, true>;
    fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_> {
        let vtable_offset =
            offset + self.header.vtable_offset() - self.header.instance().mixin_offset();
        MixinDebugVtableLayout {
            headers: DebugMixinVtableHeaders::non_exhaustive(core::slice::from_ref(&self.header)),
            vtable: self.vtable_without_super(),
            vtable_offset,
        }
    }
}
unsafe impl<V: ClassVtableBase> ClassVtable for MixinVtable<V> {}

impl<V: ClassVtableBase> MixinVtable<V> {
    pub const TYPE: Type = V::TYPE;
    pub const MIXIN_HEADER_ENTRIES: usize = V::MIXIN_HEADER_ENTRIES;
}

impl<V: ClassVtableBase> MixinVtable<V> {
    const fn cast_header(this: *const Self) -> *const VtableHeader {
        this.cast()
    }
    pub const fn header(&self) -> &VtableHeader {
        unsafe { &*Self::cast_header(self) }
    }
    #[track_caller]
    pub const fn ty(&self) -> Type {
        self.object_ty().as_type()
    }
    #[track_caller]
    pub const fn object_ty(&self) -> ConcreteClassType {
        let offset_of_object_header = self.header().offset_of_object_header();
        unsafe { &*Self::cast_header(self).byte_offset(offset_of_object_header) }
            .object_ty()
            .expect("expect object type")
    }

    pub const fn mixin_header(&self) -> &MixinVtableHeader {
        &self.header
    }

    pub const fn data_without_super(&self, data: &MixinData<V::Data>) -> &V::Data {
        unsafe {
            &*core::ptr::from_ref(data)
                .byte_add(self.mixin_header().data_offset())
                .cast()
        }
    }

    pub const fn vtable_without_super(&self) -> &V {
        unsafe {
            &*core::ptr::from_ref(self)
                .byte_add(self.mixin_header().vtable_offset())
                .cast()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VtableWithMixinHeader<V: ClassVtableBase, const N: usize> {
    headers: [MixinVtableHeader; N],
    vtable: V,
}

impl<V: ClassVtableBase, const N: usize> VtableWithMixinHeader<V, N> {
    pub const fn new(headers: [MixinVtableHeader; N], vtable: V) -> Self {
        Self { headers, vtable }
    }

    pub const fn vtable(&self) -> &V {
        unsafe { self.vtable_ptr().as_ref() }
    }
    pub const fn vtable_ptr(&self) -> NonNull<V> {
        debug_assert!(
            core::mem::align_of_val(self) == core::mem::size_of::<*const ()>(),
            "Vtable must have the same alignment as a pointer"
        );
        debug_assert!(N == V::MIXIN_HEADER_ENTRIES);
        // We do not use `self.vtable` here because we need the provenance
        // of the whole vtable, including the headers.
        unsafe {
            NonNull::new_unchecked(core::ptr::from_ref(self).cast_mut().cast())
                .byte_add(core::mem::offset_of!(Self, vtable))
        }
    }
    pub const fn debug_vtable_layout(&self) -> MixinDebugVtableLayout<'_, V, false> {
        MixinDebugVtableLayout {
            headers: DebugMixinVtableHeaders::exhaustive(&self.headers),
            vtable: &self.vtable,
            vtable_offset: 0,
        }
    }
}

struct DebugMixinVtableHeaders<'a, const NON_EXHAUSTIVE: bool> {
    headers: &'a [MixinVtableHeader],
}

impl<'a> DebugMixinVtableHeaders<'a, false> {
    pub const fn exhaustive(headers: &'a [MixinVtableHeader]) -> Self {
        DebugMixinVtableHeaders { headers }
    }
}

impl<'a> DebugMixinVtableHeaders<'a, true> {
    pub const fn non_exhaustive(headers: &'a [MixinVtableHeader]) -> Self {
        Self { headers }
    }
}

impl<'a, const NON_EXHAUSTIVE: bool> fmt::Debug for DebugMixinVtableHeaders<'a, NON_EXHAUSTIVE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_list();
        dbg.entries(self.headers);
        if NON_EXHAUSTIVE {
            dbg.finish_non_exhaustive()
        } else {
            dbg.finish()
        }
    }
}

pub struct MixinDebugVtableLayout<'a, V: ClassVtableBase, const NON_EXHAUSTIVE: bool> {
    headers: DebugMixinVtableHeaders<'a, NON_EXHAUSTIVE>,
    vtable: &'a V,
    vtable_offset: usize,
}

impl<'a, V: ClassVtableBase, const EXHAUSTIVE: bool> fmt::Debug
    for MixinDebugVtableLayout<'a, V, EXHAUSTIVE>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.headers)
            .field(&self.vtable.debug_vtable_layout(self.vtable_offset))
            .finish()
    }
}

#[repr(C)]
pub struct MaybeUninitVtableWithMixinHeader<V: ClassVtableOpt, const N: usize> {
    headers: [MaybeUninit<MixinVtableHeader>; N],
    vtable_opt: V,
}

impl<V: ClassVtableOpt, const N: usize> MaybeUninitVtableWithMixinHeader<V, N> {
    pub const fn new(vtable_opt: V) -> Self {
        Self {
            headers: [MaybeUninit::uninit(); N],
            vtable_opt,
        }
    }

    pub const fn headers_mut(&mut self) -> &mut [MaybeUninit<MixinVtableHeader>] {
        &mut self.headers
    }

    pub const fn vtable_opt_mut(&mut self) -> &mut V {
        // We do not use `self.vtable` here because we need the provenance
        // of the whole vtable, including the headers.
        unsafe {
            &mut *core::ptr::from_mut(self)
                .byte_add(core::mem::offset_of!(Self, vtable_opt))
                .cast()
        }
    }

    pub const unsafe fn headers_assume_init(self) -> ([MixinVtableHeader; N], V) {
        let headers = unsafe { *core::ptr::from_ref(&self.headers).cast() };
        (headers, self.vtable_opt)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VtableHeaderOpt {
    opt_ty: Option<Type>,
    header: VtableHeader,
}

impl Default for VtableHeaderOpt {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl VtableHeaderOpt {
    pub const DEFAULT: Self = Self { opt_ty: None };

    #[inline]
    pub const fn new(ty: Type, offset: usize) -> Self {
        let header = VtableHeader::new(ty, offset);
        Self { header }
    }

    pub const fn assert_init(self) -> VtableHeader {
        assert!(
            unsafe { self.opt_ty.is_some() },
            "vtable header must be initialized"
        );
        unsafe { self.header }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VtableHeader {
    _impl: VtableHeaderImpl,
    object: ObjectHeader,
    class: ClassHeader,
    mixin: MixinHeader,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VtableHeaderImpl {
    class_ty: Type,
    offset: usize,
}

impl VtableHeaderImpl {
    const fn offset_of_object_header(&self) -> isize {
        match self.class_ty.kind() {
            TypeKind::ConcreteClass
            | TypeKind::ConcreteMixinClass
            | TypeKind::AbstractClass
            | TypeKind::AbstractMixinClass => -(self.offset as isize),
            TypeKind::AbstractMixin => panic!("mixin type cannot appear in vtable header"),
            TypeKind::MixinInstance => unsafe {
                self.class_ty.as_mixin_instance_unchecked().mixin_offset() as isize
            },
        }
    }
}

impl VtableHeader {
    const fn _impl(&self) -> &VtableHeaderImpl {
        unsafe { &self._impl }
    }
    pub const fn class_ty(&self) -> Type {
        self._impl().class_ty
    }
    pub const fn offset_of_object_header(&self) -> isize {
        self._impl().offset_of_object_header()
    }
    pub const fn object_ty(&self) -> Option<ConcreteClassType> {
        match self.offset_of_object_header() {
            0 => Some(unsafe { self.object.object_ty }),
            _ => None,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ObjectHeader {
    object_ty: ConcreteClassType,
    offset: _Pad0,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ClassHeader {
    class_ty: Type,
    offset: NonZero<usize>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MixinHeader {
    instance: MixinInstanceType,
    vtable_offset: NonZero<usize>,
}

impl fmt::Debug for VtableHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self._impl().offset, self.class_ty().kind()) {
            (0, kind) if kind.is_class() => f
                .debug_struct("ObjectHeader")
                .field("object_ty", &unsafe { self.object.object_ty })
                .finish(),
            (_, kind) if kind.is_class() => f
                .debug_struct("ClassHeader")
                .field("class_ty", &unsafe { self.class.class_ty })
                .field("offset", &-unsafe { self.class.offset.get() as isize })
                .finish(),
            (_, TypeKind::AbstractMixin) => {
                unreachable!("mixin type cannot appear in vtable header")
            }
            (0, TypeKind::MixinInstance) => {
                unreachable!("mixin instance type should not have zero offset")
            }
            (_, TypeKind::MixinInstance) => f
                .debug_struct("MixinHeader")
                .field("instance", unsafe { self.mixin.instance.as_inner() })
                .field("vtable_offset", &-unsafe {
                    self.mixin.vtable_offset.get() as isize
                })
                .finish(),
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !f.alternate() {
            return self.as_header().fmt(f);
        }
        match self.as_header().kind() {
            TypeKind::ConcreteClass | TypeKind::ConcreteMixinClass => {
                unsafe { &self.concrete_class }.fmt(f)
            }
            TypeKind::AbstractMixin => unsafe { &self.mixin }.fmt(f),
            TypeKind::AbstractClass => unsafe { &self.abstract_class }.fmt(f),
            TypeKind::AbstractMixinClass => unsafe { &self.abstract_class }.fmt(f),
            TypeKind::MixinInstance => unsafe { &self.mixin_instance }.fmt(f),
        }
    }
}

impl VtableHeader {
    #[inline]
    pub(crate) const fn new(ty: Type, offset: usize) -> Self {
        match NonZero::new(offset) {
            None => Self::new_object(ty),
            Some(offset) => Self::new_class(ty, offset),
        }
    }
    const fn new_object(object_ty: Type) -> Self {
        let object = ObjectHeader {
            object_ty: object_ty
                .as_concrete_class()
                .expect("expect concrete class type"),
            offset: _Pad0::_Pad0,
        };
        Self { object }
    }
    const fn new_class(class_ty: Type, offset: NonZero<usize>) -> Self {
        let class = ClassHeader { class_ty, offset };
        Self { class }
    }
}

mod sealed {
    pub trait TypeInfo: Clone + Copy + 'static {}
    impl TypeInfo for super::TypeInfo {}
    impl TypeInfo for super::TypeInfoHeader {}
    impl TypeInfo for super::ConcreteClassTypeInfo {}
    impl TypeInfo for super::AbstractClassTypeInfo {}
    impl TypeInfo for super::MixinInstanceTypeInfo {}
    impl TypeInfo for super::MixinTypeInfo {}
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Type<T: sealed::TypeInfo = TypeInfo>(NonNull<T>);
pub type ConcreteClassType = Type<ConcreteClassTypeInfo>;
pub type AbstractClassType = Type<AbstractClassTypeInfo>;
pub type MixinInstanceType = Type<MixinInstanceTypeInfo>;
pub type MixinType = Type<MixinTypeInfo>;

// SAFETY: It is same because it is read-only
unsafe impl<T: sealed::TypeInfo> Sync for Type<T> {}

impl<T: sealed::TypeInfo> Deref for Type<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_inner()
    }
}

impl<T: sealed::TypeInfo> Type<T> {
    const fn as_inner(self) -> &'static T {
        unsafe { self.0.as_ref() }
    }

    pub(crate) const fn as_header(self) -> &'static TypeInfoHeader {
        self.type_info().as_header()
    }

    const fn type_info(self) -> &'static TypeInfo {
        unsafe { self.0.cast().as_ref() }
    }

    pub const fn kind(self) -> TypeKind {
        self.as_header().kind()
    }

    pub const fn as_concrete_class(self) -> Option<ConcreteClassType> {
        match self.kind() {
            TypeKind::ConcreteClass => Some(Type(self.0.cast())),
            _ => None,
        }
    }

    pub const fn as_abstract_class(self) -> Option<AbstractClassType> {
        match self.kind() {
            TypeKind::AbstractClass => Some(Type(self.0.cast())),
            _ => None,
        }
    }

    pub const fn as_mixin_instance(self) -> Option<MixinInstanceType> {
        match self.kind() {
            TypeKind::MixinInstance => Some(Type(self.0.cast())),
            _ => None,
        }
    }

    pub const fn as_mixin(self) -> Option<MixinType> {
        match self.kind() {
            TypeKind::AbstractMixin => Some(Type(self.0.cast())),
            _ => None,
        }
    }

    pub const fn as_type(self) -> Type {
        Type(self.0.cast())
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub const unsafe fn as_concrete_class_unchecked(self) -> ConcreteClassType {
        self.debug_assert_kind(TypeKind::ConcreteClass);
        Type(self.0.cast())
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub const unsafe fn as_abstract_class_unchecked(self) -> AbstractClassType {
        self.debug_assert_kind(TypeKind::AbstractClass);
        Type(self.0.cast())
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub const unsafe fn as_mixin_instance_unchecked(self) -> MixinInstanceType {
        self.debug_assert_kind(TypeKind::MixinInstance);
        Type(self.0.cast())
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub const unsafe fn as_mixin_unchecked(self) -> MixinType {
        self.debug_assert_kind(TypeKind::AbstractMixin);
        Type(self.0.cast())
    }

    #[cfg(debug_assertions)]
    #[track_caller]
    const fn debug_assert_kind(self, kind: TypeKind) {
        use TypeKind::*;
        match (self.kind(), kind) {
            (ConcreteClass, ConcreteClass)
            | (ConcreteMixinClass, ConcreteMixinClass)
            | (AbstractMixin, AbstractMixin)
            | (AbstractClass, AbstractClass)
            | (AbstractMixinClass, AbstractMixinClass)
            | (MixinInstance, MixinInstance) => {}
            (ConcreteClass, ConcreteMixinClass) => {
                panic!("expected `mixin class`, but got `class`")
            }
            (ConcreteClass, AbstractMixin) => {
                panic!("expected `abstract mixin`, but got `class`")
            }
            (ConcreteClass, AbstractClass) => {
                panic!("expected `abstract class`, but got `class`")
            }
            (ConcreteClass, AbstractMixinClass) => {
                panic!("expected `abstract mixin class`, but got `class`")
            }
            (ConcreteClass, MixinInstance) => {
                panic!("expected `mixin instance`, but got `class`")
            }
            (ConcreteMixinClass, ConcreteClass) => {
                panic!("expected `class`, but got `mixin class`")
            }
            (ConcreteMixinClass, AbstractMixin) => {
                panic!("expected `abstract mixin`, but got `mixin class`")
            }
            (ConcreteMixinClass, AbstractClass) => {
                panic!("expected `abstract class`, but got `mixin class`")
            }
            (ConcreteMixinClass, AbstractMixinClass) => {
                panic!("expected `abstract mixin class`, but got `mixin class`")
            }
            (ConcreteMixinClass, MixinInstance) => {
                panic!("expected `{{mixin-instance}}`, but got `mixin class`")
            }
            (AbstractMixin, ConcreteClass) => {
                panic!("expected `class`, but got `abstract mixin`")
            }
            (AbstractMixin, ConcreteMixinClass) => {
                panic!("expected `mixin class`, but got `abstract mixin`")
            }
            (AbstractMixin, AbstractClass) => {
                panic!("expected `class`, but got `abstract mixin`")
            }
            (AbstractMixin, AbstractMixinClass) => {
                panic!("expected `abstract mixin class`, but got `abstract mixin`")
            }
            (AbstractMixin, MixinInstance) => {
                panic!("expected `{{mixin-instance}}`, but got `abstract mixin`")
            }
            (AbstractClass, ConcreteClass) => {
                panic!("expected `class`, but got `abstract class`")
            }
            (AbstractClass, ConcreteMixinClass) => {
                panic!("expected `mixin class`, but got `abstract class`")
            }
            (AbstractClass, AbstractMixin) => {
                panic!("expected `abstract mixin`, but got `abstract class`")
            }
            (AbstractClass, AbstractMixinClass) => {
                panic!("expected `abstract mixin class`, but got `abstract class`")
            }
            (AbstractClass, MixinInstance) => {
                panic!("expected `{{mixin-instance}}`, but got `abstract class`")
            }
            (AbstractMixinClass, ConcreteClass) => {
                panic!("expected `class`, but got `abstract mixin class`")
            }
            (AbstractMixinClass, ConcreteMixinClass) => {
                panic!("expected `mixin class`, but got `abstract mixin class`")
            }
            (AbstractMixinClass, AbstractMixin) => {
                panic!("expected `abstract mixin`, but got `abstract mixin class`")
            }
            (AbstractMixinClass, AbstractClass) => {
                panic!("expected `abstract class`, but got `abstract mixin class`")
            }
            (AbstractMixinClass, MixinInstance) => {
                panic!("expected `{{mixin-instance}}`, but got `abstract mixin class`")
            }
            (MixinInstance, ConcreteClass) => {
                panic!("expected `class`, but got `{{mixin-instance}}`")
            }
            (MixinInstance, ConcreteMixinClass) => {
                panic!("expected `mixin class`, but got `{{mixin-instance}}`")
            }
            (MixinInstance, AbstractMixin) => {
                panic!("expected `abstract mixin`, but got `{{mixin-instance}}`")
            }
            (MixinInstance, AbstractClass) => {
                panic!("expected `abstract class`, but got `{{mixin-instance}}`")
            }
            (MixinInstance, AbstractMixinClass) => {
                panic!("expected `abstract mixin class`, but got `{{mixin-instance}}`")
            }
        }
    }

    #[cfg(not(debug_assertions))]
    #[inline(always)]
    const fn debug_assert_kind(self, _kind: TypeKind) {}

    // #[cfg(debug_assertions)]
    pub const fn const_eq(self, other: Self) -> bool {
        const fn str_eq(a: &str, b: &str) -> bool {
            let a = a.as_bytes();
            let b = b.as_bytes();
            if a.len() != b.len() {
                return false;
            }
            let mut i = 0;
            while i < a.len() {
                let i = inc(&mut i);
                if a[i] != b[i] {
                    return false;
                }
            }
            true
        }
        str_eq(
            self.as_header().type_id.type_name,
            other.as_header().type_id.type_name,
        ) && str_eq(
            self.as_header().type_id.module_path,
            other.as_header().type_id.module_path,
        )
    }

    pub const fn num_impls(self) -> usize {
        let header = self.as_header();
        match header.abstract_kind() {
            Some(AbstractTypeKind::MixinInstance(mixin)) => mixin.as_inner().impls.len,
            _ => header.num_impls_or_offset,
        }
    }
}

impl<T: sealed::TypeInfo> PartialEq for Type<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.addr() == other.0.addr()
    }
}

impl<T: sealed::TypeInfo> Eq for Type<T> {}
impl<T: sealed::TypeInfo> Hash for Type<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: sealed::TypeInfo> From<Type<T>> for usize {
    fn from(value: Type<T>) -> Self {
        value.0.addr().get()
    }
}

impl<T: sealed::TypeInfo> fmt::Pointer for Type<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: sealed::TypeInfo> fmt::Debug for Type<T> {
    #[cfg(debug_assertions)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = self.as_header();
        write!(
            f,
            "{}::{}",
            header.type_id.module_path, header.type_id.type_name
        )
    }

    #[cfg(not(debug_assertions))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} @ {self:p}", self.kind())
    }
}

impl<T: sealed::TypeInfo> fmt::Display for Type<T> {
    #[cfg(debug_assertions)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_header().type_id.type_name)
    }

    #[cfg(not(debug_assertions))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union TypeInfo<const N: usize = 0> {
    header: TypeInfoHeader,
    // concrete types (with drop_in_place and layout)
    concrete_class: ConcreteClassTypeInfo<N>,
    // abstract types (no drop_in_place or layout, with kind)
    abstract_class: AbstractClassTypeInfo<N>,
    mixin_instance: MixinInstanceTypeInfo<N>,
    mixin: MixinTypeInfo<N>,
}

impl<const N: usize> TypeInfo<N> {
    const fn as_header(&self) -> &TypeInfoHeader {
        unsafe { &self.header }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TypeId {
    // #[cfg(debug_assertions)]
    module_path: &'static str,
    // #[cfg(debug_assertions)]
    type_name: &'static str,
    // #[cfg(not(debug_assertions))]
    // type_id: fn() -> core::any::TypeId,
}

impl TypeId {
    fn fmt(&self, dbg: &mut fmt::DebugStruct) {
        // #[cfg(debug_assertions)]
        dbg.field("name", self);

        // #[cfg(not(debug_assertions))]
        // dbg.field("type_id", &(self.type_id)());
    }
}

#[cfg(debug_assertions)]
impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Debug for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.module_path, self.type_name)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TypeInfoHeader {
    drop_in_place: Option<unsafe fn(*mut ())>,
    layout_or_kind: LayoutOrKind,
    type_id: TypeId,
    _super: Option<Type>,
    num_impls_or_offset: usize,
}

impl fmt::Debug for TypeInfoHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("TypeInfoHeader");
        dbg.field("drop_in_place", &self.drop_in_place);
        if self.kind().is_concrete() {
            dbg.field("layout", &unsafe { self.layout() });
        } else {
            dbg.field("kind", &self.kind());
        }
        self.type_id.fmt(&mut dbg);
        dbg.field("super", &self._super);
        if self.kind().is_mixin_instance() {
            dbg.field("offset", &self.num_impls_or_offset);
        } else {
            dbg.field("num_impls", &self.num_impls_or_offset);
        }
        dbg.finish()
    }
}

impl TypeInfoHeader {
    /// # Safety
    /// - The `ptr` must be valid to drop.
    /// - The class must be a concrete class, i.e. `drop_in_place` must be present.
    pub(crate) unsafe fn drop_in_place(&self, ptr: *mut ()) {
        debug_assert!(
            self.drop_in_place.is_some(),
            "The class must be a concrete class"
        );
        unsafe { self.drop_in_place.unwrap_unchecked()(ptr) }
    }
    /// # Safety
    /// - The class must be a concrete class, i.e. `drop_in_place` must be present.
    pub(crate) const unsafe fn layout(&self) -> core::alloc::Layout {
        debug_assert!(
            self.drop_in_place.is_some(),
            "The class must be a concrete class"
        );
        unsafe { self.layout_or_kind.layout }.into_layout()
    }
    // #[cfg(debug_assertions)]
    pub(crate) const fn type_name(&self) -> &'static str {
        self.type_id.type_name
    }
    pub const fn kind(&self) -> TypeKind {
        if self.drop_in_place.is_some() {
            TypeKind::ConcreteClass
        } else {
            // SAFETY: layout_or_kind is a valid `AbstractTypeKind` when `drop_in_place` is None
            match unsafe { self.layout_or_kind.kind } {
                AbstractTypeKind::Class => TypeKind::AbstractClass,
                AbstractTypeKind::Mixin => TypeKind::AbstractMixin,
                AbstractTypeKind::MixinClass => TypeKind::AbstractMixinClass,
                AbstractTypeKind::MixinInstance(_) => TypeKind::MixinInstance,
            }
        }
    }
    const fn abstract_kind(&self) -> Option<AbstractTypeKind> {
        if self.drop_in_place.is_some() {
            None
        } else {
            Some(unsafe { self.layout_or_kind.kind })
        }
    }
}

impl MixinInstanceType {
    pub(crate) const fn mixin_offset(self) -> usize {
        self.as_inner().mixin_offset.get()
    }

    pub(crate) fn offset_of_impl_with_mixin_header(self, ty: Type) -> usize {
        for (ty1, offset) in self.impls_and_offsets() {
            if ty1.eq_ty_or_eq_mixin(ty) {
                return offset + self.mixin_offset();
            }
        }
        panic!("type not found");
    }

    pub const fn mixin(self) -> MixinType {
        self.as_inner().mixin
    }

    const fn impls_and_offsets<'a>(self) -> TypesAndOffsets<'a> {
        TypesAndOffsets::MixinInstance {
            types: self.mixin().impls(),
            offsets: unsafe {
                core::slice::from_raw_parts(
                    self.as_inner().impl_offsets.as_ptr(),
                    self.mixin().as_inner().impls.len,
                )
            },
        }
    }
}

impl MixinType {
    const fn impls<'a>(self) -> &'a [Type] {
        self.as_inner().impls.as_slice()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
union LayoutOrKind {
    layout: Layout,
    kind: AbstractTypeKind,
}

#[track_caller]
const fn mixin_as_interface_not_supported(offset: isize) {
    assert!(offset == 0, "mixin as interface is not supported");
}

#[track_caller]
pub const fn vtable_opt_upcast_mut<A: ClassVtableOpt, B: ClassVtableOpt>(
    from: &mut A,
) -> (&mut B, usize) {
    let offset = const_vtable_opt_offset_of::<_, B>(from, None).expect("not a subtype");
    let ptr = unsafe { &mut *core::ptr::from_mut(from).byte_add(offset).cast() };
    (ptr, offset)
}

#[track_caller]
pub const fn vtable_opt_upcast_mut_next<'a, A: ClassVtableOpt, B: ClassVtableOpt>(
    from: &'a mut A,
    last_offset: &mut usize,
) -> Option<&'a mut B> {
    let Some(offset) = const_vtable_opt_offset_of::<_, B>(from, Some(*last_offset)) else {
        return None;
    };
    *last_offset = offset;
    let ptr = unsafe { &mut *core::ptr::from_mut(from).byte_add(offset).cast() };
    Some(ptr)
}

const fn const_vtable_opt_offset_of<A: ClassVtableOpt, B: ClassVtableOpt>(
    this: &A,
    last_offset: Option<usize>,
) -> Option<usize> {
    let from_ty = A::Vtable::TYPE;
    assert!(!from_ty.kind().is_mixin());
    let to_ty = B::Vtable::TYPE;
    // assert!(!to_ty.kind().is_mixin_instance());

    let Some(OffsetResult {
        offset,
        to_mixin_instance,
    }) = from_ty.const_next_offset_of(to_ty, 0, last_offset)
    else {
        return None;
    };
    if let Some(mixin_instance) = to_mixin_instance {
        mixin_as_interface_not_supported(offset as isize);

        let mixin_offset = mixin_instance.mixin_offset();
        let mixin_header: &MixinVtableHeader =
            unsafe { &*core::ptr::from_ref(this).byte_sub(mixin_offset).cast() };
        let vtable_offset = mixin_header.vtable_offset();
        return Some(vtable_offset - mixin_offset);
    }
    assert!(offset < core::mem::size_of::<A>());
    Some(offset)
}

fn const_offset_of<A: ClassVtable, B: ClassVtable>() -> Option<isize> {
    struct OffsetOf<A: ClassVtable, B: ClassVtable>(A, B);
    impl<A: ClassVtable, B: ClassVtable> OffsetOf<A, B> {
        const OFFSET: Option<isize> = if A::KIND.is_mixin() {
            None
        } else if let Some(OffsetResult {
            offset,
            to_mixin_instance,
            ..
        }) = A::TYPE.const_offset_of(B::TYPE)
        {
            let mut offset = offset as isize;
            // Here the mixin instance is of mixin `B`.
            if let Some(to_mixin_instance) = to_mixin_instance {
                assert!(B::KIND.is_mixin());
                mixin_as_interface_not_supported(offset);
                offset -= to_mixin_instance.mixin_offset() as isize;
            }

            Some(offset)
        } else {
            None
        };
    }
    OffsetOf::<A, B>::OFFSET
}

fn mixin_offset_of(from_mixin_instance: MixinInstanceType, to: Type) -> Option<isize> {
    Some(
        from_mixin_instance
            .as_type()
            .offset_of(to)?
            .offset_with_mixin_header()
            + from_mixin_instance.mixin_offset() as isize,
    )
}

pub fn vtable_upcast<A: ClassVtable, B: ClassVtable>(from: &A) -> Option<&B> {
    let Some(offset) = const_offset_of::<A, B>() else {
        return vtable_cast(from);
    };
    Some(unsafe { &*core::ptr::from_ref(from).byte_offset(offset).cast() })
}

/// First cast to some subclass of `A`, then cast to its superclass `B`.
pub fn vtable_cast<A: ClassVtable, B: ClassVtable>(from: &A) -> Option<&B> {
    let offset = if from.ty().kind().is_mixin() {
        let from_mixin_instance =
            unsafe { &*core::ptr::from_ref(from).cast::<MixinVtableHeader>() }.instance();
        mixin_offset_of(from_mixin_instance, B::TYPE)?
    } else {
        let offset_from = from.header().offset_of_object_header();
        let offset_to = from.ty().offset_of(B::TYPE)?.offset_with_mixin_header();
        offset_from + offset_to
    };
    Some(unsafe { &*core::ptr::from_ref(from).byte_offset(offset).cast() })
}

pub struct OffsetResult {
    pub offset: usize,
    /// `Some` if the offset is to a mixin type.
    to_mixin_instance: Option<MixinInstanceType>,
}

impl OffsetResult {
    pub const fn offset_with_mixin_header(self) -> isize {
        match self.to_mixin_instance {
            None => self.offset as isize,
            Some(instance) => self.offset as isize - instance.mixin_offset() as isize,
        }
    }
}

const fn inc(i: &mut usize) -> usize {
    core::mem::replace(i, *i + 1)
}

impl Type {
    // #[cfg(debug_assertions)]
    const fn const_eq_ty_or_eq_mixin(self, ty: Type) -> bool {
        self.const_eq(ty) || self.const_eq_mixin(ty).is_some()
    }

    // #[cfg(debug_assertions)]
    const fn const_eq_mixin(self, ty: Type) -> Option<MixinInstanceType> {
        if let (Some(mixin_ty), Some(mixin_instance)) = (ty.as_mixin(), self.as_mixin_instance()) {
            if mixin_instance.as_inner().mixin.const_eq(mixin_ty) {
                return Some(mixin_instance);
            }
        }
        None
    }

    fn eq_ty_or_eq_mixin(self, ty: Type) -> bool {
        self == ty || self.eq_mixin(ty).is_some()
    }

    fn eq_mixin(self, ty: Type) -> Option<MixinInstanceType> {
        if let (Some(mixin_ty), Some(mixin_instance)) = (ty.as_mixin(), self.as_mixin_instance()) {
            return (mixin_instance.as_inner().mixin == mixin_ty).then_some(mixin_instance);
        }
        None
    }

    /// Return the offset of the vtable of `ty` from the vtable of `self`.
    ///
    /// Mixin headers and mixin interfaces are not included.
    pub const fn const_offset_of(self, ty: Type) -> Option<OffsetResult> {
        self.const_next_offset_of(ty, 0, None)
    }

    // #[cfg(debug_assertions)]
    /// Return the offset of the vtable of `ty` from the vtable of `self`.
    ///
    /// Mixin headers and mixin interfaces are not included.
    pub const fn const_next_offset_of(
        self,
        ty: Type,
        accum_offset: usize,
        last_offset: Option<usize>,
    ) -> Option<OffsetResult> {
        const fn filter_result(
            result: OffsetResult,
            last_offset: Option<usize>,
        ) -> Option<OffsetResult> {
            match last_offset {
                None => Some(result),
                Some(last_offset) if result.offset > last_offset => Some(result),
                Some(_) => None,
            }
        }
        if self.const_eq(ty) {
            return filter_result(
                OffsetResult {
                    offset: accum_offset,
                    to_mixin_instance: None,
                },
                last_offset,
            );
        }
        if let Some(mixin_instance) = self.const_eq_mixin(ty) {
            return filter_result(
                OffsetResult {
                    offset: accum_offset,
                    to_mixin_instance: Some(mixin_instance),
                },
                last_offset,
            );
        }
        if let Some(_super) = self.as_super() {
            if let Some(result) = _super.const_next_offset_of(ty, accum_offset, last_offset) {
                return Some(result);
            }
        }
        let impls_and_offsets = self.impls_and_offsets();
        let mut i = 0;
        while i < impls_and_offsets.len() {
            let i = inc(&mut i);
            // skip mixin on mixin instances (these are handled by its super).
            if self.kind().is_mixin_instance() && impls_and_offsets.type_at(i).kind().is_mixin() {
                continue;
            }
            let (current_type, current_offset) = impls_and_offsets.type_and_offset_at(i);
            if let Some(result) =
                current_type.const_next_offset_of(ty, accum_offset + current_offset, last_offset)
            {
                return Some(result);
            }
        }
        None
    }

    /// Return the offset of the vtable of `ty` from the vtable of `self`.
    ///
    /// Mixin headers and mixin interfaces are not included.
    fn offset_of(self, ty: Type) -> Option<OffsetResult> {
        if self == ty {
            return Some(OffsetResult {
                offset: 0,
                to_mixin_instance: None,
            });
        }
        if let Some(mixin_instance) = self.eq_mixin(ty) {
            return Some(OffsetResult {
                offset: 0,
                to_mixin_instance: Some(mixin_instance),
            });
        }
        if let Some(_super) = self.as_super() {
            if let Some(offset_result) = _super.offset_of(ty) {
                return Some(offset_result);
            }
        }
        self.impls_and_offsets()
            .into_iter()
            // mixin interfaces are prcoessed at `self.as_super()`
            .filter(|(ty1, _)| !ty1.kind().is_mixin())
            .find_map(|(ty1, offset)| {
                let result = ty1.offset_of(ty)?;
                Some(OffsetResult {
                    offset: offset + result.offset,
                    to_mixin_instance: result.to_mixin_instance,
                })
            })
    }

    pub fn mixin_instance_of(self, mixin: MixinType) -> Option<MixinInstanceType> {
        let mut this = self;
        loop {
            if let Some(ty) = this.as_mixin_instance() {
                if ty.mixin == mixin {
                    return Some(ty);
                }
            }
            match this.as_super() {
                Some(_super) => this = _super,
                None => return None,
            }
        }
    }

    // #[cfg(debug_assertions)]
    pub const fn mixin_instance_of_const(self, mixin: MixinType) -> Option<MixinInstanceType> {
        let mut this = self;
        loop {
            if let Some(ty) = this.as_mixin_instance() {
                if ty.as_inner().mixin.const_eq(mixin) {
                    return Some(ty);
                }
            }
            match this.as_super() {
                Some(_super) => this = _super,
                None => return None,
            }
        }
    }

    // Only check its direct superinterfaces and recursively check it's superclass's.
    pub fn is_subtype_of(self, ty: Type) -> bool {
        self.eq_ty_or_eq_mixin(ty)
            || self
                .as_super()
                .into_iter()
                .chain(self.impls())
                .any(|ty1| ty1.is_subtype_of(ty))
    }

    // Only check its direct superinterfaces and recursively check it's superclass's.
    // #[cfg(debug_assertions)]
    pub const fn const_is_subtype_of(self, ty: Type) -> bool {
        let mut this = self;
        while !this.const_eq_ty_or_eq_mixin(ty) {
            let impls = this.impls_and_offsets();
            let mut i = 0;
            while i < impls.len() {
                let i = inc(&mut i);
                if impls.type_at(i).const_is_subtype_of(ty) {
                    return true;
                }
            }

            match this.as_super() {
                Some(_super) => this = _super,
                None => return false,
            }
        }
        true
    }

    pub fn is_subclass_of(self, ty: Type) -> bool {
        let mut this = self;
        // while !this.eq_ty_or_eq_mixin(ty) {
        while this != ty {
            match this.as_super() {
                Some(_super) => this = _super,
                None => return false,
            }
        }
        true
    }

    // #[cfg(debug_assertions)]
    pub const fn const_is_subclass_of(self, ty: Type) -> bool {
        let mut this = self;
        // while !this.const_eq_ty_or_eq_mixin(ty) {
        while !this.const_eq(ty) {
            match this.as_super() {
                Some(_super) => this = _super,
                None => return false,
            }
        }
        true
    }

    /// Returns the offset of all mixin vtable headers added by the mixin instances before the vtable.
    pub const fn mixin_offset(self) -> usize {
        let mut this = self;
        loop {
            if let Some(ty) = this.as_mixin_instance() {
                return ty.as_inner().mixin_offset.get();
            }
            if let Some(super_ty) = this.as_super() {
                this = super_ty;
            } else {
                return 0;
            }
        }
    }

    pub const fn mixin_header_entries(self) -> usize {
        let mut this = self;
        let mut entries = 0;
        loop {
            if this.kind().is_mixin_instance() {
                entries += 1;
            }
            if let Some(super_ty) = this.as_super() {
                this = super_ty;
            } else {
                return entries;
            }
        }
    }

    pub const fn as_super(self) -> Option<Type> {
        self.as_header()._super
    }

    const fn impls<'a>(self) -> Types<'a> {
        let type_info = self.type_info();
        match type_info.as_header().abstract_kind() {
            None => Types::AndOffsets(unsafe { &type_info.concrete_class }.impls.as_slice()),
            Some(AbstractTypeKind::Class) => {
                Types::AndOffsets(unsafe { &type_info.abstract_class }.impls.as_slice())
            }
            Some(AbstractTypeKind::Mixin) => {
                Types::Types(unsafe { &type_info.mixin }.impls.as_slice())
            }
            Some(AbstractTypeKind::MixinClass) => Types::Types(&[]),
            Some(AbstractTypeKind::MixinInstance(_)) => Types::Types(
                unsafe { self.as_mixin_instance_unchecked() }
                    .mixin()
                    .impls(),
            ),
        }
    }

    const fn impls_and_offsets<'a>(self) -> TypesAndOffsets<'a> {
        let type_info = self.type_info();
        match type_info.as_header().abstract_kind() {
            None => TypesAndOffsets::Class(unsafe { &type_info.concrete_class }.impls.as_slice()),
            Some(AbstractTypeKind::Class) => {
                TypesAndOffsets::Class(unsafe { &type_info.abstract_class }.impls.as_slice())
            }
            Some(AbstractTypeKind::Mixin) => {
                TypesAndOffsets::Mixin(unsafe { &type_info.mixin }.impls.as_slice())
            }
            Some(AbstractTypeKind::MixinClass) => TypesAndOffsets::MixinInstance {
                types: &[],
                offsets: &[],
            },
            Some(AbstractTypeKind::MixinInstance(_)) => {
                unsafe { self.as_mixin_instance_unchecked() }.impls_and_offsets()
            }
        }
    }
}

#[derive(Debug)]
enum TypesAndOffsets<'a> {
    Mixin(&'a [Type]),
    MixinInstance {
        types: &'a [Type],
        offsets: &'a [usize],
    },
    Class(&'a [TypeAndOffset]),
}

#[allow(dead_code)]
impl<'a> TypesAndOffsets<'a> {
    const fn len(&self) -> usize {
        match self {
            Self::Mixin(types) => types.len(),
            Self::MixinInstance { types, .. } => types.len(),
            Self::Class(offsets) => offsets.len(),
        }
    }

    const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    const fn type_and_offset_at(&self, index: usize) -> (Type, usize) {
        match self {
            Self::Mixin(_) => panic!("mixin types don't have offsets"),
            Self::MixinInstance { types, offsets } => (types[index], offsets[index]),
            Self::Class(offsets) => (offsets[index].ty, offsets[index].offset.get()),
        }
    }

    const fn type_at(&self, index: usize) -> Type {
        match self {
            Self::Mixin(types) => types[index],
            Self::MixinInstance { types, .. } => types[index],
            Self::Class(offsets) => offsets[index].ty,
        }
    }

    const fn offset_at(&self, index: usize) -> usize {
        match self {
            Self::Mixin(_) => panic!("mixin types don't have offsets"),
            &Self::MixinInstance { offsets, .. } => offsets[index],
            Self::Class(offsets) => offsets[index].offset.get(),
        }
    }
}

impl<'a> Iterator for TypesAndOffsets<'a> {
    type Item = (Type, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            &mut Self::MixinInstance {
                types: &[first, ref rest @ ..],
                offsets: &[first_offset, ref rest_offsets @ ..],
            } => {
                *self = Self::MixinInstance {
                    types: rest,
                    offsets: rest_offsets,
                };
                return Some((first, first_offset));
            }
            &mut Self::Class(&[first, ref rest @ ..]) => {
                *self = Self::Class(rest);
                return Some((first.ty, first.offset.get()));
            }
            Self::MixinInstance { types: [], .. } | Self::Class([]) => None,
            _ => unreachable!("Length mismatch of types and offsets: {self:?}"),
        }
    }
}

enum Types<'a> {
    Types(&'a [Type]),
    AndOffsets(&'a [TypeAndOffset]),
}

impl<'a> Iterator for Types<'a> {
    type Item = Type;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            &mut Self::Types(&[first, ref rest @ ..]) => {
                *self = Types::Types(rest);
                return Some(first);
            }
            &mut Self::AndOffsets(&[first, ref rest @ ..]) => {
                *self = Types::AndOffsets(rest);
                return Some(first.ty);
            }
            Self::Types([]) | Self::AndOffsets([]) => None,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    size: usize,
    align: NonZero<usize>,
}

impl Layout {
    pub const fn new<T>() -> Self {
        let layout = core::alloc::Layout::new::<T>();
        let size = layout.size();
        // SAFETY: layout.align() is always a power of two.
        let align = unsafe { NonZero::new_unchecked(layout.align()) };
        Self { size, align }
    }
    pub const fn into_layout(self) -> core::alloc::Layout {
        unsafe { core::alloc::Layout::from_size_align_unchecked(self.size, self.align.get()) }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ConcreteClassTypeInfo<const N: usize = 0> {
    drop_in_place: unsafe fn(*mut ()),
    layout: Layout,
    type_id: TypeId,
    _super: Option<Type>,
    impls: List<TypeAndOffset, N>,
}

impl<const N: usize> fmt::Debug for ConcreteClassTypeInfo<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("Class");
        dbg.field("drop_in_place", &self.drop_in_place);
        dbg.field("layout", &self.layout);
        self.type_id.fmt(&mut dbg);
        dbg.field("super", &self._super);
        dbg.field("impls", &self.impls.as_slice());
        dbg.finish()
    }
}

#[repr(usize)]
#[derive(Clone, Copy)]
enum _Pad0 {
    _Pad0 = 0,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TypeKindFlags: usize {
        const CLASS = 1 << 0;
        const ABSTRACT = 1 << 1;
        const MIXIN = 1 << 2;
        // mixin instance is an abstract class by default.
        const MIXIN_INSTANCE = Self::ABSTRACT.bits() | 1 << 3;

        const CONCRETE_CLASS = Self::CLASS.bits();
        const CONCRETE_MIXIN_CLASS = Self::MIXIN.bits() | Self::CLASS.bits();
        // mixin is treated as an abstract type.
        const ABSTRACT_MIXIN = Self::ABSTRACT.bits() | Self::MIXIN.bits();
        const ABSTRACT_CLASS = Self::ABSTRACT.bits() | Self::CLASS.bits();
        const ABSTRACT_MIXIN_CLASS = Self::ABSTRACT.bits() | Self::MIXIN.bits() | Self::CLASS.bits();
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    // concrete types
    ConcreteClass = TypeKindFlags::CONCRETE_CLASS.bits(),
    // we cannot tell the difference between ConcreteMixinClass and ConcreteClass,
    // so we use `ConcreteClass` to represent both.
    ConcreteMixinClass = TypeKindFlags::CONCRETE_MIXIN_CLASS.bits(),
    // abstract types
    AbstractMixin = TypeKindFlags::ABSTRACT_MIXIN.bits(),
    AbstractClass = TypeKindFlags::ABSTRACT_CLASS.bits(),
    AbstractMixinClass = TypeKindFlags::ABSTRACT_MIXIN_CLASS.bits(),
    MixinInstance = TypeKindFlags::MIXIN_INSTANCE.bits(),
}

impl TypeKind {
    pub const fn is_class(self) -> bool {
        TypeKindFlags::from_type_kind(self).is_class()
    }
    pub const fn is_abstract(self) -> bool {
        TypeKindFlags::from_type_kind(self).is_abstract()
    }
    pub const fn is_concrete(self) -> bool {
        TypeKindFlags::from_type_kind(self).is_concrete()
    }
    pub const fn is_mixin(self) -> bool {
        TypeKindFlags::from_type_kind(self).is_mixin()
    }
    pub const fn is_mixin_instance(self) -> bool {
        TypeKindFlags::from_type_kind(self).is_mixin_instance()
    }
}

impl TypeKindFlags {
    const fn from_type_kind(kind: TypeKind) -> Self {
        TypeKindFlags::from_bits_truncate(kind as usize)
    }
    pub const fn is_class(self) -> bool {
        self.contains(Self::CLASS)
    }
    pub const fn is_abstract(self) -> bool {
        self.contains(Self::ABSTRACT)
    }
    pub const fn is_concrete(self) -> bool {
        !self.is_abstract()
    }
    pub const fn is_mixin(self) -> bool {
        self.contains(Self::MIXIN)
    }
    pub const fn is_mixin_instance(self) -> bool {
        let is_mixin_instance = self.contains(Self::MIXIN_INSTANCE);
        debug_assert!(is_mixin_instance == (self.bits() == Self::MIXIN_INSTANCE.bits()));
        is_mixin_instance
    }
}

#[repr(C, usize)]
#[derive(Clone, Copy)]
enum AbstractTypeKind {
    #[expect(dead_code, reason = "constructed by another union variant")]
    Class = TypeKind::AbstractClass as usize,
    #[expect(dead_code, reason = "constructed by another union variant")]
    Mixin = TypeKind::AbstractMixin as usize,
    #[expect(dead_code, reason = "constructed by another union variant")]
    MixinClass = TypeKind::AbstractMixinClass as usize,
    #[expect(dead_code, reason = "constructed by another union variant")]
    MixinInstance(MixinType) = TypeKind::MixinInstance as usize,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
enum TypeKindClass {
    Class = TypeKind::AbstractClass as usize,
}

#[repr(usize)]
#[derive(Clone, Copy)]
enum TypeKindMixin {
    Mixin = TypeKind::AbstractMixin as usize,
}

#[repr(usize)]
#[derive(Clone, Copy)]
enum TypeKindMixinInstance {
    MixinInstance = TypeKind::MixinInstance as usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AbstractClassTypeInfo<const N: usize = 0> {
    drop_in_place: _Pad0,
    kind: TypeKindClass,
    _pad: _Pad0,
    type_id: TypeId,
    _super: Option<Type>,
    impls: List<TypeAndOffset, N>,
}

impl<const N: usize> fmt::Debug for AbstractClassTypeInfo<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("AbstractClass");
        self.type_id.fmt(&mut dbg);
        dbg.field("super", &self._super);
        dbg.field("impls", &self.impls.as_slice());
        dbg.finish()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MixinInstanceTypeInfo<const N: usize = 0> {
    drop_in_place: _Pad0,
    kind: TypeKindMixinInstance,
    mixin: Type<MixinTypeInfo>,
    type_id: TypeId,
    _super: Type,
    /// Offset from the start of the mixin vtable to the start of the vtable except the mixin headers.
    mixin_offset: NonZero<usize>,
    impl_offsets: [usize; N],
}

impl<const N: usize> fmt::Debug for MixinInstanceTypeInfo<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("MixinInstance");
        dbg.field("mixin", &self.mixin);
        self.type_id.fmt(&mut dbg);
        dbg.field("super", &self._super);
        dbg.field("mixin_offset", &self.mixin_offset);
        dbg.field("impl_offsets", &unsafe {
            core::slice::from_raw_parts(self.impl_offsets.as_ptr(), self.mixin.as_inner().impls.len)
        });
        dbg.finish()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
union LayoutOrMixinKind {
    layout: Layout,
    kind: TypeKindMixin,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MixinTypeInfo<const N: usize = 0> {
    drop_in_place: Option<unsafe fn(*mut ())>,
    layout_or_kind: LayoutOrMixinKind,
    type_id: TypeId,
    _super: Option<Type>,
    impls: List<Type, N>,
}

impl<const N: usize> fmt::Debug for MixinTypeInfo<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("Mixin");
        dbg.field("drop_in_place", &self.drop_in_place);
        if self.drop_in_place.is_some() {
            dbg.field("layout", unsafe { &self.layout_or_kind.layout });
        }
        self.type_id.fmt(&mut dbg);
        dbg.field("super", &self._super);
        dbg.field("impls", &self.impls.as_slice());
        dbg.finish()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct TypeAndOffset {
    ty: Type,
    offset: NonZero<usize>,
}

impl PartialEq<Type> for TypeAndOffset {
    fn eq(&self, other: &Type) -> bool {
        self.ty == *other
    }
}

impl<const N: usize> TypeInfo<N> {
    pub const fn new_mixin_instance<T: ClassImpl>(
        _super: Type,
        mixin: MixinType,
        impl_offsets: [usize; N],
        // #[cfg(debug_assertions)]
        module_path: &'static str,
        // #[cfg(debug_assertions)]
        type_name: &'static str,
    ) -> Self {
        debug_assert!(mixin.num_impls() == N);
        let mixin_offset =
            NonZero::new(_super.mixin_offset() + core::mem::size_of::<MixinVtableHeader>())
                .unwrap();
        let this = Self {
            mixin_instance: MixinInstanceTypeInfo {
                drop_in_place: _Pad0::_Pad0,
                kind: TypeKindMixinInstance::MixinInstance,
                mixin,
                type_id: TypeId {
                    // #[cfg(debug_assertions)]
                    module_path,
                    // #[cfg(debug_assertions)]
                    type_name,
                    // #[cfg(not(debug_assertions))]
                    // type_id: core::any::TypeId::of::<T::Data>,
                },
                _super,
                mixin_offset,
                impl_offsets,
            },
        };
        debug_assert!(matches!(this.kind(), TypeKind::MixinInstance));
        this
    }

    pub const fn new_mixin<T: ClassImpl>(
        impls: [Type; N],
        // #[cfg(debug_assertions)]
        module_path: &'static str,
        // #[cfg(debug_assertions)]
        type_name: &'static str,
    ) -> Self {
        let this = Self {
            mixin: MixinTypeInfo {
                drop_in_place: None,
                layout_or_kind: LayoutOrMixinKind {
                    kind: TypeKindMixin::Mixin,
                },
                type_id: TypeId {
                    // #[cfg(debug_assertions)]
                    module_path,
                    // #[cfg(debug_assertions)]
                    type_name,
                    // #[cfg(not(debug_assertions))]
                    // type_id: core::any::TypeId::of::<T::Data>,
                },
                _super: None,
                impls: List::new(impls),
            },
        };
        debug_assert!(matches!(this.kind(), TypeKind::AbstractMixin));
        this
    }

    pub const fn new_abstract_class<T: ClassImpl>(
        _super: Option<Type>,
        impls: [(Type, usize); N],
        // #[cfg(debug_assertions)]
        module_path: &'static str,
        // #[cfg(debug_assertions)]
        type_name: &'static str,
    ) -> Self {
        let this = Self {
            abstract_class: AbstractClassTypeInfo {
                drop_in_place: _Pad0::_Pad0,
                kind: TypeKindClass::Class,
                _pad: _Pad0::_Pad0,
                type_id: TypeId {
                    // #[cfg(debug_assertions)]
                    module_path,
                    // #[cfg(debug_assertions)]
                    type_name,
                    // #[cfg(not(debug_assertions))]
                    // type_id: core::any::TypeId::of::<T::Data>,
                },
                _super,
                impls: List::from_impls(impls),
            },
        };
        debug_assert!(matches!(this.kind(), TypeKind::AbstractClass));
        this
    }

    pub const fn new_concrete_class<T: ClassImpl>(
        _super: Option<Type>,
        impls: [(Type, usize); N],
        // #[cfg(debug_assertions)]
        module_path: &'static str,
        // #[cfg(debug_assertions)]
        type_name: &'static str,
    ) -> Self {
        let this = Self {
            concrete_class: ConcreteClassTypeInfo {
                drop_in_place: |ptr| unsafe { core::ptr::drop_in_place::<T::Data>(ptr.cast()) },
                layout: Layout::new::<T::Data>(),
                type_id: TypeId {
                    // #[cfg(debug_assertions)]
                    module_path,
                    // #[cfg(debug_assertions)]
                    type_name,
                    // #[cfg(not(debug_assertions))]
                    // type_id: core::any::TypeId::of::<T::Data>,
                },
                _super,
                impls: List::from_impls(impls),
            },
        };
        debug_assert!(matches!(this.kind(), TypeKind::ConcreteClass));
        this
    }

    pub const fn as_type(&'static self) -> Type {
        Type(
            NonNull::new(core::ptr::from_ref(self).cast_mut())
                .unwrap()
                .cast(),
        )
    }

    const fn kind(&self) -> TypeKind {
        self.as_header().kind()
    }
}

impl<const N: usize> List<TypeAndOffset, N> {
    const fn from_impls(impls: [(Type, usize); N]) -> Self {
        let mut ty_and_offsets = MaybeUninit::<[TypeAndOffset; N]>::uninit();
        let mut i = 0;
        while i < N {
            let i = inc(&mut i);
            unsafe {
                ty_and_offsets
                    .as_mut_ptr()
                    .cast::<TypeAndOffset>()
                    .add(i)
                    .write(TypeAndOffset {
                        ty: impls[i].0,
                        offset: NonZero::new(impls[i].1).unwrap(),
                    });
            }
        }
        List::new(unsafe { ty_and_offsets.assume_init() })
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct List<T, const N: usize = 0> {
    len: usize,
    raw: [T; N],
}

impl<T> Deref for List<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> List<T, N> {
    pub const fn new(raw: [T; N]) -> Self {
        Self { len: N, raw }
    }
}

impl<T, const N: usize> List<T, N> {
    const fn as_slice(&self) -> &[T] {
        debug_assert!(N == 0 || self.len == N);
        unsafe { core::slice::from_raw_parts(self.raw.as_ptr(), self.len) }
    }
}

#[inline(never)]
#[cold]
#[track_caller]
pub(crate) fn cast_failed(target_ty: Type, actual_ty: Type) -> ! {
    panic!("failed to cast object of type {actual_ty} to {target_ty}")
}

// layout tests
const _: () = {
    use core::mem::offset_of;
    macro_rules! layout_test {
        ($header:ident :: $header_field:expr, $($to_test:ident :: $field:expr),* $(,)? ) => {
            assert!(
                true $(&& offset_of!($header, $header_field) == offset_of!($to_test, $field))*
            );
        };
    }
    layout_test!(
        TypeInfoHeader::drop_in_place,
        ConcreteClassTypeInfo::drop_in_place,
        AbstractClassTypeInfo::drop_in_place,
        MixinInstanceTypeInfo::drop_in_place,
        MixinTypeInfo::drop_in_place,
    );
    layout_test!(
        TypeInfoHeader::layout_or_kind.layout,
        ConcreteClassTypeInfo::layout,
        // AbstractClassTypeInfo
        // MixinInstanceTypeInfo
        MixinTypeInfo::layout_or_kind.layout,
    );
    layout_test!(
        TypeInfoHeader::layout_or_kind.kind,
        // ConcreteClassTypeInfo
        AbstractClassTypeInfo::kind,
        MixinInstanceTypeInfo::kind,
        MixinTypeInfo::layout_or_kind.kind,
    );
    layout_test!(
        TypeInfoHeader::layout_or_kind.layout.align,
        ConcreteClassTypeInfo::layout.align,
        AbstractClassTypeInfo::_pad,
        MixinInstanceTypeInfo::mixin,
        MixinTypeInfo::layout_or_kind.layout.align,
    );
    // #[cfg(debug_assertions)]
    layout_test!(
        TypeInfoHeader::type_id.module_path,
        ConcreteClassTypeInfo::type_id.module_path,
        AbstractClassTypeInfo::type_id.module_path,
        MixinInstanceTypeInfo::type_id.module_path,
        MixinTypeInfo::type_id.module_path,
    );
    // #[cfg(debug_assertions)]
    layout_test!(
        TypeInfoHeader::type_id.type_name,
        ConcreteClassTypeInfo::type_id.type_name,
        AbstractClassTypeInfo::type_id.type_name,
        MixinInstanceTypeInfo::type_id.type_name,
        MixinTypeInfo::type_id.type_name,
    );
    // #[cfg(not(debug_assertions))]
    // layout_test!(
    //     TypeInfoHeader::type_id.type_id,
    //     ConcreteClassTypeInfo::type_id.type_id,
    //     AbstractClassTypeInfo::type_id.type_id,
    //     MixinInstanceTypeInfo::type_id.type_id,
    //     MixinTypeInfo::type_id.type_id,
    // );
    layout_test!(
        TypeInfoHeader::_super,
        ConcreteClassTypeInfo::_super,
        AbstractClassTypeInfo::_super,
        MixinInstanceTypeInfo::_super,
        MixinTypeInfo::_super,
    );
    layout_test!(
        TypeInfoHeader::num_impls_or_offset,
        ConcreteClassTypeInfo::impls.len,
        AbstractClassTypeInfo::impls.len,
        MixinInstanceTypeInfo::mixin_offset,
        MixinTypeInfo::impls.len,
    );
    layout_test!(
        ConcreteClassTypeInfo::impls.raw,
        AbstractClassTypeInfo::impls.raw,
        MixinInstanceTypeInfo::impl_offsets,
        MixinTypeInfo::impls.raw,
    );
    layout_test!(
        VtableHeader::_impl.class_ty,
        VtableHeader::class.class_ty,
        VtableHeader::object.object_ty,
        VtableHeader::mixin.instance,
    );
    layout_test!(
        VtableHeader::_impl.offset,
        VtableHeader::class.offset,
        VtableHeader::object.offset,
        VtableHeader::mixin.vtable_offset,
    );
};
