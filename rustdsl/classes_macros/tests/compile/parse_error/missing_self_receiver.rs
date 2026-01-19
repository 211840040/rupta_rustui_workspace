use classes_macros::classes;

classes! {
    class A {
        struct {
            id: usize,
        }

        pub fn new(id: usize) -> Self {
            Self { id }
        }

        pub fn print() {
            //~^ ERROR: expected value, found module `self`
            println!("id: {}", self.id);
                             //^^^^ construct a compile error
        }
    }
}

fn main() {}