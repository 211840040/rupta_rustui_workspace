use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

classes! {
    abstract class Bark {
        pub fn bark(&self);
    }

    class Dog implements Bark {
        pub fn new() -> Self {
            Self {}
        }

        pub override fn bark(&self) {
            printlntb!("Dog::bark");
        }
    }
}

static EXPECTED: &[&str] = &["Dog::bark"];

#[test]
fn override_method() {
    let dog = Dog::new();
    dog.bark();
    assert_eq!(BUF.take(), EXPECTED);
}
