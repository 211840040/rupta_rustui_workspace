#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub use classes_macros::classes;

extern crate self as classes;

mod macros;

pub mod cell;
pub mod class;
pub mod eq_hash;
pub mod fmt;
pub mod get_set;
pub mod object;
pub mod ptr;
pub mod vtable;

pub use alloc::rc::Rc;

//Prelude mod collects type\struct\traits that are commonly used 
//and alias defined in prelude mod provides a convenient way to use these types and traits.
pub mod prelude {
    pub use crate::cell::{TakeCell, TakeRef, TakeRefMut};
    pub use crate::class::{Class, IsClass};
    pub use crate::eq_hash::EqHash;
    pub use crate::fmt::Format;
    pub use crate::object::Object;
    pub use crate::vtable::Type;
    pub use classes_macros::classes;

    pub use core::cell::{Ref, RefMut};

    pub type CPtr<T> = crate::ptr::PtrDyn<<AsClass<T> as crate::class::ClassImpl>::Vtable>;
    pub type CRef<'a, T> = &'a CRc<T>;
    pub type CRc<T> = <AsClass<T> as crate::class::Class>::Rc;
    pub type CWeak<T> = <AsClass<T> as crate::class::Class>::Weak;
    pub type CRcUninit<T> = crate::ptr::RcDynUninit<AsClass<T>>;

    pub unsafe trait RcLikeElement: Clone {}

    unsafe impl<T: RcLikeElement> RcLikeElement for Option<T> {}

    /// A type wrapper that tells the `classes!` macro to treat the type as an `Rc`-like type.
    #[derive(Clone)]
    pub struct RcLike<T: RcLikeElement>(T);

    impl<T: RcLikeElement> RcLike<T> {
        pub const fn new(value: T) -> Self {
            Self(value)
        }
        pub fn into_inner(this: Self) -> T {
            this.0
        }
    }

    impl<T: RcLikeElement> core::ops::Deref for RcLike<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[doc(hidden)]
    pub type CVtable<T> = <AsClass<T> as crate::class::ClassImpl>::Vtable;
    #[doc(hidden)]
    pub type CVtableBase<T> = <AsClass<T> as crate::class::ClassImpl>::VtableBase;
    #[doc(hidden)]
    pub type CVtableOpt<T> = <AsClass<T> as crate::class::ClassImpl>::VtableOpt;
    #[doc(hidden)]
    pub type CData<T> = <AsClass<T> as crate::class::ClassImpl>::Data;
    #[doc(hidden)]
    pub type CDataBase<T> = <AsClass<T> as crate::class::ClassImpl>::DataBase;
    #[doc(hidden)]
    pub type AsClass<T> = <T as crate::class::IsClass>::Class;
}
