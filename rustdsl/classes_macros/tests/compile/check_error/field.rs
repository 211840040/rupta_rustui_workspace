use classes_macros::classes;

classes! {
    class A {
        struct {
            id: usize,
        }

        pub fn new(id: usize) -> Self {
            Self { ida: id }
            //~^ ERROR: no such field `ida`
            //~| ERROR: struct `data::A` has no field named `ida`
        }
    }
}

fn main() {}