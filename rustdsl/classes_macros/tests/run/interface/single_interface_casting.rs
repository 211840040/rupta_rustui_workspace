use crate::test_utils::{BUF, printlntb};
use ::classes::prelude::*;
use classes_macros::classes;

classes! {
    abstract class Eat {
        pub fn eat(&self);
    }

    class Cat implements Eat {
        pub fn new() -> Self {
            Self {}
        }

        pub override fn Eat::eat(&self) {
            printlntb!("Cat::eat");
        }

        pub fn meow(&self) {
            printlntb!("Cat::meow");
        }
    }
}

static EXPECTED: &[&str] = &["Cat::eat", "Cat::meow"];

#[test]
fn single_interface_casting() {
    let eater = Cat::new().upcast::<CRc<Cat>, CRc<Eat>>();
    eater.eat();
    let cat = eater.downcast::<CRc<Cat>, CRc<Cat>>();
    cat.meow();
    assert_eq!(BUF.take(), EXPECTED);
}
