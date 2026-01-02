use crate::test_utils::{BUF, printlntb};
use ::classes::prelude::CRc;
use classes_macros::classes;

classes! {
    class Animal {
        pub fn new() -> Self {
            Self {}
        }
        pub fn eat(&self) {
            printlntb!("Animal::eat");
        }
    }

    class Dog extends Animal {
        pub fn new() -> Self {
            Self { super: Super::new() }
        }

        pub override fn eat(&self) {
            printlntb!("Dog::eat");
        }

        pub fn bark(&self) {
            printlntb!("Dog::bark");
        }
    }
}

static EXPECTED: &[&str] = &["Dog::eat", "Dog::bark"];

#[test]
fn two_classes_casting() {
    // upcast
    let animal = Dog::new().into_superclass::<CRc<Animal>>();
    animal.eat(); // use the override method
    // downcast
    let dog = animal.as_subclass::<CRc<Dog>>();
    dog.bark();
    assert_eq!(BUF.take(), EXPECTED);
}
