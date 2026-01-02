use classes_macros::classes;

classes! {
    class Animal {
        struct {
            id: usize,
        }

        pub fn new(id: usize) -> Self {
            Self { id }
        }
    }

    class Dog extends Animal {
        struct {
            final name: String,
        }

        pub fn new(id: usize, name: String) -> Self {
            let super = Super::new(id);
            Self { super, name }
            //~^ ERROR: expected `:`
        }
    }
}
