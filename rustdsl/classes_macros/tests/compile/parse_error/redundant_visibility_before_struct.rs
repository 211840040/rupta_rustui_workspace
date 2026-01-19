use classes_macros::classes;

classes! {
    class A {
        pub struct {
        //~^ ERROR: redundant visibility before `struct`
            id: usize,
        }

        pub fn new(id: usize) -> Self {
            Self { id }
        }
    }
}

fn main() {}