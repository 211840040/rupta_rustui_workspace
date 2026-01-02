// Description: Override constructor
// FIXME: Compilation succeeds

use classes_macros::classes;

classes! {
    abstract class Interface {
        fn plus(&self);
    }

    class A implements Interface {
        struct {
            mutable id: usize,
        }

        pub override fn new(id: usize) -> Self {
            Self { id }
        }

        pub override fn plus(&self) {
            let mut id = self.get_mut_id();
            *id += 1;
        }
    }
}

fn main() {}