//! Get and set traits for interior mutable fields.

use alloc::rc::Rc;
use core::cell::{Cell, OnceCell};
use core::hash::{BuildHasher, Hash};

use rpds::HashTrieMap;

use crate::class::ClassRcWeak;
use crate::prelude::{RcLike, RcLikeElement};

/// Getter and setter trait for default, `late` and `late final` fields with `impl Copy` types.
pub trait GetSetCopy: Copy + Sized {
    /// Getter for a default field
    #[inline]
    fn cell_get(this: &Cell<Self>) -> Self {
        this.get()
    }
    /// Getter for a `late` field
    ///
    /// # Panics
    /// Panics if the field is not initialized.
    #[inline]
    #[track_caller]
    fn cell_option_get(this: &Cell<Option<Self>>) -> Self {
        this.get().unwrap_or_else(late_field_not_initialized)
    }
    /// Getter or initializer for a `late` field
    #[inline]
    #[track_caller]
    fn cell_option_get_or_init_with(this: &Cell<Option<Self>>, f: impl FnOnce() -> Self) -> Self {
        *unsafe { &mut *this.as_ptr() }.get_or_insert_with(f)
    }

    /// Setter for a default field
    #[inline]
    fn cell_set(this: &Cell<Self>, value: Self) {
        this.set(value);
    }
    /// Setter for a default or `late` field
    #[inline]
    fn cell_option_set(this: &Cell<Option<Self>>, value: Self) {
        this.set(Some(value));
    }
}

/// Getter and setter trait for default, `late` and `late final` fields with class or `Rc` types.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not `Copy`, consider adding a `final` or a `mutable` modifier to the field",
    note = "or use `RcLike` to wrap the field if it is `Clone` and behaves like a `Rc`"
)]
pub unsafe trait GetSet: Sized {
    type Get;
    type OptionGet;
    type Set;

    /// Getter for a default field
    fn cell_get(this: &Cell<Self>) -> Self::Get;
    /// Falliable getter for a `late` field
    fn try_cell_option_get(this: &Cell<Option<Self>>) -> Option<Self::OptionGet>;
    /// Getter for a `late` field
    ///
    /// # Panics
    /// Panics if the field is not initialized.
    #[inline]
    #[track_caller]
    fn cell_option_get(this: &Cell<Option<Self>>) -> Self::OptionGet {
        Self::try_cell_option_get(this).unwrap_or_else(late_field_not_initialized)
    }
    /// Getter or initializer for a `late` field
    fn cell_option_get_or_init_with(
        this: &Cell<Option<Self>>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet;

    /// Fallible getter for a `late final` field
    fn try_once_cell_get(this: &OnceCell<Self>) -> Option<Self::OptionGet>;
    /// Getter for a `late final` field
    ///
    /// # Panics
    /// Panics if the field is not initialized.
    #[inline]
    #[track_caller]
    fn once_cell_get(this: &OnceCell<Self>) -> Self::OptionGet {
        Self::try_once_cell_get(this).unwrap_or_else(late_final_field_not_initialized)
    }
    /// Getter or initializer for a `late final` field
    ///
    /// # Panics
    /// Panics if the field is failed to upgrade.
    fn once_cell_get_or_init_with(
        this: &OnceCell<Self>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet;

    fn from_set(value: Self::Set) -> Self;

    /// Setter for a default field
    #[inline]
    fn cell_set(this: &Cell<Self>, value: Self::Set) {
        this.set(Self::from_set(value));
    }
    /// Setter for a `late` field
    #[inline]
    fn cell_option_set(this: &Cell<Option<Self>>, value: Self::Set) {
        this.set(Some(Self::from_set(value)));
    }
    /// Setter for a `late final` field
    ///
    /// # Panics
    /// Panics if the field is already initialized.
    #[inline]
    #[track_caller]
    fn once_cell_set(this: &OnceCell<Self>, value: Self::Set) {
        this.set(Self::from_set(value))
            .unwrap_or_else(late_final_field_already_initialized);
    }
}

pub trait NewCopy: GetSetCopy {
    /// Create a default field with an `impl Copy` type
    fn new_cell(value: impl Into<Self>) -> Cell<Self> {
        Cell::new(value.into())
    }
}

pub trait GetSetOnce: Sized {
    /// Getter for a `late final` field
    ///
    /// # Panics
    /// Panics if the field is not initialized.
    #[inline]
    #[track_caller]
    fn get(this: &OnceCell<Self>) -> &Self {
        this.get().unwrap_or_else(late_final_field_not_initialized)
    }

    /// Getter or initializer for a `late final` field
    #[inline]
    fn get_or_init_with(this: &OnceCell<Self>, f: impl FnOnce() -> Self) -> &Self {
        this.get_or_init(f)
    }

    /// Setter for a `late final` field
    ///
    /// # Panics
    /// Panics if the field is already initialized.
    #[inline]
    #[track_caller]
    fn set(this: &OnceCell<Self>, value: Self) {
        this.set(value)
            .unwrap_or_else(late_final_field_already_initialized);
    }
}

pub trait NewOnce: GetSetOnce {
    /// Create a `late final` field with an `impl Copy` type
    fn new(value: impl Into<Self>) -> OnceCell<Self> {
        let cell = OnceCell::new();
        let result = cell.set(value.into());
        unsafe { result.unwrap_unchecked() };
        cell
    }
}

impl<T: Copy> NewCopy for T {}
impl<T: Copy> GetSetCopy for T {}
impl<T> NewOnce for T {}
impl<T> GetSetOnce for T {}

pub trait New: GetSet {
    /// Create a default field
    fn new_cell(value: impl Into<Self::Set>) -> Cell<Self> {
        Cell::new(Self::from_set(value.into()))
    }
    /// Create a `late` field
    fn new_cell_option(value: impl Into<Self::Set>) -> Cell<Option<Self>> {
        Cell::new(Some(Self::from_set(value.into())))
    }
    /// Create a `late final` field
    fn new_once_cell(value: impl Into<Self::Set>) -> OnceCell<Self> {
        let cell = OnceCell::new();
        let result = cell.set(Self::from_set(value.into()));
        unsafe { result.unwrap_unchecked() };
        cell
    }
}

impl<T: GetSet> New for T {}

#[cold]
#[track_caller]
#[inline(never)]
fn late_field_not_initialized<T>() -> T {
    panic!("`late` field not initialized")
}

#[cold]
#[track_caller]
#[inline(never)]
fn late_field_failed_to_upgrade<T>() -> T {
    panic!("`late` field failed to upgrade")
}

#[cold]
#[track_caller]
#[inline(never)]
fn late_final_field_failed_to_upgrade<T>() -> T {
    panic!("`late final` field failed to upgrade")
}

#[cold]
#[track_caller]
#[inline(never)]
fn late_final_field_not_initialized<T>() -> T {
    panic!("`late final` field not initialized")
}

#[cold]
#[track_caller]
#[inline(never)]
fn late_final_field_already_initialized<T>(_: T) {
    panic!("`late final` field already initialized")
}

unsafe impl<T: ClassRcWeak> GetSet for T {
    type Get = T::Upgraded;
    type OptionGet = T::UpgradedOpt;
    type Set = T::DowngradeFrom;

    #[inline]
    fn cell_get(this: &Cell<Self>) -> Self::Get {
        T::upgrade(unsafe { &*this.as_ptr() })
    }

    #[inline]
    fn try_cell_option_get(this: &Cell<Option<Self>>) -> Option<Self::OptionGet> {
        T::upgrade_opt(unsafe { &*this.as_ptr() }.as_ref())
    }

    #[inline]
    fn try_once_cell_get(this: &OnceCell<Self>) -> Option<Self::OptionGet> {
        T::upgrade_opt(this.get())
    }

    #[inline]
    #[track_caller]
    fn cell_option_get_or_init_with(
        this: &Cell<Option<Self>>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        T::upgrade_opt(Some(
            unsafe { &mut *this.as_ptr() }.get_or_insert_with(|| Self::from_set(f())),
        ))
        .unwrap_or_else(late_field_failed_to_upgrade)
    }

    #[inline]
    #[track_caller]
    fn once_cell_get_or_init_with(
        this: &OnceCell<Self>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        T::upgrade_opt(Some(this.get_or_init(|| Self::from_set(f()))))
            .unwrap_or_else(late_final_field_failed_to_upgrade)
    }

    #[inline]
    fn from_set(value: Self::Set) -> Self {
        T::downgrade_from(&value)
    }
}

unsafe impl<T: ClassRcWeak> GetSet for Option<T> {
    type Get = Option<T::UpgradedOpt>;
    type OptionGet = Option<T::UpgradedOpt>;
    type Set = Option<T::DowngradeFrom>;

    #[inline]
    fn cell_get(this: &Cell<Self>) -> Self::Get {
        T::upgrade_opt(unsafe { &*this.as_ptr() }.as_ref())
    }

    #[inline]
    fn try_cell_option_get(this: &Cell<Option<Self>>) -> Option<Self::OptionGet> {
        Some(Self::cell_option_get(this))
    }

    #[inline]
    fn cell_option_get(this: &Cell<Option<Self>>) -> Self::OptionGet {
        T::upgrade_opt(unsafe { &*this.as_ptr() }.as_ref().and_then(Option::as_ref))
    }

    #[inline]
    fn cell_option_get_or_init_with(
        this: &Cell<Option<Self>>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        T::upgrade_opt(
            unsafe { &mut *this.as_ptr() }
                .get_or_insert_with(|| Self::from_set(f()))
                .as_ref(),
        )
    }

    #[inline]
    fn try_once_cell_get(this: &OnceCell<Self>) -> Option<Self::OptionGet> {
        Some(Self::once_cell_get(this))
    }

    #[inline]
    fn once_cell_get(this: &OnceCell<Self>) -> Self::OptionGet {
        T::upgrade_opt(this.get().and_then(Option::as_ref))
    }

    #[inline]
    fn once_cell_get_or_init_with(
        this: &OnceCell<Self>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        T::upgrade_opt(this.get_or_init(|| Self::from_set(f())).as_ref())
    }

    #[inline]
    fn from_set(value: Self::Set) -> Self {
        value.as_ref().map(T::downgrade_from)
    }
}

unsafe impl<T: ?Sized> GetSet for Rc<T> {
    type Get = Rc<T>;
    type OptionGet = Rc<T>;
    type Set = Rc<T>;

    #[inline]
    fn cell_get(this: &Cell<Self>) -> Self::Get {
        Rc::clone(unsafe { &*this.as_ptr() })
    }

    #[inline]
    fn try_cell_option_get(this: &Cell<Option<Self>>) -> Option<Self::OptionGet> {
        unsafe { &*this.as_ptr() }.as_ref().cloned()
    }

    #[inline]
    #[track_caller]
    fn try_once_cell_get(this: &OnceCell<Self>) -> Option<Self::OptionGet> {
        this.get().cloned()
    }

    #[inline]
    fn cell_option_get_or_init_with(
        this: &Cell<Option<Self>>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        unsafe { &mut *this.as_ptr() }
            .get_or_insert_with(|| Self::from_set(f()))
            .clone()
    }

    #[inline]
    fn once_cell_get_or_init_with(
        this: &OnceCell<Self>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        this.get_or_init(|| Self::from_set(f())).clone()
    }

    #[inline]
    fn from_set(value: Self::Set) -> Self {
        value
    }
}

unsafe impl<T: ?Sized> GetSet for Option<Rc<T>> {
    type Get = Option<Rc<T>>;
    type OptionGet = Option<Rc<T>>;
    type Set = Option<Rc<T>>;

    #[inline]
    fn cell_get(this: &Cell<Self>) -> Self::Get {
        unsafe { &*this.as_ptr() }.as_ref().cloned()
    }

    #[inline]
    fn try_cell_option_get(this: &Cell<Option<Self>>) -> Option<Self::OptionGet> {
        Some(Self::cell_option_get(this))
    }

    #[inline]
    fn cell_option_get(this: &Cell<Option<Self>>) -> Self::OptionGet {
        unsafe { &*this.as_ptr() }
            .as_ref()
            .and_then(Option::as_ref)
            .cloned()
    }

    #[inline]
    fn try_once_cell_get(this: &OnceCell<Self>) -> Option<Self::OptionGet> {
        Some(Self::once_cell_get(this))
    }

    #[inline]
    fn once_cell_get(this: &OnceCell<Self>) -> Self::OptionGet {
        this.get().and_then(Option::as_ref).cloned()
    }

    #[inline]
    fn cell_option_get_or_init_with(
        this: &Cell<Option<Self>>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        unsafe { &mut *this.as_ptr() }
            .get_or_insert_with(|| Self::from_set(f()))
            .clone()
    }

    #[inline]
    fn once_cell_get_or_init_with(
        this: &OnceCell<Self>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        this.get_or_init(|| Self::from_set(f())).clone()
    }

    #[inline]
    fn from_set(value: Self::Set) -> Self {
        value
    }
}

unsafe impl<T: ?Sized + RcLikeElement> GetSet for RcLike<T> {
    type Get = T;
    type OptionGet = T;
    type Set = T;

    #[inline]
    fn cell_get(this: &Cell<Self>) -> Self::Get {
        unsafe { &**this.as_ptr() }.clone()
    }

    #[inline]
    fn try_cell_option_get(this: &Cell<Option<Self>>) -> Option<Self::OptionGet> {
        unsafe { &*this.as_ptr() }.clone().map(RcLike::into_inner)
    }

    #[inline]
    fn try_once_cell_get(this: &OnceCell<Self>) -> Option<Self::OptionGet> {
        this.get().cloned().map(RcLike::into_inner)
    }

    #[inline]
    fn cell_option_get_or_init_with(
        this: &Cell<Option<Self>>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        RcLike::into_inner(
            unsafe { &mut *this.as_ptr() }
                .get_or_insert_with(|| Self::from_set(f()))
                .clone(),
        )
    }

    #[inline]
    fn once_cell_get_or_init_with(
        this: &OnceCell<Self>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        RcLike::into_inner(this.get_or_init(|| Self::from_set(f())).clone())
    }

    #[inline]
    fn from_set(value: Self::Set) -> Self {
        RcLike::new(value)
    }
}

unsafe impl<T: ?Sized + RcLikeElement> GetSet for Option<RcLike<T>> {
    type Get = Option<T>;
    type OptionGet = Option<T>;
    type Set = Option<T>;

    #[inline]
    fn cell_get(this: &Cell<Self>) -> Self::Get {
        unsafe { &*this.as_ptr() }
            .as_ref()
            .cloned()
            .map(RcLike::into_inner)
    }

    #[inline]
    fn try_cell_option_get(this: &Cell<Option<Self>>) -> Option<Self::OptionGet> {
        Some(Self::cell_option_get(this))
    }

    fn cell_option_get(this: &Cell<Option<Self>>) -> Self::OptionGet {
        unsafe { &*this.as_ptr() }
            .as_ref()
            .and_then(Option::as_ref)
            .cloned()
            .map(RcLike::into_inner)
    }

    #[inline]
    fn try_once_cell_get(this: &OnceCell<Self>) -> Option<Self::OptionGet> {
        Some(Self::once_cell_get(this))
    }

    #[inline]
    fn once_cell_get(this: &OnceCell<Self>) -> Self::OptionGet {
        this.get()
            .and_then(Option::as_ref)
            .cloned()
            .map(RcLike::into_inner)
    }

    #[inline]
    fn cell_option_get_or_init_with(
        this: &Cell<Option<Self>>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        unsafe { &mut *this.as_ptr() }
            .get_or_insert_with(|| Self::from_set(f()))
            .clone()
            .map(RcLike::into_inner)
    }

    #[inline]
    fn once_cell_get_or_init_with(
        this: &OnceCell<Self>,
        f: impl FnOnce() -> Self::Set,
    ) -> Self::OptionGet {
        this.get_or_init(|| Self::from_set(f()))
            .clone()
            .map(RcLike::into_inner)
    }

    #[inline]
    fn from_set(value: Self::Set) -> Self {
        value.map(RcLike::new)
    }
}

unsafe impl<K: Eq + Hash, V, H: BuildHasher + Clone> RcLikeElement
    for HashTrieMap<K, V, archery::RcK, H>
{
}
