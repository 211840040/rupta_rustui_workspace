use classes_macros::classes;

classes! {
    abstract class Interface {
        fn plus(&self);
    }

    class A implements Interface {
        struct {
            mutable id: usize,
        }

        pub fn new(id: usize) -> Self {
            Self { id }
        }

        pub override fn sub(&self) {
            //~^ ERROR[E0609]: no field `sub` on type `&mut _Interface::vtable::opt::Interface`
            //~| ERROR[E0599]: no method named `sub` found for struct `_Interface::Interface<RcDyn<_Interface::Interface>>` in the current scope
            let mut id = self.get_mut_id();
            *id -= 1;
        }

        pub override fn plus(&self) {
            let mut id = self.get_mut_id();
            *id += 1;
        }
    }
}

fn main() {}
