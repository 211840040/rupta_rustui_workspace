// FIXME: Wrong error location

use classes_macros::classes;

classes! {
    class A {
        struct {
            #[cfg(false)]
        }

        pub fn new(id: usize) -> Self {
        //~^ ERROR: expect field
            Self { id }
        }
    }
}

fn main() {}