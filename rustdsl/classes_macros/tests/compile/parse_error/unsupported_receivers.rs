// Description: Invalid/Unsupported receivers

use classes_macros::classes;

classes! {
    class A {
        struct {
            mutable id: usize,
        }

        pub fn new(id: usize) -> Self {
            Self { id }
        }

        pub fn x(self) {} //~ ERROR: unsupported receiver type, supported receiver types are "&self",
        pub fn y(mut self) {} //~ ERROR: unsupported receiver type, supported receiver types are "&self",
        pub fn z(&mut self) {} //~ ERROR: unsupported receiver type, supported receiver types are "&self",
    }
}

fn main() {}