use core::hash::Hasher;

use dyn_hash::DynHash;

classes_macros::class! {
    #[no_super]
    pub class Object {
        #[derive(Default)]
        struct {}
        pub fn new() -> Self { Self {} }
    }
}

// provides the default implementation of `EqHash` and `Format`
impl<V> Object<crate::ptr::RcDyn<Object>, V> {
    pub fn eq(&self, other: &CRc<Object>) -> bool {
        CRc::<Self>::as_ptr(self.as_virtual()) == CRc::<Object>::as_ptr(other)
    }
    pub fn hash(&self, state: &mut (dyn Hasher + '_)) {
        CRc::<Self>::as_ptr(self.as_virtual()).dyn_hash(state);
    }
    pub fn fmt_debug(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&CRc::<Self>::as_ptr(self.as_virtual()), f)
    }
}
