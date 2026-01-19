use classes_macros::classes;

classes! {
    class A {
        struct {
            id: usize,
        }

        pub fn new(#[cfg(false)]) -> Self { 
            //~^ ERROR: unexpected end of input, expect param
            Self { id }
        }
    }
}

fn main() {}