use core::cell::Cell;
use core::mem::ManuallyDrop;

pub struct TakeCell<T> {
    value: Cell<Option<T>>,
    #[cfg(debug_assertions)]
    borrowed: Cell<bool>,
}

impl<T> Default for TakeCell<T> {
    fn default() -> Self {
        Self::from_option(None)
    }
}

const SIZE_LIMIT: usize = core::mem::size_of::<usize>() * 8;

impl<T> TakeCell<T> {
    pub const fn empty() -> Self {
        Self::from_option(None)
    }
    pub const fn new(value: T) -> Self {
        Self::from_option(Some(value))
    }
    pub const fn from_option(value: Option<T>) -> Self {
        _ = {
            struct S<T>(T);
            impl<T> S<T> {
                const CHECK: () = if core::mem::size_of::<T>() > SIZE_LIMIT {
                    panic!("T is too large to fit in a TakeCell");
                };
            }
            S::<T>::CHECK
        };
        Self {
            value: Cell::new(value),
            #[cfg(debug_assertions)]
            borrowed: Cell::new(false),
        }
    }
    pub fn take(&self) -> Option<T> {
        self.value.take()
    }
    pub fn set(&self, value: Option<T>) {
        self.value.set(value);
    }
    pub fn replace(&self, value: Option<T>) -> Option<T> {
        self.value.replace(value)
    }
    pub fn into_inner(self) -> Option<T> {
        self.value.into_inner()
    }

    pub fn borrow(&self) -> Option<TakeRef<'_, T>> {
        #[cfg(debug_assertions)]
        assert!(!self.borrowed.replace(true), "TakeCell already borrowed");
        let ret = self.value.take().map(|value| TakeRef {
            value: ManuallyDrop::new(value),
            cell: self,
        });
        #[cfg(debug_assertions)]
        if ret.is_none() {
            self.borrowed.set(false);
        }
        ret
    }

    pub fn borrow_mut(&self) -> Option<TakeRefMut<'_, T>> {
        #[cfg(debug_assertions)]
        assert!(!self.borrowed.replace(true), "TakeCell already borrowed");
        let ret = self.value.take().map(|value| TakeRefMut {
            value: ManuallyDrop::new(value),
            cell: self,
        });
        #[cfg(debug_assertions)]
        if ret.is_none() {
            self.borrowed.set(false);
        }
        ret
    }

    pub fn borrow_mut_or_insert_with(&self, f: impl FnOnce() -> T) -> TakeRefMut<'_, T> {
        #[cfg(debug_assertions)]
        assert!(!self.borrowed.replace(true), "TakeCell already borrowed");
        TakeRefMut {
            value: ManuallyDrop::new(self.value.take().unwrap_or_else(f)),
            cell: self,
        }
    }

    pub fn borrow_mut_or_insert(&self, value: T) -> TakeRefMut<'_, T> {
        self.borrow_mut_or_insert_with(|| value)
    }

    pub fn borrow_mut_or_insert_default(&self) -> TakeRefMut<'_, T>
    where
        T: Default,
    {
        self.borrow_mut_or_insert_with(T::default)
    }
}

impl<T> From<Option<T>> for TakeCell<T> {
    fn from(value: Option<T>) -> Self {
        Self {
            value: Cell::new(value),
            #[cfg(debug_assertions)]
            borrowed: Cell::new(false),
        }
    }
}

impl<T> From<T> for TakeCell<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

pub struct TakeRef<'a, T> {
    value: ManuallyDrop<T>,
    cell: &'a TakeCell<T>,
}

impl<T> Drop for TakeRef<'_, T> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        self.cell.borrowed.set(false);
        self.cell
            .set(Some(unsafe { ManuallyDrop::take(&mut self.value) }));
    }
}

impl<T> core::ops::Deref for TakeRef<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub struct TakeRefMut<'a, T> {
    value: ManuallyDrop<T>,
    cell: &'a TakeCell<T>,
}

impl<'a, T> TakeRefMut<'a, T> {
    pub fn take(self) -> T {
        let mut this = ManuallyDrop::new(self);
        unsafe { ManuallyDrop::take(&mut this.value) }
    }
    pub fn replace(&mut self, value: T) -> T {
        core::mem::replace(&mut self.value, value)
    }
}

impl<T> Drop for TakeRefMut<'_, T> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        self.cell.borrowed.set(false);
        self.cell
            .set(Some(unsafe { ManuallyDrop::take(&mut self.value) }));
    }
}

impl<T> core::ops::Deref for TakeRefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> core::ops::DerefMut for TakeRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
