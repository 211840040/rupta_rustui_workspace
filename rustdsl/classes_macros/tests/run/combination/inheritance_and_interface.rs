// Description: Inheritance and multiple interfaces

use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;
classes! {
    abstract class Play {
        pub fn play(&self);
    }

    abstract class Sleep {
        pub fn sleep(&self);
    }

    class Animal {
        pub fn new() -> Self {
            Self {}
        }
        pub fn drink(&self) {
            printlntb!("Animal::drink");
        }
        pub fn eat(&self) {
            printlntb!("Animal::eat");
        }
    }

    class Dog extends Animal implements Play, Sleep {
        pub fn new() -> Self {
            Self { super: Super::new() }
        }

        pub override fn Play::play(&self) {
            printlntb!("Dog::play");
        }

        pub override fn Sleep::sleep(&self) {
            printlntb!("Dog::sleep");
        }

        pub override fn eat(&self) {
            printlntb!("Dog::eat");
        }
    }
}

static EXPECTED: &[&str] = &["Animal::drink", "Dog::eat", "Dog::play", "Dog::sleep"];

#[test]
fn inheritance_and_interface() {
    let dog = Dog::new();
    dog.drink();
    dog.eat();
    dog.play();
    dog.sleep();
    assert_eq!(BUF.take(), EXPECTED);
}
