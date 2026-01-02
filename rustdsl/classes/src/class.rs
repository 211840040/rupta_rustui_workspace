use core::{alloc::Layout, any::Any, borrow::Borrow, ptr::NonNull};

use crate::{
    ptr::RcDyn,
    vtable::{
        ConcreteClassType, MixinData, MixinInstanceType, MixinVtable, MixinVtableHeader, Type,
        TypeKind, VtableHeader,
    },
};

pub trait IsClass: Sized {
    type Class: Class;
}

pub trait Class: ClassImpl {
    type Rc: ClassRcWeak<Upgraded = Self::Rc, UpgradedOpt = Self::Rc, DowngradeFrom = Self::Rc>;
    type Weak: ClassRcWeak<Upgraded = Option<Self::Rc>, UpgradedOpt = Self::Rc, DowngradeFrom = Self::Rc>;
    type Ptr;
}

pub trait ClassRcWeak: IsClass + ClassImpl + Clone {
    type Upgraded;
    type UpgradedOpt;
    type DowngradeFrom;

    fn as_ptr(this: &Self) -> crate::prelude::CPtr<Self>;
    fn vtable(this: &Self) -> &Self::Vtable;
    fn upgrade(this: &Self) -> Self::Upgraded;
    fn upgrade_opt(this: Option<&Self>) -> Option<Self::UpgradedOpt>;
    fn downgrade_from(from: &Self::DowngradeFrom) -> Self;
}

pub trait ClassRc:
    ClassRcWeak + Borrow<RcDyn<Self::Class>> + From<RcDyn<Self::Class>> + Into<RcDyn<Self::Class>>
where
    for<'a> &'a Self: From<&'a RcDyn<Self::Class>>,
{
    #[cfg_attr(debug_assertions, track_caller)]
    unsafe fn as_superclass_unchecked<A>(&self) -> &A
    where
        A: ClassRc,
        for<'a> &'a A: From<&'a crate::ptr::RcDyn<A::Class>>,
    {
        unsafe { RcDyn::as_superclass_unchecked::<A::Class>(self.borrow()) }.into()
    }

    fn as_superclass<A>(&self) -> &A
    where
        A: ClassRc,
        for<'a> &'a A: From<&'a crate::ptr::RcDyn<A::Class>>,
    {
        RcDyn::as_superclass::<A::Class>(self.borrow()).into()
    }

    fn try_as_superclass<A>(&self) -> Option<&A>
    where
        A: ClassRc,
        for<'a> &'a A: From<&'a crate::ptr::RcDyn<A::Class>>,
    {
        RcDyn::try_as_superclass::<A::Class>(self.borrow()).map(Into::into)
    }

    unsafe fn into_subclass_unchecked<D>(self) -> D
    where
        D: ClassRc,
        for<'a> &'a D: From<&'a RcDyn<D::Class>>,
    {
        unsafe { RcDyn::into_subclass_unchecked::<D::Class>(self.into()) }.into()
    }

    fn to_supertype<A: ClassRc + From<crate::ptr::RcDyn<A::Class>>>(&self) -> A
    where
        for<'a> &'a A: From<&'a crate::ptr::RcDyn<A::Class>>,
    {
        RcDyn::to_supertype::<A::Class>(self.borrow()).into()
    }
}

/// # Safety
/// The `VTABLE` must be a valid pointer to the vtable of the class,
/// and it must have the provenance of the whole vtable, including
/// the headers (if any offset is applied to the vtable pointer).
pub unsafe trait ConcreteClass: ClassImpl {
    const VTABLE: NonNull<Self::Vtable>;
}

/// # Safety
/// Any fat-pointer to a class must be castable to a fat-pointer to its superclass,
/// i.e., both the `CData` pointer and the `CVtable` must also be castable to the superclass's.
#[diagnostic::on_unimplemented(message = "class `{Self}` doesn't has a superclass")]
pub unsafe trait HasSuper:
    ClassImpl<
        Data: DataHasSuper<SuperData = <Self::Super as ClassImpl>::Data>,
        Vtable: VtableHasSuper<SuperVtable = <Self::Super as ClassImpl>::Vtable>,
    >
{
    type Super: ClassImpl;
    fn cast_super(this: *const Self) -> *const Self::Super {
        this.cast()
    }
    fn as_super(&self) -> &Self::Super {
        unsafe { &*Self::cast_super(self) }
    }
    fn to_super(&self) -> Self::Super
    where
        Self::Super: Clone,
    {
        self.as_super().clone()
    }
    fn into_super(self) -> Self::Super;
}

#[diagnostic::on_unimplemented(message = "class `{Self}` doesn't implements interface `{I}`")]
pub trait HasImpl<I: ClassImpl>: ClassImpl<Vtable: VtableHasImpl<I>> {
    fn to_impl(&self) -> I;
}

#[diagnostic::on_unimplemented(message = "mixin `{Self}` doesn't have a super interface `{I}`")]
pub trait MixinHasImpl<I: ClassImpl>:
    MixinClassImpl<VtableWithoutSuper: MixinVtableHasImpl<I>>
{
    fn to_impl(&self) -> I;
}

// Marker type for a class
#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum ClassMarker {}

// Marker type for virtual functions
#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum Virtual {}

// Marker type for non-virtual functions
#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum NonVirtual {}

#[doc(hidden)]
pub trait ClassImpl: Sized {
    type DataBase: ClassDataBase;
    type Data: ClassData<Vtable = Self::Vtable>;
    type VtableBase: ClassVtableBase;
    type Vtable: ClassVtable<Data = Self::Data, Opt = Self::VtableOpt>;
    type VtableOpt: ClassVtableOpt<Vtable = Self::Vtable>;
}

pub trait MixinClassImpl:
    ClassImpl<
        Data = MixinData<Self::DataWithoutSuper>,
        VtableBase = Self::VtableWithoutSuper,
        Vtable = MixinVtable<Self::VtableWithoutSuper>,
    >
{
    type DataWithoutSuper;
    type VtableWithoutSuper: ClassVtableBase<Data = Self::DataWithoutSuper>;
}

pub trait ClassVtableBase: 'static + Copy + Sized {
    type Data: ClassDataBase;
    type Opt: ClassVtableOpt;
    type DebugVtableLayout<'a>: core::fmt::Debug
    where
        Self: 'a;

    const TYPE: Type;
    const KIND: TypeKind = Self::TYPE.kind();
    const MIXIN_HEADER_ENTRIES: usize = {
        let entries = Self::TYPE.mixin_header_entries();
        assert!(Self::TYPE.mixin_offset() == entries * core::mem::size_of::<MixinVtableHeader>());
        entries
    };

    fn debug_vtable_layout(&self, offset: usize) -> Self::DebugVtableLayout<'_>;
}

/// The vtable of a class, containing the Type header, and function pointers
/// for all the virtual functions.
///
/// # Safety
/// Any pointer to a class's vtable should be castable to the `VtableHeader`
pub unsafe trait ClassVtable: ClassVtableBase {
    #[inline]
    fn cast_header(this: *const Self) -> *const VtableHeader {
        this.cast()
    }
    #[inline]
    fn header(&self) -> &VtableHeader {
        unsafe { &*Self::cast_header(self) }
    }
    #[track_caller]
    fn ty(&self) -> Type {
        self.object_ty().as_type()
    }
    fn object_header(&self) -> &VtableHeader {
        let offset = self.header().offset_of_object_header();
        unsafe { &*Self::cast_header(self).byte_offset(offset) }
    }
    #[track_caller]
    fn object_ty(&self) -> ConcreteClassType {
        self.object_header()
            .object_ty()
            .expect("expect object type")
    }
    #[inline]
    /// # Safety
    /// The `ptr` must be valid to drop.
    unsafe fn drop_in_place(&self, ptr: *mut ()) {
        unsafe { self.object_ty().as_header().drop_in_place(ptr) };
    }
    #[inline]
    unsafe fn layout(&self) -> Layout {
        unsafe { self.object_ty().as_header().layout() }
    }
    #[inline]
    fn is_subtype_of_ty(&self, ty: Type) -> bool {
        self.object_ty().as_type().is_subtype_of(ty)
    }
    #[inline]
    fn is_subclass_of_ty(&self, ty: Type) -> bool {
        self.object_ty().as_type().is_subclass_of(ty)
    }
    #[inline]
    fn is_subtype_of<C: ClassVtable>(&self) -> bool {
        self.object_ty().as_type().is_subtype_of(C::TYPE)
    }
    #[inline]
    fn is_subclass_of<C: ClassVtable>(&self) -> bool {
        self.object_ty().as_type().is_subclass_of(C::TYPE)
    }
    #[inline]
    fn mixin_instance_of<M: ClassVtableBase>(&self) -> Option<MixinInstanceType> {
        self.object_ty()
            .as_type()
            .mixin_instance_of(M::TYPE.as_mixin()?)
    }
    #[inline]
    // #[cfg(debug_assertions)]
    fn type_name(&self) -> &'static str {
        self.object_ty().as_header().type_name()
    }
}

// pub trait MixinInstanceVtable: ClassVtable<Data: MixinInstanceData> {
//     type WithMixinHeader: ClassVtable<Data = Self::Data, Opt = Self::Opt>;
// }
//
// pub unsafe trait MixinVtable: ClassVtable {
//     type VtableIgnoreSuper;
// }

/// The optional vtable of a class comes from the vtable of the class with
/// each of the vtable entry replaced with an `Option` type.
///
/// It is used to initialize the vtable. All vtable entrires are `None`
/// by default, and each implemented function is assigned a `Some` value,
/// from the most superclass to the current class of which the vtable is
/// being constructed.
///
/// After that, the real vtable is built from the optional vtable by
/// unwraping all the `Option` entries. If any of the entry fail to unwrap,
/// it will cause a panic in the `const` context, which will then trigger
/// a compile error, informing which methods are not implemented for any
/// of the concrete classes.
pub trait ClassVtableOpt: 'static + Copy + Sized {
    type VtableBase: ClassVtableBase;
    type Vtable: ClassVtable<Opt = Self>;
}

pub trait ClassDataBase: Any + Sized {
    type Vtable: ClassVtableBase;
}

pub trait ClassData: ClassDataBase<Vtable: ClassVtable> {}

/// # Safety
/// Any pointer to a class's data should be castable to the pointer
/// to the superclass's data, i.e., `Self::SuperData` must be a
/// prefix in `Self`'s memory layout.
pub unsafe trait DataHasSuper: ClassData {
    type SuperData: ClassData;
    fn cast_super(this: *const Self) -> *const Self::SuperData {
        this.cast()
    }
    fn cast_mut_super(this: *mut Self) -> *mut Self::SuperData {
        this.cast()
    }
    fn as_super(&self) -> &Self::SuperData {
        unsafe { &*Self::cast_super(self) }
    }
    fn as_mut_super(&mut self) -> &mut Self::SuperData {
        unsafe { &mut *Self::cast_mut_super(self) }
    }
}

/// # Safety
/// Any pointer to a class's vtable should be castable to the pointer
/// to the superclass's vtable, i.e., `Self::SuperData` must be a
/// prefix in `Self`'s memory layout.
pub unsafe trait VtableHasSuper: ClassVtable {
    type SuperVtable: ClassVtable;
    fn cast_super(this: NonNull<Self>) -> NonNull<Self::SuperVtable> {
        this.cast()
    }
    fn as_super(&self) -> &Self::SuperVtable {
        unsafe { Self::cast_super(self.into()).as_ref() }
    }
}

/// Implement this trait for the vtable of a class which implements another class
/// with a vtable `V`.
///
/// # Safety
///
/// The `OFFSET` must be the field offset of `V`.
pub unsafe trait VtableHasImpl<C: ClassImpl>: ClassVtable {
    const OFFSET: usize;
    fn cast_impl(this: NonNull<Self>) -> NonNull<C::Vtable> {
        unsafe { this.byte_add(Self::OFFSET).cast() }
    }
    fn downcast_impl(imp: NonNull<C::Vtable>) -> NonNull<Self> {
        unsafe { imp.byte_sub(Self::OFFSET).cast() }
    }
}

/// Implement this trait for the vtable of a class which implements another class
/// with a vtable `V`.
///
/// # Safety
///
/// The `OFFSET` must be the field offset of `V`.
pub unsafe trait MixinVtableHasImpl<C: ClassImpl>: ClassVtableBase {
    unsafe fn cast_impl(this: NonNull<MixinVtable<Self>>) -> NonNull<C::Vtable> {
        let offset = unsafe { this.as_ref() }
            .mixin_header()
            .instance()
            .mixin_offset();
        unsafe { this.byte_add(offset).cast() }
    }
    fn downcast_impl(
        imp: NonNull<C::Vtable>,
        instance: MixinInstanceType,
    ) -> NonNull<MixinVtable<Self>> {
        let offset = instance.mixin_offset();
        unsafe { imp.byte_sub(offset).cast() }
    }
}

// /// Implement this trait for the vtable of a class which implements another mixin class
// /// with a vtable `V`.
// ///
// /// # Safety
// ///
// /// The `OFFSET` must be the field offset of `V`.
// pub unsafe trait MixinVtableHasImplBase<V: ClassVtableBase>: ClassVtableBase {}
//
// unsafe impl<C: IsClass, T: MixinVtableHasImplBase<crate::prelude::CVtableBase<C>>>
//     MixinVtableHasImpl<MixinVtable<crate::prelude::CVtableBase<C>>> for T
// {
// }

pub trait MixinWith<S: MixinClassImpl>: IsClass {
    type Instance<T, V>: IsClass;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vtable::TypeInfo;

    use core::hash::{BuildHasher, Hash, Hasher};
    use core::ops::Not;
    use dyn_hash::DynHash;

    macro_rules! def_class {
        ($class:ident $(extends $super:ident)? $(implements $($impl:ident),+)?) => {
            #[derive(Debug, Clone, Copy, Default)]
            struct $class;
            impl ClassImpl for $class {
                type DataBase = $class;
                type Data = $class;
                type VtableBase = $class;
                type Vtable = $class;
                type VtableOpt = $class;
            }
            impl ClassDataBase for $class {
                type Vtable = $class;
            }
            impl ClassData for $class {}
            impl ClassVtableBase for $class {
                const TYPE: Type = TypeInfo::new_abstract_class::<$class>(
                    const {
                        let mut _super = None;
                        $( _super = Some($super::TYPE); )?
                        _super
                    },
                    [ $($(($impl::TYPE, 1),)*)? ],
                    // #[cfg(debug_assertions)]
                    MODULE_PATH,
                    // #[cfg(debug_assertions)]
                    stringify!($class),
                ).as_type();

                type Data = $class;
                type Opt = $class;
                type DebugVtableLayout<'a> = $class;

                fn debug_vtable_layout(&self, _offset: usize) -> Self::DebugVtableLayout<'_> {
                    $class
                }
            }
            unsafe impl ClassVtable for $class {}
            impl Hash for $class {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    ::core::ptr::from_ref(self).hash(state);
                }
            }
            impl ClassVtableOpt for $class {
                type VtableBase = $class;
                type Vtable = $class;
            }
        };
    }

    def_class!(I);
    def_class!(I1 extends I);
    def_class!(I2);
    def_class!(A implements I1);
    def_class!(B extends A);
    def_class!(C extends B implements I1, I2);
    def_class!(D implements A);

    #[test]
    fn test_is_instance_of() {
        macro_rules! test_subtype_of {
            ($a:ident <: $b:ident) => {
                assert!($a::TYPE.is_subtype_of($b::TYPE));
                // #[cfg(debug_assertions)]
                const _: () = assert!($a::TYPE.const_is_subtype_of($b::TYPE));
            };
            ($a:ident !<: $b:ident) => {
                assert!($a::TYPE.is_subtype_of($b::TYPE).not());
                // #[cfg(debug_assertions)]
                const _: () = assert!(!$a::TYPE.const_is_subtype_of($b::TYPE));
            };
        }
        test_subtype_of!(I1 <: I);
        test_subtype_of!(I2 !<: I);
        test_subtype_of!(A <: A);
        test_subtype_of!(A <: I1);
        test_subtype_of!(A !<: I2);
        test_subtype_of!(B <: A);
        test_subtype_of!(B <: B);
        test_subtype_of!(C <: A);
        test_subtype_of!(C <: B);
        test_subtype_of!(C <: C);
        test_subtype_of!(C <: I1);
        test_subtype_of!(C <: I2);
        test_subtype_of!(C <: I);
        test_subtype_of!(D <: A);
        test_subtype_of!(D !<: B);
        test_subtype_of!(D !<: C);
        test_subtype_of!(D <: D);
        test_subtype_of!(D <: I1);
        test_subtype_of!(D !<: I2);
        test_subtype_of!(D <: I);
    }

    #[test]
    fn test_is_subclass_of() {
        macro_rules! test_subclass_of {
            ($a:ident <: $b:ident) => {
                assert!($a::TYPE.is_subclass_of($b::TYPE));
                // #[cfg(debug_assertions)]
                const _: () = assert!($a::TYPE.const_is_subclass_of($b::TYPE));
            };
            ($a:ident !<: $b:ident) => {
                assert!($a::TYPE.is_subclass_of($b::TYPE).not());
                // #[cfg(debug_assertions)]
                const _: () = assert!(!$a::TYPE.const_is_subclass_of($b::TYPE));
            };
        }
        test_subclass_of!(I1 <: I);
        test_subclass_of!(I2 !<: I);
        test_subclass_of!(A <: A);
        test_subclass_of!(A !<: I1);
        test_subclass_of!(A !<: I2);
        test_subclass_of!(B <: A);
        test_subclass_of!(B <: B);
        test_subclass_of!(C <: A);
        test_subclass_of!(C <: B);
        test_subclass_of!(C <: C);
        test_subclass_of!(C !<: I1);
        test_subclass_of!(C !<: I2);
        test_subclass_of!(C !<: I);
        test_subclass_of!(D !<: A);
        test_subclass_of!(D !<: B);
        test_subclass_of!(D !<: C);
        test_subclass_of!(D <: D);
        test_subclass_of!(D !<: I1);
        test_subclass_of!(D !<: I2);
        test_subclass_of!(D !<: I);
    }

    fn hash<T: Hash>(t: &T) -> u64 {
        core::hash::BuildHasherDefault::<std::hash::DefaultHasher>::new().hash_one(t)
    }

    // test eqality in a table-like way
    // a === b  ->  hash(a) == hash(b) && a == b
    // a  == b  ->  hash(a) == hash(b)
    // a !== b  ->  hash(a) != hash(b) && a != b
    // a  != b  ->  hash(a) != hash(b)
    macro_rules! eq_tests {
        (
            (() $($items:ident)*)
            $( ($head:ident $($eqs:tt)*) )*
        ) => {
            eq_tests! {
                (@ $($items)* )
                (() $($items)* )
                $( ( $head $($eqs)* ) )*
            }
        };
        (
            (@ $($saved_items:ident)* )
            (() $($items:ident)*)
        ) => {};
        (
            (@ $($items:ident)* )
            (() )
            ($cur:ident )
            $( $rows:tt )*
        ) => {
            eq_tests! {
                (@ $($items)* )
                (() $($items)* )
                $( $rows )*
            }
        };
        (
            (@           $($items:ident)* )
            (()          $cur:ident $($next:ident)* )
            ($head:ident (==)       $($eqs:tt)* )
            $( $rows:tt )*
        ) => {
            // println!("{} == {}", stringify!($head), stringify!($cur));
            assert!(hash(&$head) == hash(&$cur));
            eq_tests! {
                (@ $($items)* )
                (() $($next)* )
                ($head $($eqs)* )
                $( $rows )*
            }
        };
        (
            (@           $($items:ident)* )
            (()          $cur:ident $($next:ident)* )
            ($head:ident (!=)       $($eqs:tt)* )
            $( $rows:tt )*
        ) => {
            // println!("{} != {}", stringify!($head), stringify!($cur));
            assert!(hash(&$head) != hash(&$cur));
            eq_tests! {
                (@ $($items)* )
                (() $($next)* )
                ($head $($eqs)* )
                $( $rows )*
            }
        };
        (
            (@           $($items:ident)* )
            (()          $cur:ident $($next:ident)* )
            ($head:ident (===)       $($eqs:tt)* )
            $( $rows:tt )*
        ) => {
            // println!("{} === {}", stringify!($head), stringify!($cur));
            assert!($head == $cur);
            assert!(hash(&$head) == hash(&$cur));
            eq_tests! {
                (@ $($items)* )
                (() $($next)* )
                ($head $($eqs)* )
                $( $rows )*
            }
        };
        (
            (@           $($items:ident)* )
            (()          $cur:ident $($next:ident)* )
            ($head:ident (!==)       $($eqs:tt)* )
            $( $rows:tt )*
        ) => {
            // println!("{} !== {}", stringify!($head), stringify!($cur));
            assert!($head != $cur);
            assert!(hash(&$head) != hash(&$cur));
            eq_tests! {
                (@ $($items)* )
                (() $($next)* )
                ($head $($eqs)* )
                $( $rows )*
            }
        };
    }

    classes_macros::classes! {
        class Cls { pub fn new() -> Self { Self {} } }
        class Interface { pub fn new() -> Self { Self {} } }
        class MyClassExt extends Cls implements Interface {
            pub fn new() -> Self {
                Self { super: Super::new() }
            }
        }
        class ClsWithData implements EqHash {
            struct { x: u32 }

            pub fn new(x: u32) -> Self {
                Self { x }
            }

            pub override fn EqHash::eq(&self, other: &CRc<Object>) -> bool {
                let Some(other) = other.try_as_subclass::<CRc<Self>>() else { return false };
                self.get_x() == other.get_x()
            }

            pub override fn EqHash::hash(&self, state: &mut dyn Hasher) {
                self.get_x().dyn_hash(state);
            }
        }
    }

    #[test]
    fn test_eq_and_hash() {
        let c = Cls::new();
        let cw = Cls::downgrade(&c);

        let ce = MyClassExt::new();
        let cew = MyClassExt::downgrade(&ce);

        let ces = ce.to_super();
        let cesw = Cls::downgrade(&ces);

        let cei = ce.to_impl();
        let ceiw = Interface::downgrade(&cei);

        let i = Interface::new();
        let iw = Interface::downgrade(&i);

        eq_tests! {
            (()   c     cw   ce    cew   ces  cesw cei   ceiw  i    iw  )
            (c    (===) (==) (!==) (!=) (!==) (!=) (!==) (!=) (!==) (!=))
            (cw   (===) (==) (!==) (!=) (!==) (!=) (!==) (!=) (!==) (!=))
            (ce   (!==) (!=) (===) (==) (===) (==) (===) (==) (!==) (!=))
            (cew  (!==) (!=) (===) (==) (===) (==) (===) (==) (!==) (!=))
            (ces  (!==) (!=) (===) (==) (===) (==) (===) (==) (!==) (!=))
            (cesw (!==) (!=) (===) (==) (===) (==) (===) (==) (!==) (!=))
            (cei  (!==) (!=) (===) (==) (===) (==) (===) (==) (!==) (!=))
            (ceiw (!==) (!=) (===) (==) (===) (==) (===) (==) (!==) (!=))
            (i    (!==) (!=) (!==) (!=) (!==) (!=) (!==) (!=) (===) (==))
            (iw   (!==) (!=) (!==) (!=) (!==) (!=) (!==) (!=) (===) (==))
        }

        let d1_1 = ClsWithData::new(1);
        let d1_2 = ClsWithData::new(1);
        let d1o1 = ClsWithData::new(1).into_super();
        let d1o2 = ClsWithData::new(1).into_super();
        let d2_1 = ClsWithData::new(2);
        let d2_2 = ClsWithData::new(2);
        let d2o1 = ClsWithData::new(2).into_super();
        let d2o2 = ClsWithData::new(2).into_super();

        eq_tests! {
            (()   d1_1  d1_2  d1o1  d1o2  d2_1   d2_2  d2o1 d2o2 )
            (d1_1 (===) (===) (== ) (== ) (!==) (!==) (!= ) (!= ))
            (d1_2 (===) (===) (== ) (== ) (!==) (!==) (!= ) (!= ))
            (d1o1 (== ) (== ) (===) (===) (!= ) (!= ) (!==) (!==))
            (d1o2 (== ) (== ) (===) (===) (!= ) (!= ) (!==) (!==))
            (d2_1 (!==) (!==) (!= ) (!= ) (===) (===) (== ) (== ))
            (d2_2 (!==) (!==) (!= ) (!= ) (===) (===) (== ) (== ))
            (d2o1 (!= ) (!= ) (!==) (!==) (== ) (== ) (===) (===))
            (d2o2 (!= ) (!= ) (!==) (!==) (== ) (== ) (===) (===))
        }
    }
}
