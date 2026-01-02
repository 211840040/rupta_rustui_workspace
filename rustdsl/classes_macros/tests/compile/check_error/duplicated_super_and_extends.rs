// FIXME: The error location contains the entire `classes! {}`
// FIXME: Two useful error messages are reversed

use classes_macros::classes;

classes! {
    class A {
        struct {
            id: usize,
        }

        pub fn new(id: usize) -> Self {
            Self { id }
        }
    }

    class B extends A {
        //~^ ERROR: `super: A` duplicated with `extends A`
        struct {
            super: A,
            //~^ ERROR: `extends A` declared here, consiter remove it
            name: usize, 
        }

        pub fn new(id: usize, name: usize) -> Self {
            Self {
                super: A::new(id),
                name,
            }
        }
    }
}

fn main() {}